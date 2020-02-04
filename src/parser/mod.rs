pub mod error;
pub mod syntax;
//mod machine;
//mod states;

use std::{iter::Peekable, slice::Iter};
use super::{
    lexer::{Symbol, Token, TokenSequence},
    parser::{
        error::ParseErr,
        //machine::Machine,
        syntax::{Expr, Operation},
    },
};


pub fn parse(seq: &TokenSequence) -> Result<Expr, ParseErr> {
    if seq.len() == 0 {
        return Ok(Expr::Empty);
    }

    to_ast(&mut seq.iter().peekable(), Expr::Empty)
}

fn to_ast(tokens: &mut Peekable<Iter<Token>>, starting: Expr) -> Result<Expr, ParseErr> {
    let mut expr = starting;

    type Ex = Expr;
    type Op = Operation;
    type Sy = Symbol;
    type Tk = Token;

    while let Some(&t) = tokens.next() {
        let next: Option<&Token> = tokens.peek().map(|t| t.clone());

        expr = match t {

            Tk::Sym(Sy::ParenClose) => return Ok(expr),

            Tk::Sym(Sy::ParenOpen) => {
                let sub_expr = to_ast(tokens, Ex::Empty)?;

                match expr {
                    Ex::Empty =>
                        sub_expr,

                    Ex::Negation(expr) if *expr == Ex::Empty =>
                        Ex::Negation(Box::new(sub_expr)),

                    Ex::BinaryOp(lhs, op, rhs) if *rhs == Ex::Empty =>
                        Ex::BinaryOp(lhs, op, Box::new(sub_expr)),

                    _ => {
                        eprintln!("expr: {:?}", expr);
                        return Err(ParseErr::UnexpectedToken(t));
                    },
                }
            },

            Tk::Num(n) => match expr {

                Ex::Empty =>
                    Ex::Literal(n),

                Ex::Negation(val) if *val == Ex::Empty =>
                    Ex::Negation(Box::new(Ex::Literal(n))),

                // If this number is finishing off a binary expression, we need to
                // peek at the next token to see if it's another operator and, if so,
                // if it's of higher precedence than the previous one.
                Ex::BinaryOp(lhs, prior_op, rhs) => match *rhs {
                    Ex::Empty =>
                        check_for_operator(tokens,
                                           lhs,
                                           prior_op,
                                           Ex::Literal(n),
                                           next)?,

                    Ex::Negation(val) if *val == Ex::Empty =>
                        check_for_operator(tokens,
                                           lhs,
                                           prior_op,
                                           Ex::Negation(Box::new(Ex::Literal(n))),
                                           next)?,

                    _ => {
                        eprintln!("(lhs, rhs): {:?}", (lhs, rhs));
                        return Err(ParseErr::UnexpectedToken(t));
                    },
                },

                //Ex::BinaryOp(lhs, prior_op, rhs) => match *rhs {
                    //Ex::Empty =>
                        //Ex::BinaryOp(lhs, prior_op, Box::new(Ex::Literal(n))),

                    //Ex::Negation(val) if *val == Ex::Empty =>
                        //Ex::BinaryOp(
                            //lhs,
                            //prior_op,
                            //Box::new(Ex::Negation(Box::new(Ex::Literal(n))))
                        //),

                    //_ => {
                        //eprintln!("(lhs, rhs): {:?}", (lhs, rhs));
                        //return Err(ParseErr::UnexpectedToken(t));
                    //},
                //},

                _ => {
                    eprintln!("expr: {:?}", expr);
                    return Err(ParseErr::UnexpectedToken(t));
                },
            },


            // Minus is the only operator that's both unary and binary,
            // so it needs its own handling
            Tk::Sym(Sy::Minus) => match expr {
                Ex::Empty =>
                    Ex::Negation(Box::new(Ex::Empty)),

                Ex::Literal(n) =>
                    Ex::BinaryOp(Box::new(Ex::Literal(n)), Op::Sub, Box::new(Ex::Empty)),

                Ex::BinaryOp(lhs, op, rhs) => match *rhs {
                    Ex::Empty =>
                        Ex::BinaryOp(lhs, op, Box::new(Ex::Negation(Box::new(Ex::Empty)))),

                    _ => 
                        Ex::BinaryOp(
                            Box::new(Ex::BinaryOp(lhs, op, rhs)),
                            Op::Sub,
                            Box::new(Ex::Empty)
                        ),
                },

                _ => {
                    eprintln!("expr: {:?}", expr);
                    return Err(ParseErr::UnexpectedToken(t));
                },
            },

            // Asterisk | Caret | FwdSlash | Percent | Plus
            Tk::Sym(s) => match Op::from_symbol(s) {

                Ok(op) => match expr {
                    Ex::Empty =>
                        return Err(ParseErr::UnexpectedToken(t)),

                    Ex::BinaryOp(lhs, prior_op, rhs) => match *rhs {
                        Ex::Empty => return Err(ParseErr::UnexpectedToken(t)),

                        _ =>
                            Ex::BinaryOp(
                                Box::new(Ex::BinaryOp(lhs, prior_op, rhs)),
                                op,
                                Box::new(Ex::Empty)
                            ),
                    },

                    expr =>
                        Ex::BinaryOp(Box::new(expr), op, Box::new(Ex::Empty)),
                },
                Err(e) =>
                    return Err(ParseErr::GeneralError(e)),
            },
        };
    }

    Ok(expr)
}

fn check_for_operator(tokens: &mut Peekable<Iter<Token>>,
                      lhs: Box<Expr>,
                      prior_op: Operation,
                      rhs: Expr,
                      next: Option<&Token>) -> Result<Expr, ParseErr> {
    // If the upcoming token is an operator and if it takes precedence
    // over the previous, then restructure the nodes and run recursively
    if let Some(&Token::Sym(s)) = next {
        if let Ok(next_op) = Operation::from_symbol(s) {
            if next_op.has_precedence_over(prior_op) {
                return Ok(Expr::BinaryOp(
                    lhs,
                    prior_op,
                    Box::new(to_ast(tokens, rhs)?)
                ));
            }
        }
    }

    // Next token is either not an operator or doesn't have precedence,
    // so ignore it and finish the expression
    Ok(Expr::BinaryOp(lhs, prior_op, Box::new(rhs)))
}


#[cfg(test)]
mod tests {
    use super::{
        TokenSequence,
        Symbol as Sy,
        Token as Tk,
        Operation as Op,
        Expr as Ex,
        parse,
    };

    fn assert(tokens: Vec<super::Token>, expr: super::Expr) {
        assert_eq!(
            parse(&TokenSequence::with_tokens(tokens)),
            Ok(expr),
        );
    }

    #[test]
    fn parse_empty() {
        use self::*;

        assert(vec![], Ex::Empty);
    }

    #[test]
    fn parse_literal() {
        use self::*;

        assert(vec![Tk::Num(15)], Ex::Literal(15));
    }

    #[test]
    fn parse_parenthesized_literal() {
        use self::*;

        assert(
            vec![
                Tk::Sym(Sy::ParenOpen),
                Tk::Num(15),
                Tk::Sym(Sy::ParenClose),
            ],
            Ex::Literal(15)
        );
    }

    #[test]
    fn parse_binary_op() {
        use self::*;

        let pairs = [
            (Sy::Plus,     Op::Add),
            (Sy::Minus,    Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret,    Op::Exp),
            (Sy::Percent,  Op::Mod),
        ];

        for (sym, op) in pairs.iter() {
            assert(
                vec![
                    Tk::Num(15),
                    Tk::Sym(*sym),
                    Tk::Num(0),
                ],
                Ex::BinaryOp(
                    Box::new(Ex::Literal(15)),
                    *op,
                    Box::new(Ex::Literal(0)),
                )
            );
        }
    }

    #[test]
    fn parse_parenthesized_binary_op() {
        use self::*;

        let pairs = [
            (Sy::Plus,     Op::Add),
            (Sy::Minus,    Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret,    Op::Exp),
            (Sy::Percent,  Op::Mod),
        ];

        for (sym, op) in pairs.iter() {
            assert(
                vec![
                    Tk::Sym(Sy::ParenOpen),
                    Tk::Num(15),
                    Tk::Sym(*sym),
                    Tk::Num(0),
                    Tk::Sym(Sy::ParenClose),
                ],
                Ex::BinaryOp(
                    Box::new(Ex::Literal(15)),
                    *op,
                    Box::new(Ex::Literal(0)),
                )
            );
        }
    }

    #[test]
    fn parse_negation() {
        use self::*;

        assert(
            vec![
                Tk::Sym(Sy::ParenOpen),
                Tk::Sym(Sy::Minus),
                Tk::Num(15),
                Tk::Sym(Sy::ParenClose),
            ],
            Ex::Negation(
                Box::new(Ex::Literal(15))
            )
        );
    }

    #[test]
    fn parse_parenthesized_negation() {
        use self::*;

        assert(
            vec![
                Tk::Sym(Sy::Minus),
                Tk::Num(15),
            ],
            Ex::Negation(Box::new(Ex::Literal(15))),
        );
    }

    #[test]
    fn parse_multiple_identical_operations_left_associative() {
        use self::*;

        let pairs = [
            (Sy::Plus,     Op::Add),
            (Sy::Minus,    Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret,    Op::Exp),
            (Sy::Percent,  Op::Mod),
        ];

        for (sym, op) in pairs.iter() {
            assert(
                vec![
                    Tk::Num(1),
                    Tk::Sym(*sym),
                    Tk::Num(3),
                    Tk::Sym(*sym),
                    Tk::Num(5),
                ],
                Ex::BinaryOp(
                    Box::new(Ex::BinaryOp(
                        Box::new(Ex::Literal(1)),
                        *op,
                        Box::new(Ex::Literal(3)),
                    )),
                    *op,
                    Box::new(Ex::Literal(5)),
                ),
            );
        }
    }

    #[test]
    fn parse_negation_after_operator() {
        use self::*;

        let pairs = [
            (Sy::Plus,     Op::Add),
            (Sy::Minus,    Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret,    Op::Exp),
            (Sy::Percent,  Op::Mod),
        ];

        for (sym, op) in pairs.iter() {
            assert(
                vec![
                    Tk::Num(1),
                    Tk::Sym(*sym),
                    Tk::Sym(Sy::Minus),
                    Tk::Num(5),
                ],
                Ex::BinaryOp(
                    Box::new(Ex::Literal(1)),
                    *op,
                    Box::new(Ex::Negation(
                        Box::new(Ex::Literal(5))
                    )),
                ),
            );
        }
    }

    #[test]
    fn parse_operator_precedence() {
        use self::*;

        // FIXME check all operator combos

        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(3),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(1)),
            Op::Add,
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(3)),
                Op::Mul,
                Box::new(Ex::Literal(5)),
            )),
        ));
    }

    #[test]
    fn parse_parentheses_before_override_operator_precedence() {
        use self::*;

        // (1 + 3) * 5

        assert(vec![
            Tk::Sym(Sy::ParenOpen),
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(3),
            Tk::Sym(Sy::ParenClose),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
        ], Ex::BinaryOp(
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(1)),
                Op::Add,
                Box::new(Ex::Literal(3)),
            )),
            Op::Mul,
            Box::new(Ex::Literal(5)),
        ));
    }

    #[test]
    fn parse_parentheses_after_override_operator_precedence() {
        use self::*;

        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Asterisk),
            Tk::Sym(Sy::ParenOpen),
                Tk::Num(5),
                Tk::Sym(Sy::Plus),
                Tk::Num(2),
            Tk::Sym(Sy::ParenClose),
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(1)),
            Op::Mul,
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(5)),
                Op::Add,
                Box::new(Ex::Literal(2)),
            )),
        ));
    }

    #[test]
    fn parse_gnarly_thing_with_parens_and_no_precedence() {
        use self::*;

        // Test WITHOUT operator precedence
        // 1 + ((5 * 2) ^ (4 - 2))
        assert(vec![
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
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(1)),
            Op::Add,
            Box::new(Ex::BinaryOp(
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(5)),
                    Op::Mul,
                    Box::new(Ex::Literal(2)),
                )),
                Op::Exp,
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(4)),
                    Op::Sub,
                    Box::new(Ex::Literal(2)),
                )),
            )),
        ));
    }

    #[test]
    fn parse_precedence1() {
        use self::*;

        // 1 * 2 * 5
        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(2),
            Tk::Sym(Sy::Plus),
            Tk::Num(5),
        ], Ex::BinaryOp(
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(1)),
                Op::Mul,
                Box::new(Ex::Literal(2)),
            )),
            Op::Add,
            Box::new(Ex::Literal(5)),
        ));

        // 1 + 2 * 5
        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(2),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(1)),
            Op::Add,
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(2)),
                Op::Mul,
                Box::new(Ex::Literal(5)),
            )),
        ));
    }


    #[test]
    fn parse_precedence2() {
        use self::*;
        /*
            1 * 5 + 4 - 2

            BinaryOp
                BinaryOp
                    BinaryOp
                        Literal(1)
                        Mul
                        Literal(5)
                    Add
                    Literal(4)
                Sub
                Literal(2)
        */
        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
            Tk::Sym(Sy::Plus),
            Tk::Num(4),
            Tk::Sym(Sy::Minus),
            Tk::Num(2),
        ], Ex::BinaryOp(
            Box::new(Ex::BinaryOp(
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(1)),
                    Op::Mul,
                    Box::new(Ex::Literal(5)),
                )),
                Op::Add,
                Box::new(Ex::Literal(4)),
            )),
            Op::Sub,
            Box::new(Ex::Literal(2)),
        ));
    }

    #[test]
    fn parse_precedence3() {
        use self::*;

        // 1 + 5 * 2 ^ 4 - 2

        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(5),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(2),
            Tk::Sym(Sy::Caret),
            Tk::Num(4),
            Tk::Sym(Sy::Minus),
            Tk::Num(2),
        ], Ex::BinaryOp(
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(1)),
                Op::Add,
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(5)),
                    Op::Mul,
                    Box::new(Ex::BinaryOp(
                        Box::new(Ex::Literal(2)),
                        Op::Exp,
                        Box::new(Ex::Literal(4)),
                    ))
                )),
            )),
            Op::Sub,
            Box::new(Ex::Literal(2)),
        ));

        // 1 + 5 * 2 ^ 4 / 2

        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(5),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(2),
            Tk::Sym(Sy::Caret),
            Tk::Num(4),
            Tk::Sym(Sy::Minus),
            Tk::Num(2),
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(1)),
            Op::Add,
            Box::new(Ex::BinaryOp(
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(5)),
                    Op::Mul,
                    Box::new(Ex::BinaryOp(
                        Box::new(Ex::Literal(2)),
                        Op::Exp,
                        Box::new(Ex::Literal(4)),
                    ))
                )),
                Op::Div,
                Box::new(Ex::Literal(2)),
            )),
        ));
    }

    #[test]
    fn parse_precedence4() {
        use self::*;

        // 1 * 5 + 2 ^ 4

        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
            Tk::Sym(Sy::Plus),
            Tk::Num(2),
            Tk::Sym(Sy::Caret),
            Tk::Num(4),
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(1)),
            Op::Add,
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(5)),
                Op::Mul,
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(2)),
                    Op::Exp,
                    Box::new(Ex::Literal(4)),
                )),
            )),
        ));

        // 1 * 5 ^ 2 + 4

        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
            Tk::Sym(Sy::Caret),
            Tk::Num(2),
            Tk::Sym(Sy::Plus),
            Tk::Num(4),
        ], Ex::BinaryOp(
            Box::new(Ex::BinaryOp(
                Box::new(Ex::Literal(1)),
                Op::Mul,
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(5)),
                    Op::Exp,
                    Box::new(Ex::Literal(4)),
                )),
            )),
            Op::Add,
            Box::new(Ex::Literal(4)),
        ));
    }
}
