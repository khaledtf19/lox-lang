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
    fn comparison(&mut self) -> Expr {
        todo!()
    }
    fn match_token_types(&self,token_types: Vec<TokenType>) -> bool {
        todo!()
    }
    fn previous(&mut self) -> Token{
        todo!()
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
}
