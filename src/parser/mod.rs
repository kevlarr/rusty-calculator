pub mod syntax;
mod machine;

use {
    super::{
        lexer::{
            Symbol,
            Token,
            TokenSequence,
        },
        parser::{
            syntax::{
                AST,
                BinaryOp,
                Syntax,
            },
        },
    },
};



/// Attempts to parse a sequence of tokens into an AST
pub fn parse(seq: &TokenSequence) -> Result<AST, ()> {
    if seq.len() == 0 {
        return Ok(AST::new());
    }

    let mut seq = seq.iter().peekable();

    //while let Some(&t) = seq.peek() {
    //}

    Ok(AST::new())
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
                syntax::{
                    AST,
                    BinaryOp,
                    Syntax,
                },
            },
        },
    };

    #[test]
    fn parse_success() {
        use super::{
            AST,
            BinaryOp::*,
            Symbol::*,
            TokenSequence,
            Token::*,
            Syntax::*,
            parse,
        };

        assert_eq!(
            parse(&TokenSequence::new()),
            Ok(AST::new()),
        );

        let assert = |tokens, syntax| assert_eq!(
            parse(&TokenSequence::with_tokens(tokens)),
            Ok(AST::with_syntax(syntax)),
        );

        assert(vec![
            Num(15),
            Sym(Plus),
            Num(20),
        ], Expression(
            Box::new(Literal(15)),
            Add,
            Box::new(Literal(20)),
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
        ], Expression(
            Box::new(Expression(
                Box::new(Literal(1)),
                Add,
                Box::new(Literal(3)),
            )),
            Mul,
            Box::new(Literal(5)),
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
        ], Expression(
            Box::new(Literal(1)),
            Add,
            Box::new(Expression(
                Box::new(Literal(5)),
                Mul,
                Box::new(Expression(
                    Box::new(Literal(2)),
                    Exp,
                    Box::new(Expression(
                        Box::new(Literal(4)),
                        Sub,
                        Box::new(Literal(2)),
                    ))
                )),
            )),
        ));
    }
}
