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
                        format!(": {}", explicit_type.lexeme)
                    } else {
                        String::new()
                    },
                    assignedValue
                )
            }
            Stmt::ExpressionStmt { expression } => {
                write!(f, "{}", expression)
            }
        }
    }
}
