use std::{fmt::Debug, rc::Rc};

use crate::{ast::expr::LiteralValue, error::RunTimeError, interpreter::Interpreter};

pub enum Callable {
    Function(Rc<dyn LoxCallable>),
}

impl Clone for Callable {
    fn clone(&self) -> Self {
        match self {
            Callable::Function(f) => Callable::Function(Rc::clone(f)),
        }
    }
}
impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(arg0) => f.debug_tuple("Function").field(&arg0.to_string()).finish(),
        }
    }
}

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[LiteralValue],
    ) -> Result<LiteralValue, RunTimeError>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub callable: fn(&mut Interpreter, &[LiteralValue]) -> Result<LiteralValue, RunTimeError>,
    pub params: usize,
}

impl NativeFunction {
    pub fn new(
        callable: fn(&mut Interpreter, &[LiteralValue]) -> Result<LiteralValue, RunTimeError>,
        params: usize,
    ) -> Self {
        Self { callable, params }
    }
}

impl LoxCallable for NativeFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[LiteralValue],
    ) -> Result<LiteralValue, RunTimeError> {
        return (self.callable)(interpreter, arguments);
    }

    fn arity(&self) -> usize {
        self.params
    }
    fn to_string(&self) -> String {
        return "<Native Function>".to_string();
    }
}
