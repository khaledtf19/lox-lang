use crate::{
    ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralValue, UnaryExpr},
    error::RunTimeError,
    lox::Lox,
    token::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {
    pub hadError: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { hadError: false }
    }
    pub fn visitLitearalExpr(&self, expr: LiteralExpr) -> Result<LiteralValue, RunTimeError> {
        Ok(expr.value)
    }
    pub fn visitGroupingExpr(&self, expr: GroupingExpr) -> Result<LiteralValue, RunTimeError> {
        self.evaluate(*expr.expression)
    }
    pub fn visitUnaryExpr(&self, expr: UnaryExpr) -> Result<LiteralValue, RunTimeError> {
        let right = self.evaluate(*expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => match right {
                LiteralValue::Number(num) => return Ok(LiteralValue::Number(-num)),

                _ => Err(RunTimeError::new(
                    expr.operator,
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
    pub fn visitBinaryExpr(&self, expr: BinaryExpr) -> Result<LiteralValue, RunTimeError> {
        let left = self.evaluate(*expr.left)?;
        let right = self.evaluate(*expr.right)?;

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
                _ => Err(RunTimeError::new(
                    expr.operator,
                    "Unexpected operator".to_string(),
                )),
            },
            (LiteralValue::String(l), LiteralValue::String(r)) => match expr.operator.token_type {
                TokenType::PLUS => return Ok(LiteralValue::String(l + &r)),
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(l == r)),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(l != r)),
                _ => Err(RunTimeError::new(
                    expr.operator,
                    "Unexpected operator".to_string(),
                )),
            },

            (l, r) => match expr.operator.token_type {
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(self.isEqual(l, r))),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(self.isEqual(l, r))),
                _ => Err(RunTimeError::new(
                    expr.operator,
                    "Unexpected operator".to_string(),
                )),
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
    fn evaluate(&self, expr: Expr) -> Result<LiteralValue, RunTimeError> {
        match expr {
            Expr::Binary(binary_expr) => self.visitBinaryExpr(binary_expr),
            Expr::Grouping(grouping_expr) => self.visitGroupingExpr(grouping_expr),
            Expr::Literal(literal_expr) => self.visitLitearalExpr(literal_expr),
            Expr::Unary(unary_expr) => self.visitUnaryExpr(unary_expr),
            Expr::Separator(_) => todo!(),
            Expr::Ternary(_) => todo!(),
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
    pub fn interpret(&mut self, expr: Expr) {
        match self.evaluate(expr) {
            Ok(value) => {
                println!("{}", self.stringify(value))
            }
            Err(err) => {
                self.hadError = true;
                Lox::runTimeErro(err)
            }
        }
    }
}
