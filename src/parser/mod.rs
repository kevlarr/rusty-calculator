pub mod syntax;
mod machine;

use {
    self::syntax::Node,
    super::{
        lexer::{
            Symbol,
            Token,
            TokenSequence,
        },
        parser::{
            syntax::Operation,
        },
    },
};



/// Attempts to parse a sequence of tokens into an AST
pub fn parse(seq: &TokenSequence) -> Result<Node, ()> {
    if seq.len() == 0 {
        return Ok(Node::NoOp);
    }

    let mut seq = seq.iter().peekable();

    //while let Some(&t) = seq.peek() {
    //}

    Ok(Node::Val(0))
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            lexer::{
                Symbol,
                Token,
                TokenSequence,
            },
            parser::{
                syntax::Operation,
            },
        },
    };

    #[test]
    fn parse_success() {
        use super::{
            Node::*,
            Operation::*,
            Symbol::*,
            TokenSequence,
            Token::*,
            parse,
        };

        assert_eq!(
            parse(&TokenSequence::new()),
            Ok(NoOp),
        );

        let assert = |tokens, nodes| assert_eq!(
            parse(&TokenSequence::with_tokens(tokens)),
            Ok(nodes),
        );

        assert(vec![
            Num(15),
            Sym(Plus),
            Num(20),
        ], Expr(
            Box::new(Val(15)),
            Add,
            Box::new(Val(20)),
        ));

        assert(vec![
            // (1 + 3) * 5
            Sym(ParenOpen),
            Num(1),
            Sym(Plus),
            Num(3),
            Sym(ParenClose),
            Sym(Asterisk),
            Num(5),
        ], Expr(
            Box::new(Expr(
                Box::new(Val(1)),
                Add,
                Box::new(Val(3)),
            )),
            Mul,
            Box::new(Val(5)),
        ));

        assert(vec![
            // 1 + 5 * 2 ^ (4 - 2)
            Num(1),
            Sym(Plus),
            Num(5),
            Sym(Asterisk),
            Num(2),
            Sym(Caret),
            Sym(ParenOpen),
            Num(4),
            Sym(Minus),
            Num(2),
            Sym(ParenClose),
        ], Expr(
            Box::new(Val(1)),
            Add,
            Box::new(Expr(
                Box::new(Val(5)),
                Mul,
                Box::new(Expr(
                    Box::new(Val(2)),
                    Exp,
                    Box::new(Expr(
                        Box::new(Val(4)),
                        Sub,
                        Box::new(Val(2)),
                    ))
                )),
            )),
        ));
    }
}
