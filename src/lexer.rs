use {
    std::{
        collections::HashMap,
        error,
        fmt},
};


/// The set of available mathematical operations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symbol {
    Asterisk,
    Caret,
    Comma,
    ForwardSlash,
    Minus,
    ParenClose,
    ParenOpen,
    Percent,
    Period,
    Plus,
}


/// The set of possible tokens.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Sym(Symbol),
    Num(i64),
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

/// Represents a character that cannot be tokenized.
#[derive(Debug, PartialEq)]
pub struct InvalidCharacter(pub char);

impl fmt::Display for InvalidCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid character: '{}'", self.0)
    }
}

impl error::Error for InvalidCharacter {}


/// Receives input text and attempts to generate a valid token stream.
pub fn lex(s: &str) -> Result<TokenSequence, InvalidCharacter> {
    use self::Symbol::*;

    let charmap: HashMap<char, Symbol> = vec![
        ('*', Asterisk),
        ('^', Caret),
        (',', Comma),
        ('/', ForwardSlash),
        ('-', Minus),
        (')', ParenClose),
        ('(', ParenOpen),
        ('%', Percent),
        ('.', Period),
        ('+', Plus),
    ].into_iter().collect();

    let mut tokens = TokenSequence::new();
    let mut chars = s.chars().peekable();

    // Each loop through can advance the iterator an arbitrary number of times
    while let Some(_) = chars.peek() {
        let c = match chars.next() {
            Some(chr) => chr,
            None => unreachable!(),
        };

        if c.is_whitespace() { continue; }

        if let Some(symbol) = charmap.get(&c) {
            tokens.add(Token::Sym(symbol.clone()));
            continue;
        }

        if c.is_digit(10) {
            let mut num = c.to_string();
            while let Some(&c2) = chars.peek() {
                if !c2.is_digit(10) { break; }

                num.push(chars.next().unwrap());
            }
            tokens.add(
                Token::Num(num.parse::<i64>().unwrap())
            );
            continue;
        }

        return Err(InvalidCharacter(c));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_success() {
        use super::Token::*;
        use super::Symbol::*;

        let assert = |s: &str, v: Vec<Token>| assert_eq!(
            lex(s).unwrap(),
            TokenSequence(v),
        );

        assert("", vec![]);
        assert("     \t\n    ", vec![]);

        assert("123\t432      ", vec![
            Num(123),
            Num(432),
        ]);

        assert("1234567 7890 5432", vec![
            Num(1234567),
            Num(7890),
            Num(5432),
        ]);

        assert("0 -0", vec![
            Num(0),
            Sym(Minus),
            Num(0),
        ]);

        assert("5 + 4", vec![
            Num(5),
            Sym(Plus),
            Num(4),
        ]);

        assert("5 + 4*(-2/      0)", vec![
            Num(5),
            Sym(Plus),
            Num(4),
            Sym(Asterisk),
            Sym(ParenOpen),
            Sym(Minus),
            Num(2),
            Sym(ForwardSlash),
            Num(0),
            Sym(ParenClose),
        ]);

        assert("*^\n,/-)(%.+", vec![
            Sym(Asterisk),
            Sym(Caret),
            Sym(Comma),
            Sym(ForwardSlash),
            Sym(Minus),
            Sym(ParenClose),
            Sym(ParenOpen),
            Sym(Percent),
            Sym(Period),
            Sym(Plus),
        ]);
    }

    #[test]
    fn test_lex_error() {
        let e = lex("x").err().unwrap();

        assert_eq!(e, InvalidCharacter('x'));
        assert_eq!(e.to_string(), String::from("Invalid character: 'x'"));

        let e = lex("5asdf").err().unwrap();

        assert_eq!(e, InvalidCharacter('a'));
    }
}
