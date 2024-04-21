use super::lookups::{
    create_led_lookups, create_nud_lookups, create_stmt_lookups, LedHandler, NudHandler,
    StmtHandler, BP_TABLE, PREC,
};
use crate::ast::expr::Expr;
use crate::ast::stmt::{self, Stmt};
use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    nud_lookup: HashMap<TokenType, NudHandler>,
    led_lookup: HashMap<TokenType, LedHandler>,
    stmt_lookup: HashMap<TokenType, StmtHandler>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            nud_lookup: create_nud_lookups(),
            led_lookup: create_led_lookups(),
            stmt_lookup: create_stmt_lookups(),
        }
    }

    ///Starts to parse the tokens inside our Parser
    pub fn parse(&mut self) -> Vec<Stmt> {
        self.tokens.pop(); // Assuming popping the last token is intentional
        let mut statements = Vec::new(); // Collect parsed statements
        while !self.is_eof() {
            statements.push(self.parse_stmt());
        }
        // Return the last parsed statement, assuming that's the intended behavior
        // You might want to return a list of statements instead
        statements
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        let stmt_fn = self.stmt_lookup.get(&self.at().ttype);

        if stmt_fn.is_some() {
            return stmt_fn.unwrap()(self);
        } else {
            let expression = self.parse_expr(PREC::DefaultBp);

            self.expect(TokenType::SEMICOLON);

            return Stmt::ExpressionStmt { expression };
        }
    }

    ///Starts to parse the tokens inside our Parser
    pub fn parse_expr(&mut self, bp: PREC) -> Expr {
        let token = self.at().clone();

        let mut left = self.nud_lookup.get(&token.ttype).unwrap()(self);
        while !self.is_eof() && self.token_bp().map_or(false, |&next_bp| next_bp >= bp) {
            left = self.led_lookup.get(&self.at().ttype).unwrap()(self, left);
        }
        left
    }

    pub fn peek_next_token_type(&self) -> Option<&TokenType> {
        if self.current + 1 < self.tokens.len() {
            let token_type = &self.tokens[self.current + 1].ttype;
            return Some(token_type);
        }
        None
    }

    pub fn token_bp(&self) -> Option<&PREC> {
        if self.current + 1 < self.tokens.len() {
            let token_type = &self.tokens[self.current].ttype;
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

    pub fn advance_and_get_current(&mut self) -> Token {
        let current = self.at().clone();
        self.advance();
        return current;
    }

    pub fn expect(&mut self, expected_type: TokenType) {
        if self.at().ttype != expected_type {
            panic!("Expected token of type {:?}", expected_type);
        }
        self.advance();
    }
}
