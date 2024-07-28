#[cfg(test)]

mod tests {
    use core::panic;
    use std::collections::VecDeque;

    use crate::{
        ast::{Identifier, LetStatement, Program, Statement},
        token::{Token, TokenType},
    };

    #[test]
    fn test_display_ast() {
        let statments = VecDeque::from([Box::new(LetStatement {
            token: Token::new(TokenType::LET, "let"),
            name: Box::new(Identifier {
                token: Token::new(TokenType::IDENT, "myVar"),
                value: "myVar".to_string(),
            }),
            value: Box::new(Identifier {
                value: "anotherVar".to_string(),
                token: Token::new(TokenType::IDENT, "anotherVar"),
            }),
        }) as Box<dyn Statement>]);

        let program = Program { statments };

        if program.to_string() != "let myVar = anotherVar;".to_string() {
            panic!("Couldn't print properly, got={}", program)
        }
    }
}
