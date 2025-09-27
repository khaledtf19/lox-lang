use std::collections::HashMap;

use crate::ast::expr::LiteralValue;

#[derive(Debug, Clone)]
struct Environment {
    pub values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new(&mut self) -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}
