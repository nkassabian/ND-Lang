use core::fmt;

use crate::tokens::token::Token;

#[derive(Debug, PartialEq)] // Add PartialEq derive
pub enum Expr {
    // --------------------
    // Literal Expressions
    // --------------------
    Number(String),
    String(String),
    Identifier(String),

    // --------------------
    // Complex Expressions
    // --------------------
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Assignment {
        assignee: Box<Expr>,
        op: Token,
        assigned: Box<Expr>,
    },
    Grouping {
        group: Box<Expr>,
    }, // Add more node types as needed
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(x) => write!(f, "{}", x),
            Expr::String(x) => write!(f, "\"{}\"", x),
            Expr::Unary { op, right } => write!(f, "({}{})", op.lexeme, right),
            Expr::Identifier(s) => write!(f, "{}", s),
            Expr::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op.lexeme, right),
            Expr::Grouping { group } => write!(f, "({})", group),
            Expr::Assignment {
                assignee,
                op,
                assigned,
            } => {
                write!(f, "Assigned {} {} {}", assignee, op.lexeme, assigned)
            } // Add more match arms for other node types if needed
        }
    }
}
