use crate::{ast::expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub struct Stmt {
    pub expresstion: StmtExpr,
}

#[derive(Debug, Clone)]
pub enum StmtExpr {
    Print(Expr),
    Expresstion(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
}

impl Stmt {
    pub fn print_stmt(value: Expr) -> Self {
        Self {
            expresstion: StmtExpr::Print(value),
        }
    }

    pub fn expresstion_stmt(value: Expr) -> Self {
        Self {
            expresstion: StmtExpr::Expresstion(value),
        }
    }

    pub fn var_stmt(name: Token, initializer: Option<Expr>) -> Self {
        Self {
            expresstion: StmtExpr::Var(name, initializer),
        }
    }

    pub fn block_stmt(statements: Vec<Stmt>) -> Self {
        Self {
            expresstion: StmtExpr::Block(statements),
        }
    }

    pub fn if_stmt(
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    ) -> Self {
        Self {
            expresstion: StmtExpr::If(condition, then_branch, else_branch),
        }
    }

    pub fn while_stmt(condition: Expr, body: Stmt) -> Self {
        Self {
            expresstion: StmtExpr::While(condition, Box::new(body)),
        }
    }
}
