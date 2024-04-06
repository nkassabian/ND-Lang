use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{ast::expr::Expr, tokens::token_type::TokenType};

use super::parser::Parser;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum PREC {
    defalt_bp = 0,
    primary,
    number,
    comma,
    assignment,
    logical,
    relational,
    additive,
    multiplicative,
    // add power here
    unary,
    call,
    member,
}

pub static mut BP_TABLE: Lazy<HashMap<TokenType, PREC>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(TokenType::NUMBER, PREC::primary);
    map.insert(TokenType::STAR, PREC::multiplicative);

    map.insert(TokenType::PLUS, PREC::additive);
    map.insert(TokenType::EOF, PREC::defalt_bp);

    map
});
