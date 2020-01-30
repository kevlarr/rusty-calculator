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

/// The possible syntax tree elements.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Empty,
    BinaryOp(Box<Expr>, Operation, Box<Expr>),
    Literal(i64),
    Negation(Box<Expr>),
    Nested(Box<Expr>),
}
