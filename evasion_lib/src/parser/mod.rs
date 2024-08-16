mod parser_test;

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
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl From<TokenTypes> for Precedence {
    fn from(value: TokenTypes) -> Self {
        match value {
            TokenTypes::EQ => Precedence::Equals,
            TokenTypes::NOTEq => Precedence::Equals,
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
    fn new(lexer: Lexer) -> Box<Self> {
        let mut p = Self {
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

    // ------------------------
    // * Entry Point
    // ------------------------

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
        let mut left_expression = match self.parse_prefix_expression() {
            Some(expression) => expression,
            None => return None,
        };

        while !self.peek_token_is(TokenTypes::SEMICOLON) && (precedence < self.peek_precedence()) {
            let infix_expression = self.parse_infix_expression(&left_expression);
            left_expression = infix_expression;
        }

        Some(left_expression)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expressions> {
        let expression = match self.cur_token.token_type {
            TokenTypes::IDENT => self.parse_identifier(),
            TokenTypes::BANG | TokenTypes::MINUS => self.parse_infix(),
            TokenTypes::FALSE | TokenTypes::TRUE => self.parse_boolean(),
            TokenTypes::INT => self.parse_integer_litteral(),
            TokenTypes::LPAREN => self.parse_left_paren(),
            _ => return None,
        };

        Some(expression)
    }

    fn parse_infix_expression(&mut self, left_expression: &Expressions) -> Expressions {
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
                expression
            }
            _ => panic!(
                "Couldn't parse infix expression, got={}",
                self.peek_token.token_type
            ),
        }
    }

    // ------------------------
    // * Expressions
    // ------------------------

    fn parse_identifier(&self) -> Expressions {
        Expressions::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.litteral.clone(),
        }
    }

    fn parse_infix(&mut self) -> Expressions {
        let token_operator = self.cur_token.clone();
        self.next_token();

        let expression = self.parse_expression_stmt(Precedence::Prefix).unwrap();

        Expressions::Prefix {
            token: token_operator.clone(),
            operator: token_operator.litteral,
            right: Box::new(expression),
        }
    }

    fn parse_integer_litteral(&self) -> Expressions {
        let number = if let Ok(num) = self.cur_token.clone().litteral.parse::<u64>() {
            num
        } else {
            panic!("Couldn't parse integer value to u64");
        };
        Expressions::IntegerLiteral {
            token: self.cur_token.clone(),
            value: number,
        }
    }

    fn parse_boolean(&self) -> Expressions {
        Expressions::Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token.clone().litteral.parse::<bool>().unwrap(),
        }
    }

    fn parse_left_paren(&mut self) -> Expressions {
        self.next_token();
        let expression = self.parse_expression_stmt(Precedence::Lowest).unwrap();

        if !self.expect_peek(TokenTypes::RPAREN) {
            panic!("Couldn't find right parenthesis")
        }

        expression
    }

    // ------------------------
    // * Statements
    // ------------------------

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

    // ------------------------
    // * Helpers
    // ------------------------

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
    fn peek_precedence(&self) -> Precedence {
        Precedence::from(self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::from(self.cur_token.token_type)
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
