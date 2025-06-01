use crate::token::{Token, TokenType};

use crate::ast::expr::{self, Expr};

use super::error::ParserError;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
    pub is_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        println!("{:?}", tokens);
        Parser {
            curr: 0,
            tokens,
            is_error: false,
        }
    }
    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(ex) => return Some(ex),
            Err(_) => {
                self.is_error = true;
                return None;
            }
        }
    }
    fn expression(&mut self) -> Result<Expr, ParserError> {
        // println!("{}", self.peek().token_type);

        let expr = self.equality()?;
        if self.match_token_types(vec![TokenType::QUESTION]) {
            let left = self.expression()?;
            self.consume(TokenType::COLON, "Expect ')' after expression.".to_string())?;
            let right = self.expression()?;
            return Ok(Expr::ternary(expr, left, right));

            
        }


        if self.match_token_types(vec![TokenType::COMMA]) {
            let right = self.expression()?;
            return Ok(Expr::separator(expr, right));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.match_token_types(vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token_types(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token_types(vec![TokenType::STAR, TokenType::SLASH]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token_types(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token_types(vec![TokenType::FALSE]) {
            return Ok(Expr::literal(expr::LiteralValue::Boolean(false)));
        }
        if self.match_token_types(vec![TokenType::TRUE]) {
            return Ok(Expr::literal(expr::LiteralValue::Boolean(true)));
        }
        if self.match_token_types(vec![TokenType::NIL]) {
            return Ok(Expr::literal(expr::LiteralValue::Nil));
        }

        if self.match_token_types(vec![TokenType::NUMBER, TokenType::STRING]) {
            let curr = self.previous();
            match curr.literal.unwrap() {
                crate::token::TokenLiteral::Float(f) => {
                    return Ok(Expr::literal(expr::LiteralValue::Number(f)));
                }
                crate::token::TokenLiteral::Text(s) => {
                    return Ok(Expr::literal(expr::LiteralValue::String(s)));
                }
            }
        }
        if self.match_token_types(vec![TokenType::COMMA]) {
            println!("here: =>>")
        }
        if self.match_token_types(vec![TokenType::LEFTPAREN]) {
            let expr = self.expression()?;

            match self.consume(
                TokenType::RIGHTPAREN,
                "Expect ')' after expression.".to_string(),
            ) {
                Ok(_) => return Ok(Expr::grouping(expr)),
                Err(err) => {
                    self.is_error = true;
                    return Err(err);
                }
            }
        }

        self.is_error = true;
        Err(ParserError::new(
            self.peek().clone(),
            "Expect expression.".to_string(),
        ))
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        error_message: String,
    ) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        } else {
            self.is_error = true;
            let err = ParserError::new(self.peek().clone(), error_message);
            Err(err)
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::IF
                | TokenType::FUN
                | TokenType::FOR
                | TokenType::CLASS
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN
                | TokenType::VAR => return,
                _ => {}
            }
            self.advance();
        }
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
