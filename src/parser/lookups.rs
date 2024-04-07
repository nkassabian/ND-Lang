use std::{
    cell::RefCell,
    collections::HashMap,
    path::Prefix,
    sync::{Arc, RwLock},
};

use once_cell::sync::Lazy;

use crate::{
    ast::expr::Expr,
    tokens::{
        token::{self, Token},
        token_type::{self, TokenType},
    },
};

use super::parser::Parser;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum PREC {
    DefaltBp = 0,
    Primary = 1,
    Number,
    Comma,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Power,
    Unary,
    Call,
    Member,
    Group,
}

pub static mut BP_TABLE: Lazy<HashMap<TokenType, PREC>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(TokenType::NUMBER, PREC::Primary);
    map.insert(TokenType::STRING, PREC::Primary);

    map.insert(TokenType::PLUS, PREC::Additive);
    map.insert(TokenType::MINUS, PREC::Additive);

    map.insert(TokenType::STAR, PREC::Multiplicative);
    map.insert(TokenType::SLASH, PREC::Multiplicative);
    map.insert(TokenType::MODULO, PREC::Multiplicative);

    map.insert(TokenType::POW, PREC::Power);

    map.insert(TokenType::LEFTPAREN, PREC::Group);

    map.insert(TokenType::EOF, PREC::DefaltBp);

    map
});

pub type led_handler = fn(&mut Parser, Expr) -> Expr;
pub type nud_handler = fn(&mut Parser) -> Expr;

pub fn create_nud_lookups() -> HashMap<TokenType, nud_handler> {
    let mut map = HashMap::new();

    // Populate the map
    map.insert(TokenType::NUMBER, parse_num as nud_handler);
    map.insert(TokenType::STRING, parse_string as nud_handler);
    map.insert(TokenType::IDENTIFIER, parse_identifier as nud_handler);
    map.insert(TokenType::MINUS, parse_unary as nud_handler);

    map.insert(TokenType::LEFTPAREN, parse_grouping_expr as nud_handler);
    map
}

pub fn create_led_lookups() -> HashMap<TokenType, led_handler> {
    let mut map = HashMap::new();

    // Populate the map
    map.insert(TokenType::PLUS, parse_binary_expr as led_handler);
    map.insert(TokenType::MINUS, parse_binary_expr as led_handler);
    map.insert(TokenType::STAR, parse_binary_expr as led_handler);
    map.insert(TokenType::SLASH, parse_binary_expr as led_handler);
    map.insert(TokenType::MODULO, parse_binary_expr as led_handler);
    map.insert(TokenType::POW, parse_binary_expr as led_handler);
    map
}

fn parse_num(parser: &mut Parser) -> Expr {
    // Implementation
    Expr::Number(parser.at().clone().literal.to_string())
}

fn parse_string(parser: &mut Parser) -> Expr {
    // Implementation
    Expr::String(parser.at().clone().literal.to_string())
}

fn parse_identifier(parser: &mut Parser) -> Expr {
    // Implementation
    Expr::Identifier(parser.at().clone().lexeme.to_string())
}

fn parse_grouping_expr(parser: &mut Parser) -> Expr {
    parser.expect(TokenType::LEFTPAREN);
    let group = parser.parse_expr(PREC::DefaltBp);
    parser.expect(TokenType::RIGHTPAREN);
    Expr::Grouping {
        group: Box::new(group),
    }
}

fn parse_unary(parser: &mut Parser) -> Expr {
    let op = parser.at().clone();
    parser.advance();

    let right = parser.parse_expr(*unsafe { BP_TABLE.get(&op.ttype).unwrap() });
    Expr::Unary {
        op,
        right: Box::new(right),
    }
}

fn parse_binary_expr(parser: &mut Parser, left: Expr) -> Expr {
    let op = parser.at().clone();
    parser.advance();

    let right = parser.parse_expr(*unsafe { BP_TABLE.get(&op.ttype).unwrap() });
    Expr::BinaryOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    }
}
