use std::{cell::RefCell, rc::Rc};

use crate::{
    Environment::{Env, Environment},
    ast::expr::{
        AssessmentExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralValue, LogicalExpr,
        UnaryExpr, VariableExpr,
    },
    error::RunTimeError,
    stmt::{ControlFlow, Stmt, StmtExpr, StmtResult},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Interpreter {
    pub has_error: bool,
    environment: Env,
}

type InterpreterResult<T> = std::result::Result<T, RunTimeError>;

impl Interpreter {
    pub fn new() -> Self {
        Self {
            has_error: false,
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }
    pub fn visit_litearal_expr(&self, expr: &LiteralExpr) -> Result<LiteralValue, RunTimeError> {
        Ok(expr.value.clone())
    }
    pub fn visit_logical_exper(&mut self, expr: &LogicalExpr) -> InterpreterResult<LiteralValue> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.token_type == TokenType::OR {
            if self.is_truthy(left.clone()) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(left.clone()) {
                return Ok(left);
            }
        }
        return self.evaluate(&expr.right);
    }
    pub fn visit_grouping_expr(
        &mut self,
        expr: &GroupingExpr,
    ) -> Result<LiteralValue, RunTimeError> {
        self.evaluate(&expr.expression)
    }
    pub fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<LiteralValue, RunTimeError> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.token_type {
            TokenType::MINUS => match right {
                LiteralValue::Number(num) => return Ok(LiteralValue::Number(-num)),
                _ => Err(RunTimeError::new(
                    expr.operator.clone(),
                    "Expected a number".to_string(),
                )),
            },
            // if Nil = false & !Nil = true & !any = fasle
            TokenType::BANG => match right {
                LiteralValue::Boolean(bol) => return Ok(LiteralValue::Boolean(!bol)),
                LiteralValue::Nil => return Ok(LiteralValue::Boolean(true)),
                _ => return Ok(LiteralValue::Boolean(false)),
            },

            _ => {
                unreachable!()
            }
        }
    }
    pub fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<LiteralValue, RunTimeError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => match expr.operator.token_type {
                TokenType::MINUS => return Ok(LiteralValue::Number(l - r)),
                TokenType::SLASH => {
                    if r == 0.0 {
                        return Err(RunTimeError::new(
                            expr.operator.clone(),
                            "Can't divide by Zero".to_string(),
                        ));
                    }
                    return Ok(LiteralValue::Number(l / r));
                }
                TokenType::STAR => return Ok(LiteralValue::Number(l * r)),
                TokenType::PLUS => return Ok(LiteralValue::Number(l + r)),
                TokenType::GREATER => return Ok(LiteralValue::Boolean(l > r)),
                TokenType::GREATEREQUAL => return Ok(LiteralValue::Boolean(l >= r)),
                TokenType::LESS => return Ok(LiteralValue::Boolean(l < r)),
                TokenType::LESSEQUAL => return Ok(LiteralValue::Boolean(l <= r)),
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(l == r)),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(l != r)),

                TokenType::EQUAL => todo!(),
                _ => Err(RunTimeError::new(
                    expr.operator.clone(),
                    "Unexpected operator".to_string(),
                )),
            },
            (LiteralValue::String(l), LiteralValue::String(r)) => match expr.operator.token_type {
                TokenType::PLUS => return Ok(LiteralValue::String(l + &r)),
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(l == r)),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(l != r)),
                _ => Err(RunTimeError::new(
                    expr.operator.clone(),
                    "Unexpected operator".to_string(),
                )),
            },
            (LiteralValue::String(st), LiteralValue::Number(num)) => match expr.operator.token_type
            {
                TokenType::PLUS => {
                    return Ok(LiteralValue::String(st.to_string() + &num.to_string()));
                }
                _ => Err(RunTimeError::new(
                    expr.operator.clone(),
                    "Unexpected operator".to_string(),
                )),
            },

            (LiteralValue::Number(num), LiteralValue::String(st)) => match expr.operator.token_type
            {
                TokenType::PLUS => {
                    return Ok(LiteralValue::String(st.to_string() + &num.to_string()));
                }
                _ => Err(RunTimeError::new(
                    expr.operator.clone(),
                    "Unexpected operator".to_string(),
                )),
            },
            (l, r) => match expr.operator.token_type {
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(self.is_equal(l, r))),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(self.is_equal(l, r))),
                _ => Err(RunTimeError::new(
                    expr.operator.clone(),
                    "Unexpected operator".to_string(),
                )),
            },
        }
    }

    pub fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<LiteralValue, RunTimeError> {
        return self.environment.borrow().get(expr.name.clone());
    }

    pub fn visit_expresstion_stmt(&mut self, expr: &Expr) -> StmtResult {
        self.evaluate(&expr)?;
        return Ok(None);
    }
    pub fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> StmtResult {
        let is_true = self.evaluate(condition)?;
        if self.is_truthy(is_true) {
            return self.execute(then_branch);
        } else {
            if let Some(branch2) = else_branch {
                return self.execute(branch2);
            }
            Ok(None)
        }
    }
    fn is_truthy(&self, value: LiteralValue) -> bool {
        match value {
            LiteralValue::String(_) | LiteralValue::Number(_) => return true,
            LiteralValue::Boolean(bol) => return bol,
            LiteralValue::Nil => return false,
        }
    }

    pub fn visit_print_stmt(&mut self, expr: &Expr) -> StmtResult {
        match self.evaluate(&expr) {
            Ok(value) => println!("{}", self.stringify(value)),
            Err(_) => {
                self.has_error = true;
            }
        }
        Ok(None)
    }

    pub fn visit_var_stmt(&mut self, name: &Token, init: &Option<Expr>) -> StmtResult {
        match init {
            Some(expr) => match self.evaluate(&expr) {
                Ok(val) => {
                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.clone(), Some(val));
                }
                Err(_) => {
                    self.has_error = true;
                }
            },
            None => {
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), None);
            }
        }
        return Ok(None);
    }
    pub fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> StmtResult {
        loop {
            let value = self.evaluate(condition)?;
            if self.is_truthy(value) {
                if let Some(flow) = self.execute(body)? {
                    match flow {
                        ControlFlow::Return(literal_value) => {
                            return Ok(Some(ControlFlow::Return(literal_value)));
                        }
                        ControlFlow::Break => return Ok(None),
                        ControlFlow::Continue => return Ok(None),
                    }
                }
            } else {
                break;
            }
        }
        Ok(None)
    }
    pub fn visit_break_stmt(&self) -> StmtResult {
        return Ok(Some(ControlFlow::Break));
    }
    pub fn visit_assign_expr(&mut self, exper: &AssessmentExpr) -> InterpreterResult<LiteralValue> {
        let value = self.evaluate(&exper.value)?;
        self.environment
            .borrow_mut()
            .assign(exper.name.clone(), value.clone())?;
        Ok(value.clone())
    }

    fn is_equal(&self, l: LiteralValue, r: LiteralValue) -> bool {
        match (l, r) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => l == r,
            (LiteralValue::String(l), LiteralValue::String(r)) => l == r,
            (LiteralValue::Boolean(l), LiteralValue::Boolean(r)) => l == r,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            _ => false,
        }
    }
    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RunTimeError> {
        match expr {
            Expr::Binary(binary_expr) => self.visit_binary_expr(binary_expr),
            Expr::Grouping(grouping_expr) => self.visit_grouping_expr(grouping_expr),
            Expr::Literal(literal_expr) => self.visit_litearal_expr(literal_expr),
            Expr::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::Separator(_) => todo!(),
            Expr::Ternary(_) => todo!(),
            Expr::Variable(var_expr) => self.visit_variable_expr(var_expr),
            Expr::Assgin(assessment_expr) => self.visit_assign_expr(assessment_expr),
            Expr::Logical(logical_expr) => self.visit_logical_exper(logical_expr),
        }
    }
    fn stringify(&self, value: LiteralValue) -> String {
        match value {
            LiteralValue::String(str) => str,
            LiteralValue::Number(num) => {
                let numStr = num.to_string();
                if numStr.ends_with(".0") {
                    return numStr.split_at(numStr.len() - 2).0.to_string();
                }
                numStr
            }
            LiteralValue::Boolean(bol) => bol.to_string(),
            LiteralValue::Nil => "Nil".to_string(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        statements.iter().for_each(|stmt| match self.execute(stmt) {
            Ok(_) => {}
            Err(_) => self.has_error = true,
        });
    }
    pub fn execute(&mut self, statement: &Stmt) -> StmtResult {
        match &statement.expresstion {
            StmtExpr::Print(expr) => return self.visit_print_stmt(expr),
            StmtExpr::Expresstion(expr) => return self.visit_expresstion_stmt(expr),
            StmtExpr::Var(name, init) => return self.visit_var_stmt(name, init),
            StmtExpr::Block(statements) => return self.visit_block_stmt(statements),
            StmtExpr::If(condition, then_branch, else_branch) => {
                return self.visit_if_stmt(condition, then_branch, else_branch.as_deref());
            }
            StmtExpr::While(expr, stmt) => return self.visit_while_stmt(&expr, &stmt),
            StmtExpr::Break => return self.visit_break_stmt(),
        }
    }
    pub fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> StmtResult {
        return self.exeucute_block(
            statements,
            Rc::new(RefCell::new(Environment::new(Some(
                self.environment.clone(),
            )))),
        );
    }
    pub fn exeucute_block(&mut self, statements: &Vec<Stmt>, environment: Env) -> StmtResult {
        let previous = self.environment.clone();
        self.environment = environment;

        let result = (|| {
            for stmt in statements {
                if let Some(flow) = self.execute(stmt)? {
                    return Ok(Some(flow));
                }
            }
            Ok(None)
        })();

        self.environment = previous;
        result
    }
}
