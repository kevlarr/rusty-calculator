pub mod error;
pub mod syntax;
//mod machine;
//mod states;

use std::slice::Iter;
use {
    super::{
        lexer::{Symbol, Token, TokenSequence},
        parser::{
            error::ParseErr,
            //machine::Machine,
            syntax::{AST, Expr, Operation},
        },
    },
};


pub type ParseResult = Result<AST, ParseErr>;


pub fn parse(seq: &TokenSequence) -> Result<Expr, ParseErr> {
    if seq.len() == 0 {
        return Ok(Expr::Empty);
    }

    to_ast(&mut seq.iter())
}

fn to_ast(tokens: &mut Iter<Token>) -> Result<Expr, ParseErr> {
    let mut expr = Expr::Empty;

    type Ex = Expr;
    type Op = Operation;
    type Sy = Symbol;
    type Tk = Token;

    while let Some(&t) = tokens.next() {
        expr = match t {
            Tk::Num(n) => match expr {
                Ex::Empty =>
                    Ex::Literal(n),

                Ex::BinaryOp(lhs, op, rhs) if *lhs != Ex::Empty && *rhs == Ex::Empty =>
                    Ex::BinaryOp(lhs, op, Box::new(Ex::Literal(n))),

                Ex::Negation(expr) if *expr == Ex::Empty =>
                    Ex::Negation(Box::new(Ex::Literal(n))),

                _ => return Err(ParseErr::UnexpectedToken(t)),
            },

            Tk::Sym(Sy::ParenOpen) => {
                let sub_expr = to_ast(tokens)?;

                match expr {
                    Ex::Empty =>
                        sub_expr,

                    Ex::BinaryOp(lhs, op, rhs) if *lhs != Ex::Empty && *rhs == Ex::Empty =>
                        Ex::BinaryOp(lhs, op, Box::new(sub_expr)),

                    Ex::Negation(expr) if *expr == Ex::Empty =>
                        Ex::Negation(Box::new(sub_expr)),

                    _ => return Err(ParseErr::UnexpectedToken(t)),
                }
            },

            Tk::Sym(Sy::ParenClose) => return Ok(expr),

            Tk::Sym(Sy::Minus) => match expr {
                Ex::Empty =>
                    Ex::Negation(Box::new(Ex::Empty)),


                Ex::BinaryOp(lhs, op, rhs) if *lhs != Ex::Empty && *rhs == Ex::Empty =>
                    Ex::BinaryOp(lhs, op, Box::new(Ex::Negation(Box::new(Ex::Empty)))),

                Ex::Literal(n) =>
                    Ex::BinaryOp(Box::new(Ex::Literal(n)), Op::Sub, Box::new(Ex::Empty)),

                _ => return Err(ParseErr::UnexpectedToken(t)),
            },

            // Asterisk | Caret | ForwardSlash | Percent | Plus
            Tk::Sym(s) => match expr {
                Ex::Empty =>
                    return Err(ParseErr::UnexpectedToken(t)),

                e => match Op::from_symbol(s) {
                    Ok(op) =>
                        Ex::BinaryOp(Box::new(e), op, Box::new(Ex::Empty)), 
                    Err(e) =>
                        return Err(ParseErr::GeneralError(e)),
                },
            },
        };
    }

    Ok(expr)
}


#[cfg(test)]
mod tests {
    use {
        crate::{
            lexer::{Symbol, Token, TokenSequence},
            parser::syntax::{Operation, Expr},
        },
    };

    #[test]
    fn parse_success() {
        use self::{
            TokenSequence,
            Operation as Op,
            Symbol as Sy,
            Token as Tk,
            Expr as Ex,
        };
        use super::parse;

        assert_eq!(
            parse(&TokenSequence::new()),
            Ok(Expr::Empty),
        );

        let assert = |tokens, expr| assert_eq!(
            parse(&TokenSequence::with_tokens(tokens)),
            Ok(expr),
        );

        assert(vec![
            Tk::Num(15),
            Tk::Sym(Sy::Plus),
            Tk::Num(20),
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(15)),
            Op::Add,
            Box::new(Ex::Literal(20)),
        ));

        assert(vec![
            // (1 + 3) * 5
            Tk::Sym(Sy::ParenOpen),
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(3),
            Tk::Sym(Sy::ParenClose),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
        ], Expr::BinaryOp(
            Box::new(Expr::BinaryOp(
                Box::new(Expr::Literal(1)),
                Op::Add,
                Box::new(Expr::Literal(3)),
            )),
            Op::Mul,
            Box::new(Expr::Literal(5)),
        ));

        // Test WITHOUT operator precedence
        assert(vec![
            // 1 + ((5 * 2) ^ (4 - 2))
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Sym(Sy::ParenOpen),
            Tk::Sym(Sy::ParenOpen),
            Tk::Num(5),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(2),
            Tk::Sym(Sy::ParenClose),
            Tk::Sym(Sy::Caret),
            Tk::Sym(Sy::ParenOpen),
            Tk::Num(4),
            Tk::Sym(Sy::Minus),
            Tk::Num(2),
            Tk::Sym(Sy::ParenClose),
            Tk::Sym(Sy::ParenClose),
        ], Expr::BinaryOp(
            Box::new(Expr::Literal(1)),
            Op::Add,
            Box::new(Expr::BinaryOp(
                Box::new(Expr::BinaryOp(
                    Box::new(Expr::Literal(5)),
                    Op::Mul,
                    Box::new(Expr::Literal(2)),
                )),
                Op::Exp,
                Box::new(Expr::BinaryOp(
                    Box::new(Expr::Literal(4)),
                    Op::Sub,
                    Box::new(Expr::Literal(2)),
                )),
            )),
        ));

        // Test WITH OPERATOR PRECEDENCE
        //assert(vec![
            //// 1 + 5 * 2 ^ (4 - 2)
            //Tk::Num(1),
            //Tk::Sym(Sy::Plus),
            //Tk::Num(5),
            //Tk::Sym(Sy::Asterisk),
            //Tk::Num(2),
            //Tk::Sym(Sy::Caret),
            //Tk::Sym(Sy::ParenOpen),
            //Tk::Num(4),
            //Tk::Sym(Sy::Minus),
            //Tk::Num(2),
            //Tk::Sym(Sy::ParenClose),
        //], Expr::BinaryOp(
            //Box::new(Expr::Literal(1)),
            //Op::Add,
            //Box::new(Expr::BinaryOp(
                //Box::new(Expr::Literal(5)),
                //Op::Mul,
                //Box::new(Expr::BinaryOp(
                    //Box::new(Expr::Literal(2)),
                    //Op::Exp,
                    //Box::new(Expr::BinaryOp(
                        //Box::new(Expr::Literal(4)),
                        //Op::Sub,
                        //Box::new(Expr::Literal(2)),
                    //))
                //)),
            //)),
        //));
    }
}
