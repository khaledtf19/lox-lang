use std::{collections::HashMap, ops::Deref};

use crate::{
    error::LoxError,
    expr::{
        AssginExpr, BinaryExpr, CallExpr, Expr, ExprKind, GroupingExpr, LiteralExpr, LogicalExpr,
        UnaryExpr, VariableExpr,
    },
    interpreter::Interpreter,
    stmt::{
        BlockStmt, ExpresstionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, StmtExpr,
        VarStmt, WhileStmt,
    },
    token::Token,
};

pub struct Resolver<'a> {
    pub had_error: bool,
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    curr_function: Option<FunctionType>,
}

#[derive(Debug, Clone, Copy)]
pub enum FunctionType {
    FUNCTION,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            had_error: false,
            interpreter,
            scopes: vec![],
            curr_function: None,
        }
    }
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) {
        self.begin_scope();
        self.resolve_stmts(&stmt.statements);
        self.end_scope();
    }
    fn visit_expresstion_stmt(&mut self, stmt: &ExpresstionStmt) {
        self.resolve_exper(&stmt.expresstion);
    }
    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(stmt, FunctionType::FUNCTION);
    }
    fn visit_if_stmt(&mut self, stmt: &IfStmt) {
        self.resolve_exper(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(else_stmt) = &stmt.else_branch {
            self.resolve_stmt(else_stmt);
        }
    }
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) {
        self.resolve_exper(&stmt.expr);
    }
    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) {
        if self.curr_function.is_none() {
            LoxError::token_errro(
                &stmt.keyword,
                "Can't return from top-level code.".to_string(),
            );
            self.had_error = true;
        }
        if let Some(value) = &stmt.value {
            self.resolve_exper(value);
        }
    }
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) {
        self.resolve_exper(&stmt.condition);
        self.resolve_stmt(&stmt.body);
    }
    fn visit_var_stmt(&mut self, stmt: &VarStmt) {
        self.declare(&stmt.name);
        if let Some(init) = &stmt.initializer {
            self.resolve_exper(init);
        }
        self.define(&stmt.name);
    }
    fn visit_assign_expr(&mut self, id: usize, expr: &AssginExpr) {
        self.resolve_exper(expr.value.deref());
        self.resolve_local(
            &Expr {
                kind: ExprKind::Assgin(expr.clone()),
                id: id,
            },
            &expr.name,
        );
    }
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) {
        self.resolve_exper(&expr.left);
        self.resolve_exper(&expr.right);
    }
    fn visit_call_expr(&mut self, expr: &CallExpr) {
        self.resolve_exper(&expr.callee);

        for argument in &expr.arguments {
            self.resolve_exper(argument);
        }
    }
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) {
        self.resolve_exper(&expr.expression);
    }
    fn visit_literal_expr(&mut self, _: &LiteralExpr) {}
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) {
        self.resolve_exper(&expr.left);
        self.resolve_exper(&expr.right);
    }
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) {
        self.resolve_exper(&expr.right);
    }
    pub fn resolve_stmts(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }
    fn visit_break_stmt(&mut self) {}
    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match &stmt.expresstion {
            StmtExpr::Print(stmt) => return self.visit_print_stmt(stmt),
            StmtExpr::Expresstion(stmt) => return self.visit_expresstion_stmt(stmt),
            StmtExpr::Var(stmt) => return self.visit_var_stmt(stmt),
            StmtExpr::Block(block) => return self.visit_block_stmt(block),
            StmtExpr::If(stmt) => {
                return self.visit_if_stmt(stmt);
            }
            StmtExpr::While(stmt) => return self.visit_while_stmt(stmt),
            StmtExpr::Break => return self.visit_break_stmt(),
            StmtExpr::Function(function_stmt) => return self.visit_function_stmt(function_stmt),
            StmtExpr::Return(return_stmt) => self.visit_return_stmt(return_stmt),
        }
    }
    fn resolve_exper(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Binary(binary_expr) => self.visit_binary_expr(binary_expr),
            ExprKind::Grouping(grouping_expr) => self.visit_grouping_expr(grouping_expr),
            ExprKind::Literal(literal_expr) => self.visit_literal_expr(literal_expr),
            ExprKind::Unary(unary_expr) => self.visit_unary_expr(unary_expr),
            ExprKind::Separator(_) => todo!(),
            ExprKind::Ternary(_) => todo!(),
            ExprKind::Variable(var_expr) => self.visit_variable_expr(expr.id, var_expr),
            ExprKind::Assgin(assessment_expr) => self.visit_assign_expr(expr.id, assessment_expr),
            ExprKind::Logical(logical_expr) => self.visit_logical_expr(logical_expr),
            ExprKind::Call(call_expr) => self.visit_call_expr(call_expr),
        }
    }
    fn resolve_function(&mut self, stmt: &FunctionStmt, f_type: FunctionType) {
        let enclosing_function = self.curr_function;
        self.curr_function = Some(f_type);

        self.begin_scope();
        for param in &stmt.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_stmts(&stmt.body);
        self.end_scope();
        self.curr_function = enclosing_function;
    }
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                LoxError::token_errro(
                    name,
                    "Already a variable with this name in this scope.".to_string(),
                );
                self.had_error = true;
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }
    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }
    fn visit_variable_expr(&mut self, id: usize, expr: &VariableExpr) {
        if self
            .scopes
            .last_mut()
            .is_some_and(|val| val.get(&expr.name.lexeme) == Some(&false))
        {
            LoxError::token_errro(
                &expr.name,
                "Can't read local variable in its own initializer.".to_string(),
            );
            self.had_error = true;
        }

        self.resolve_local(
            &Expr {
                kind: ExprKind::Variable(expr.clone()),
                id: id,
            },
            &expr.name,
        );
    }
    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme.clone()) {
                self.interpreter.resolve(expr.id, self.scopes.len() - 1 - i);
                return;
            }
        }
        // Not found in any local scope → treat as global
    }
}
