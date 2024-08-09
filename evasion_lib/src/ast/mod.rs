mod ast_test;
use crate::token::Token;

use std::fmt::Display;

// ------------------------
// * TRAITS
// ------------------------

pub trait Node: Display {
    fn token_litteral(&self) -> &str;
    fn display_type(&self) -> &str;
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
            let _ = match stmt {
                Nodes::Statement(stmt) => write!(f, "{stmt}"),
                Nodes::Expression(stmt) => write!(f, "{stmt}"),
            };
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
            Nodes::Statement(stmt) => write!(f, "Statement Node, Value='{}'", stmt),
            Nodes::Expression(stmt) => write!(f, "Expression Node, Value='{}'", stmt),
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
    fn display_type(&self) -> &str {
        match self {
            Statements::Return { .. } => "Return Statement",
            Statements::Let { .. } => "Let Statement",
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
    Generic {
        token: Token,
        expression: Box<Expressions>,
    },
    Identifier {
        token: Token,
        value: String,
    },

    IntegerLiteral {
        token: Token,
        value: u64,
    },
    Infix {
        token: Token,
        left: Box<Expressions>,
        operator: String,
        right: Box<Expressions>,
    },
    Prefix {
        token: Token,
        operator: String,
        right: Box<Expressions>,
    },
}

impl Node for Expressions {
    fn token_litteral(&self) -> &str {
        match self {
            Expressions::Generic { token, .. } => &token.litteral,
            Expressions::Identifier { token, .. } => &token.litteral,
            Expressions::IntegerLiteral { token, .. } => &token.litteral,
            Expressions::Infix { token, .. } => &token.litteral,
            Expressions::Prefix { token, .. } => &token.litteral,
        }
    }

    fn display_type(&self) -> &str {
        match self {
            Expressions::Infix { .. } => "Infix Expression",
            Expressions::Prefix { .. } => "Prefic Expression",
            Expressions::IntegerLiteral { .. } => "Integer Literal Expression",
            Expressions::Generic { .. } => "Expression",
            Expressions::Identifier { .. } => "Identifer",
        }
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expressions::Generic { token, .. } => write!(f, "{}", token.litteral),
            Expressions::Identifier { token, .. } => write!(f, "{}", token.litteral),
            Expressions::IntegerLiteral { token, .. } => write!(f, "{}", token.litteral),
            Expressions::Infix {
                token: _,
                left,
                right,
                operator,
            } => write!(f, "({} {} {})", left, operator, right),
            Expressions::Prefix {
                token: _,
                right,
                operator,
            } => write!(f, "({} {})", operator, right),
        }
    }
}
