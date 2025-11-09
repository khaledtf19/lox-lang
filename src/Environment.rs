use std::collections::HashMap;

use crate::{ast::expr::LiteralValue, error::RunTimeError, token::Token};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
}

type EnvironmentResult<T> = std::result::Result<T, RunTimeError>;

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<LiteralValue>) {
        match value {
            Some(token) => {
                self.values.insert(name, token);
            }
            None => {
                self.values.insert(name, LiteralValue::Nil);
            }
        }
    }

    pub fn get(&self, name: Token) -> EnvironmentResult<LiteralValue> {
        match self.values.get(&name.lexeme) {
            Some(value) => return Ok(value.clone()),
            None => {
                return Err(RunTimeError::new(
                    name.clone(),
                    "Undefined variable '".to_string() + &name.lexeme + "'.",
                ));
            }
        }
    }
}
