use super::Parser;
// use crate::ast::token::TokenKind;
use super::error::{Error, ErrorKind, ErrorResult};
// use crate::ast::{SourceView, Node}

impl<'a> Parser<'a> {
    pub fn parse_explicit_type(&mut self) -> ErrorResult<()> {
        Err(Error {
            kind: ErrorKind::NotImplemented("Explicit type parsing is not yet implemented.".to_string()),
            view: None,
        })
    }
}
