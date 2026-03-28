use std::fmt::Display;

use crate::{lox_callable::Callable, token::Token};

#[derive(Debug, Clone)]
pub struct Expr {
    pub id: usize,
    pub kind: ExprKind,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Ternary(TernaryExpr),
    Literal(LiteralExpr),
    Separator(SeparatorExpr),
    Grouping(GroupingExpr),
    Variable(VariableExpr),
    Assgin(AssginExpr),
    Logical(LogicalExpr),
    Call(CallExpr),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct AssginExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Callable),
}
#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: LiteralValue,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LiteralValue::Number(v) => write!(f, "{}", v),
            LiteralValue::String(v) => write!(f, "\"{}\"", v),
            LiteralValue::Boolean(v) => write!(f, "{}", v),
            LiteralValue::Nil => write!(f, "Nil"),
            LiteralValue::Callable(callable) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct SeparatorExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct TernaryExpr {
    pub condition: Box<Expr>,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Binary(expr) => write!(
                f,
                "({} {} {})",
                expr.operator, expr.left.kind, expr.right.kind
            ),
            ExprKind::Grouping(expr) => write!(f, "(group {})", expr.expression.kind),
            ExprKind::Literal(expr) => write!(f, "{}", expr.value),
            ExprKind::Unary(expr) => write!(f, "({} {})", expr.operator, expr.right.kind),
            ExprKind::Separator(expr) => {
                write!(f, "(separator {} {})", expr.left.kind, expr.right.kind)
            }
            ExprKind::Ternary(exper) => write!(
                f,
                "(ternary {} {} {})",
                exper.condition.kind, exper.left.kind, exper.right.kind
            ),
            ExprKind::Variable(expr) => write!(f, "(Variable {})", expr.name),
            ExprKind::Assgin(assessment_expr) => todo!(),
            ExprKind::Logical(logical_expr) => todo!(),
            ExprKind::Call(call_expr) => todo!(),
        }
    }
}

impl Expr {
    pub fn binary(id: usize, left: Expr, operator: Token, right: Expr) -> Self {
        Expr {
            id,
            kind: ExprKind::Binary(BinaryExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }),
        }
    }

    pub fn logical(id: usize, left: Expr, operator: Token, right: Expr) -> Self {
        Expr {
            id,
            kind: ExprKind::Logical(LogicalExpr {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }),
        }
    }

    pub fn grouping(id: usize, expr: Expr) -> Self {
        Expr {
            id,
            kind: ExprKind::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }),
        }
    }
    pub fn separator(id: usize, left: Expr, right: Expr) -> Self {
        Expr {
            id,
            kind: ExprKind::Separator(SeparatorExpr {
                left: Box::new(left),
                right: Box::new(right),
            }),
        }
    }
    pub fn unary(id: usize, operator: Token, right: Expr) -> Self {
        Expr {
            id,
            kind: ExprKind::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }),
        }
    }
    pub fn literal(id: usize, literal_value: LiteralValue) -> Self {
        Expr {
            id,
            kind: ExprKind::Literal(LiteralExpr {
                value: literal_value,
            }),
        }
    }
    pub fn ternary(id: usize, condition: Expr, left: Expr, right: Expr) -> Self {
        Expr {
            id,
            kind: ExprKind::Ternary(TernaryExpr {
                condition: Box::new(condition),
                right: Box::new(right),
                left: Box::new(left),
            }),
        }
    }
    pub fn variable(id: usize, name: Token) -> Self {
        Expr {
            id,
            kind: ExprKind::Variable(VariableExpr { name: name }),
        }
    }

    pub fn assign(id: usize, name: Token, value: Expr) -> Self {
        Expr {
            id,
            kind: ExprKind::Assgin(AssginExpr {
                name,
                value: Box::new(value),
            }),
        }
    }

    pub fn call(id: usize, callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Expr {
            id,
            kind: ExprKind::Call(CallExpr {
                callee: Box::new(callee),
                paren,
                arguments,
            }),
        }
    }
}
