mod token;

use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub use token::Token;

pub struct Lexer {
    reader: BufReader<std::fs::File>,
    path: PathBuf,
    current_line: String,
    current_line_number: usize,
    current_pos: usize,
    chars: Vec<char>,
    tokens: Vec<Token>,
}

impl Lexer {
    /// Create a new Lexer from a given file path.
    ///
    /// # Errors
    ///
    /// This function will return an error if opening the file failed.
    #[must_use = "Please use me uwu"]
    pub fn new(path: PathBuf) -> Result<Self, String> {
        // Creating a filehandle here might be a bad idea because this blocks the file
        // as long as the Lexer is in scope, even if Lexer::lex() is never called...
        // PERF: Maybe this should only happen when lexing is actually started.
        let file_handle = std::fs::File::open(&path).map_err(|err| err.to_string())?;
        let reader = BufReader::new(file_handle);
        Ok(Self {
            path,
            reader,
            current_line: String::new(),
            current_line_number: 0,
            current_pos: 0,
            chars: Vec::new(),
            tokens: Vec::new(),
        })
    }

    /// Lex a preprocessed C file into a list of tokens.
    ///
    /// # Errors
    ///
    /// This function will return an error if the given input is not a valid preprocessed C
    /// source file.
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        // read_line returns the amount of bytes read and Ok(0) if EOF is reached,
        // so we iterate over all lines until no more bytes were read or reading failed :)
        // This uses an if-let-chain (https://rust-lang.github.io/rfcs/2497-if-let-chains.html),
        // which were added in Rust2024, yay
        while let Ok(read_bytes) = self.reader.read_line(&mut self.current_line)
            && read_bytes > 0
        {
            self.current_line_number += 1;

            println!(
                "{0}: {1}",
                self.current_line_number,
                self.current_line.trim_end()
            );

            self.current_line.clear();
        }

        Err("Lol nope".to_string())
    }
}

