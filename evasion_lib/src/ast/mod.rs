mod ast_test;
use crate::token::Token;

use std::fmt::{write, Display};

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
        Ok(())
    }
}

// ------------------------
// * NODES
// ------------------------

#[derive(Clone)]
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

#[derive(Clone)]
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
    BlockStatements {
        token: Token,
        statements: Vec<Nodes>,
    },
}

impl TryFrom<Nodes> for Statements {
    type Error = &'static str;

    fn try_from(node: Nodes) -> Result<Self, Self::Error> {
        match node {
            Nodes::Statement(stmt) => Ok(stmt),
            _ => Err("Cannot convert to Statement"),
        }
    }
}

impl Node for Statements {
    fn token_litteral(&self) -> &str {
        match self {
            Statements::Return { token, .. } => &token.litteral,
            Statements::Let { token, .. } => &token.litteral,
            Statements::BlockStatements { token, .. } => &token.litteral,
        }
    }
    fn display_type(&self) -> &str {
        match self {
            Statements::Return { .. } => "Return Statement",
            Statements::Let { .. } => "Let Statement",
            Statements::BlockStatements { .. } => "Block Statements",
        }
    }
}

impl Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statements::Return { token, value } => write!(f, "{} {}", token.litteral, value),
            Statements::Let { token, name, value } => {
                write!(f, "{} {} = {};", token.litteral, name, value)
            }
            Statements::BlockStatements {
                token: _token,
                statements,
            } => {
                for statement in statements {
                    write!(f, "{}", statement);
                }
                Ok(())
            }
        }
    }
}

// ------------------------
// * EXPRESSIONS
// ------------------------

#[derive(Clone)]
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
    Boolean {
        token: Token,
        value: bool,
    },
    IfExpression {
        token: Token,
        condition: Box<Expressions>,
        consequence: Box<Statements>, // Should only be block statements but my API suck
        alternative: Option<Box<Statements>>, // Same here...
    },
    FnExpression {
        token: Token,
        parameters: Vec<Expressions>,
        body: Box<Statements>, // Sane here...
    },
    CallExpression {
        token: Token,
        function: Box<Expressions>,
        arguments: Vec<Expressions>,
    },
}

impl TryFrom<Nodes> for Expressions {
    type Error = &'static str;

    fn try_from(node: Nodes) -> Result<Self, Self::Error> {
        match node {
            Nodes::Expression(stmt) => Ok(stmt),
            _ => Err("Cannot convert to Expression"),
        }
    }
}

impl Node for Expressions {
    fn token_litteral(&self) -> &str {
        match self {
            Expressions::Generic { token, .. } => &token.litteral,
            Expressions::Identifier { token, .. } => &token.litteral,
            Expressions::IntegerLiteral { token, .. } => &token.litteral,
            Expressions::Infix { token, .. } => &token.litteral,
            Expressions::Prefix { token, .. } => &token.litteral,
            Expressions::Boolean { token, .. } => &token.litteral,
            Expressions::IfExpression { token, .. } => &token.litteral,
            Expressions::FnExpression { token, .. } => &token.litteral,
            Expressions::CallExpression { token, .. } => &token.litteral,
        }
    }

    fn display_type(&self) -> &str {
        match self {
            Expressions::Infix { .. } => "Infix Expression",
            Expressions::Prefix { .. } => "Prefic Expression",
            Expressions::IntegerLiteral { .. } => "Integer Literal Expression",
            Expressions::Generic { .. } => "Expression",
            Expressions::Identifier { .. } => "Identifer",
            Expressions::Boolean { .. } => "Boolean",
            Expressions::IfExpression { .. } => "If Expression",
            Expressions::FnExpression { .. } => "Fn Expression",
            Expressions::CallExpression { .. } => "Call Expression",
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
            } => write!(f, "({}{})", operator, right),
            Expressions::Boolean { token, .. } => write!(f, "{}", token.litteral),
            Expressions::IfExpression {
                token,
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if {} {}", condition, consequence);

                if let Some(alt) = alternative {
                    write!(f, "else {}", alt).unwrap()
                }
                Ok(())
            }
            Expressions::FnExpression {
                token,
                parameters,
                body,
            } => {
                write!(f, "fn");

                let params = parameters
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>();

                write!(f, "({})", params.join(", "));
                write!(f, "{body}");
                Ok(())
            }
            Expressions::CallExpression {
                token,
                function,
                arguments,
            } => {
                let arguments = arguments
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>();

                write!(f, "{}({})", function, arguments.join(", "))
            }
        }
    }
}
