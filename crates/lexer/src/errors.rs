use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LexingError {
    NonAsciiInput(usize),
    IOError(std::io::Error),
}

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LexingError::*;
        match self {
            NonAsciiInput(line_number) => write!(f, "Non-ASCII input found in line {line_number}"),
            IOError(err) => write!(f, "Unable to open file: {err}"),
        }
    }
}

impl Error for LexingError {}
