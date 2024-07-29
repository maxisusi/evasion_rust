mod parser_test;

use std::collections::HashMap;

use crate::{
    ast::{Expressions, Nodes, Program, Statements},
    lexer::Lexer,
    token::{Token, TokenTypes},
};

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_expr: HashMap<String, fn() -> Expressions>,
    infix_expr: HashMap<String, fn() -> Expressions>,
}

impl Parser {
    fn new(lexer: Lexer) -> Box<Parser> {
        let mut p = Parser {
            lexer,
            cur_token: Token::new(TokenTypes::ILLEGLAL, ""),
            peek_token: Token::new(TokenTypes::ILLEGLAL, ""),
            errors: vec![],
            infix_expr: HashMap::new(),
            prefix_expr: HashMap::new(),
        };

        // Read twice so cur_token and peek_token are both set
        p.next_token();
        p.next_token();

        return Box::new(p);
    }

    fn register_prefix(&mut self, token_type: TokenTypes, reg_fn: fn() -> Expressions) {
        self.infix_expr
            .insert(token_type.to_string(), reg_fn)
            .unwrap();
    }

    fn register_infix(&mut self, token_type: TokenTypes, reg_fn: fn() -> Expressions) {
        self.infix_expr
            .insert(token_type.to_string(), reg_fn)
            .unwrap();
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, tok: TokenTypes) {
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
            statments: Vec::new(),
        };

        while self.cur_token.token_type != TokenTypes::EOF {
            match self.cur_token.token_type {
                TokenTypes::LET => {
                    if let Some(stmt) = self.parse_let_statement() {
                        program.statments.push(Nodes::from(stmt));
                    }
                }
                TokenTypes::RETURN => {
                    if let Some(stmt) = self.parse_return_statement() {
                        program.statments.push(Nodes::from(stmt));
                    }
                }
                _ => {}
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_return_statement(&mut self) -> Option<Statements> {
        let stmt_tok = self.cur_token.clone();

        // TODO: Skipping expression until we encounter
        // a semicolon

        while !self.cur_tok_is(TokenTypes::SEMICOLON) {
            self.next_token();
        }

        let stmt = Statements::Return {
            token: stmt_tok,
            value: Expressions::Identifier {
                // Dummy value for now
                token: Token::new(TokenTypes::ILLEGLAL, ""),
                value: "".to_string(),
            },
        };

        Some(stmt)
    }

    fn parse_let_statement(&mut self) -> Option<Statements> {
        let stmt_tok = self.cur_token.clone();

        if !self.expect_peek(TokenTypes::IDENT) {
            return None;
        }

        let identifier = Expressions::Identifier {
            value: self.cur_token.litteral.clone(),
            token: self.cur_token.clone(),
        };

        if !self.expect_peek(TokenTypes::ASSIGN) {
            return None;
        }

        // TODO: Skipping expression until we encounter
        // a semicolon

        while !self.cur_tok_is(TokenTypes::SEMICOLON) {
            self.next_token();
        }

        let stmt = Statements::Let {
            token: stmt_tok.clone(),
            name: identifier,
            value: Expressions::Identifier {
                // Dummy value for now.
                token: Token::new(TokenTypes::ILLEGLAL, ""),
                value: "".to_string(),
            },
        };

        Some(stmt)
    }

    fn cur_tok_is(&self, tok: TokenTypes) -> bool {
        self.cur_token.token_type == tok
    }

    fn peek_token_is(&self, tok: TokenTypes) -> bool {
        self.peek_token.token_type == tok
    }

    fn expect_peek(&mut self, tok: TokenTypes) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            return true;
        } else {
            self.peek_error(tok);
            return false;
        }
    }
}
