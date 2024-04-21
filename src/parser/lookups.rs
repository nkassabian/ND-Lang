use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    ast::{expr::Expr, stmt::Stmt},
    tokens::token_type::TokenType,
};

use super::{helpers::*, parser::Parser};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum PREC {
    DefaultBp = 0,
    Primary = 1,
    // Comma,
    Assignment = 3,
    Logical = 4,
    Relational = 5,
    Additive = 6,
    Multiplicative = 7,
    Power = 8,
    Unary = 9,
    Prefix = 10,
    // Call,
    // Member,
    Group = 12,
}

pub static mut BP_TABLE: Lazy<HashMap<TokenType, PREC>> = Lazy::new(|| {
    let mut map = HashMap::new();

    //Assignment
    map.insert(TokenType::EQUAL, PREC::Assignment);
    map.insert(TokenType::PLUSEQUALS, PREC::Assignment);
    map.insert(TokenType::MINUSEQUALS, PREC::Assignment);

    //Primary
    map.insert(TokenType::NUMBER, PREC::Primary);
    map.insert(TokenType::STRING, PREC::Primary);
    map.insert(TokenType::IDENTIFIER, PREC::Primary);

    //Additive
    map.insert(TokenType::PLUS, PREC::Additive);
    map.insert(TokenType::MINUS, PREC::Additive);

    //Multiplicative
    map.insert(TokenType::STAR, PREC::Multiplicative);
    map.insert(TokenType::SLASH, PREC::Multiplicative);
    map.insert(TokenType::MODULO, PREC::Multiplicative);

    //Relational
    map.insert(TokenType::LESS, PREC::Relational);
    map.insert(TokenType::GREATER, PREC::Relational);
    map.insert(TokenType::LESSEQUAL, PREC::Relational);
    map.insert(TokenType::GREATEREQUAL, PREC::Relational);

    //Logical
    map.insert(TokenType::OR, PREC::Logical);

    //Power
    map.insert(TokenType::POW, PREC::Power);

    //Unary
    map.insert(TokenType::BANG, PREC::Unary);

    //Group
    map.insert(TokenType::LEFTPAREN, PREC::Group);

    //Default
    map.insert(TokenType::EOF, PREC::DefaultBp);
    map
});

pub type LedHandler = fn(&mut Parser, Expr) -> Expr;
pub type NudHandler = fn(&mut Parser) -> Expr;
pub type StmtHandler = fn(&mut Parser) -> Stmt;

pub fn create_nud_lookups() -> HashMap<TokenType, NudHandler> {
    let mut map = HashMap::new();

    // Literals & Symbols
    map.insert(TokenType::NUMBER, parse_num as NudHandler);
    map.insert(TokenType::STRING, parse_string as NudHandler);
    map.insert(TokenType::IDENTIFIER, parse_identifier as NudHandler);

    // Unary/Prefix
    map.insert(TokenType::MINUS, parse_unary as NudHandler);
    map.insert(TokenType::BANG, parse_unary as NudHandler);

    // Grouping Expr
    map.insert(TokenType::LEFTPAREN, parse_grouping_expr as NudHandler);
    map
}

pub fn create_led_lookups() -> HashMap<TokenType, LedHandler> {
    let mut map = HashMap::new();

    //Assignment
    map.insert(TokenType::EQUAL, parse_assignment_expr as LedHandler);
    map.insert(TokenType::PLUSEQUALS, parse_assignment_expr as LedHandler);
    map.insert(TokenType::MINUSEQUALS, parse_assignment_expr as LedHandler);

    // Logical
    map.insert(TokenType::AND, parse_binary_expr as LedHandler);
    map.insert(TokenType::OR, parse_binary_expr as LedHandler);

    // Relational
    map.insert(TokenType::GREATER, parse_binary_expr as LedHandler);
    map.insert(TokenType::LESS, parse_binary_expr as LedHandler);
    map.insert(TokenType::LESSEQUAL, parse_binary_expr as LedHandler);
    map.insert(TokenType::GREATEREQUAL, parse_binary_expr as LedHandler);

    //Additive & Multiplicative
    map.insert(TokenType::PLUS, parse_binary_expr as LedHandler);
    map.insert(TokenType::MINUS, parse_binary_expr as LedHandler);
    map.insert(TokenType::STAR, parse_binary_expr as LedHandler);
    map.insert(TokenType::SLASH, parse_binary_expr as LedHandler);
    map.insert(TokenType::MODULO, parse_binary_expr as LedHandler);
    map.insert(TokenType::POW, parse_binary_expr as LedHandler);

    map
}

pub fn create_stmt_lookups() -> HashMap<TokenType, StmtHandler> {
    let mut map = HashMap::new();

    // Populate the map
    //map.insert(TokenType::LET, parse_var_decl_stmt as StmtHandler);
    //map.insert(TokenType::CONST, parse_var_decl_stmt as StmtHandler);

    map.insert(TokenType::LEFTBRACE, parse_block_stmt as StmtHandler);
    map.insert(TokenType::IF, parse_if_stmt as StmtHandler);

    map
}
