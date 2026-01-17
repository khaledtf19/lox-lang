use std::{cell::RefCell, rc::Rc};

use crate::{
    Environment::{Env, Environment},
    ast::expr::LiteralValue,
    error::RunTimeError,
    interpreter::Interpreter,
    lox_callable::LoxCallable,
    stmt::FunctionStmt,
};

pub struct LoxFunction {
    pub declaration: Rc<FunctionStmt>,
    pub closure: Env,
}

impl LoxFunction {
    pub fn new(declaration: Rc<FunctionStmt>, closure: Env) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[LiteralValue],
    ) -> Result<LiteralValue, RunTimeError> {
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.closure,
        )))));

        for (i, param) in self.declaration.params.iter().enumerate() {
            if let Some(curr_value) = arguments.get(i) {
                env.borrow_mut()
                    .define(param.lexeme.clone(), Some(curr_value.clone()));
            } else {
                env.borrow_mut().define(param.lexeme.clone(), None);
            }
        }
        let value = interpreter.exeucute_block(&self.declaration.body, env)?;
        if let Some(value) = value {
            match value {
                crate::stmt::ControlFlow::Return(literal_value) => {
                    return Ok(literal_value.unwrap_or(LiteralValue::Nil));
                }
                _ => Err(RunTimeError::new(
                    self.declaration.name.clone(),
                    "Expect return expretion".to_string(),
                )),
            }
        } else {
            return Ok(LiteralValue::Nil);
        }
    }
    fn arity(&self) -> usize {
        return self.declaration.params.len();
    }

    fn to_string(&self) -> String {
        return format!("<fn {}>", self.declaration.name.lexeme);
    }
}
