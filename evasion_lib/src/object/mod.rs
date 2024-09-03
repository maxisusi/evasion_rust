use std::{fmt::Display, usize};

#[derive(Clone, Copy, Debug)]
pub enum ObjectType {
    Integer(usize),
    Boolean(bool),
    Null,
    NoOp,
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::NoOp
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Integer(value) => write!(f, "{value}"),
            ObjectType::Boolean(value) => write!(f, "{value}"),
            ObjectType::Null => write!(f, ""),
            ObjectType::NoOp => write!(f, ""),
        }
    }
}
