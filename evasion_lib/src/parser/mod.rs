mod parser_test;

use crate::{
    ast::{Expressions, Node, Nodes, Program, Statements},
    lexer::Lexer,
    token::{self, Token, TokenTypes},
};
use std::{collections::HashMap, path::Prefix};

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Call,
}

impl From<TokenTypes> for Precedence {
    fn from(value: TokenTypes) -> Self {
        match value {
            TokenTypes::EQ => Precedence::Equals,
            TokenTypes::LT => Precedence::LessGreater,
            TokenTypes::GT => Precedence::LessGreater,
            TokenTypes::PLUS => Precedence::Sum,
            TokenTypes::MINUS => Precedence::Sum,
            TokenTypes::ASTERISK => Precedence::Product,
            TokenTypes::SLASH => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

impl Parser {
    fn new(lexer: Lexer) -> Box<Parser> {
        let mut p = Parser {
            lexer,
            cur_token: Token::new(TokenTypes::ILLEGLAL, ""),
            peek_token: Token::new(TokenTypes::ILLEGLAL, ""),
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
                _ => {
                    if let Some(stmt) = self.parse_expression_stmt(Precedence::Lowest) {
                        program.statments.push(Nodes::from(stmt));
                    }
                }
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_expression_stmt(&mut self, precedence: Precedence) -> Option<Expressions> {
        let token = self.cur_token.clone();

        let mut left_expression = match token.token_type {
            // Parse Identifier
            TokenTypes::IDENT => Expressions::Identifier {
                token: token.clone(),
                value: token.litteral,
            },
            // Parse Infix Expression
            TokenTypes::BANG | TokenTypes::MINUS => {
                let token_operator = token.clone();
                self.next_token();

                if let Some(expresson) = self.parse_expression_stmt(Precedence::Lowest) {
                    let prefix_expression = Expressions::Prefix {
                        token: token_operator.clone(),
                        operator: token_operator.litteral,
                        right: Box::new(expresson),
                    };
                    return Some(prefix_expression);
                } else {
                    return None;
                }
            }
            // Parse Integer litteral
            TokenTypes::INT => {
                let number = if let Ok(num) = token.litteral.parse::<u64>() {
                    num
                } else {
                    println!("Couldn't parse integer value to u64");
                    return None;
                };
                Expressions::IntegerLiteral {
                    token: token.clone(),
                    value: number,
                }
            }
            _ => return None,
        };

        // Parse infix expression
        while !self.peek_token_is(TokenTypes::SEMICOLON) && (precedence < self.peek_precedence()) {
            match self.peek_token.token_type {
                TokenTypes::PLUS
                | TokenTypes::MINUS
                | TokenTypes::SLASH
                | TokenTypes::ASTERISK
                | TokenTypes::EQ
                | TokenTypes::NOTEq
                | TokenTypes::LT
                | TokenTypes::GT => {
                    self.next_token();

                    let precedence = self.cur_precedence();
                    let operator = self.cur_token.clone();

                    self.next_token();

                    let right_expression = self.parse_expression_stmt(precedence).unwrap();

                    let expression = Expressions::Infix {
                        token: self.cur_token.clone(),
                        left: Box::new(left_expression.clone()),
                        operator: operator.litteral,
                        right: Box::new(right_expression),
                    };

                    left_expression = expression;
                }
                _ => todo!(),
            }
        }

        Some(left_expression)
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from(self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::from(self.cur_token.token_type)
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
            self.next_token()
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

    fn get_precedence(next_token: TokenTypes) -> u8 {
        todo!()
    }

    fn cur_tok_is(&self, tok: TokenTypes) -> bool {
        self.cur_token.token_type == tok
    }

    fn peek_token_is(&mut self, tok: TokenTypes) -> bool {
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
