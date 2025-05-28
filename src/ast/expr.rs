use std::fmt::Display;

use crate::token::{Token, TokenLiteral, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: LiteralValue,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LiteralValue::Number(v) => write!(f, "{}", v),
            LiteralValue::String(v) => write!(f, "\"{}\"", v),
            LiteralValue::Boolean(v) => write!(f, "{}", v),
            LiteralValue::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(expr) => write!(f, "({} {} {})", expr.operator, expr.left, expr.right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr.expression),
            Expr::Literal(expr) => write!(f, "{}", expr.value),
            Expr::Unary(expr) => write!(f, "({} {})", expr.operator, expr.right),
        }
    }
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary(BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    pub fn grouping(expr: Expr) -> Self {
        Expr::Grouping(GroupingExpr {
            expression: Box::new(expr),
        })
    }
    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary(UnaryExpr {
            operator,
            right: Box::new(right),
        })
    }
    pub fn literal(literal_value: LiteralValue) -> Self {
        Expr::Literal(LiteralExpr {
            value: literal_value,
        })
    }
}
