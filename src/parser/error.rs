use core::fmt;

use crate::{error::LoxError, token::Token};

#[derive(Debug)]
pub struct ParserError{
    token: Token,
    message:String,
} 

impl ParserError {
    pub fn new(token: Token, message: String) ->Self {
        LoxError::token_errro(&token, message.to_string());
        Self { token, message }
    }
}

impl fmt::Display for ParserError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) line: {} => ({})", self.token.lexeme, self.token.line, self.message)
    }
}

