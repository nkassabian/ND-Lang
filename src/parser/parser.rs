use super::lookups::{
    create_led_lookups, create_nud_lookups, create_stmt_lookups, LedHandler, NudHandler,
    StmtHandler, BP_TABLE, PREC,
};
use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::errors::error::Error;
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
        //self.tokens.pop();
        let mut statements = Vec::new();
        while !self.is_eof() {
            statements.push(self.parse_stmt());
        }
        statements
    }

    ///Starts to parse the tokens inside our Parser
    ///
    /// #Example
    /// ```rust
    /// let mut parser = Parser::new(tokens),
    /// ```
    pub fn parse_stmt(&mut self) -> Stmt {
        let stmt_fn = self.stmt_lookup.get(&self.at().ttype);

        if stmt_fn.is_some() {
            return stmt_fn.unwrap()(self);
        } else {
            let expression = self.parse_expr(PREC::DefaultBp);

            self.expect(TokenType::SEMICOLON, ';');

            return Stmt::ExpressionStmt { expression };
        }
    }

    ///Starts to parse the tokens inside our Parser
    pub fn parse_expr(&mut self, bp: PREC) -> Expr {
        let token = self.at().clone();

        let mut left = self.nud_lookup.get(&token.ttype).unwrap()(self);
        while self.token_bp().map_or(false, |&next_bp| next_bp >= bp) {
            left = self.led_lookup.get(&self.at().ttype).unwrap()(self, left);
        }
        left
    }

    ///Get the precedence of the current token
    pub fn token_bp(&self) -> Option<&PREC> {
        if self.current + 1 < self.tokens.len() {
            let token_type = &self.tokens[self.current].ttype;
            return unsafe { BP_TABLE.get(&token_type) };
        }
        None
    }

    ///Checks if we are at the end of the file
    pub fn is_eof(&self) -> bool {
        self.current + 1 > self.tokens.len()
    }

    ///Advances the current position
    pub fn advance(&mut self) {
        if !self.is_eof() {
            self.current += 1;
        }
        self.current;
    }

    ///Returns the current token
    pub fn at(&self) -> &Token {
        &self.tokens[self.current]
    }

    ///Returns the current token and advances
    pub fn advance_and_get_current(&mut self) -> Token {
        let current = self.at().clone();
        self.advance();
        return current;
    }

    ///Expects the current token to be of the given type
    pub fn expect(&mut self, expected_type: TokenType, expected_char: char) {
        if self.at().ttype != expected_type {
            let error = Error::new(
                self.at().position,
                self.at().line,
                format!(
                    "Expected token of type \"{}\" but found \"{}\"",
                    expected_char.to_string(),
                    self.at().lexeme
                ),
                "".to_string(),
            );

            error.report();
            //std::process::exit(65);
        }
        self.advance();
    }
}
