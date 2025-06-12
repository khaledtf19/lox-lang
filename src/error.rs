use crate::token::{Token, TokenType};

pub struct LoxError;
impl LoxError {
    pub fn error(line: usize, message: String) {
        LoxError::report(line, "".to_string(), message, &mut false);
    }

    pub fn report(line: usize, error_where: String, message: String, has_error: &mut bool) {
        println!("[line {line}] Error {error_where} => {message} ");
        *has_error = true;
    }
    pub fn token_errro(token: &Token, message: String) {
        if token.token_type == TokenType::EOF {
            LoxError::report(token.line, " at end".to_string(), message, &mut false);
        } else {
            LoxError::report(
                token.line,
                " at '".to_string() + &token.lexeme + "'",
                message,
                &mut false,
            );
        }
    }
}

pub struct RunTimeError {
    pub token: Token,
    pub message: String
}
impl RunTimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token , message}
    }
}
