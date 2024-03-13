pub mod error;
pub mod syntax;

use crate::{
    lexer::Token,
    parser::{
        error::ParseErr,
        syntax::*,
    },
    Number,
};
use std::{iter::Peekable, slice::Iter};

pub fn parse(seq: &Vec<Token>) -> Result<Box<dyn Expression>, ParseErr> {
    if seq.len() == 0 {
        return Ok(Box::new(Empty));
    }

    to_ast(&mut seq.iter().peekable(), Box::new(Empty))
}

fn to_ast(
    tokens: &mut Peekable<Iter<Token>>,
    starting: Box<dyn Expression>,
) -> Result<Box<dyn Expression>, ParseErr> {
    let mut expr = starting;

    Ok(starting)

    /*
    type Op = Operator;
    type Tk = Token;

    while let Some(&t) = tokens.next() {
        expr = match t {
            Tk::ParenClose => return Ok(expr),

            Tk::ParenOpen => {
                let sub_expr = to_ast(tokens, Ex::Empty)?;

                match expr {
                    Ex::Empty => Ex::SubExpr(Box::new(sub_expr)),

                    Ex::Negation(expr) if *expr == Ex::Empty => {
                        Ex::Negation(Box::new(Ex::SubExpr(Box::new(sub_expr))))
                    }

                    Ex::BinOp(mut tree) => tree
                        .append_expr(Ex::SubExpr(Box::new(sub_expr)))
                        .map(|()| Ex::BinOp(tree))?,

                    _ => {
                        eprintln!("expr: {:?}", expr);
                        return Err(ParseErr::UnexpectedToken(t));
                    }
                }
            }

            Tk::Num(n) => match expr {
                Ex::Empty => Ex::Literal(Number::from_int(n)),

                Ex::Negation(val) if *val == Ex::Empty => Ex::Negation(Box::new(Ex::Literal(Number::from_int(n)))),

                Ex::BinOp(mut tree) => tree
                    .append_expr(Ex::Literal(Number::from_int(n)))
                    .map(|()| Ex::BinOp(tree))?,

                _ => {
                    eprintln!("expr: {:?}", expr);
                    return Err(ParseErr::UnexpectedToken(t));
                }
            },

            // Minus is the only operator that's both unary and binary,
            // so it needs some special treatment
            Tk::Minus => match expr {
                Ex::Empty => Ex::Negation(Box::new(Ex::Empty)),

                Ex::Literal(n) => Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(n),
                    Op::Sub,
                    Ex::Empty,
                ))),

                Ex::BinOp(mut tree) => tree
                    .append_minus()
                    .map(|()| Ex::BinOp(tree))?,

                _ => {
                    eprintln!("expr: {:?}", expr);
                    return Err(ParseErr::UnexpectedToken(t));
                }
            },

            // Asterisk | Caret | FwdSlash | Percent | Plus
            tk => match Op::from_token(tk) {
                Ok(op) => match expr {
                    Ex::Empty => return Err(ParseErr::UnexpectedToken(t)),

                    Ex::BinOp(mut tree) => {
                        tree.append_operation(op);
                        Ex::BinOp(tree)
                    },

                    expr => Ex::BinOp(Box::new(BinaryOp::new(expr, op, Ex::Empty))),
                },
                Err(e) => return Err(ParseErr::GeneralError(e)),
            },
        };
    }

    Ok(expr)
    */
}

#[cfg(test)]
mod tests {
    use super::{parse, Expr as Ex, Operation as Op, Symbol as Sy, Token as Tk, BinaryOp};

    fn assert(tokens: Vec<super::Token>, expr: super::Expr) {
        assert_eq!(parse(&TokenSequence::with_tokens(tokens)), Ok(expr),);
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
            vec![Tk::Sym(Sy::ParenOpen), Tk::Num(15), Tk::Sym(Sy::ParenClose)],
            Ex::SubExpr(Box::new(Ex::Literal(15))),
        );
    }

    #[test]
    fn parse_binary_op() {
        use self::*;

        let pairs = [
            (Sy::Plus, Op::Add),
            (Sy::Minus, Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret, Op::Exp),
            (Sy::Percent, Op::Mod),
        ];

        for (sym, op) in pairs.iter() {
            assert(vec![
                Tk::Num(15),
                Tk::Sym(*sym),
                Tk::Num(0)
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::Literal(15),
                *op,
                Ex::Literal(0))
            )));
        }
    }

    #[test]
    fn parse_parenthesized_binary_op() {
        use self::*;

        let pairs = [
            (Sy::Plus, Op::Add),
            (Sy::Minus, Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret, Op::Exp),
            (Sy::Percent, Op::Mod),
        ];

        for (sym, op) in pairs.iter() {
            assert( vec![
                Tk::Sym(Sy::ParenOpen),
                Tk::Num(15),
                Tk::Sym(*sym),
                Tk::Num(0),
                Tk::Sym(Sy::ParenClose),
            ],
            Ex::SubExpr(Box::new(
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(15),
                    *op,
                    Ex::Literal(0),
                )))
            )));
        }
    }

    #[test]
    fn parse_negation() {
        use self::*;

        assert(
            vec![Tk::Sym(Sy::Minus), Tk::Num(15)],
            Ex::Negation(Box::new(Ex::Literal(15))),
        );
    }

    #[test]
    fn parse_negation_with_subexpr() {
        use self::*;

        // -(4 + 2)

        assert(vec![
            Tk::Sym(Sy::Minus),
            Tk::Sym(Sy::ParenOpen),
            Tk::Num(4),
            Tk::Sym(Sy::Plus),
            Tk::Num(2),
            Tk::Sym(Sy::ParenClose),
        ],
        Ex::Negation(Box::new(
            Ex::SubExpr(Box::new(
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(4),
                    Op::Add,
                    Ex::Literal(2),
                )))
            )),
        )));
    }

    #[test]
    fn parse_negation_in_subexpr() {
        use self::*;

        // (-15)

        assert(
            vec![
                Tk::Sym(Sy::ParenOpen),
                Tk::Sym(Sy::Minus),
                Tk::Num(15),
                Tk::Sym(Sy::ParenClose),
            ],
            Ex::SubExpr(Box::new(Ex::Negation(Box::new(Ex::Literal(15))))),
        );
    }

    #[test]
    fn parse_negation_after_operator() {
        use self::*;

        let pairs = [
            (Sy::Plus, Op::Add),
            (Sy::Minus, Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret, Op::Exp),
            (Sy::Percent, Op::Mod),
        ];

        for (sym, op) in pairs.iter() {
            assert(
                vec![
                    Tk::Num(1),
                    Tk::Sym(*sym),
                    Tk::Sym(Sy::Minus),
                    Tk::Num(5)
                ],
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(1),
                    *op,
                    Ex::Negation(Box::new(Ex::Literal(5))),
                )))
            );
        }
    }

    #[test]
    fn parse_multiple_identical_operations_left_associative() {
        use self::*;

        let pairs = [
            (Sy::Plus, Op::Add),
            (Sy::Minus, Op::Sub),
            (Sy::Asterisk, Op::Mul),
            (Sy::FwdSlash, Op::Div),
            (Sy::Caret, Op::Exp),
            (Sy::Percent, Op::Mod),
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
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::Literal(1),
                        *op,
                        Ex::Literal(3),
                    ))),
                    *op,
                    Ex::Literal(5),
                )))
            );
        }
    }

    #[test]
    fn parse_operator_precedence() {
        use self::*;

        // FIXME check all operator combos

        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Plus),
                Tk::Num(3),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(5),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::Literal(1),
                Op::Add,
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(3),
                    Op::Mul,
                    Ex::Literal(5),
                ))),
            )))
        );
    }

    #[test]
    fn parse_parentheses_before_override_operator_precedence() {
        use self::*;

        // (1 + 3) * 5

        assert(
            vec![
                Tk::Sym(Sy::ParenOpen),
                Tk::Num(1),
                Tk::Sym(Sy::Plus),
                Tk::Num(3),
                Tk::Sym(Sy::ParenClose),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(5),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::SubExpr(Box::new(
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::Literal(1),
                        Op::Add,
                        Ex::Literal(3),
                    ))),
                )),
                Op::Mul,
                Ex::Literal(5),
            )))
        );
    }

    #[test]
    fn parse_parentheses_after_override_operator_precedence() {
        use self::*;

        // 1 * (5 + 2)

        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Asterisk),
                Tk::Sym(Sy::ParenOpen),
                Tk::Num(5),
                Tk::Sym(Sy::Plus),
                Tk::Num(2),
                Tk::Sym(Sy::ParenClose),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::Literal(1),
                Op::Mul,
                Ex::SubExpr(Box::new(
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::Literal(5),
                        Op::Add,
                        Ex::Literal(2),
                    )))
                ))
            )))
        );
    }

    #[test]
    fn parse_gnarly_thing_with_parens_and_no_precedence() {
        use self::*;

        // 1 + ((5 * 2) ^ (4 - 2))

        assert(
            vec![
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
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::Literal(1),
                Op::Add,
                Ex::SubExpr(Box::new(
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::SubExpr(Box::new(
                            Ex::BinOp(Box::new(BinaryOp::new(
                                Ex::Literal(5),
                                Op::Mul,
                                Ex::Literal(2),
                            ))),
                        )),
                        Op::Exp,
                        Ex::SubExpr(Box::new(
                            Ex::BinOp(Box::new(BinaryOp::new(
                                Ex::Literal(4),
                                Op::Sub,
                                Ex::Literal(2),
                            ))),
                        )),
                    ))),
                )),
            )))
        );
    }

    #[test]
    fn parse_precedence1() {
        use self::*;

        // 1 * 2 * 5
        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(2),
                Tk::Sym(Sy::Plus),
                Tk::Num(5),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(1),
                    Op::Mul,
                    Ex::Literal(2),
                ))),
                Op::Add,
                Ex::Literal(5),
            )))
        );

        // 1 + 2 * 5
        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Plus),
                Tk::Num(2),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(5),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::Literal(1),
                Op::Add,
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(2),
                    Op::Mul,
                    Ex::Literal(5),
                )))
            )))
        );
    }

    #[test]
    fn parse_precedence2() {
        use self::*;

        // 1 * 5 + 4 - 2

        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(5),
                Tk::Sym(Sy::Plus),
                Tk::Num(4),
                Tk::Sym(Sy::Minus),
                Tk::Num(2),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::Literal(1),
                        Op::Mul,
                        Ex::Literal(5),
                    ))),
                    Op::Add,
                    Ex::Literal(4),
                ))),
                Op::Sub,
                Ex::Literal(2),
            )))
        );
    }

    #[test]
    fn parse_precedence3() {
        use self::*;

        // 1 + 5 * 2 ^ 4 - 2

        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Plus),
                Tk::Num(5),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(2),
                Tk::Sym(Sy::Caret),
                Tk::Num(4),
                Tk::Sym(Sy::Minus),
                Tk::Num(2),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(1),
                    Op::Add,
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::Literal(5),
                        Op::Mul,
                        Ex::BinOp(Box::new(BinaryOp::new(
                            Ex::Literal(2),
                            Op::Exp,
                            Ex::Literal(4),
                        ))),
                    ))),
                ))),
                Op::Sub,
                Ex::Literal(2),
            )))
        );

        // All same except precedence of last operator
        // 1 + 5 * 2 ^ 4 / 2

        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Plus),
                Tk::Num(5),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(2),
                Tk::Sym(Sy::Caret),
                Tk::Num(4),
                Tk::Sym(Sy::FwdSlash),
                Tk::Num(2),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::Literal(1),
                Op::Add,
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::Literal(5),
                        Op::Mul,
                        Ex::BinOp(Box::new(BinaryOp::new(
                            Ex::Literal(2),
                            Op::Exp,
                            Ex::Literal(4),
                        )))
                    ))),
                    Op::Div,
                    Ex::Literal(2),
                )))
            )))
        );
    }

    #[test]
    fn parse_precedence4() {
        use self::*;

        // 1 * 5 + 2 ^ 4

        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(5),
                Tk::Sym(Sy::Plus),
                Tk::Num(2),
                Tk::Sym(Sy::Caret),
                Tk::Num(4),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(1),
                    Op::Mul,
                    Ex::Literal(5),
                ))),
                Op::Add,
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(2),
                    Op::Exp,
                    Ex::Literal(4),
                )))
            )))
        );

        // 1 * 5 ^ 2 + 4

        assert(
            vec![
                Tk::Num(1),
                Tk::Sym(Sy::Asterisk),
                Tk::Num(5),
                Tk::Sym(Sy::Caret),
                Tk::Num(2),
                Tk::Sym(Sy::Plus),
                Tk::Num(4),
            ],
            Ex::BinOp(Box::new(BinaryOp::new(
                Ex::BinOp(Box::new(BinaryOp::new(
                    Ex::Literal(1),
                    Op::Mul,
                    Ex::BinOp(Box::new(BinaryOp::new(
                        Ex::Literal(5),
                        Op::Exp,
                        Ex::Literal(2),
                    ))),
                ))),
                Op::Add,
                Ex::Literal(4),
            )))
        );
    }
}
