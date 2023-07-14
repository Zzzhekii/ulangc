pub mod token;
pub mod ast;

mod lexer;
mod parser;

pub use lexer::Lexer;
pub use parser::Parser;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SourcePos {
    pub line:   usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SourceRange {
    pub start:  SourcePos,
    pub end:    SourcePos,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SourceView<'v> {
    pub range: SourceRange,
    pub view:  &'v str,
}

#[derive(thiserror::Error, Debug)]
pub enum Error<'e> {
    #[error("Expected character '{expected:?}', got '{got:?}'")]
    UnexpectedChar {
        expected: char,
        got: char,
    },
    #[error("Expected token '{expected:?}', got '{got:?}'")]
    UnexpectedToken {
        expected: Vec<token::TokenData<'e>>,
        got: token::TokenData<'e>,
    },
    #[error("Unexpected end of token stream")]
    UnexpectedEndOfTokenStream,
    #[error("Feature not yet implemented: '{0}'")]
    NotImplemented(String),
    #[error("Unreachable: '{0}'")]
    Unreachable(String),
}
