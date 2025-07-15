///Library for Parsing a list of Tokens into an Abstract Syntax Tree.
mod abstract_syntax_tree;
pub use abstract_syntax_tree::AST;

pub struct Parser {}

impl Parser {
    #[must_use = "Why wouldn't you?"]
    pub const fn new() -> Self {
        Self {}
    }

    /// .Parse a list of tokens into an abstract syntax tree (AST).
    ///
    /// # Errors
    ///
    /// This function will return an error if the Tokens cannot be parsed to a valid AST.
    pub fn parse(&self, tokens: &[lexer::Token]) -> Result<AST, String> {
        todo!()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
