use crate::ast::lexer::Lexer;
use crate::ast::lexer::Token;
use crate::ast::{ASTStatement, ASTExpression};
use crate::ast::lexer::TokenKind;
pub struct Parser {
    tokens: Vec<crate::ast::lexer::Token>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn from_input(input: &str) -> Self {
        let mut lexer: Lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        Self { tokens, current: 0 }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement>{
        return self.parse_statement();
    }

    pub fn parse_statement(&mut self) -> Option<ASTStatement> {
        let token: &Token = self.current()?;
        if token.kind == TokenKind::EOF {
            return None;
        }
        let expr = self.parse_expression()?;
        return Some(ASTStatement::expression(expr));
    }

    pub fn parse_expression(&mut self) -> Option<ASTExpression> {
        let token: &Token = self.consume()?;
        match token.kind {
            TokenKind::Number(number) => {
                // doubt number is i64
                return Some(ASTExpression::number(number));
            },
            _ => None,
        }
    }

    pub fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    pub fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    pub fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token: &Token = self.peek(-1)?;
        return Some(token);
    }
}