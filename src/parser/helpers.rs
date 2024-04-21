use crate::{
    ast::{expr::Expr, stmt::Stmt},
    tokens::token_type::TokenType,
};

use super::{
    lookups::{BP_TABLE, PREC},
    parser::Parser,
};

pub fn parse_num(parser: &mut Parser) -> Expr {
    let token = parser.advance_and_get_current();
    // Implementation
    Expr::Number(token.lexeme.to_string())
}

pub fn parse_string(parser: &mut Parser) -> Expr {
    // Implementation
    let token = parser.advance_and_get_current();
    // Implementation
    Expr::String(token.lexeme.to_string())
}

pub fn parse_assignment_expr(parser: &mut Parser, left: Expr) -> Expr {
    let op = parser.advance_and_get_current();
    let rhs = parser.parse_expr(PREC::DefaultBp);

    return Expr::Assignment {
        assignee: Box::new(left),
        op: op.clone(),
        assigned: Box::new(rhs),
    };
}

pub fn parse_identifier(parser: &mut Parser) -> Expr {
    // Implementation
    let token = parser.advance_and_get_current();
    // Implementation
    Expr::Identifier(token.lexeme.to_string())
}

pub fn parse_grouping_expr(parser: &mut Parser) -> Expr {
    parser.expect(TokenType::LEFTPAREN);
    let group = parser.parse_expr(PREC::DefaultBp);
    parser.expect(TokenType::RIGHTPAREN);
    Expr::Grouping {
        group: Box::new(group),
    }
}

pub fn parse_unary(parser: &mut Parser) -> Expr {
    let op = parser.advance_and_get_current();

    let right = parser.parse_expr(PREC::Prefix);
    Expr::Unary {
        op,
        right: Box::new(right),
    }
}

pub fn parse_binary_expr(parser: &mut Parser, left: Expr) -> Expr {
    let op = parser.advance_and_get_current();

    let right = parser.parse_expr(*unsafe { BP_TABLE.get(&op.ttype).unwrap() });
    Expr::BinaryOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    }
}

pub fn parse_block_stmt(parser: &mut Parser) -> Stmt {
    // parser.expect(TokenType::LEFTBRACE);
    parser.advance();
    let mut body: Vec<Stmt> = Vec::new();

    while !parser.is_eof() && parser.at().ttype != TokenType::RIGHTBRACE {
        body.push(parser.parse_stmt());
    }
    parser.expect(TokenType::RIGHTBRACE);

    Stmt::BlockStmt { body }
}

pub fn parse_if_stmt(parser: &mut Parser) -> Stmt {
    parser.advance();
    let condition = parser.parse_expr(PREC::Assignment);
    let consequent = parser.parse_stmt();
    let mut alternate = None; // Initialize alternate with None

    if !parser.is_eof() {
        if parser.at().ttype == TokenType::ELSE {
            parser.advance();

            if parser.at().ttype == TokenType::IF {
                alternate = Some(Box::new(parse_if_stmt(parser))); // Use Some() to wrap the alternate value
            } else {
                alternate = Some(Box::new(parse_block_stmt(parser))); // Use Some() to wrap the alternate value
            }
        }
    }

    Stmt::IfStmt {
        condition,
        consequent: Box::new(consequent),
        alternate, // Use the initialized alternate value
    }
}

pub fn parse_var_decl_stmt(parser: &mut Parser) -> Stmt {
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
        identifier: symbol_name.lexeme,
        assignedValue: assignment_value.unwrap(),
        explicitType: explicit_type,
    }
}
