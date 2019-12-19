use {
    crate::{
        lexer::{
            Symbol,
            Token,
            TokenSequence,
        },
        parser::{
            syntax::{
                AST,
                BinaryOp,
                Syntax,
            },
        },
    },
    std::{
        collections::HashSet,
        error,
        fmt,
    },
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


struct Stack;

impl Stack {
    fn new() -> Self {
        Stack
    }
}




#[derive(Debug, PartialEq)]
pub enum StepError {
    UnexpectedToken,
    IncompleteSequence,
}

impl error::Error for StepError {}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::StepError::*;

        match self {
            UnexpectedToken => write!(f, "A token was unexpected"),
            IncompleteSequence => write!(f, "Another token was expected"),
        }
    }
}

trait Step {
    fn receive(&mut self,
               stack: &mut Stack,
               tree: &mut AST,
               t: Token,
    ) -> Result<Box<dyn Step>, StepError>;

    fn finishable(&self) -> bool;
}

struct ExprBegin;

impl Step for ExprBegin {
    fn finishable(&self) -> bool {
        false
    }

    fn receive(&mut self,
               stack: &mut Stack,
               tree: &mut AST,
               t: Token,
    ) -> Result<Box<dyn Step>, StepError> {
        //match t {
            //Token::Num(_) => true,
            //Token::Sym(Symbol::ParenOpen) => true,
            //Token::Sym(Symbol::Minus) => true,
            //_ => panic!(),
        //}
        unimplemented!()
    }
}

struct Machine {
    stack: Stack,
    step: Box<dyn Step>,
    tree: AST,
}

impl Machine {
    fn begin_with(step: Box<dyn Step>) -> Self {
        Self {
            stack: Stack::new(),
            tree: AST::new(),
            step,
        }
    }

    fn to_ast(mut self, tokens: &TokenSequence) -> Result<AST, StepError> {
        for t in tokens.iter() {
            self.step = self.step.receive(&mut self.stack, &mut self.tree, *t)?;
        }

        if self.step.finishable() {
            return Ok(self.tree);
        }

        Err(StepError::IncompleteSequence)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestStep1;

    impl Step for TestStep1 {
        fn receive(&mut self,
                   _stack: &mut Stack,
                   _tree: &mut AST,
                   _t: Token,
        ) -> Result<Box<dyn Step>, StepError> {
            Ok(Box::new(self.clone()))
        }

        fn finishable(&self) -> bool { true }
    }

    #[derive(Clone)]
    struct TestStep2;

    impl Step for TestStep2 {
        fn receive(&mut self,
                   _stack: &mut Stack,
                   _tree: &mut AST,
                   _t: Token,
        ) -> Result<Box<dyn Step>, StepError> {
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

        Machine::begin_with(Box::new(TestStep1))
            .to_ast(&seq)
            .expect("Result should be an AST");

        assert_eq!(
            Machine::begin_with(Box::new(TestStep2))
                .to_ast(&seq)
                .err(),
            Some(StepError::IncompleteSequence)
        );
    }
}
