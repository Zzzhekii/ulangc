use super::SourceView;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Token<'t> {
    pub data: TokenData<'t>,
    pub view: SourceView<'t>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenData<'d> {
    // Keywords
    KwEnd,
    KwDo,
    KwStatic,
    KwConst,
    KwLet,
    KwFn,

    // Punctuation
    LParen,
    RParen,
    Comma,
    Dot,
    Column,

    // Operators
    Assign,
    Plus,
    AssignPlus,
    Minus,
    AssignMinus,
    Mul,
    AssignMul,
    Div,
    AssignDiv,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equal,
    Not,
    NotEq,
    ScopeAccess,

    // Other
    Str(&'d str),
    Ident(&'d str),
}
