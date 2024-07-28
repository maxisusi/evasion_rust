mod ast_test;
use crate::token::Token;

use std::{collections::VecDeque, fmt::Display};

// ------------------------
// * TYPE DEFINITIONS
// ------------------------

pub trait Node: Display {
    fn token_litteral(&self) -> &str;
}

// ------------------------
// * PROGRAM
// ------------------------

pub struct Programs {
    pub statments: VecDeque<Statements>,
}

impl Programs {
    fn token_litteral(&self) -> &str {
        if self.statments.len() > 0 {
            return self.statments[0].token_litteral();
        } else {
            ""
        }
    }
}

impl Display for Programs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in self.statments.iter() {
            write!(f, "{}", stmt).unwrap()
        }
        return write!(f, "");
    }
}

// ------------------------
// * STATEMENTS
// ------------------------

pub enum Statements {
    ReturnStatement {
        token: Token,
        value: Expressions,
    },
    LetStatement {
        token: Token,
        name: Expressions,
        value: Expressions,
    },
}

impl Node for Statements {
    fn token_litteral(&self) -> &str {
        match self {
            Statements::ReturnStatement { token, .. } => &token.litteral,
            Statements::LetStatement { token, .. } => &token.litteral,
        }
    }
}

impl Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statements::ReturnStatement { token, .. } => write!(f, "{}", token.litteral),
            Statements::LetStatement { token, name, value } => write!(
                f,
                "{} {} = {};",
                token.litteral,
                name,
                value.token_litteral()
            ),
        }
    }
}

// ------------------------
// * EXPRESSIONS
// ------------------------

pub enum Expressions {
    ExpressionStatement {
        token: Token,
        expression: Box<Expressions>,
    },
    Identifier {
        token: Token,
        value: String,
    },
}

impl Node for Expressions {
    fn token_litteral(&self) -> &str {
        match self {
            Expressions::ExpressionStatement { token, .. } => &token.litteral,
            Expressions::Identifier { token, .. } => &token.litteral,
        }
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expressions::ExpressionStatement { token, .. } => write!(f, "{}", token.litteral),
            Expressions::Identifier { token, .. } => write!(f, "{}", token.litteral,),
        }
    }
}
