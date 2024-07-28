#[allow(unused)]
mod lexer_test;
use super::token::{Token, TokenTypes, KEYWORDS};

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
                    Token::new(TokenTypes::EQ, litteral)
                } else {
                    Token::new(TokenTypes::ASSIGN, "=")
                }
            }
            '+' => Token::new(TokenTypes::PLUS, "+"),
            '(' => Token::new(TokenTypes::LPAREN, "("),
            ')' => Token::new(TokenTypes::RPAREN, ")"),
            '{' => Token::new(TokenTypes::LBRACE, "{"),
            '}' => Token::new(TokenTypes::RBRACE, "}"),
            ',' => Token::new(TokenTypes::COMMA, ","),
            ';' => Token::new(TokenTypes::SEMICOLON, ";"),
            '!' => {
                if self.peek() == '=' {
                    let tok_lit = self.ch;
                    self.read_char();
                    let litteral = format!("{}{}", tok_lit, self.ch); // litteral is: !=
                    Token::new(TokenTypes::NOTEq, litteral)
                } else {
                    Token::new(TokenTypes::BANG, "!")
                }
            }
            '-' => Token::new(TokenTypes::MINUS, "-"),
            '/' => Token::new(TokenTypes::SLASH, "/"),
            '*' => Token::new(TokenTypes::ASTERISK, "*"),
            '<' => Token::new(TokenTypes::LT, "<"),
            '>' => Token::new(TokenTypes::GT, ">"),
            '\0' => Token::new(TokenTypes::EOF, ""),
            c => {
                if self.is_letter(c) {
                    let identifier = self.read_identifier();
                    let identifier = identifier.iter().collect::<String>();

                    if let Some(token_type) = KEYWORDS.get(identifier.as_str()).clone() {
                        return Token::new(*token_type, identifier);
                    } else {
                        return Token::new(TokenTypes::IDENT, identifier);
                    }
                }

                if self.is_number(c) {
                    let number = self.read_number().iter().collect::<String>();
                    return Token::new(TokenTypes::INT, number);
                }
                // Check number
                return Token::new(TokenTypes::ILLEGLAL, c);
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
