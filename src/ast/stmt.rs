use core::fmt;

use crate::tokens::token::Token;

use super::expr::Expr;

pub enum Stmt {
    ExpressionStmt {
        expression: Expr,
    },
    BlockStmt {
        body: Vec<Stmt>,
    },
    IfStmt {
        condition: Expr,
        consequent: Box<Stmt>,
        alternate: Option<Box<Stmt>>,
    },
    VarDeclarationStmt {
        identifier: String,
        isConstant: bool,
        assignedValue: Expr,
        explicitType: Option<Token>,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::ExpressionStmt { expression } => {
                write!(f, "{}", expression)
            }
            Stmt::BlockStmt { body } => {
                writeln!(f, "Block Stmt: {{")?;
                for stmt in body {
                    writeln!(f, "    {}", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::IfStmt {
                condition,
                consequent,
                alternate,
            } => {
                writeln!(f, "If Statement:")?;
                writeln!(f, "    Condition: {}", condition)?;
                writeln!(f, "    Consequent: {}", consequent)?;
                if alternate.is_some() {
                    writeln!(
                        f,
                        "Alternate: {}",
                        Some(alternate).unwrap().as_ref().unwrap()
                    )?;
                }
                Ok(())
            }
            Stmt::VarDeclarationStmt {
                identifier,
                isConstant,
                assignedValue,
                explicitType,
            } => {
                write!(
                    f,
                    "let {}{}{} = {}",
                    if *isConstant { "const " } else { "" },
                    identifier,
                    if let Some(explicitType) = explicitType {
                        format!(": of type {}", explicitType.lexeme)
                    } else {
                        String::new()
                    },
                    assignedValue
                )
            }
        }
    }
}
