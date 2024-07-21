#[allow(unused)]
mod parser_test;
use std::collections::VecDeque;

use crate::{
    ast::{Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    fn new(lexer: Lexer) -> Box<Parser> {
        let mut p = Parser {
            lexer,
            cur_token: Token::new(TokenType::ILLEGLAL, ""),
            peek_token: Token::new(TokenType::ILLEGLAL, ""),
            errors: vec![],
        };

        // Read twice so cur_token and peek_token are both set
        p.next_token();
        p.next_token();

        return Box::new(p);
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, tok: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            tok, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statments: VecDeque::new(),
        };

        while self.cur_token.token_type != TokenType::EOF {
            match self.cur_token.token_type {
                TokenType::LET => {
                    if let Some(stmt) = self.parse_let_statement() {
                        program.statments.push_front(stmt);
                    }
                }
                TokenType::RETURN => {
                    if let Some(stmt) = self.parse_return_statement() {
                        program.statments.push_front(stmt);
                    }
                }
                _ => {}
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt_tok = self.cur_token.clone();

        // TODO: Skipping expression until we encounter
        // a semicolon

        while !self.cur_tok_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        let stmt = ReturnStatement { token: stmt_tok };

        Some(Box::new(stmt))
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt_tok = self.cur_token.clone();

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let identifier = Identifier {
            value: self.cur_token.litteral.clone(),
            token: self.cur_token.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO: Skipping expression until we encounter
        // a semicolon

        while !self.cur_tok_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        let stmt = LetStatement {
            token: stmt_tok.clone(),
            name: Box::new(identifier),
            // value: Box::new(), <- Will be added once the expression is being parsed
        };

        Some(Box::new(stmt))
    }

    fn cur_tok_is(&self, tok: TokenType) -> bool {
        self.cur_token.token_type == tok
    }

    fn peek_token_is(&self, tok: TokenType) -> bool {
        self.peek_token.token_type == tok
    }

    fn expect_peek(&mut self, tok: TokenType) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            return true;
        } else {
            self.peek_error(tok);
            return false;
        }
    }
}
