use crate::{
    lexer::{Symbol, Token},
    parser::{
        ParseErr, ParseResult,
        syntax::{AST, Operation, Node},
    },
};

/*
 * STATES
 *
 * State: 1
 * Rules:
 *      Num            -> 2
 *      Sym(Minus)     -> 3
 *      Sym(ParenOpen) -> 1
 *
 * State: 2 **
 * Rules:
 *      Op              -> 1
 *      Sym(ParenClose) -> 2
 *
 * State: 3
 * Rules:
 *      Num             -> ???
 *      Sym(ParenOpen)  -> 1
 *
 */
type RuleResult = Result<Box<dyn State>, ParseErr>;

pub trait State {
    fn receive(&self, ast: &mut AST, t: Token) -> RuleResult;

    fn finishable(&self) -> bool { false }

    fn finish(&self, ast: &mut AST) -> ParseResult {
        Err(ParseErr::StateNotFinishable)
    }
}


pub struct State1;

impl State1 {

    /// A literal value is received.
    fn literal(&self, ast: &mut AST, n: i64) -> RuleResult {
        match ast {
        }
        // If stack is empty..
        //     .. append literal
        //     stack.push(Node::Literal(n));
        // else..
        //     .. append binary operation
        //
        Ok(Box::new(State2))
    }

    /// An opening parenthesis is encountered.
    fn paren_open(&self, ast: &mut AST) -> RuleResult {
        // TODO: Stack-y things...?
        // Or... should this be starting a new thingy..?
        Ok(Box::new(State1))
    }

    /// A minus sign was encountered
    fn negation(&self, ast: &mut AST) -> RuleResult {
        Ok(Box::new(State3))
    }
}

impl State for State1 {
    fn receive(&self, ast: &mut AST, t: Token) -> RuleResult {
        match t {
            Token::Num(n) => self.literal(ast, n),
            Token::Sym(Symbol::ParenOpen) => self.paren_open(ast),
            Token::Sym(Symbol::Minus) => self.negation(ast),

            _ => return Err(ParseErr::UnexpectedToken(t)),
        }
    }
}

struct State2;

impl State2 {
    fn operation(&self, ast: &mut AST, s: Symbol) -> RuleResult {
        use self::{Operation::*, Symbol::*};

        let op = match s {
            Asterisk => Mul,
            Caret => Exp,
            FwdSlash => Div,
            Minus => Sub,
            Percent => Mod,
            Plus => Add,

            _ => return Err(ParseErr::UnexpectedToken(Token::Sym(s))),
        };

        // The exact operands cannot be known yet
        //stack.push(
            //Node::BinaryOp(
                //Box::new(Node::Empty),
                //op,
                //Box::new(Node::Empty)
            //)
        //);

        Ok(Box::new(State1))
    }
}

impl State for State2 {
    fn finishable(&self) -> bool { true }

    fn receive(&self, ast: &mut AST, t: Token) -> RuleResult {
        match t {
            Token::Sym(s) => self.operation(ast, s),
            _ => return Err(ParseErr::UnexpectedToken(t))
        }
    }

    fn finish(&self, ast: &mut AST) -> ParseResult {
        unimplemented!();
    }
}


struct State3;

impl State3 {
    fn literal(&self, ast: &mut AST, n: i64) -> RuleResult {
        unimplemented!(); 
    }

    fn paren_open(&self, ast: &mut AST) -> RuleResult {
        // TODO
        Ok(Box::new(State1))
    }
}

impl State for State3 {
    fn receive(&self, ast: &mut AST, t: Token) -> RuleResult {
        match t {
            Token::Num(n) => self.literal(ast, n),
            Token::Sym(Symbol::ParenOpen) => self.paren_open(ast),

            _ => return Err(ParseErr::UnexpectedToken(t))
        }
    }
}
