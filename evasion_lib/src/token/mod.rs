use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenTypes,
    pub litteral: String,
}

impl Token {
    pub fn new<T>(token_type: TokenTypes, litteral: T) -> Token
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
pub enum TokenTypes {
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
    pub static ref KEYWORDS: HashMap<&'static str, TokenTypes> = HashMap::from([
        ("let", TokenTypes::LET),
        ("fn", TokenTypes::FUNCTION),
        ("return", TokenTypes::RETURN),
        ("if", TokenTypes::IF),
        ("else", TokenTypes::ELSE),
        ("true", TokenTypes::TRUE),
        ("false", TokenTypes::FALSE)
    ]);
}
