use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum DriverError {
    LexerError(lexer::LexingError),
    ParserError,
    AbsolutePathError(String),
}

impl Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DriverError::*;
        match self {
            LexerError(err) => write!(f, "Unable to lex input: {err}"),
            ParserError => write!(f, "Unable to parse tokens"),
            AbsolutePathError(relative_file) => write!(
                f,
                "Unable to canonicalize absolute file path for {relative_file}"
            ),
        }
    }
}

impl Error for DriverError {}
