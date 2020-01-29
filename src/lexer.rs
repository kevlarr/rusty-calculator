use {
    std::{
        collections::HashMap,
        error,
        fmt,
        slice,
    },
};


/// The white-listed set of non-digit symbols.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symbol {
    Asterisk,
    Caret,
    ForwardSlash,
    Minus,
    ParenClose,
    ParenOpen,
    Percent,
    Plus,
}

/// The set of possible tokens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Num(i64),
    Sym(Symbol),
}


/// Surprise, a holder of tokens
#[derive(Clone, Debug, PartialEq)]
pub struct TokenSequence(Vec<Token>);

impl TokenSequence {
    pub fn new() -> Self {
        TokenSequence(Vec::new())
    }

    pub fn with_tokens(tokens: Vec<Token>) -> Self {
        TokenSequence(tokens)
    }

    fn last(&self) -> Option<&Token> {
        let len = self.0.len();

        if len == 0 { None } else { Some(&self.0[len - 1]) }
    }

    fn add(&mut self, t: Token) {
        self.0.push(t);
    }

    pub fn iter(&self) -> slice::Iter<Token> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// The set of possible lexer errors.
#[derive(Debug, PartialEq)]
pub enum LexErr {
    InvalidCharacter(char),
    UnexpectedCharacter { position: usize, chr: char },
}
impl fmt::Display for LexErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::LexErr::*;

        match self {
            InvalidCharacter(c) => write!(f, "Invalid character: '{}'", c),
            UnexpectedCharacter { position, chr} => write!(
                f, "Unexpected character at {}: '{}'", position, chr
            ),
        }
    }
}

impl error::Error for LexErr {}

type LexResult = Result<TokenSequence, LexErr>;

/// Receives input text and attempts to generate a valid token stream.
pub fn lex(s: &str) -> LexResult {
    use self::Symbol::*;

    let charmap = map!{
        '*' => Asterisk,
        '^' => Caret,
        '/' => ForwardSlash,
        '-' => Minus,
        ')' => ParenClose,
        '(' => ParenOpen,
        '%' => Percent,
        '+' => Plus
    };

    let mut tokens = TokenSequence::new();
    let mut chars = s.chars().enumerate().peekable();

    while let Some((_i1, c)) = chars.next() {
        if c.is_whitespace() { continue; }

        if let Some(symbol) = charmap.get(&c) {
            tokens.add(Token::Sym(symbol.clone()));
            continue;
        }

        if c.is_digit(10) {
            let mut num = c.to_string();
            let mut comma_last = false;

            while let Some(&(i2, c2)) = chars.peek() {
                if c2 == ',' {
                    if comma_last {
                        return Err(LexErr::UnexpectedCharacter {
                            position: i2 + 1,
                            chr: c2,
                        });
                    }

                    chars.next();
                    comma_last = true;
                    continue;
                }

                if !c2.is_digit(10) { break; }

                num.push(chars.next().unwrap().1);
                comma_last = false;
            }
            tokens.add(
                Token::Num(num.parse::<i64>().unwrap())
            );
            continue;
        }

        return Err(LexErr::InvalidCharacter(c));
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

        assert("5 - -4", vec![
            Num(5),
            Sym(Minus),
            Sym(Minus),
            Num(4),
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

        assert("5 + -12,192,293", vec![
            Num(5),
            Sym(Plus),
            Sym(Minus),
            Num(12_192_293),
        ]);

        assert("*^\n/-)(%+", vec![
            Sym(Asterisk),
            Sym(Caret),
            Sym(ForwardSlash),
            Sym(Minus),
            Sym(ParenClose),
            Sym(ParenOpen),
            Sym(Percent),
            Sym(Plus),
        ]);
    }

    #[test]
    fn test_lex_error() {
        use self::LexErr::*;

        let e = lex("x").err().unwrap();

        assert_eq!(e, InvalidCharacter('x'));
        assert_eq!(e.to_string(), String::from("Invalid character: 'x'"));

        let e = lex("5asdf").err().unwrap();

        assert_eq!(e, InvalidCharacter('a'));

        let e = lex("5 + -12,192,,293").err().unwrap();

        assert_eq!(e, UnexpectedCharacter {
            position: 13,
            chr: ',',
        });
    }
}
