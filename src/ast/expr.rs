use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Ternary(TernaryExpr),
    Literal(LiteralExpr),
    Separator(SeparatorExpr),
    Grouping(GroupingExpr),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}


#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: LiteralValue
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

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(expr) => write!(f, "({} {} {})", expr.operator, expr.left, expr.right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr.expression),
            Expr::Literal(expr) => write!(f, "{}", expr.value),
            Expr::Unary(expr) => write!(f, "({} {})", expr.operator, expr.right),
            Expr::Separator(expr) => write!(f, "(separator {} {})", expr.left, expr.right),
            Expr::Ternary(exper) => write!(f, "(ternary {} {} {})", exper.condition, exper.left, exper.right)
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
    pub fn separator(left: Expr, right: Expr) -> Self{
        Expr::Separator(SeparatorExpr {left: Box::new(left), right: Box::new(right)})
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
    pub fn ternary(condition: Expr,left: Expr,  right: Expr)-> Self{
    Expr::Ternary(TernaryExpr {
            condition: Box::new(condition),
            right: Box::new(right),
            left: Box::new(left)
        })
    }
}
