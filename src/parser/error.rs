use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    IncompleteSequence,
    StateNotFinishable,
    UnexpectedToken,
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ParseError::*;

        match self {
            IncompleteSequence => write!(f, "Another token was expected"),
            StateNotFinishable => write!(f, "Current state is not a finish state."),
            UnexpectedToken => write!(f, "A token was unexpected"),
        }
    }
}
