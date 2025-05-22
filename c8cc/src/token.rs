#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// (
    OpenParenthesis,
    /// )
    CloseParenthesis,
    /// ;
    Semicolon,
    /// u8, return, etc.
    Keyword(Keyword),
    /// foo, bar, etc.
    Identifier(String),
    /// 1, 2, 3, etc.
    IntegerLiteral(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
}

impl TokenInfo {
    pub fn new(token: Token, line: usize) -> Self {
        Self { token, line }
    }
}
