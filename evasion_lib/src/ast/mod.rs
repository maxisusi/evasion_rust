mod ast_test;
use crate::token::Token;

use std::fmt::Display;

// ------------------------
// * TRAITS
// ------------------------

pub trait Node: Display {
    fn token_litteral(&self) -> &str;
}

// ------------------------
// * PROGRAM
// ------------------------

pub struct Program {
    pub statments: Vec<Nodes>,
}

impl Program {
    fn token_litteral(&self) -> &str {
        if self.statments.len() > 0 {
            match self.statments[0] {
                Nodes::Statement(ref stmt) => stmt.token_litteral(),
                Nodes::Expression(ref expr) => expr.token_litteral(),
            }
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
// * NODES
// ------------------------

pub enum Nodes {
    Statement(Statements),
    Expression(Expressions),
}

impl Display for Nodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nodes::Statement(stmt) => write!(f, "{}", stmt),
            Nodes::Expression(stmt) => write!(f, "{}", stmt),
        }
    }
}

impl From<Statements> for Nodes {
    fn from(stmt: Statements) -> Self {
        Nodes::Statement(stmt)
    }
}

impl From<Expressions> for Nodes {
    fn from(expr: Expressions) -> Self {
        Nodes::Expression(expr)
    }
}

// ------------------------
// * STATEMENTS
// ------------------------

pub enum Statements {
    Return {
        token: Token,
        value: Expressions,
    },
    Let {
        token: Token,
        name: Expressions,
        value: Expressions,
    },
}

impl Node for Statements {
    fn token_litteral(&self) -> &str {
        match self {
            Statements::Return { token, .. } => &token.litteral,
            Statements::Let { token, .. } => &token.litteral,
        }
    }
}

impl Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statements::Return { token, .. } => write!(f, "{}", token.litteral),
            Statements::Let { token, name, value } => write!(
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
    Expression {
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
            Expressions::Expression { token, .. } => &token.litteral,
            Expressions::Identifier { token, .. } => &token.litteral,
        }
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expressions::Expression { token, .. } => write!(f, "{}", token.litteral),
            Expressions::Identifier { token, .. } => write!(f, "{}", token.litteral,),
        }
    }
}
