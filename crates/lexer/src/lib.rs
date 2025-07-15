mod token;

pub use token::Token;

pub struct Lexer {}

impl Lexer {
    #[must_use = "Please use me uwu"]
    pub const fn new() -> Self {
        Self {}
    }

    /// .Lex a preprocessed C file into a list of tokens.
    ///
    /// # Errors
    ///
    /// This function will return an error if the given input is not a valid C preprocessed C
    /// source file.
    pub fn lex(&self, input: &std::path::Path) -> Result<Vec<Token>, String> {
        todo!()
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

