use std::fmt::Display;

enum ObjectType {
    Integer,
    Boolean,
    Null,
}

trait Object: Display {
    fn object_type() -> ObjectType;
}

// Integer
struct Integer {
    value: usize,
}
impl Object for Integer {
    fn object_type() -> ObjectType {
        ObjectType::Integer
    }
}
impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Boolean
struct Boolean {
    value: bool,
}

impl Object for Boolean {
    fn object_type() -> ObjectType {
        ObjectType::Boolean
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Null
struct Null {}

impl Object for Null {
    fn object_type() -> ObjectType {
        ObjectType::Null
    }
}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
