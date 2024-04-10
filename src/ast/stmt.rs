use core::fmt;
use std::string;

use crate::tokens::{token::Token, token_type::TokenType};

use super::expr::Expr;

pub enum Stmt {
    VarDeclarationStmt {
        Identifier: String,
        isConstant: bool,
        assignedValue: Expr,
        explicitType: Option<Token>,
    },
    ExpressionStmt {
        expression: Expr,
    },
    BlockStmt {
        Body: Vec<Stmt>,
    },
    IfStmt {
        condition: Expr,
        consequent: Box<Stmt>,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::VarDeclarationStmt {
                Identifier,
                isConstant,
                assignedValue,
                explicitType,
            } => {
                write!(
                    f,
                    "let {}{}{} = {}",
                    if *isConstant { "const " } else { "" },
                    Identifier,
                    if let Some(explicit_type) = explicitType {
                        format!(": of type {}", explicit_type.lexeme)
                    } else {
                        String::new()
                    },
                    assignedValue
                )
            }
            Stmt::ExpressionStmt { expression } => {
                write!(f, "{}", expression)
            }
            Stmt::BlockStmt { Body } => {
                writeln!(f, "{{")?;
                for stmt in Body {
                    writeln!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::IfStmt {
                condition,
                consequent,
            } => {
                writeln!(f, "{{}}")?;
            }
        }
    }
}
