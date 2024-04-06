use core::fmt;

use crate::tokens::{token::Token, token_type::TokenType};

pub enum Expr {
    NumberLiteral(String),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    // Add more node types as needed
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::NumberLiteral(x) => write!(f, "{}", x),
            Expr::Identifier(s) => write!(f, "{}", s),
            Expr::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op.lexeme, right),
            // Add more match arms for other node types if needed
        }
    }
}
