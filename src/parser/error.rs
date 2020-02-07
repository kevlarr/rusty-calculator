use crate::lexer::Token;
use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum ParseErr {
    IncompleteSequence,
    StateNotFinishable,
    UnexpectedToken(Token),
    NoEmptyNodeFound,
    GeneralError(String),
}

impl error::Error for ParseErr {}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ParseErr::*;

        match self {
            IncompleteSequence => write!(f, "Another token was expected"),
            StateNotFinishable => write!(f, "Current state is not a finish state."),
            UnexpectedToken(t) => write!(f, "Token {:?} was unexpected", t),
            NoEmptyNodeFound => write!(f, "No empty node was found to insert expression"),
            GeneralError(e) => write!(f, "{}", e),
        }
    }
}
