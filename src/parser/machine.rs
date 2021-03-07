use crate::{
    lexer::Token,
    parser::{
        ParseResult,
        error::ParseErr,
        states::{State, State1},
        syntax::AST,
    },
};


pub struct Machine {
    state: Box<dyn State>,
    ast: AST,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            ast: AST::new(),
            state: Box::new(State1),
        }
    }

    pub fn to_ast(mut self, tokens: &mut std::slice::Iter<Token>) -> ParseResult {
        //for t in tokens.iter() {
            //self.state = self.state.receive(
                //&mut self.ast,
                //*t
            //)?;
        //}

        if self.state.finishable() {
            return Ok(self.ast);
        }

        Err(ParseErr::IncompleteSequence)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestState1;

    impl State for TestState1 {
        fn receive(&self,
                   _stack: &mut Stack,
                   _tree: &mut AST,
                   _t: Token,
        ) -> Result<Box<dyn State>, ParseError> {
            Ok(Box::new(self.clone()))
        }

        fn finishable(&self) -> bool { true }
    }

    #[derive(Clone)]
    struct TestState2;

    impl State for TestState2 {
        fn receive(&self,
                   _stack: &mut Stack,
                   _tree: &mut AST,
                   _t: Token,
        ) -> Result<Box<dyn State>, ParseError> {
            Ok(Box::new(self.clone()))
        }

        fn finishable(&self) -> bool { false }
    }

    #[test]
    fn test_machine() {
        let seq = TokenSequence::with_tokens(vec![
            Token::Num(42),
            Token::Num(13),
        ]);

        Machine::begin_with(Box::new(TestState1))
            .to_ast(&seq)
            .expect("Result should be an AST");

        assert_eq!(
            Machine::begin_with(Box::new(TestState2))
                .to_ast(&seq)
                .err(),
            Some(ParseError::IncompleteSequence)
        );
    }
}
*/
