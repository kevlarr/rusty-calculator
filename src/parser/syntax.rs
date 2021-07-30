use super::error::ParseErr;
use crate::{
    lexer::Token,
    Number,
};

/// The supported binary BinaryOp for building a syntax tree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    //Exp,
    //Mod,
}

impl Operation {
    pub fn from_token(t: Token) -> Result<Self, String> {
        use self::Operation::*;

        Ok(match t {
            Token::Asterisk => Mul,
            Token::FwdSlash => Div,
            Token::Minus => Sub,
            Token::Plus => Add,
            // Token::Caret => Exp,
            // Token::Percent => Mod,

            _ => return Err(format!("Cannot convert token '{:?}' to operation", t)),
        })
    }

    pub fn has_precedence_over(&self, other: Operation) -> bool {
        self.precedence() > other.precedence()
    }

    fn precedence(&self) -> usize {
        use self::Operation::*;

        match self {
            //Exp => 3,
            Mul | Div /* | Mod */ => 2,
            Add | Sub => 1,
        }
    }

    fn evaluate(&self, lhs: &Expr, rhs: &Expr) -> Number {
        use self::Operation::*;

        let (lhs, rhs) = (lhs.evaluate(), rhs.evaluate());

        match self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
            //Mod => lhs % rhs,
            //Exp => lhs.powf(rhs),
        }
    }
}

/// A hierachical syntax element that enables the parsing of expressions
/// that rely on operator precedence rather than parentheses.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOp(Expr, Operation, Expr);

impl BinaryOp {
    pub fn new(lhs: Expr, op: Operation, rhs: Expr) -> Self {
        Self(lhs, op, rhs)
    }

    fn evaluate(&self) -> Number {
        let Self(lhs, op, rhs) = self;

        op.evaluate(lhs, rhs)
    }

    /// Traverses down the right-most branch to compare itself against
    /// existing operators, stopping when the new operation no longer has
    /// precedence and restructuring the expression to suit.
    pub fn append_operation(&mut self, next_op: Operation) {
        if next_op.has_precedence_over(self.1) {
            match &mut self.2 {
                Expr::BinOp(tree) => {
                    tree.append_operation(next_op);
                },
                _ => {
                    // Since new operation takes precedence over existing one,
                    // assuming self is equivalent to "1 + 3" and the incoming
                    // operation is "*", then self should be restructured to
                    // "1 + BinaryOp(3 * empty)"
                    let mut new_rhs = Self(Expr::Empty, next_op, Expr::Empty);

                    std::mem::swap(&mut self.2, &mut new_rhs.0);
                    self.2 = Expr::BinOp(Box::new(new_rhs));
                }
            }
            return
        }

        // Existing operation takes precedence, so assuming self is "1 * 3"
        // and incoming operation is "+", self should become
        // "BinaryOp(1 * 3) + empty"
        let mut new_lhs = Self(Expr::Empty, self.1, Expr::Empty);

        std::mem::swap(&mut self.0, &mut new_lhs.0);
        std::mem::swap(&mut self.2, &mut new_lhs.2);

        self.0 = Expr::BinOp(Box::new(new_lhs));
        self.1 = next_op;
    }

    /// Inspects the right-most branch to determine if and how a Minus token
    /// can be aded, either as a unary or binary operation
    pub fn append_minus(&mut self) -> Result<(), ParseErr> {
        if self.has_empty() {
            self.append_expr(Expr::Negation(Box::new(Expr::Empty)))
        } else {
            self.append_operation(Operation::Sub);
            Ok(())
        }
    }

    /// Traverses down the right-most branch to find an Empty expression
    /// to replace with the new expression.
    pub fn append_expr(&mut self, expr: Expr) -> Result<(), ParseErr> {
        match &mut self.2 {
            Expr::Empty => {
                self.2 = expr;
                Ok(())
            }
            Expr::Negation(val) if **val == Expr::Empty => {
                self.2 = Expr::Negation(Box::new(expr));
                Ok(())
            }

            Expr::BinOp(tree) => {
                Ok(tree.append_expr(expr)?)
            },

            _ => Err(ParseErr::NoEmptyNodeFound),
        }
    }

    fn has_empty(&self) -> bool {
        match &self.2 {
            Expr::BinOp(tree) => tree.has_empty(),
            Expr::Empty => true,
            _ => false,
        }
    }
}

/// The possible syntax tree elements.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Empty,
    BinOp(Box<BinaryOp>),
    Literal(Number),
    Negation(Box<Expr>),
    SubExpr(Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> Number {
        use self::Expr::*;

        match self {
            Empty => panic!("Cannot evaluate empty node"),
            BinOp(binary_op) => binary_op.evaluate(),
            Literal(n) => *n,
            Negation(expr) => -expr.evaluate(),
            SubExpr(expr) => expr.evaluate(),
        }
    }
}
