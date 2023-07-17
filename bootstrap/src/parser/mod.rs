// Common modules
pub mod error;

// Lexer modules
pub mod lexer;

// Parser modules
mod expr;
mod stmt;
mod scope;

use crate::ast::Ast;

pub type ParseResult<'a> = Result<Ast, error::Error<'a>>;

pub struct Parser<'a> {
    lexer:      &'a mut lexer::Lexer<'a>,
    ast:        Ast,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut lexer::Lexer<'a>)-> Self {
        Self {
            lexer,
            ast: Ast::new(),
        }
    }

    pub fn parse(lexer: &'a mut lexer::Lexer<'a>) -> ParseResult<'a> {
        let mut parser = Self::new(lexer);
        Ok(parser.ast)
    }
}
