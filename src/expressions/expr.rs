use core::fmt;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::tokens::{token::Token, token_type::TokenType};

pub enum Expr {
    NumberLiteral(i32),
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

#[derive(PartialEq, PartialOrd)]
pub enum PREC {
    defalt_bp = 0,
    comma,
    assignment,
    logical,
    relational,
    additive,
    multiplicative,
    unary,
    call,
    member,
    primary,
}

pub static PRECEDENCE_TABLE: Lazy<HashMap<TokenType, PREC>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(TokenType::EOF, PREC::defalt_bp);
    // Parentheses have the highest precedence
    map.insert(TokenType::LEFTPAREN, PREC::call);
    map.insert(TokenType::RIGHTPAREN, PREC::call);
    map.insert(TokenType::NUMBER, PREC::defalt_bp);

    // Power operator has precedence level 3
    map.insert(TokenType::POW, PREC::multiplicative);

    // Multiplication, division, and modulus have precedence level 2
    map.insert(TokenType::STAR, PREC::multiplicative);
    map.insert(TokenType::SLASH, PREC::multiplicative);
    map.insert(TokenType::MODULO, PREC::multiplicative);

    // Addition and subtraction have precedence level 1
    map.insert(TokenType::PLUS, PREC::additive);
    map.insert(TokenType::MINUS, PREC::additive);

    map
});
