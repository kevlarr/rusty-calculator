use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken,
    IncompleteSequence,
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ParseError::*;

        match self {
            UnexpectedToken => write!(f, "A token was unexpected"),
            IncompleteSequence => write!(f, "Another token was expected"),
        }
    }
}
