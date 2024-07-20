use crate::token::Token;
use std::{any::Any, collections::VecDeque};

pub trait Node {
    fn token_litteral(&self) -> &str;
}

pub trait Statement: Node {
    fn statment_node(&self);
    fn as_any(&self) -> &dyn Any;
}

trait Expression: Node {
    fn expression_node(&self);
}

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

pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    // pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_litteral(&self) -> &str {
        &self.token.litteral
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

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_litteral(&self) -> &str {
        &self.token.litteral
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}
