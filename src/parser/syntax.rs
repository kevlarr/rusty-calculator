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
            ForwardSlash => Div,
            Minus => Sub,
            Percent => Mod,
            Plus => Add,

            _ => return Err(format!("Cannot convert symbol '{:?}' to operation", s)),
        })
    }
}

/// The possible syntax tree elements.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Empty,
    BinaryOp(Box<Expr>, Operation, Box<Expr>),
    Negation(Box<Expr>),
    Literal(i64),
}

#[derive(Debug, PartialEq)]
pub struct AST(Expr);

impl AST {
    pub fn new() -> Self {
        AST(Expr::Empty)
    }

    pub fn with_syntax(n: Expr) -> Self {
        AST(n)
    }

    //pub fn root(&self) -> &Expr {
        //&self.0
    //}

    //pub fn evaluate() {
    //}
}
