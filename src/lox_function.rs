use std::{cell::RefCell, rc::Rc};

use crate::{
    Environment::Environment, ast::expr::LiteralValue, error::RunTimeError,
    interpreter::Interpreter, lox_callable::LoxCallable, stmt::FunctionStmt,
};

pub struct LoxFunction {
    pub declaration: Rc<FunctionStmt>,
}

impl LoxFunction {
    pub fn new(declaration: Rc<FunctionStmt>) -> Self {
        Self { declaration }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[LiteralValue],
    ) -> Result<LiteralValue, RunTimeError> {
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &interpreter.globals,
        )))));

        for (i, param) in self.declaration.params.iter().enumerate() {
            if let Some(curr_value) = arguments.get(i) {
                env.borrow_mut()
                    .define(param.lexeme.clone(), Some(curr_value.clone()));
            } else {
                env.borrow_mut().define(param.lexeme.clone(), None);
            }
        }
        let _ = interpreter.exeucute_block(&self.declaration.body, env);
        return Ok(LiteralValue::Nil);
    }
    fn arity(&self) -> usize {
        return self.declaration.params.len();
    }

    fn to_string(&self) -> String {
        return format!("<fn {}>", self.declaration.name.lexeme);
    }
}
