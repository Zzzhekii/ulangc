use crate::ast::{SourceView, token};

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind<'e> {
    #[error("Expected character '{expected:?}', got '{got:?}'")]
    UnexpectedChar {
        expected: char,
        got: char,
    },
    #[error("Expected token '{expected:?}', got '{got:?}'")]
    UnexpectedToken {
        expected: Vec<token::TokenKind<'e>>,
        got: token::TokenKind<'e>,
    },
    #[error("Unexpected end of token stream")]
    UnexpectedEndOfTokenStream,
    #[error("Feature not yet implemented: '{0}'")]
    NotImplemented(String),
    #[error("Unreachable: '{0}'")]
    Unreachable(String),
}

pub struct Error<'e> {
    pub kind: ErrorKind<'e>,
    pub view: SourceView<'e>,
}
