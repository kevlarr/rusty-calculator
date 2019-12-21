mod stack;
mod states;

use {
    crate::{
        lexer::{Symbol, Token, TokenSequence},
        parser::{
            error::ParseError,
            syntax::{AST, BinaryOp, Syntax},
        },
    },
    self::{
        stack::Stack,
        states::State,
    },
    std::collections::HashSet,
};


// State: 01
// Rules:
//     Num            -> 02 (Literal)
//     Sym(ParenOpen) -> 04 (ExprStart)
//     Sym(Minus)     -> 03 (Negation)

// State: 02 **
// Rules:
//     Op              -> 04 (Operation)

// State: 03
// Rules:
//     Num       -> 02 (Literal)
//     ParenOpen -> 04 (ExprStart)


// State: 04
//     This state is similar to 01, except that the
//     rules expect stack values to be present.
// Rules:
//     Num            -> 02 (Literal)
//     Sym(ParenOpen) -> 04 (ExprStart)
//     Sym(Minus)     -> 03 (Negation)
//     Sym(ParenClose) -> 02 (ExprEnd)




struct Machine {
    stack: Stack,
    state: Box<dyn State>,
    tree: AST,
}

impl Machine {
    fn begin_with(state: Box<dyn State>) -> Self {
        Self {
            stack: Stack::new(),
            tree: AST::new(),
            state,
        }
    }

    fn to_ast(mut self, tokens: &TokenSequence) -> Result<AST, ParseError> {
        for t in tokens.iter() {
            self.state = self.state.receive(&mut self.stack, &mut self.tree, *t)?;
        }

        if self.state.finishable() {
            return Ok(self.tree);
        }

        Err(ParseError::IncompleteSequence)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestState1;

    impl State for TestState1 {
        fn receive(&mut self,
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
        fn receive(&mut self,
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
