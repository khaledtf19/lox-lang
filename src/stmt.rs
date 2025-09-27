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
}
