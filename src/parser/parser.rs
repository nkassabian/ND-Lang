use core::panic;
use std::string;

use crate::parser::expressions::*;
use crate::tokens::token::{self, Token};
use crate::tokens::token_type::TokenType;
use crate::{ast::expr::Expr, object::object::Object};

use super::lookups::{BP_TABLE, PREC};
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

//TODO: LOOK INTO ITERATORS
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0, // Assigning initial value to current field
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.tokens.pop();
        return self.parse_expr(PREC::defalt_bp);
    }

    pub fn parse_expr(&mut self, bp: PREC) -> Expr {
        let mut cur_tok = self.at();
        let mut left = self.handle_nud(cur_tok.clone()); // Assuming handle_nud is defined

        if !self.is_eof() {
            self.advance();
        }

        let value = unsafe {
            BP_TABLE
                .get(&self.at().ttype) // Assuming cur_tok has a field named Type representing the token type
                .unwrap_or_else(|| panic!("NO WAY"))
        };

        while !self.is_eof() && value > &bp {
            left = self.handle_led(left, bp); // Assuming handle_led is defined
        }

        left // Return the resulting expression
    }

    pub fn handle_led(&mut self, left: Expr, bp: PREC) -> Expr {
        let op = self.at();
        let value = unsafe {
            BP_TABLE
                .get(&self.at().ttype) // Assuming cur_tok has a field named Type representing the token type
                .unwrap_or_else(|| panic!("NO WAY"))
        };
        self.advance();

        let right = self.parse_expr(*value);
        return Expr::BinaryOp {
            left: Box::new(left),
            op: op,
            right: Box::new(right),
        };
    }

    pub fn handle_nud(&mut self, token: Token) -> Expr {
        match token.ttype {
            TokenType::NUMBER => match token.literal {
                Object::Num(num) => {
                    return Expr::NumberLiteral(num);
                }
                _ => panic!("Invalid number"),
            },
            _ => todo!(),
        }
    }

    //led and nud shouldnt compare

    pub fn cur_tok_type(&mut self) -> TokenType {
        return self.at().ttype;
    }

    pub fn is_eof(&mut self) -> bool {
        return self.current >= (self.tokens.len() - 1);
    }

    pub fn advance(&mut self) -> &Token {
        let cur_tok = &self.tokens[self.current];
        self.current += 1;
        return cur_tok;
    }

    pub fn at(&mut self) -> Token {
        return self.tokens[self.current].clone();
    }
}
