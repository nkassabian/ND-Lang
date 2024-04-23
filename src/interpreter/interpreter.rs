use crate::{
    ast::{expr::Expr, stmt::Stmt},
    tokens::token_type::TokenType,
};

pub struct Interpreter {
    statements: Vec<Stmt>,
}

impl Interpreter {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }

    // HACK: MAKE LOOKUP TABLE TO STORE INTERPRETER FUNCTIONS
    pub fn interpret<T>(&self) -> T {
        for stmt in &self.statements {
            return match stmt {
                Stmt::ExpressionStmt { expression } => self.interpret_expr(&expression),
                _ => todo!(),
            };
        }
    }

    pub fn interpret_expr<T>(&self, expr: &Expr) -> T {
        match expr {
            Expr::BinaryOp { left, right, op } => {
                let left = self.interpret_expr(left);
                let right = self.interpret_expr(right);

                match op.ttype {
                    TokenType::PLUS => return left + right,
                    TokenType::STAR => return left * right,
                    TokenType::POW => return left.pow(right),
                    _ => todo!(),
                }
            }
            Expr::Number(n) => n.parse::<T>().unwrap(),
            _ => todo!(),
        }
    }
}
