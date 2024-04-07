use core::fmt;

use crate::tokens::token::Token;

pub enum Expr {
    Number(String),
    String(String),
    Identifier(String),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping {
        group: Box<Expr>,
    }, // Add more node types as needed
}

// pub enum Stmt {
//     VarDeclarationStmt {
//         identifier: String,
//         constant: bool,
//         AssignedValue: Expr,
//         // ExplicitType: TokenType,
//     },
// }

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(x) => write!(f, "{}", x),
            Expr::String(x) => write!(f, "\"{}\"", x),
            Expr::Unary { op, right } => write!(f, "({}{})", op.lexeme, right),
            Expr::Identifier(s) => write!(f, "{}", s),
            Expr::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op.lexeme, right),
            Expr::Grouping { group } => write!(f, "({})", group),
            // Add more match arms for other node types if needed
        }
    }
}
