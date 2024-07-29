use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};

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

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
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

impl Display for TokenTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenTypes::ILLEGLAL => write!(f, "ILLEGLAL"),
            TokenTypes::EOF => write!(f, "EOF"),
            TokenTypes::IDENT => write!(f, "IDENT"),
            TokenTypes::INT => write!(f, "INT"),
            TokenTypes::ASSIGN => write!(f, "ASSIGN"),
            TokenTypes::PLUS => write!(f, "PLUS"),
            TokenTypes::MINUS => write!(f, "MINUS"),
            TokenTypes::BANG => write!(f, "BANG"),
            TokenTypes::ASTERISK => write!(f, "ASTERISK"),
            TokenTypes::SLASH => write!(f, "SLASH"),
            TokenTypes::EQ => write!(f, "EQ"),
            TokenTypes::NOTEq => write!(f, "NOTEq"),
            TokenTypes::LT => write!(f, "LT"),
            TokenTypes::GT => write!(f, "GT"),
            TokenTypes::COMMA => write!(f, "COMMA"),
            TokenTypes::SEMICOLON => write!(f, "SEMICOLON"),
            TokenTypes::LPAREN => write!(f, "LPAREN"),
            TokenTypes::RPAREN => write!(f, "RPAREN"),
            TokenTypes::LBRACE => write!(f, "LBRACE"),
            TokenTypes::RBRACE => write!(f, "RBRACE"),
            TokenTypes::FUNCTION => write!(f, "FUNCTION"),
            TokenTypes::LET => write!(f, "LET"),
            TokenTypes::TRUE => write!(f, "TRUE"),
            TokenTypes::FALSE => write!(f, "FALSE"),
            TokenTypes::IF => write!(f, "IF"),
            TokenTypes::ELSE => write!(f, "ELSE"),
            TokenTypes::RETURN => write!(f, "RETURN"),
        }
    }
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
