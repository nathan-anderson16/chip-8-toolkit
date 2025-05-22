use regex::Regex;

use crate::token::{Keyword, Token, TokenInfo};

struct Lexer<'a> {
    buf: &'a str,
    current: usize,
    current_line: usize,
    integer_regex: Regex,
    identifier_regex: Regex,
}

impl<'a> Lexer<'a> {
    fn new(buf: &'a str) -> Self {
        Self {
            buf,
            current: 0,
            current_line: 0,
            integer_regex: Regex::new("\\d").unwrap(),
            identifier_regex: Regex::new("[a-zA-Z0-9_]").unwrap(),
        }
    }

    /// Return the character at the given offset, but don't consume it.
    fn peek(&self, offset: usize) -> Option<char> {
        self.buf.chars().nth(self.current + offset)
    }

    /// Return the current character without consuming it. Returns None if there are no more characters.
    fn current(&self) -> Option<char> {
        self.peek(0)
    }

    /// Return the character that was last consumed.
    fn prev(&self) -> Option<char> {
        self.buf.chars().nth(self.current - 1)
    }

    fn advance(&mut self) -> Option<char> {
        let val = self.buf.chars().nth(self.current);
        self.current += 1;
        if val == Some('\n') {
            self.current_line += 1;
        }
        val
    }

    fn is_integer(&self, c: char) -> bool {
        self.integer_regex.is_match(c.to_string().as_str())
    }

    /// Return the next token. If there is no next token, return None.
    fn next_token(&mut self) -> Option<TokenInfo> {
        loop {
            let current = self.advance()?;

            if current.is_whitespace() {
                continue;
            }

            return match current {
                '{' => Some(TokenInfo::new(Token::OpenBrace, self.current_line)),
                '}' => Some(TokenInfo::new(Token::CloseBrace, self.current_line)),
                '(' => Some(TokenInfo::new(Token::OpenParenthesis, self.current_line)),
                ')' => Some(TokenInfo::new(Token::CloseParenthesis, self.current_line)),
                ';' => Some(TokenInfo::new(Token::Semicolon, self.current_line)),
                '~' => Some(TokenInfo::new(Token::BitwiseNot, self.current_line)),
                '!' => Some(TokenInfo::new(Token::LogicalNot, self.current_line)),
                _ if self.is_integer(current) => self.parse_integer_literal(),
                _ => self.parse_identifier(),
            };
        }
    }

    /// Parses an integer literal at the current index of the lexer.
    fn parse_integer_literal(&mut self) -> Option<TokenInfo> {
        let mut s = String::new();
        s.push(self.prev()?);

        while let Some(c) = self.current() {
            if self.is_integer(c) {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        // Should never panic since the string is *only* constructed from valid digits
        let token = Token::IntegerLiteral(s.parse::<usize>().unwrap());

        Some(TokenInfo::new(token, self.current_line))
    }

    fn is_valid_identifier(&self, c: char) -> bool {
        self.identifier_regex.is_match(c.to_string().as_str())
    }

    fn parse_identifier(&mut self) -> Option<TokenInfo> {
        let mut s = String::new();
        s.push(self.prev()?);

        while let Some(c) = self.current() {
            if self.is_valid_identifier(c) {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let token = match s.as_str() {
            "int" => Token::Keyword(Keyword::Int),
            "return" => Token::Keyword(Keyword::Return),
            _ => Token::Identifier(s),
        };

        Some(TokenInfo::new(token, self.current_line))
    }
}

pub fn lex(buf: &str) -> Vec<TokenInfo> {
    let mut lexer = Lexer::new(buf);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    tokens
}
