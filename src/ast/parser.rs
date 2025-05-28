use std::usize;

use crate::token::{Token, TokenType};

use super::expr::Expr;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { curr: 0, tokens }
    }
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token_types(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::binary(expr, operator, right)
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token_types(vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::binary(expr, operator, right);
        }
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token_types(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::binary(expr, operator, right);
        }
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token_types(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::binary(expr, operator, right);
        }
        expr
    }
    fn unary(&mut self)->Expr{
        let mut expr = self.unary();

        while self.match_token_types(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::binary(expr, operator, right);
        }
        expr
    }
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }
    fn match_token_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for &token_type in token_types.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn previous(&mut self) -> Token {
        self.tokens.get(self.curr - 1).unwrap().clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.curr += 1
        }
        self.previous()
    }
    fn peek(&self) -> &Token {
        self.tokens.get(self.curr).unwrap()
    }
    fn is_at_end(&self) -> bool {
        if self.curr >= self.tokens.len() {
            return true;
        }
        false
    }
}
