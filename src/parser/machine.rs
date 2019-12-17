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
    },
};


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


// 1 + 5 * 2 ^ (4 - 2)





