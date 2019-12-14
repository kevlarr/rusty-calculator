use {
    super::lexer::{
        Symbol,
        Token,
        TokenSequence,
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
}

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Val(i64),
    Expr(Box<Node>, Operation, Box<Node>),
    NoOp,
}


/// Attempts to parse a sequence of tokens into an AST
fn parse(seq: &TokenSequence) -> Result<Node, ()> {
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

        assert_eq!(
            parse(&TokenSequence::with_tokens(vec![
                Num(15),
                Sym(Plus),
                Num(20),
            ])),
            Ok(Expr(
                Box::new(Val(15)),
                Add,
                Box::new(Val(20)),
            )),
        );

        assert_eq!(
            parse(&TokenSequence::with_tokens(vec![
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
            ])),
            Ok(Expr(
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
            )),
        );
    }
}
