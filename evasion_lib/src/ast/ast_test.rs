#[cfg(test)]

mod tests {
    use core::panic;
    use std::collections::VecDeque;

    use crate::{
        ast::{Expressions, Programs, Statements},
        token::{Token, TokenType},
    };

    #[test]
    fn test_display_ast() {
        let statments = VecDeque::from([Statements::LetStatement {
            token: Token::new(TokenType::LET, "let"),
            name: Expressions::Identifier {
                token: Token::new(TokenType::IDENT, "myVar"),
                value: "myVar".to_string(),
            },
            value: Expressions::Identifier {
                value: "anotherVar".to_string(),
                token: Token::new(TokenType::IDENT, "anotherVar"),
            },
        }]);

        let program = Programs { statments };

        if program.to_string() != "let myVar = anotherVar;".to_string() {
            panic!("Couldn't print properly, got={}", program)
        }
    }
}
