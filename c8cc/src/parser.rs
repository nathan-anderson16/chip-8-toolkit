use crate::{
    node::{ExprNode, FunctionNode, ProgramNode, StatementNode},
    token::{Keyword, Token, TokenInfo},
};

macro_rules! error {
    ($self:tt, $($arg:tt)*) => {
        ::std::panic!("Error at line {}: {}", $self.current_line, format!($($arg)*))
    };
}

macro_rules! get_token {
    ($self:tt, $ttype:tt) => {{
        let Some(token) = $self.advance() else {
            error!($self, "expected {}, found EOF", $ttype);
        };
        token
    }};
}

struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    current_line: usize,
}

impl Parser {
    fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            tokens,
            current: 0,
            current_line: 0,
        }
    }

    /// Return the token at the given offset, but don't consume it.
    fn peek(&self, offset: usize) -> Option<&TokenInfo> {
        self.tokens.get(self.current + offset)
    }

    /// Return the current token without consuming it. Returns None if there are no more tokens.
    fn current(&self) -> Option<&TokenInfo> {
        println!();
        self.peek(0)
    }

    fn advance(&mut self) -> Option<TokenInfo> {
        let token = self.tokens.get(self.current)?;
        self.current += 1;
        self.current_line = token.line;
        Some(token.clone())
    }

    /// Parse a `ProgramNode`.
    fn parse_program(&mut self) -> ProgramNode {
        ProgramNode {
            func: self.parse_function(),
        }
    }

    /// Parse a `FunctionNode`.
    fn parse_function(&mut self) -> FunctionNode {
        let token = get_token!(self, "'int'");
        if token.token != Token::Keyword(Keyword::Int) {
            error!(self, "expected \"int\" keyword, found {:?}", token.token);
        }

        let token = get_token!(self, "identifier");
        let Token::Identifier(id) = token.token else {
            error!(self, "expected identifier, found {:?}", token.token);
        };

        let token = get_token!(self, "'('");
        if token.token != Token::OpenParenthesis {
            error!(self, "expected '(' token, found {:?}", token.token);
        }

        let token = get_token!(self, "')'");
        if token.token != Token::CloseParenthesis {
            error!(self, "expected ')' token, found {:?}", token.token);
        }

        let token = get_token!(self, "'{'");
        if token.token != Token::OpenBrace {
            error!(self, "expected '{{' token, found {:?}", token.token);
        }

        let statement = self.parse_statement();

        let token = get_token!(self, "'}'");
        if token.token != Token::CloseBrace {
            error!(self, "expected '}}' token, found {:?}", token.token);
        }

        FunctionNode { id, statement }
    }

    /// Parse a `StatementNode`.
    fn parse_statement(&mut self) -> StatementNode {
        let token = get_token!(self, "statement");
        if token.token != Token::Keyword(Keyword::Return) {
            error!(self, "expected \"return\" keyword, found {:?}", token.token);
        }

        let expr = self.parse_expr();

        let token = get_token!(self, "\';\'");
        if token.token != Token::Semicolon {
            error!(self, "expected ';' token, found {:?}", token.token);
        }

        StatementNode { expr }
    }

    /// Parse an `ExprNode`.
    fn parse_expr(&mut self) -> ExprNode {
        let token = get_token!(self, "expr");

        let Token::IntegerLiteral(expr) = token.token else {
            error!(self, "expected expr, found {:?}", token.token);
        };

        ExprNode { value: expr }
    }
}

pub fn parse(tokens: Vec<TokenInfo>) -> ProgramNode {
    let mut parser = Parser::new(tokens);

    parser.parse_program()
}
