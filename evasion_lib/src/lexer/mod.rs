#[allow(unused)]
mod lexer_test;
use super::token::{Token, TokenType, KEYWORDS};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_pos: usize,
    ch: char,
}

const NULL_TERMINATOR: char = '\0';

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_pos: 0,
            ch: '\0',
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = NULL_TERMINATOR
        } else {
            self.ch = self.input[self.read_pos];
        }
        self.position = self.read_pos;
        self.read_pos += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek() == '=' {
                    let tok_lit = self.ch;
                    self.read_char();
                    let litteral = format!("{}{}", tok_lit, self.ch); // litteral is: ==
                    Token::new(TokenType::EQ, litteral)
                } else {
                    Token::new(TokenType::ASSIGN, "=")
                }
            }
            '+' => Token::new(TokenType::PLUS, "+"),
            '(' => Token::new(TokenType::LPAREN, "("),
            ')' => Token::new(TokenType::RPAREN, ")"),
            '{' => Token::new(TokenType::LBRACE, "{"),
            '}' => Token::new(TokenType::RBRACE, "}"),
            ',' => Token::new(TokenType::COMMA, ","),
            ';' => Token::new(TokenType::SEMICOLON, ";"),
            '!' => {
                if self.peek() == '=' {
                    let tok_lit = self.ch;
                    self.read_char();
                    let litteral = format!("{}{}", tok_lit, self.ch); // litteral is: !=
                    Token::new(TokenType::NOTEq, litteral)
                } else {
                    Token::new(TokenType::BANG, "!")
                }
            }
            '-' => Token::new(TokenType::MINUS, "-"),
            '/' => Token::new(TokenType::SLASH, "/"),
            '*' => Token::new(TokenType::ASTERISK, "*"),
            '<' => Token::new(TokenType::LT, "<"),
            '>' => Token::new(TokenType::GT, ">"),
            '\0' => Token::new(TokenType::EOF, ""),
            c => {
                if self.is_letter(c) {
                    let identifier = self.read_identifier();
                    let identifier = identifier.iter().collect::<String>();

                    if let Some(token_type) = KEYWORDS.get(identifier.as_str()).clone() {
                        return Token::new(*token_type, identifier);
                    } else {
                        return Token::new(TokenType::IDENT, identifier);
                    }
                }

                if self.is_number(c) {
                    let number = self.read_number().iter().collect::<String>();
                    return Token::new(TokenType::INT, number);
                }
                // Check number
                return Token::new(TokenType::ILLEGLAL, c);
            }
        };
        self.read_char();
        token
    }

    fn peek(&self) -> char {
        self.input[self.read_pos]
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
                self.read_char()
            } else {
                break;
            }
        }
    }

    fn is_number(&self, c: char) -> bool {
        if c.is_ascii_digit() {
            return true;
        }
        return false;
    }

    fn is_letter(&self, c: char) -> bool {
        if c.is_ascii_alphabetic() {
            return true;
        }
        return false;
    }

    fn read_number(&mut self) -> &[char] {
        let cur_pos = self.position;

        loop {
            if !self.is_number(self.ch) {
                break;
            }
            self.read_char()
        }
        &self.input[cur_pos..self.position]
    }

    fn read_identifier(&mut self) -> &[char] {
        let cur_pos = self.position;

        loop {
            if !self.is_letter(self.ch) {
                break;
            }
            self.read_char()
        }
        &self.input[cur_pos..self.position]
    }
}
