use {
    crate::{
        lexer::{Symbol, Token},
        parser::{
            error::ParseError,
            syntax::{AST, BinaryOp, Node},
        },
    },
    super::{
        stack::Stack,
    },
    std::result::Result as StdResult,
};


type Result = StdResult<Box<dyn State>, ParseError>;


// State: 01
// Rules:
//     Num            -> 02
//     Sym(Minus)     -> 03
//     Sym(ParenOpen) -> 01

// State: 02 **Finish
// Rules:
//     Op              -> 01
//     Sym(ParenClose) -> 02

// State: Negation
// Rules:
//     Num       -> 02
//     ParenOpen -> 01

pub trait State {
    fn rule_for(&self,
               stk: &mut Stack,
               tr: &mut AST,
               t: Token,
    ) -> Result;

    fn finishable(&self) -> bool {
        false
    }

    fn finish(&self, _stk: &mut Stack, _tr: &mut AST) -> StdResult<(), ParseError> {
        Err(ParseError::StateNotFinishable)
    }
}

/// This state represents the beginning of an expression,
/// which can occur via a...
///     - literal value
///     - minus sign indicating the negation of a following expression
///     - the beginning of a nested expression via parentheses
pub struct State01;

impl State01 {
    fn literal(&self, stk: &mut Stack, n: i64) -> Result {
        stk.push(Node::Literal(n));
        Ok(Box::new(State02))
    }

    fn paren_open(&self, stk: &mut Stack, tr: &mut AST) -> Result {
        unimplemented!();
    }

    fn negation(&self, stk: &mut Stack) -> Result {
        unimplemented!();
    }
}

impl State for State01 {
    fn rule_for(&self, stk: &mut Stack, tr: &mut AST, t: Token) -> Result {
        match t {
            Token::Num(n) => self.literal(stk, n),
            Token::Sym(Symbol::ParenOpen) => self.paren_open(stk, tr),
            Token::Sym(Symbol::Minus) => self.negation(stk),

            _ => return Err(ParseError::UnexpectedToken),
        }
    }
}

/// This state follows a literal value having been added
/// to the stack.
///
/// This state is finishable because an expression is valid
/// if a literal value was the only token. Additionally, it
/// could also mark the second literal in an expression such as
/// `5 + 2` depending on the values sitting on the stack.
pub struct State02;

impl State02 {
    fn operation(&self, stk: &mut Stack, s: Symbol) -> Result {
        type B = BinaryOp;
        type S = Symbol;

        let op = match s {
            S::Asterisk => B::Mul,
            S::Caret => B::Exp,
            S::ForwardSlash => B::Div,
            S::Minus => B::Sub,
            S::Percent => B::Mod,
            S::Plus => B::Add,

            _ => return Err(ParseError::UnexpectedToken),
        };

        // The exact operands cannot be known yet
        stk.push(
            Node::Expression(
                Box::new(Node::Empty),
                op,
                Box::new(Node::Empty)
            )
        );

        Ok(Box::new(State01))
    }
}

impl State for State02 {
    fn finishable(&self) -> bool { true }

    fn finish(&self, _stk: &mut Stack, _tr: &mut AST) -> StdResult<(), ParseError> {
        unimplemented!();
    }

    fn rule_for(&self, stk: &mut Stack, _tr: &mut AST, t: Token) -> Result {
        match t {
            Token::Sym(s) => self.operation(stk, s),
            _ => return Err(ParseError::UnexpectedToken)
        }
    }
}
