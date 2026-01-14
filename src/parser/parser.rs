use std::fmt::format;
use std::vec::IntoIter;

use crate::parser::error;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

use crate::ast::expr::{self, Expr, LiteralExpr, LiteralValue};

use super::error::ParserError;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
    pub has_error: bool,
}

type ParserResult<T> = std::result::Result<T, ParserError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            curr: 0,
            tokens,
            has_error: false,
        }
    }
    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statments: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(value) => statments.push(value),
                Err(_) => {
                    self.synchronize();
                    return None;
                }
            }
        }
        return Some(statments);
    }
    fn declaration(&mut self) -> ParserResult<Stmt> {
        if self.match_token_types(vec![TokenType::FUN]) {
            return self.function("function");
        }
        if self.match_token_types(vec![TokenType::VAR]) {
            return self.var_declaration();
        }
        return self.statment();
    }
    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.".to_string());
        let mut initializer: Option<Expr> = None;

        if self.match_token_types(vec![TokenType::EQUAL]) {
            initializer = Some(self.expression()?);
        }
        match self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.".to_string(),
        ) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
        return Ok(Stmt::var_stmt(name?, initializer));
    }
    fn while_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(
            TokenType::LEFTPAREN,
            "Expect '(' after 'while'.".to_string(),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RIGHTPAREN,
            "Expect ')' after condition.".to_string(),
        )?;
        let body = self.statment()?;

        return Ok(Stmt::while_stmt(condition, body));
    }
    fn statment(&mut self) -> ParserResult<Stmt> {
        if self.match_token_types(vec![TokenType::FOR]) {
            return self.for_statement();
        }
        if self.match_token_types(vec![TokenType::IF]) {
            return self.if_statment();
        }
        if self.match_token_types(vec![TokenType::PRINT]) {
            return self.print_statment();
        }
        if self.match_token_types(vec![TokenType::RETURN]) {
            return self.return_statement();
        }
        if self.match_token_types(vec![TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.match_token_types(vec![TokenType::LEFTBRACE]) {
            return Ok(Stmt::block_stmt(self.block()));
        }
        if self.match_token_types(vec![TokenType::BREAK]) {
            println!("{:?} , {:?}", self.previous(), self.peek());
            self.consume(TokenType::SEMICOLON, "Expect ; after break".to_string());
            return Ok(Stmt::break_stmt());
        }
        return self.expression_statment();
    }
    fn for_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenType::LEFTPAREN, "Expect '(' after 'for'.".to_string())?;
        let mut initializer = None;

        if self.match_token_types(vec![TokenType::SEMICOLON]) {
            initializer = None;
        } else if self.match_token_types(vec![TokenType::VAR]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statment()?);
        }

        let mut condition = None;
        if !self.check(TokenType::SEMICOLON) {
            condition = Some(self.expression()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after loop condition.".to_string(),
        )?;

        let mut increment = None;
        if !self.check(TokenType::RIGHTPAREN) {
            increment = Some(self.expression()?);
        }
        self.consume(
            TokenType::RIGHTPAREN,
            "Expect ')' after for clauses.".to_string(),
        )?;

        let mut body = self.statment()?;

        if let Some(inc) = increment {
            body = Stmt::block_stmt(vec![body, Stmt::expresstion_stmt(inc)]);
        }

        if condition.is_none() {
            condition = Some(Expr::Literal(LiteralExpr {
                value: LiteralValue::Boolean(true),
            }));
        }
        body = Stmt::while_stmt(condition.unwrap(), body);

        if let Some(init) = initializer {
            body = Stmt::block_stmt(vec![init, body]);
        }
        Ok(body)
    }
    fn if_statment(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenType::LEFTPAREN, "Expect '(' after 'if'.".to_string())?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RIGHTPAREN,
            "Expect ')' after if condition.".to_string(),
        )?;

        let then_branch = Box::new(self.statment()?);
        let mut else_branch = None;
        if self.match_token_types(vec![TokenType::ELSE]) {
            else_branch = Some(Box::new(self.statment()?));
        }
        return Ok(Stmt::if_stmt(condition, then_branch, else_branch));
    }
    fn print_statment(&mut self) -> ParserResult<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
        Ok(Stmt::print_stmt(value))
    }

    fn return_statement(&mut self) -> ParserResult<Stmt> {
        let keyword = self.previous();

        let mut value: Option<Expr> = None;

        if !self.check(TokenType::SEMICOLON) {
            value = Some(self.expression()?);
        }

        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after return value.".to_string(),
        )?;

        return Ok(Stmt::return_stmt(keyword, value));
    }

    fn expression_statment(&mut self) -> ParserResult<Stmt> {
        let expr = self.assignment()?;
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after expression.".to_string(),
        )?;
        Ok(Stmt::expresstion_stmt(expr))
    }

    fn function(&mut self, kind: &str) -> ParserResult<Stmt> {
        let name = self.consume(TokenType::IDENTIFIER, format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LEFTPAREN,
            format!("Expect ( after {} name", kind),
        )?;

        let mut parameters = vec![];

        if !self.check(TokenType::RIGHTPAREN) {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParserError::new(
                        self.peek().clone(),
                        "Can't have more than 255 parameters.".to_string(),
                    ));
                }

                parameters.push(
                    self.consume(TokenType::IDENTIFIER, "Expect parameter name.".to_string())?,
                );

                if !self.match_token_types(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(
            TokenType::RIGHTPAREN,
            "Expect ')' after parameters.".to_string(),
        )?;

        self.consume(
            TokenType::LEFTBRACE,
            "Expect '{' before ".to_string() + kind + " body.",
        )?;

        let mut body = self.block();

        return Ok(Stmt::function_stmt(name, parameters, body));
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];
        while !self.check(TokenType::RIGHTBRACE) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(_) => {
                    self.synchronize();
                    return statements;
                }
            }
        }

        match self.consume(TokenType::RIGHTBRACE, "Expect '}' after block.".to_string()) {
            Ok(_) => return statements,
            Err(_) => {
                self.synchronize();
                return statements;
            }
        }
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.or()?;

        if self.match_token_types(vec![TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;
            match &expr {
                Expr::Variable(variable_expr) => {
                    let name = variable_expr.name.clone();
                    return Ok(Expr::assign(name, value.clone()));
                }
                _ => {
                    return Err(ParserError::new(
                        equals,
                        "Invalid assignment target.".to_string(),
                    ));
                }
            }
        }

        return Ok(expr);
    }
    fn or(&mut self) -> ParserResult<Expr> {
        let mut expr = self.and()?;
        while self.match_token_types(vec![TokenType::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right)
        }
        Ok(expr)
    }

    fn and(&mut self) -> ParserResult<Expr> {
        let mut expr = self.equality()?;
        while self.match_token_types(vec![TokenType::AND]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right)
        }
        Ok(expr)
    }
    fn expression(&mut self) -> Result<Expr, ParserError> {
        return self.assignment();
        // let expr = self.equality()?;
        // if self.match_token_types(vec![TokenType::QUESTION]) {
        //     let left = self.expression()?;
        //     self.consume(TokenType::COLON, "Expect ')' after expression.".to_string())?;
        //     let right = self.expression()?;
        //     return Ok(Expr::ternary(expr, left, right));
        // }

        // if self.match_token_types(vec![TokenType::COMMA]) {
        //     let right = self.expression()?;
        //     return Ok(Expr::separator(expr, right));
        // }
        // Ok(expr)
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

        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> ParserResult<Expr> {
        let mut arguments: Vec<Expr> = vec![];

        if !self.check(TokenType::RIGHTPAREN) {
            loop {
                if arguments.len() >= 255 {
                    ParserError::new(
                        self.peek().clone(),
                        "Can't have more than 255 arguments.".to_string(),
                    );
                }
                arguments.push(self.expression()?);

                if !self.match_token_types(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RIGHTPAREN,
            "Expect ')' after arguments.".to_string(),
        )?;

        return Ok(Expr::call(callee, paren, arguments));
    }

    fn call(&mut self) -> ParserResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token_types(vec![TokenType::LEFTPAREN]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
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

        if self.match_token_types(vec![TokenType::IDENTIFIER]) {
            return Ok(Expr::variable(self.previous()));
        }

        if self.match_token_types(vec![TokenType::LEFTPAREN]) {
            let expr = self.expression()?;

            match self.consume(
                TokenType::RIGHTPAREN,
                "Expect ')' after expression.".to_string(),
            ) {
                Ok(_) => return Ok(Expr::grouping(expr)),
                Err(err) => {
                    self.has_error = true;
                    return Err(err);
                }
            }
        }

        if self.match_token_types(vec![TokenType::EOF]) {}

        self.has_error = true;

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
            self.has_error = true;
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
        self.peek().token_type == TokenType::EOF
    }
}
