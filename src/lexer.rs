use {
    std::{error, fmt},
};


/// The set of available mathematical operations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}


/// The set of possible tokens.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Op(Operation),
    Num(i64),
    OpenParens,
    CloseParens,
}


/// A sequence of tokens.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenSequence(Vec<Token>);

impl TokenSequence {
    fn new() -> Self {
        TokenSequence(Vec::new())
    }

    fn last(&self) -> Option<&Token> {
        let len = self.0.len();

        if len == 0 { None } else { Some(&self.0[len - 1]) }
    }

    fn add(&mut self, t: Token) {
        self.0.push(t);
    }
}

//impl fmt::Debug for TokenSequence {
    //fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "{}", self.0.map(|&t| format!("{:?}", t)).join(", "))
    //}
//}


/// TODO more descriptive error type
#[derive(Debug)]
pub struct LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There was an error lexing the input")
    }
}

impl error::Error for LexerError {}


/// Receives input text and attempts to generate a valid token stream.
pub fn lex(s: &str) -> Result<TokenSequence, LexerError> {
    let mut tokens = TokenSequence::new();
    let mut chars = s.chars().peekable();

    // Each loop through can advance the iterator an arbitrary number of times
    while let Some(_) = chars.peek() {
        let c = match chars.next() {
            Some(chr) => chr,
            None => unreachable!(),
        };

        if c.is_whitespace() { continue; }

        if c.is_digit(10) {
            let mut num = c.to_string();
            while let Some(&c2) = chars.peek() {
                if !c2.is_digit(10) { break; }

                num.push(chars.next().unwrap());
            }
            tokens.add(
                Token::Num(num.parse::<i64>().unwrap())
            );
        }

        println!("{:?}", c);
    }

    println!("{:?}", tokens);

    //Err(LexerError {})
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        use super::Token::*;

        let assert = |s: &str, v: Vec<Token>| assert_eq!(
            lex(s).unwrap(),
            TokenSequence(v),
        );

        assert("123 432", vec![
            Num(123),
            Num(432),
        ]);

        assert("1234567 7890 5432", vec![
            Num(1234567),
            Num(7890),
            Num(5432),
        ]);
    }
}
