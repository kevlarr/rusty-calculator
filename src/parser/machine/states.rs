use {
    crate::{
        lexer::{Symbol, Token},
        parser::{
            error::ParseError,
            syntax::{AST, BinaryOp, Fragment},
        },
    },
    super::{
        stack::Stack,
    },
    std::result,
};


type Result = result::Result<Box<dyn State>, ParseError>;


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
    fn receive(&self,
               stack: &mut Stack,
               tree: &mut AST,
               t: Token,
    ) -> Result;

    fn finishable(&self) -> bool {
        false
    }

    fn finish(&self,
              stack: &mut Stack,
              tree: &mut AST,
    ) -> result::Result<AST, ParseError> {
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
    fn literal(&self, stack: &mut Stack, n: i64) -> Result {
        stack.push(Fragment::Literal(n));
        Ok(Box::new(State02))
    }

    fn paren_open(&self, stack: &mut Stack, tree: &mut AST) -> Result {
        unimplemented!();
    }

    fn negation(&self, stack: &mut Stack) -> Result {
        unimplemented!();
    }
}

impl State for State01 {
    fn receive(&self,
               stack: &mut Stack,
               tree: &mut AST,
               t: Token,
    ) -> Result {
        match t {
            Token::Num(n) => self.literal(stack, n),
            Token::Sym(Symbol::ParenOpen) => self.paren_open(stack, tree),
            Token::Sym(Symbol::Minus) => self.negation(stack),

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
    fn operation(&self, stack: &mut Stack, s: Symbol) -> Result {
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

        stack.push(Fragment::Op(op));

        Ok(Box::new(State01))
    }
}

impl State for State02 {
    fn finishable(&self) -> bool { true }

    //fn finish(&self)

    fn receive(&self,
               stack: &mut Stack,
               _tree: &mut AST,
               t: Token,
    ) -> Result {
        match t {
            Token::Sym(s) => self.operation(stack, s),

            _ => return Err(ParseError::UnexpectedToken),
        }
    }
}
