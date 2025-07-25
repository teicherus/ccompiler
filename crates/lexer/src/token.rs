#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    /// Any identifier (name)
    Identifier(String),
    /// Any integer constant, e.g. *42*
    Constant(i32),

    /// Keyword: *int*
    Int,
    /// Keyword: *void*
    Void,
    /// Keyword: *return*
    Return,

    // Punctuation
    /// Semicolon *;*
    Semicolon,

    /// Left Parenthesis *(*
    LPar,
    /// Right parenthesis *)*
    RPar,
    /// Left brace *{*
    LBrace,
    /// Right brace *}*
    RBrace,
}
