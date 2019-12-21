use {
    crate::{
        lexer::Token,
        parser::{
            error::ParseError,
            syntax::AST,
        },
    },
    super::{
        stack::Stack,
    },
};

pub trait State {
    fn receive(&mut self,
               stack: &mut Stack,
               tree: &mut AST,
               t: Token,
    ) -> Result<Box<dyn State>, ParseError>;

    fn finishable(&self) -> bool;
}

pub struct ExprBegin;

impl State for ExprBegin {
    fn finishable(&self) -> bool {
        false
    }

    fn receive(&mut self,
               stack: &mut Stack,
               tree: &mut AST,
               t: Token,
    ) -> Result<Box<dyn State>, ParseError> {
        //match t {
            //Token::Num(_) => true,
            //Token::Sym(Symbol::ParenOpen) => true,
            //Token::Sym(Symbol::Minus) => true,
            //_ => panic!(),
        //}
        unimplemented!()
    }
}
