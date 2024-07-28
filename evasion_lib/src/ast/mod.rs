#[allow(unused)]
mod ast_test;
use crate::token::Token;

use std::{
    any::Any,
    collections::VecDeque,
    fmt::{write, Display},
};

// ------------------------
// * TYPE DEFINITIONS
// ------------------------

pub trait Node: Display {
    fn token_litteral(&self) -> &str;
}

pub trait Statement: Node {
    fn statment_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

// ------------------------
// * PROGRAM
// ------------------------

pub struct Program {
    pub statments: VecDeque<Box<dyn Statement>>,
}

impl Program {
    fn token_litteral(&self) -> &str {
        if self.statments.len() > 0 {
            return self.statments[0].token_litteral();
        } else {
            ""
        }
    }
}

impl Display for Program {
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

// RETURN STATEMENT

pub struct ReturnStatement {
    pub token: Token,
    // pub return_value: Box<dyn Expression>,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.token_litteral());
    }
}

impl Node for ReturnStatement {
    fn token_litteral(&self) -> &str {
        &self.token.litteral
    }
}

impl Statement for ReturnStatement {
    fn statment_node(&self) {
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// LET STATEMENT

pub struct LetStatement {
    pub token: Token,               // Let token
    pub name: Box<Identifier>,      // Identifier name
    pub value: Box<dyn Expression>, // Expression
}

impl Node for LetStatement {
    fn token_litteral(&self) -> &str {
        &self.token.litteral
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token_litteral(),       // Let
            self.name.token_litteral(),  // Identifier
            self.value.token_litteral(), // Missing expression...
        )
    }
}

impl Statement for LetStatement {
    fn statment_node(&self) {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ------------------------
// * EXPRESSIONS
// ------------------------

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Expression for ExpressionStatement {
    fn expression_node(&self) {
        todo!()
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_litteral())
    }
}

impl Node for ExpressionStatement {
    fn token_litteral(&self) -> &str {
        &self.token.litteral
    }
}

// IDENTIFIER

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_litteral(&self) -> &str {
        &self.token.litteral
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}
