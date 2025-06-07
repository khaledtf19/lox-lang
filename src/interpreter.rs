use crate::{
    ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralValue, UnaryExpr},
    token::TokenType,
};

#[derive(Debug)]
struct VisitingError {
    pub expr: Expr,
    pub message: String,
}

impl VisitingError {
    fn new(expr: Expr, message: String) -> Self {
        Self { message, expr }
    }
}

#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    pub fn visitLitearalExpr(&self, expr: LiteralExpr) -> Result<LiteralValue, VisitingError> {
        Ok(expr.value)
    }
    pub fn visitGroupingExpr(&self, expr: GroupingExpr) -> Result<LiteralValue, VisitingError> {
        self.evaluate(expr.expression)
    }
    pub fn visitUnaryExpr(&self, expr: UnaryExpr) -> Result<LiteralValue, VisitingError> {
        let right = self.evaluate(expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => match right {
                LiteralValue::Number(num) => return Ok(LiteralValue::Number(-num)),

                _ => unreachable!(),
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
    pub fn visitBinaryExpr(&self, expr: BinaryExpr) -> Result<LiteralValue, VisitingError> {
        let left = self.evaluate(expr.left)?;
        let right = self.evaluate(expr.right)?;

        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => match expr.operator.token_type {
                TokenType::MINUS => return Ok(LiteralValue::Number(l - r)),
                TokenType::SLASH => return Ok(LiteralValue::Number(l / r)),
                TokenType::STAR => return Ok(LiteralValue::Number(l * r)),
                TokenType::PLUS => return Ok(LiteralValue::Number(l + r)),
                TokenType::GREATER => return Ok(LiteralValue::Boolean(l > r)),
                TokenType::GREATEREQUAL => return Ok(LiteralValue::Boolean(l >= r)),
                TokenType::LESS => return Ok(LiteralValue::Boolean(l < r)),
                TokenType::LESSEQUAL => return Ok(LiteralValue::Boolean(l <= r)),
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(l == r)),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(l != r)),

                TokenType::EQUAL => todo!(),
                _ => todo!(),
            },
            (LiteralValue::String(l), LiteralValue::String(r)) => match expr.operator.token_type {
                TokenType::PLUS => return Ok(LiteralValue::String(l + &r)),
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(l == r)),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(l != r)),
                _ => unreachable!(),
            },

            (l, r) => match expr.operator.token_type {
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(self.isEqual(l, r))),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(self.isEqual(l, r))),
                _ => unreachable!(),
            },
        }
    }
    fn isEqual(&self, l: LiteralValue, r: LiteralValue) -> bool {
        match (l, r) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => l == r,
            (LiteralValue::String(l), LiteralValue::String(r)) => l == r,
            (LiteralValue::Boolean(l), LiteralValue::Boolean(r)) => l == r,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            _ => false,
        }
    }
    pub fn evaluate(&self, expr: Box<Expr>) -> Result<LiteralValue, VisitingError> {
        match *expr {
            Expr::Binary(binary_expr) => self.visitBinaryExpr(binary_expr),
            Expr::Grouping(grouping_expr) => self.visitGroupingExpr(grouping_expr),
            Expr::Literal(literal_expr) => self.visitLitearalExpr(literal_expr),
            Expr::Unary(unary_expr) => self.visitUnaryExpr(unary_expr),
            Expr::Separator(_) => todo!(),
            Expr::Ternary(_) => todo!(),
        }
    }
}
