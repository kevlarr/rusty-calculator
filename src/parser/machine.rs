use {
    crate::{
        lexer::{
            Symbol,
            Token,
            TokenSequence,
        },
        parser::{
            syntax::{
                Node,
                Operation,
            },
        },
    },
    std::{
        collections::HashSet,
        error,
        fmt,
    },
};

struct Stack;

struct Syntax;

#[derive(Debug, PartialEq)]
pub enum StateError {
    UnexpectedToken,
    IncompleteSequence,
}

impl error::Error for StateError {}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::StateError::*;

        match self {
            UnexpectedToken => write!(f, "A token was unexpected"),
            IncompleteSequence => write!(f, "Another token was expected"),
        }
    }
}

trait State {
    fn receive(&mut self,
               stack: &mut Stack,
               syntax: &mut Syntax,
               t: Token,
    ) -> Result<Box<dyn State>, StateError>;

    fn finishable(&self) -> bool;
}

struct ExprBegin;

impl State for ExprBegin {
    fn finishable(&self) -> bool {
        false
    }

    fn receive(&mut self,
               stack: &mut Stack,
               syntax: &mut Syntax,
               t: Token,
    ) -> Result<Box<dyn State>, StateError> {
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
    state: Box<dyn State>,
    stack: Stack,
    syntax: Syntax,
}

impl Machine {
    fn new(state: Box<dyn State>) -> Self {
        Self {
            stack: Stack,
            syntax: Syntax,
            state,
        }
    }

    fn to_syntax(mut self, tokens: &TokenSequence) -> Result<Syntax, StateError> {
        for t in tokens.iter() {
            self.state = self.state.receive(&mut self.stack, &mut self.syntax, *t)?;
        }

        if self.state.finishable() {
            return Ok(self.syntax);
        }

        Err(StateError::IncompleteSequence)
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
                   _syntax: &mut Syntax,
                   _t: Token,
        ) -> Result<Box<dyn State>, StateError> {
            Ok(Box::new(self.clone()))
        }

        fn finishable(&self) -> bool { true }
    }

    #[derive(Clone)]
    struct TestState2;

    impl State for TestState2 {
        fn receive(&mut self,
                   _stack: &mut Stack,
                   _syntax: &mut Syntax,
                   _t: Token,
        ) -> Result<Box<dyn State>, StateError> {
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

        Machine::new(Box::new(TestState1))
            .to_syntax(&seq)
            .expect("Result should be a syntax");

        assert_eq!(
            Machine::new(Box::new(TestState2))
                .to_syntax(&seq)
                .err(),
            Some(StateError::IncompleteSequence)
        );
    }
}







/*
use std::cell::RefCell;

#[derive(Debug)]
struct Rule<'a> {
    next_state: &'a State<'a>,
}

#[derive(Debug)]
struct State<'a> {
    rules: RefCell<Vec<Rule<'a>>>,
}

impl<'a> State<'a> {
    fn new() -> Self {
        State {
            rules: RefCell::new(Vec::new()),
        }
    }

    fn add_rule(&'a self, r: Rule<'a>) {
        self.rules.borrow_mut().push(r);
    }
}

#[derive(Debug)]
struct Machine<'a> {
    states: Vec<&'a State<'a>>,
}

#[cfg(test)]
mod tests {
    use super::{Machine, Rule, State};

    #[test]
    fn testy() {
        let state1 = State::new();
        let state2 = State::new();

        state1.add_rule(Rule {
            next_state: &state2,
        });
        state2.add_rule(Rule {
            next_state: &state1,
        });

        let machine = Machine {
            states: vec![&state1, &state2],
        };
    }
}

*/





/*
struct Rule {
    accepts: Token,
}

struct State {
    rules: Vec<Rule>,
}

fn wat() {
    let expr_start = State {
        rules: vec![
            Rule { accepts: Token
        ],
    };
    let 
}
*/

/*
enum StackItem {
    Value(i64),
    //Negation,
    Op(Operation),
}

/// A simple stack of items to enable the machine to
/// determine precedences and create nested expressions.
/// The top of the stack is to the right.
struct Stack(Vec<StackItem>);

impl Stack {
    fn new() -> Self {
        Stack(Vec::new())
    }
}

struct Rule<T> {
    state: usize,
    next_state: usize,
    //accepts: Token,
    //pop_items: Vec<StackItem>,
    //push_items: Vec<StackItem>,
}

impl<T> Rule<T> {
}

struct Rulebook(Vec<Rule>);

impl Rulebook {
}


pub struct Machine {
    finish_states: HashSet<usize>,
    rules: Rulebook,
    stack: Stack,
    state: usize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            finish_states: {
                let mut hs = HashSet::new();
                hs.insert(2);
                hs
            },
            rules: Rulebook(vec![
                // State: 01
                // Rules:
                //     Num            -> 02 (Literal)
                //     Sym(ParenOpen) -> 04 (ExprStart)
                //     Sym(Minus)     -> 03 (Negation)
                Rule {
                    state: 1,
                    next_state: 2,
                    accepts: Token::Num, // FIXME
                },

                // State: 02 **
                // Stack: ??
                // Rules:
                //     Op              -> 04 (Operation)

                // State: 03
                // Stack: ??
                // Rules:
                //     Num       -> 02 (Literal)
                //     ParenOpen -> 04 (ExprStart)


                // State: 04
                //     This state is similar to 01, except that the
                //     rules expect stack values to be present.
                // Stack: ??
                // Rules:
                //     Num            -> 02 (Literal)
                //     Sym(ParenOpen) -> 04 (ExprStart)
                //     Sym(Minus)     -> 03 (Negation)
                //     Sym(ParenClose) -> 02 (ExprEnd)
            ]),
            stack: Stack::new(),
            state: 1,
        }
    }
}
*/
