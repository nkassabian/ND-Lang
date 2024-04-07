use super::lookups::{
    create_led_lookups, create_nud_lookups, led_handler, nud_handler, BP_TABLE, PREC,
};
use crate::ast::expr::Expr;
use crate::object::object::Object;
use crate::tokens::token::{self, Token};
use crate::tokens::token_type::TokenType;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    nud_lookup: HashMap<TokenType, nud_handler>,
    led_lookup: HashMap<TokenType, led_handler>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            nud_lookup: create_nud_lookups(),
            led_lookup: create_led_lookups(),
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.tokens.pop(); // Assuming popping the last token is intentional
        self.parse_expr(PREC::DefaltBp)
    }

    pub fn parse_expr(&mut self, bp: PREC) -> Expr {
        let token = self.at().clone();
        let mut left = self.nud_lookup.get(&token.ttype).unwrap()(self);
        while !self.is_eof()
            && self
                .peek_next_token_bp()
                .map_or(false, |&next_bp| next_bp >= bp)
        {
            self.advance();
            let at = self.at();
            left = self.led_lookup.get(&self.at().ttype).unwrap()(self, left);
        }
        left
    }

    pub fn peek_next_token_bp(&self) -> Option<&PREC> {
        if self.current + 1 < self.tokens.len() {
            let token_type = &self.tokens[self.current + 1].ttype;
            return unsafe { BP_TABLE.get(&token_type) };
        }
        None
    }

    pub fn is_eof(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn at(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn expect(&mut self, expected_type: TokenType) {
        let test = self.at();
        if self.at().ttype != expected_type {
            panic!("Expected token of type {:?}", expected_type);
        }
        self.advance();
    }
}
