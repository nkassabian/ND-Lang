use core::fmt;

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
                    writeln!(f, "{}", Some(alternate).unwrap().as_ref().unwrap())?;
                }
                Ok(())
            }
        }
    }
}
