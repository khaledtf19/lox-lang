use crate::ast::expr::Expr;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub expresstion: StmtExpr,
}

#[derive(Debug, Clone)]
pub enum StmtExpr {
    Print(Expr),
    Expresstion(Expr)
}

impl Stmt {
    pub fn print_stmt(value: Expr) -> Self {
        Stmt { expresstion: StmtExpr::Print(value)}
    }

    pub fn expresstion_stmt(value: Expr) -> Self {
        Stmt { expresstion: StmtExpr::Expresstion(value) }
    }
}
