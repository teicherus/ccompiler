use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::errors::LexingError;
use crate::token::Token;

pub struct Lexer {
    current_line: String,
    current_line_number: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    /// Create a new Lexer from a given file path.
    ///
    /// # Errors
    ///
    /// This function will return an error if opening the file failed.
    #[must_use = "Please use me uwu"]
    pub const fn new() -> Self {
        Self {
            current_line: String::new(),
            current_line_number: 0,
            tokens: Vec::new(),
        }
    }

    /// Lex a preprocessed C file into a list of tokens.
    ///
    /// # Errors
    ///
    /// This function will return an error if the given input is not a valid preprocessed C
    /// source file.
    pub fn lex_file(&mut self, filepath: &PathBuf) -> Result<&Vec<Token>, LexingError> {
        let file = std::fs::File::open(filepath).map_err(LexingError::IOError)?;
        let mut reader = BufReader::new(file);

        // read_line returns the amount of bytes read and Ok(0) if EOF is reached,
        // so we iterate over all lines until no more bytes were read or reading failed :)
        // This uses an if-let-chain (https://rust-lang.github.io/rfcs/2497-if-let-chains.html),
        // which were added in Rust2024, yay
        while let Ok(read_bytes) = reader.read_line(&mut self.current_line)
            && read_bytes > 0
        {
            self.lex_current_line()?;
            self.current_line_number += 1;
            // Need to clear the line since the BufReader appends read lines
            self.current_line.clear();
        }
        Ok(&self.tokens)
    }

    /// Lex a (multiline) string.
    ///
    /// # Errors
    ///
    /// If lexing fails, duh
    pub fn lex_string(&mut self, string: &str) -> Result<&Vec<Token>, LexingError> {
        let lines = string.lines();

        for line in lines {
            self.current_line = line.to_string();
            self.lex_current_line()?;
            self.current_line_number += 1;
        }

        Ok(&self.tokens)
    }

    fn lex_current_line(&self) -> Result<(), LexingError> {
        if !self.current_line.is_ascii() {
            return Err(LexingError::NonAsciiInput(self.current_line_number));
        }
        println!(
            "{0}: {1}",
            self.current_line_number,
            self.current_line.trim_end()
        );

        Ok(())
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn int_main_void() -> Result<(), LexingError> {
        let mut lexer = Lexer::new();
        let input = r"
        int main(void) {
	        return 2;
        }";

        let tokens = lexer.lex_string(input)?.clone();

        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".to_string()),
                Token::LPar,
                Token::Void,
                Token::RPar,
                Token::LBrace,
                Token::Return,
                Token::Constant(2),
                Token::Semicolon,
                Token::RBrace
            ]
        );

        Ok(())
    }
}
