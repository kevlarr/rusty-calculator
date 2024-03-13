use crate::{
    parser::error::ParseErr,
    lexer::Token,
    Number,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    //Exp,
    //Mod,
}

impl Operator {
    pub fn from_token(t: Token) -> Result<Self, String> {
        use self::Operator::*;

        Ok(match t {
            Token::Asterisk => Mul,
            Token::FwdSlash => Div,
            Token::Minus => Sub,
            Token::Plus => Add,
            // Token::Caret => Exp,
            // Token::Percent => Mod,

            _ => return Err(format!("Cannot convert token '{:?}' to operator", t)),
        })
    }

    pub fn has_precedence_over(&self, other: Operator) -> bool {
        self.precedence() > other.precedence()
    }

    fn precedence(&self) -> usize {
        use self::Operator::*;

        match self {
            //Exp => 3,
            Mul | Div /* | Mod */ => 2,
            Add | Sub => 1,
        }
    }
}


pub trait Expression : Clone + std::fmt::Debug {
    fn evaluate(&self) -> Number;

    fn append_operator(&self, op: Operator) -> BinaryOperation {
        BinaryOperation(Box::new(self.clone()), op, Box::new(Empty))
    }
}

#[derive(Clone, Debug)]
pub struct BinaryOperation(Box<dyn Expression>, Operator, Box<dyn Expression>);
    
impl BinaryOperation {

    /*
    /// Inspects the right-most branch to determine if and how a Minus token
    /// can be aded, either as a unary or binary operator
    pub fn append_minus(&mut self) -> Result<(), ParseErr> {
        if self.has_empty() {
            self.append_expr(Negation(Empty)))
        } else {
            self.append_operator(Operator::Sub);
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
    */

    /*
    fn has_empty(&self) -> bool {
        match &self.2 {
            Expr::BinOp(tree) => tree.has_empty(),
            Expr::Empty => true,
            _ => false,
        }
    }
    */
}
    
impl Expression for BinaryOperation {
    fn evaluate(&self) -> Number {
        use self::Operator::*;

        let (lhs, rhs) = (self.0.evaluate(), self.2.evaluate());

        match self.1 {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
            //Mod => lhs % rhs,
            //Exp => lhs.powf(rhs),
        }
    }

    /// Traverses down the right-most branch to compare itself against
    /// existing operators, stopping when the new operator no longer has
    /// precedence and restructuring the expression to suit.
    fn append_operator(&self, next_op: Operator) -> Self {

        /*
        if next_op.has_precedence_over(self.1) {
            self.2.append_operator(next_op);
            return;
        }

        if next_op.has_precedence_over(self.1) {
            match &mut self.2 {
                Expr::BinOp(tree) => {
                    tree.append_operator(next_op);
                },
                _ => {
                    // Since new operator takes precedence over existing one,
                    // assuming self is equivalent to "1 + 3" and the incoming
                    // operator is "*", then self should be restructured to
                    // "1 + BinaryOp(3 * empty)"
                    let mut new_rhs = Self(Empty, next_op, Empty);

                    std::mem::swap(&mut self.2, &mut new_rhs.0);
                    self.2 = new_rhs;
                }
            }
            return
        }

        // Existing operator takes precedence, so assuming self is "1 * 3"
        // and incoming operator is "+", self should become
        // "BinaryOp(BinaryOp(1, Mul, 3), Add, Empty)"
        let mut new_lhs = Self(Expr::Empty, self.1, Expr::Empty);

        std::mem::swap(&mut self.0, &mut new_lhs.0);
        std::mem::swap(&mut self.2, &mut new_lhs.2);

        self.0 = Expr::BinOp(Box::new(new_lhs));
        self.1 = next_op;
        */
        self
    }
}

/// An empty, non-evaluatable expression that serves as a useful
/// placeholder during parsing
#[derive(Clone, Debug)]
pub struct Empty; // TODO is this more useful than just None?

impl Expression for Empty {
    fn evaluate(&self) -> Number {
        panic!("Cannot evaluate empty expression");
    }
}

#[derive(Clone, Debug)]
pub struct Literal(pub Number);

impl Expression for Literal {
    fn evaluate(&self) -> Number {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct Negation<T: Expression>(pub T);

impl<T: Expression> Expression for Negation<T> {
    fn evaluate(&self) -> Number {
        -self.0.evaluate()
    }
}

#[derive(Clone, Debug)]
pub struct SubExpression<T: Expression>(pub T);

impl<T: Expression> Expression for SubExpression<T> {
    fn evaluate(&self) -> Number {
        self.0.evaluate()
    }
}
