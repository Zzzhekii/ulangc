use super::Lexer;
use super::ast::Ast;
use super::Error;
use super::token::TokenData;

use log::{debug};

/// Intermediate error
type ParseInterResult<'e> = Result<(), Error<'e>>;

pub struct Parser<'l> {
    lexer:  &'l mut Lexer<'l>,
    ast:    Ast,
}

// Check how it's used and ig you'll get it >:( .
macro_rules! common_cmp_td {
    ($td_got: expr, $td_expected: expr) => {
        {
            return if std::mem::discriminant(&$td_got) == std::mem::discriminant(&$td_expected) {
                Ok(())
            } else {
                Err(Error::UnexpectedToken {
                    expected: vec![$td_expected],
                    got: $td_got,
                })
            };
        }
    };
}

macro_rules! unwrap_tok_eof {
    ($tok_opt: expr) => {
        {
            if let Some(tok) = $tok_opt {
                tok
            } else {
                return Err(Error::UnexpectedEndOfTokenStream)
            }
        }
    };
}

impl<'l> Parser<'l> {
    fn assert_seek_td(&mut self, offset: usize, td: TokenData<'l>) -> ParseInterResult<'l> {
        common_cmp_td!(td, unwrap_tok_eof!(self.lexer.seek(offset)).data)
    }

    fn assert_peek_td(&mut self, offset: usize, td: TokenData<'l>) -> ParseInterResult<'l> {
        common_cmp_td!(td, unwrap_tok_eof!(self.lexer.peek(offset)).data)
    }

    fn new(lexer: &'l mut Lexer<'l>) -> Self {
        Self {
            ast: Ast::new(),
            lexer,
        }
    }

    pub fn parse(lexer: &'l mut Lexer<'l>) -> Result<Ast, Error<'l>> {
        let mut parser = Self::new(lexer);

        parser.parse_module()?;

        Ok(parser.ast)
    }

    fn parse_module(&mut self) -> ParseInterResult<'l> {
        while let Some(tok) = self.lexer.peek(0) {
            self.parse_statement()?;
        }

        Ok(())
    }

    fn parse_statement(&mut self) -> ParseInterResult<'l> {
        let cur_tok = unwrap_tok_eof!(self.lexer.seek(0));
        match cur_tok.data {
            TokenData::KwStatic => self.parse_binding(BindType::Static)?,
            TokenData::KwConst => self.parse_binding(BindType::Const)?,
            TokenData::KwLet => self.parse_binding(BindType::Let)?,
            _ => return Err(Error::UnexpectedToken {
                expected: vec![TokenData::KwStatic, TokenData::KwConst, TokenData::KwLet],
                got: cur_tok.data,
            })
        }

        Ok(())
    }

    fn parse_binding(&mut self, bt: BindType) -> ParseInterResult<'l> {
        debug!("Parsing a binding of BindType '{:?}'...", bt);

        // Get the ident
        self.assert_peek_td(0, TokenData::Ident(""))?;
        let TokenData::Ident(ident) = self.lexer.peek(0).unwrap().data else { panic!() };
        self.lexer.seek(0);

        let is_known_data_type = match unwrap_tok_eof!(self.lexer.seek(0)).data {
            // Type annotation
            TokenData::Colon => {
                    true    
                },
            TokenData::
            _ => (),
        }

        Err(Error::NotImplemented("parse_binding".to_string()))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BindType { Static, Const, Let }
