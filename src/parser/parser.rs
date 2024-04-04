use crate::expressions::expr::*;
use crate::object::object::Object;
use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;
pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    bp: &'a PREC,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,           // Assigning initial value to current field
            bp: &PREC::defalt_bp, // Assigning initial value to current field
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.build_ast(self.bp)
    }

    fn is_eof(&mut self) -> bool {
        return self.current >= self.tokens.len();
    }

    fn at(&mut self) -> Token {
        if self.is_eof() {
            return Token::new(TokenType::EOF, "eof".to_owned(), Object::Nil, 0, 0);
        }
        return self.tokens[self.current].clone();
    }

    fn build_ast(&mut self, mut rbp: &PREC) -> Expr {
        let mut left = self.nud();
        let cur_token = self.at();

        while PRECEDENCE_TABLE.get(&cur_token.clone().ttype).unwrap() > rbp && !self.is_eof() {
            rbp = PRECEDENCE_TABLE.get(&cur_token.ttype).unwrap();
            left = self.led(left, rbp)
        }

        return left;
    }

    fn nud(&mut self) -> Expr {
        let cur_token = self.tokens[self.current].clone();
        self.current += 1;

        match cur_token.ttype {
            TokenType::NUMBER => match cur_token.literal {
                Object::Num(num) => return Expr::NumberLiteral(num),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn led(&mut self, left_expr: Expr, prec: &PREC) -> Expr {
        //check current token to be a binary expression

        let cur_token = self.at();
        self.current += 1;

        match cur_token.ttype {
            TokenType::PLUS | TokenType::STAR => {
                //OPERATOR IS THE CUR_TOKE
                //NEXT ONE
                let right = self.build_ast(&PREC::defalt_bp);
                return Expr::BinaryOp {
                    left: Box::new(left_expr),
                    op: cur_token,
                    right: Box::new(right),
                };
            }
            _ => todo!(),
        }
    }
}
