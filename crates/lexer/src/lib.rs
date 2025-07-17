mod token;

use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub use token::Token;

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
    pub fn lex_file(&mut self, filepath: &PathBuf) -> Result<&Vec<Token>, String> {
        let file = std::fs::File::open(filepath).map_err(|err| err.to_string())?;
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
    pub fn lex_string(&mut self, string: &str) -> Result<&Vec<Token>, String> {
        let lines = string.lines();

        for line in lines {
            self.current_line = line.to_string();
            self.lex_current_line()?;
            self.current_line_number += 1;
        }

        Ok(&self.tokens)
    }

    fn lex_current_line(&self) -> Result<(), String> {
        if !self.current_line.is_ascii() {
            return Err(format!(
                "Non-ascii characters found on line {0}: {1}",
                self.current_line_number, self.current_line
            ));
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
