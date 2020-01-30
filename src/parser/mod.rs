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
            syntax::{Expr, Operation},
        },
    },
};


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

                Ex::Negation(val) if *val == Ex::Empty =>
                    Ex::Negation(Box::new(Ex::Literal(n))),

                Ex::BinaryOp(lhs, op, rhs) => match *rhs {
                    Ex::Empty =>
                        Ex::BinaryOp(lhs, op, Box::new(Ex::Literal(n))),

                    Ex::Negation(val) if *val == Ex::Empty =>
                        Ex::BinaryOp(lhs, op, Box::new(Ex::Negation(
                           Box::new( Ex::Literal(n))
                        ))),

                    // For operator precedence
                    Ex::Nested(val) => match *val {
                        Ex::BinaryOp(lhs2, op2, rhs2) if *rhs2 == Ex::Empty =>
                            Ex::BinaryOp(lhs, op, Box::new(Ex::BinaryOp(
                                lhs2, op2, Box::new(Ex::Literal(n))
                            ))),

                        _ => {
                            eprintln!("val: {:?}", val);
                            return Err(ParseErr::UnexpectedToken(t));
                        },
                    },

                    _ => {
                        eprintln!("(lhs, rhs): {:?}", (lhs, rhs));
                        return Err(ParseErr::UnexpectedToken(t));
                    },
                },




                //Ex::Negation(expr) if *expr == Ex::Empty =>
                    //Ex::Negation(Box::new(Ex::Literal(n))),

                //Ex::BinaryOp(lhs, op, rhs)

                //Ex::BinaryOp(lhs, op, rhs) if *rhs == Ex::Empty =>
                    //Ex::BinaryOp(Box::new(lhs), op, Box::new(Ex::Literal(n))),


                //Ex::BinaryOp(lhs, op, rhs) => match (*lhs, *rhs) {

                    //(lhs, Ex::Empty) =>
                        //Ex::BinaryOp(Box::new(lhs), op, Box::new(Ex::Literal(n))),

                    //(Ex::Nested(expr), rhs) => match *expr {
                        //Ex::BinaryOp(lhs2, op, rhs2) if 
                    //},


                    // FIXME these need to look for nested
                    //(lhs, Ex::BinaryOp(lhs2, op2, rhs2)) if *rhs2 == Ex::Empty =>
                        //Ex::BinaryOp(
                            //Box::new(lhs),
                            //op,
                            //Box::new(Ex::Nested(
                                //Box::new(Ex::BinaryOp(
                                    //lhs2,
                                    //op2,
                                    //Box::new(Ex::Literal(n)),
                                //)),
                            //)),
                        //),

                    // FIXME these need to look for nested
                    //(Ex::BinaryOp(lhs2, op2, rhs2), rhs) if *rhs2 == Ex::Empty =>
                        //Ex::BinaryOp(
                            //Box::new(Ex::BinaryOp(lhs2, op2, Box::new(Ex::Literal(n)))),
                            //op,
                            //Box::new(rhs)
                        //),

                    //_ => return Err(ParseErr::UnexpectedToken(t)),
                    //(lhs, rhs) => {
                        //eprintln!("(lhs, rhs): {:?}", (lhs, rhs));
                        //return Err(ParseErr::UnexpectedToken(t));
                    //},
                //},

                //Ex::BinaryOp(lhs, op, rhs) if *lhs != Ex::Empty && *rhs == Ex::Empty =>
                    //Ex::BinaryOp(lhs, op, Box::new(Ex::Literal(n))),

                _ => {
                    eprintln!("expr: {:?}", expr);
                    return Err(ParseErr::UnexpectedToken(t));
                },
            },

            Tk::Sym(Sy::ParenOpen) => {
                let sub_expr = to_ast(tokens)?;

                match expr {
                    Ex::Empty =>
                        Ex::Nested(Box::new(sub_expr)),

                    Ex::Negation(expr) if *expr == Ex::Empty =>
                        Ex::Negation(Box::new(Ex::Nested(Box::new(sub_expr)))),

                    Ex::BinaryOp(lhs, op, rhs) if *lhs != Ex::Empty && *rhs == Ex::Empty =>
                        Ex::BinaryOp(
                            lhs,
                            op,
                            Box::new(Ex::Nested(Box::new(sub_expr)))
                        ),

                    _ => {
                        eprintln!("expr: {:?}", expr);
                        return Err(ParseErr::UnexpectedToken(t));
                    },
                }
            },

            Tk::Sym(Sy::ParenClose) => return Ok(expr),

            // Minus is the only operator that's both unary and binary,
            // so it needs its own handling
            Tk::Sym(Sy::Minus) => match expr {
                Ex::Empty =>
                    Ex::Negation(Box::new(Ex::Empty)),

                Ex::BinaryOp(lhs, op, rhs) => match *rhs {
                    Ex::Empty =>
                        Ex::BinaryOp(lhs, op, Box::new(Ex::Negation(Box::new(Ex::Empty)))),

                        // Don't need to check precedence, because subtraction has
                        // precedence over nothing
                        _ => 
                            Ex::BinaryOp(
                                Box::new(Ex::BinaryOp(lhs, op, rhs)),
                                Op::Sub,
                                Box::new(Ex::Empty)
                            )
                },

                Ex::Literal(n) =>
                    Ex::BinaryOp(Box::new(Ex::Literal(n)), Op::Sub, Box::new(Ex::Empty)),

                //_ => return Err(ParseErr::UnexpectedToken(t)),
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

                        _ => if op.has_precedence_over(prior_op) {
                            Ex::BinaryOp(
                                lhs,
                                prior_op,
                                Box::new(Ex::Nested(
                                    Box::new(Ex::BinaryOp(
                                        rhs,
                                        op,
                                        Box::new(Ex::Empty)
                                    )),
                                )),
                            )
                        } else {
                            Ex::BinaryOp(
                                Box::new(Ex::BinaryOp(lhs, prior_op, rhs)),
                                op,
                                Box::new(Ex::Empty)
                            )
                        }
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
            Ex::Nested(Box::new(Ex::Literal(15)))
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
                Ex::Nested(
                    Box::new(Ex::BinaryOp(
                        Box::new(Ex::Literal(15)),
                        *op,
                        Box::new(Ex::Literal(0)),
                    ))
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
            Ex::Nested(
                Box::new(Ex::Negation(
                    Box::new(Ex::Literal(15))
                )),
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

        assert(vec![
            Tk::Sym(Sy::ParenOpen),
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(3),
            Tk::Sym(Sy::ParenClose),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(5),
        ], Ex::BinaryOp(
            Box::new(Ex::Nested(
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(1)),
                    Op::Add,
                    Box::new(Ex::Literal(3)),
                )),
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
            Box::new(Ex::Nested(
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(5)),
                    Op::Add,
                    Box::new(Ex::Literal(2)),
                )),
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
            Box::new(Ex::Nested(
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Nested(
                        Box::new(Ex::BinaryOp(
                            Box::new(Ex::Literal(5)),
                            Op::Mul,
                            Box::new(Ex::Literal(2)),
                        )),
                    )),
                    Op::Exp,
                    Box::new(Ex::Nested(
                        Box::new(Ex::BinaryOp(
                            Box::new(Ex::Literal(4)),
                            Op::Sub,
                            Box::new(Ex::Literal(2)),
                        )),
                    )),
                )),
            )),
        ));
    }

    #[test]
    fn parse_gnarly_thing_with_precedence_and_some_parens() {
        use self::*;

        // Test WITH operator precedence...
        //
        // 1 + 5 * 2 ^ (4 - 2)
        //
        // .. should be => 1 + (5 * (2 ^ (4 - 2)))

        /*
          left: `Err(UnexpectedToken(Sym(ParenOpen)))`,

        BinaryOp
            lhs: Literal(1)
            Add
            rhs: Nested
                BinaryOp
                    BinaryOp
                        Literal(5)
                        Mul
                        Literal(2))
                    Exp
                    Empty
        */


        assert(vec![
            Tk::Num(1),
            Tk::Sym(Sy::Plus),
            Tk::Num(5),
            Tk::Sym(Sy::Asterisk),
            Tk::Num(2),
            Tk::Sym(Sy::Caret),
            Tk::Sym(Sy::ParenOpen),
                Tk::Num(4),
                Tk::Sym(Sy::Minus),
                Tk::Num(2),
            Tk::Sym(Sy::ParenClose),
        ], Ex::BinaryOp(
            Box::new(Ex::Literal(1)),
            Op::Add,
            Box::new(Ex::Nested(
                Box::new(Ex::BinaryOp(
                    Box::new(Ex::Literal(5)),
                    Op::Mul,
                    Box::new(Ex::Nested(
                        Box::new(Ex::BinaryOp(
                            Box::new(Ex::Literal(2)),
                            Op::Exp,
                            Box::new(Ex::Nested(
                                Box::new(Ex::BinaryOp(
                                    Box::new(Ex::Literal(4)),
                                    Op::Sub,
                                    Box::new(Ex::Literal(2)),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ));
    }
}
