#[cfg(test)]

mod tests {
    use core::panic;
    use std::collections::VecDeque;

    use crate::{
        ast::{Expressions, Program, Statements},
        token::{Token, TokenTypes},
    };

    #[test]
    fn test_display_ast() {
        let statments = VecDeque::from([Statements::Let {
            token: Token::new(TokenTypes::LET, "let"),
            name: Expressions::Identifier {
                token: Token::new(TokenTypes::IDENT, "myVar"),
                value: "myVar".to_string(),
            },
            value: Expressions::Identifier {
                value: "anotherVar".to_string(),
                token: Token::new(TokenTypes::IDENT, "anotherVar"),
            },
        }]);

        let program = Program { statments };

        if program.to_string() != "let myVar = anotherVar;".to_string() {
            panic!("Couldn't print properly, got={}", program)
        }
    }
}
