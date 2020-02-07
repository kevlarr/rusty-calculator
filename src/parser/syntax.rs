use super::error::ParseErr;
use crate::lexer::Symbol;

/// The supported binary BinaryOp for building a syntax tree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
}

impl Operation {
    pub fn from_symbol(s: Symbol) -> Result<Self, String> {
        use self::{Operation::*, Symbol::*};

        Ok(match s {
            Asterisk => Mul,
            Caret => Exp,
            FwdSlash => Div,
            Minus => Sub,
            Percent => Mod,
            Plus => Add,

            _ => return Err(format!("Cannot convert symbol '{:?}' to operation", s)),
        })
    }

    pub fn has_precedence_over(&self, other: Operation) -> bool {
        self.precedence() > other.precedence()
    }

    fn precedence(&self) -> usize {
        use self::Operation::*;

        match self {
            Exp => 3,
            Mul | Div | Mod => 2,
            Add | Sub => 1,
        }
    }
}

/// A hierachical syntax element that enables the parsing of expressions
/// that rely on operator precedence rather than parentheses.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOpTree(Expr, Operation, Expr);

impl BinaryOpTree {
    pub fn new(lhs: Expr, op: Operation, rhs: Expr) -> Self {
        Self(lhs, op, rhs)
    }

    /// Traverses down the right-most branch to compare itself against
    /// existing operators, stopping when the new operation no longer has
    /// precedence and restructuring the expression to suit.
    pub fn append_operation(self, op: Operation) -> Result<Expr, ParseErr> {
        Err(ParseErr::GeneralError("Wat".into()))
    }

    /// Traverses down the right-most branch to find an Empty expression
    /// to replace with the new expression.
    pub fn append_expr(mut self, expr: Expr) -> Result<Expr, ParseErr> {
        match self.2 {
            Expr::Empty => {
                self.2 = expr;
                Ok(Expr::OpTree(Box::new(self)))
            }
            Expr::Negation(val) if *val == Expr::Empty => {
                self.2 = Expr::Negation(Box::new(expr));
                Ok(Expr::OpTree(Box::new(self)))
            }

            Expr::OpTree(tree) => Ok(tree.append_expr(expr)?),

            _ => Err(ParseErr::NoEmptyNodeFound),
        }
    }
}

/// The possible syntax tree elements.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Empty,
    OpTree(Box<BinaryOpTree>),
    Literal(i64),
    Negation(Box<Expr>),
    SubExpr(Box<Expr>),
}
