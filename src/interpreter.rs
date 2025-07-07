use crate::{
    ast::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralValue, UnaryExpr},
    error::RunTimeError,
    lox::Lox,
    stmt::Stmt,
    token::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {
    pub has_error: bool,
}

type InterpreterResult<T> = std::result::Result<T, RunTimeError>;

impl Interpreter {
    pub fn new() -> Self {
        Self { has_error: false }
    }
    pub fn visitLitearalExpr(&self, expr: LiteralExpr) -> Result<LiteralValue, RunTimeError> {
        Ok(expr.value.clone())
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
    pub fn visitBinaryExpr(&self, expr: BinaryExpr) -> Result<LiteralValue, RunTimeError> {
        let left = self.evaluate(*expr.left)?;
        let right = self.evaluate(*expr.right)?;

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
                TokenType::EQUALEQUAL => return Ok(LiteralValue::Boolean(self.isEqual(l, r))),
                TokenType::BANGEQUAL => return Ok(LiteralValue::Boolean(self.isEqual(l, r))),
                _ => Err(RunTimeError::new(
                    expr.operator.clone(),
                    "Unexpected operator".to_string(),
                )),
            },
        }
    }

    pub fn visit_expresstion_stmt(&mut self, expr: Expr) {
        self.evaluate(expr);
    }

    pub fn visit_print_stmt(&mut self, expr: Expr) {
        match self.evaluate(expr) {
            Ok(value) => println!("{}", self.stringify(value)),
            Err(_) => {
                self.has_error = true;
            }
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
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        statements.iter().for_each(|stmt| match self.execute(stmt) {
            Ok(st) => {}
            Err(_) => self.has_error = true,
        });
    }
    pub fn execute(&mut self, statement: &Stmt) -> InterpreterResult<()> {
        match &statement.expresstion {
            crate::stmt::StmtExpr::Print(expr) => self.visit_print_stmt(expr.clone()),
            crate::stmt::StmtExpr::Expresstion(expr) => self.visit_expresstion_stmt(expr.clone()),
        }
        Ok(())
    }
}
