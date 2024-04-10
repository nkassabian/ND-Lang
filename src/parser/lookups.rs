use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    ast::{expr::Expr, stmt::Stmt},
    tokens::token_type::TokenType,
};

use super::parser::Parser;

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
    // Call,
    // Member,
    Group = 12,
}

pub static mut BP_TABLE: Lazy<HashMap<TokenType, PREC>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(TokenType::EQUAL, PREC::Assignment);
    map.insert(TokenType::PLUSEQUALS, PREC::Assignment);
    map.insert(TokenType::MINUSEQUALS, PREC::Assignment);
    map.insert(TokenType::NUMBER, PREC::Primary);
    map.insert(TokenType::STRING, PREC::Primary);
    map.insert(TokenType::IDENTIFIER, PREC::Primary);
    map.insert(TokenType::PLUS, PREC::Additive);
    map.insert(TokenType::MINUS, PREC::Additive);
    map.insert(TokenType::STAR, PREC::Multiplicative);
    map.insert(TokenType::SLASH, PREC::Multiplicative);
    map.insert(TokenType::MODULO, PREC::Multiplicative);
    map.insert(TokenType::LESS, PREC::Relational);
    map.insert(TokenType::GREATER, PREC::Relational);
    map.insert(TokenType::LESSEQUAL, PREC::Relational);
    map.insert(TokenType::GREATEREQUAL, PREC::Relational);
    map.insert(TokenType::OR, PREC::Logical);
    map.insert(TokenType::POW, PREC::Power);
    map.insert(TokenType::BANG, PREC::Unary);
    map.insert(TokenType::LEFTPAREN, PREC::Group);
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
    map.insert(TokenType::LET, parse_var_decl_stmt as StmtHandler);
    map.insert(TokenType::CONST, parse_var_decl_stmt as StmtHandler);

    map.insert(TokenType::LEFTBRACE, parse_block_stmt as StmtHandler);
    map.insert(TokenType::IF, parse_if_stmt as StmtHandler);

    map
}

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
    let op = parser.advance_and_get_current();
    let rhs = parser.parse_expr(PREC::DefaultBp);

    return Expr::Assignment {
        assignee: Box::new(left),
        op: op.clone(),
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
    let group = parser.parse_expr(PREC::DefaultBp);
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
fn parse_var_decl_stmt(parser: &mut Parser) -> Stmt {
    let start_token = parser.advance_and_get_current();
    let is_constant = start_token.clone().ttype == TokenType::CONST;
    let symbol_name = parser.advance_and_get_current().clone();

    let explicit_type = if parser.at().ttype == TokenType::COLON {
        parser.expect(TokenType::COLON);
        Some(parser.advance_and_get_current())
    } else {
        None
    };

    let assignment_value = if parser.at().ttype != TokenType::SEMICOLON {
        parser.expect(TokenType::EQUAL);
        Some(parser.parse_expr(PREC::Assignment))
    } else {
        None
    };

    parser.expect(TokenType::SEMICOLON);

    if is_constant && assignment_value.is_none() {
        panic!("Cannot define constant variable without providing default value.")
    }

    Stmt::VarDeclarationStmt {
        isConstant: is_constant,
        Identifier: symbol_name.lexeme,
        assignedValue: assignment_value.unwrap(),
        explicitType: explicit_type,
    }
}

fn parse_block_stmt(parser: &mut Parser) -> Stmt {
    // parser.expect(TokenType::LEFTBRACE);
    parser.advance();
    let mut body: Vec<Stmt> = Vec::new();

    while !parser.is_eof() && parser.at().ttype != TokenType::RIGHTBRACE {
        body.push(parser.parse_stmt());
    }
    parser.expect(TokenType::RIGHTBRACE);

    return Stmt::BlockStmt { Body: body };
}

fn parse_if_stmt(parser: &mut Parser) -> Stmt {
    parser.advance();
    let condition = parser.parse_expr(PREC::Assignment);
    let consequent = parser.parse_stmt();
    let mut alternate = None; // Initialize alternate with None

    if parser.at().ttype == TokenType::ELSE {
        parser.advance();

        if parser.at().ttype == TokenType::IF {
            alternate = Some(Box::new(parse_if_stmt(parser))); // Use Some() to wrap the alternate value
        } else {
            alternate = Some(Box::new(parse_block_stmt(parser))); // Use Some() to wrap the alternate value
        }
    }

    Stmt::IfStmt {
        condition,
        consequent: Box::new(consequent),
        alternate, // Use the initialized alternate value
    }
}
