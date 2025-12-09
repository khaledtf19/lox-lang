use std::collections::HashMap;

use crate::{ast::expr::LiteralValue, error::RunTimeError, token::Token};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, LiteralValue>,
}

type EnvironmentResult<T> = std::result::Result<T, RunTimeError>;

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
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
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get(name),
                None => {
                    return Err(RunTimeError::new(
                        name.clone(),
                        "Undefined variable '".to_string() + &name.lexeme + "'.",
                    ));
                }
            },
        }
    }
    pub fn assign(&mut self, name: Token, value: LiteralValue) -> Result<(), RunTimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else {
            match &mut self.enclosing {
                Some(enclosing) => enclosing.assign(name, value),
                None => Err(RunTimeError::new(
                    name.clone(),
                    "Undefined variable '".to_string() + &name.lexeme + "'.",
                )),
            }
        }
    }
}
