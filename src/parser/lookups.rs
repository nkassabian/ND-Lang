use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{ast::expr::Expr, tokens::token_type::TokenType};

use super::parser::Parser;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum PREC {
    DefaltBp = 0,
    Primary = 1,
    // Comma,
    Assignment = 3,
    Logical = 4,
    // Relational,
    Additive = 6,
    Multiplicative = 7,
    Power = 8,
    Unary = 9,
    // Call,
    // Member,
    Group = 12,
}

pub static mut BP_TABLE: Lazy<HashMap<TokenType, PREC>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(TokenType::EQUAL, PREC::Assignment);

    map.insert(TokenType::NUMBER, PREC::Primary);
    map.insert(TokenType::STRING, PREC::Primary);
    map.insert(TokenType::IDENTIFIER, PREC::Primary);

    //Additive & mutiplicative
    map.insert(TokenType::PLUS, PREC::Additive);
    map.insert(TokenType::MINUS, PREC::Additive);
    map.insert(TokenType::STAR, PREC::Multiplicative);
    map.insert(TokenType::SLASH, PREC::Multiplicative);
    map.insert(TokenType::MODULO, PREC::Multiplicative);

    map.insert(TokenType::AND, PREC::Logical);
    map.insert(TokenType::OR, PREC::Logical);

    map.insert(TokenType::POW, PREC::Power);

    map.insert(TokenType::BANG, PREC::Unary);

    map.insert(TokenType::LEFTPAREN, PREC::Group);
    // map.insert(TokenType::RIGHTPAREN, PREC::Group);

    map.insert(TokenType::EOF, PREC::DefaltBp);
    map.insert(TokenType::PLUS, PREC::DefaltBp);

    map
});

pub type LedHandler = fn(&mut Parser, Expr) -> Expr;
pub type NudHandler = fn(&mut Parser) -> Expr;
// pub type stmt_handler = fn(&mut Parser) -> Stmt;

pub fn create_nud_lookups() -> HashMap<TokenType, NudHandler> {
    let mut map = HashMap::new();

    // Populate the map
    map.insert(TokenType::NUMBER, parse_num as NudHandler);
    map.insert(TokenType::STRING, parse_string as NudHandler);
    map.insert(TokenType::IDENTIFIER, parse_identifier as NudHandler);
    map.insert(TokenType::MINUS, parse_unary as NudHandler);
    map.insert(TokenType::BANG, parse_unary as NudHandler);

    map.insert(TokenType::LEFTPAREN, parse_grouping_expr as NudHandler);
    map
}

pub fn create_led_lookups() -> HashMap<TokenType, LedHandler> {
    let mut map = HashMap::new();

    // Logical
    map.insert(TokenType::AND, parse_binary_expr as LedHandler);
    map.insert(TokenType::OR, parse_binary_expr as LedHandler);

    map.insert(TokenType::PLUS, parse_binary_expr as LedHandler);
    map.insert(TokenType::MINUS, parse_binary_expr as LedHandler);
    map.insert(TokenType::STAR, parse_binary_expr as LedHandler);
    map.insert(TokenType::SLASH, parse_binary_expr as LedHandler);
    map.insert(TokenType::MODULO, parse_binary_expr as LedHandler);
    map.insert(TokenType::POW, parse_binary_expr as LedHandler);
    map.insert(TokenType::EQUAL, parse_assignment_expr as LedHandler);
    map
}

// pub fn create_stmt_lookups() -> HashMap<TokenType, stmt_handler> {
//     let mut map = HashMap::new();

//     // Populate the map
//     map.insert(TokenType::LET, parse_var_decl_stmt as stmt_handler);

//     map
// }

fn parse_num(parser: &mut Parser) -> Expr {
    let token = parser.at().clone();
    parser.advance();
    // Implementation
    Expr::Number(token.lexeme.to_string())
}

fn parse_string(parser: &mut Parser) -> Expr {
    // Implementation
    let token = parser.at().clone();
    parser.advance();
    // Implementation
    Expr::String(token.lexeme.to_string())
}

fn parse_assignment_expr(parser: &mut Parser, left: Expr) -> Expr {
    parser.advance();
    let rhs = parser.parse_expr(PREC::DefaltBp);

    return Expr::Assignment {
        assignee: Box::new(left),
        assigned: Box::new(rhs),
    };
}

fn parse_identifier(parser: &mut Parser) -> Expr {
    // Implementation
    let token = parser.at().clone();
    parser.advance();
    // Implementation
    Expr::Identifier(token.lexeme.to_string())
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

//TODO: Make function that increments current, and returns the current before the incrementation.
// fn parse_var_decl_stmt(parser: &mut Parser) -> Stmt {
//     let start_token = parser.at();
//     parser.advance();
//     let is_constant = start_token.clone().ttype == TokenType::CONST;
//     let symbol_name = parser.at();

//     // let explicit_type = if parser.at().ttype == TokenType::COLON {
//     //     p.expect(lexer::TokenKind::COLON);
//     //     Some(parse_type(p, PREC::DefaltBp))
//     // } else {
//     //     None
//     // };

//     let assignment_value = if parser.at().ttype != TokenType::SEMICOLON {
//         parser.expect(TokenType::EQUAL);
//         Some(parser.parse_expr(PREC::Assignment))
//     } else {
//         None
//     };

//     parser.expect(TokenType::SEMICOLON);

//     if is_constant && assignment_value.is_none() {
//         panic!("Cannot define constant variable without providing default value.")
//     }

//     Stmt::VarDeclarationStmt {
//         constant: is_constant,
//         identifier: symbol_name.lexeme,
//         AssignedValue: assignment_value.unwrap(),
//         // explicit_type,
//     }
// }
