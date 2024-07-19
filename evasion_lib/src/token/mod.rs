use std::collections::HashMap;

use lazy_static::lazy_static;

pub struct Token {
    pub token_type: TokenType,
    pub litteral: String,
}

impl Token {
    pub fn new<T>(token_type: TokenType, litteral: T) -> Token
    where
        T: Into<String>,
    {
        Token {
            token_type,
            litteral: litteral.into(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    ILLEGLAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    EQ,    // ==
    NOTEq, // !=

    LT,
    GT,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("let", TokenType::LET),
        ("fn", TokenType::FUNCTION),
        ("return", TokenType::RETURN),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE)
    ]);
}
