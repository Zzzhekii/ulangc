use super::token::{Token, TokenData};
use super::{SourcePos, SourceRange, SourceView};

use std::collections::VecDeque;

pub struct Lexer<'i> {
    input: &'i str,
    cursor: usize,
    position: SourcePos,

    lookahead: VecDeque<Token<'i>>,
}

impl<'i> Lexer<'i> {
    pub fn from_str(input: &'i str) -> Self {
        Self {
            input,
            cursor: 0,
            position: SourcePos{line: 0, column: 0},
            lookahead: VecDeque::new(),
        }
    }

    pub fn peek(&mut self, offset: usize) -> Option<Token<'i>> {
        if let Err(_) = self.populate_lookahead(offset) {
            return None;
        }
        Some(self.lookahead[offset])
    }

    pub fn seek(&mut self, offset: usize) -> Option<Token<'i>> {
        if let Err(_) = self.populate_lookahead(offset) {
            return None;
        }

        let last = self.lookahead[offset];
        self.discard_lookahead(offset).unwrap();
        Some(last)
    }

    fn populate_lookahead(&mut self, up_to: usize) -> Result<(), ()> {
        let range = self.lookahead.len()..(up_to + 1);
        for _ in range {
            if let Some(t) = self.next_token() {
                self.lookahead.push_back(t)
            } else {
                return Err(())
            }
        }

        Ok(())
    }

    fn discard_lookahead(&mut self, up_to: usize) -> Result<(), ()> {
        let range = 0..(up_to + 1);
        for _ in range {
            if self.lookahead.pop_front().is_none() {
                return Err(())
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn peek_ch(&self, offset: usize) -> Option<char> { return self.input.chars().nth(self.cursor + offset) }

    fn advance(&mut self, times: usize) {
        for _ in 0..times {
            if self.peek_ch(0) == Some('\n') {
                self.position.line += 1;
                self.position.column = 0;
            } else {
                self.position.column += 1;
            }

            self.cursor += 1;
        }
    }

    #[inline(always)]
    fn skip_comments(&mut self) {
        if self.peek_ch(0) != Some('-') || self.peek_ch(1) != Some('-') {
            return
        }

        if self.peek_ch(2) == Some('-') {
            // Multiline comment
            self.advance(3);
            let mut consequent_count: u8 = 0;
            while consequent_count < 3 {
                let Some(ch) = self.peek_ch(0) else { return };
                if ch == '-' {
                    consequent_count += 1;
                } else {
                    consequent_count = 0;
                }
                self.advance(1);
            }
        } else {
            // Single-line comment
            self.advance_while(|ch| ch != '\n');
        }
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        self.skip_comments();
        while [Some(' '), Some('\t'), Some('\n'), Some('\r')].contains(&self.peek_ch(0)) {
            self.advance(1)
        }
    }

    fn advance_while<F: Fn(char) -> bool>(&mut self, f: F) {
        while let Some(ch) = self.peek_ch(0) {
            if f(ch) {
                self.advance(1)
            } else {
                return
            }
        }
    }

    fn next_token(&mut self) -> Option<Token<'i>> {
        loop {
            let cur = self.cursor;
            self.skip_comments();
            self.skip_whitespace();
            if cur == self.cursor {
                break;
            }
        }

        let start_pos = self.position;
        let start_cur = self.cursor;

        macro_rules! ret_kw {
            ($td:expr) => {
                {
                    self.advance(1);
                    return Some(Token {
                        data: $td,
                        view: SourceView {
                            range: SourceRange {
                                start: start_pos,
                                end: self.position,
                            },
                            view: &self.input[start_cur..self.cursor - 1]
                        }
                    })
                }
            };
        }
        
        macro_rules! ret_tok1 {
            ($td:expr) => {
                {
                    self.advance(1);
                    return Some(Token {
                        data: $td,
                        view: SourceView {
                            range: SourceRange {
                                start: start_pos,
                                end: self.position,
                            },
                            view: &self.input[start_cur..self.cursor]
                        }
                    })
                }
            };
        }
        
        macro_rules! ret_tok2 {
            ($td_fallback:expr, $pairs:expr) => {
                {
                    let Some(next_char) = self.peek_ch(1) else { ret_tok1!($td_fallback) };
                    for (nch, ntd) in $pairs {
                        if next_char == nch {
                            self.advance(2);

                            return Some(Token {
                                data: ntd,
                                view: SourceView {
                                    range: SourceRange {
                                        start: start_pos,
                                        end: self.position,
                                    },
                                    view: &self.input[start_cur..self.cursor]
                                }
                            })
                        }
                    }

                    ret_tok1!($td_fallback)
                }
            };
        }

        let Some(start_ch) = self.peek_ch(0) else { return None };

        match start_ch {
            '.' => ret_tok1!(TokenData::Dot),
            ',' => ret_tok1!(TokenData::Comma),
            '(' => ret_tok1!(TokenData::LParen),
            ')' => ret_tok1!(TokenData::RParen),
            '-' => {

                ret_tok2!(TokenData::Minus,      [('=', TokenData::AssignMinus)])
            },
            '+' => ret_tok2!(TokenData::Plus,       [('=', TokenData::AssignPlus)]),
            '*' => ret_tok2!(TokenData::Mul,        [('=', TokenData::AssignMul)]),
            '/' => ret_tok2!(TokenData::Div,        [('=', TokenData::AssignDiv)]),
            '!' => ret_tok2!(TokenData::Not,        [('=', TokenData::NotEq)]),
            '=' => ret_tok2!(TokenData::Assign,     [('=', TokenData::Equal)]),
            '>' => ret_tok2!(TokenData::Greater,    [('=', TokenData::GreaterEq)]),
            '<' => ret_tok2!(TokenData::Less,       [('=', TokenData::LessEq)]),
            ':' => ret_tok2!(TokenData::Column,     [(':', TokenData::ScopeAccess)]),

            '\'' => {
                self.advance(1);
                self.advance_while(|ch| ch != '\'');

                self.advance(1);

                return Some(Token {
                    data: TokenData::Str(&self.input[start_cur + 1..self.cursor - 1]),
                    view: SourceView {
                        range: SourceRange {
                            start: start_pos,
                            end: self.position,
                        },
                        view: &self.input[start_cur..self.cursor]
                    }
                })
            }, 
            _ => (),
        }

        self.advance_while(|ch| is_letter(ch));
        let ident = &self.input[start_cur..self.cursor];

        if ident == "" {
            return None
        }

        match ident {
            "do" =>     ret_kw!(TokenData::KwDo),
            "end" =>    ret_kw!(TokenData::KwEnd),
            "static" => ret_kw!(TokenData::KwStatic),
            "const" =>  ret_kw!(TokenData::KwConst),
            "let" =>    ret_kw!(TokenData::KwLet),
            "fn" =>     ret_kw!(TokenData::KwFn),
            _ => (),
        }

        return Some(Token {
            data: TokenData::Ident(ident),
            view: SourceView {
                range: SourceRange {
                    start: start_pos,
                    end: self.position,
                },
                view: &self.input[start_cur..self.cursor]
            }
        })
    }
}

fn is_letter(ch: char) -> bool {
    let ch = ch as u8;
    return (('A' as u8) <= ch && ch >= ('Z' as u8))
        || (('a' as u8) <= ch && ch >= ('z' as u8))
        ||  ('_' as u8) == ch;
}
