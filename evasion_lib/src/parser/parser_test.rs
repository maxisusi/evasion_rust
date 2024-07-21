#[cfg(test)]

mod tests {
    use core::panic;
    use std::any::Any;

    use crate::{
        ast::{LetStatement, Node, Statement},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_let_statments() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if let Some(mut program) = program {
            if program.statments.len() != 3 {
                panic!(
                    "program.statments doesn't contain 3 statments, got={}",
                    program.statments.len()
                )
            }

            let expected_identifier = ["x", "y", "foobar"];
            for identif in expected_identifier.into_iter() {
                let stmt = program.statments.pop_back().unwrap();

                h_test_let_statments(stmt, identif);
            }
        } else {
            panic!("parse_program() return null")
        }
    }
    // TODO: Fix it  https://bennett.dev/dont-use-boxed-trait-objects-for-struct-internals/
    fn h_test_let_statments(stmt: Box<dyn Statement>, ident: &str) {
        match stmt.as_ref().as_any().downcast_ref::<LetStatement>() {
            Some(stmt) => {
                if stmt.token_litteral() != "let" {
                    panic!("token_litteral() not 'let', got={}", stmt.token_litteral());
                }

                if stmt.name.value.as_str() != ident {
                    panic!("Identifier.value is not={} got={}", ident, stmt.name.value);
                }

                if stmt.name.token_litteral() != ident {
                    panic!(
                        "Identifier.name.token_litteral() is not={} got={}",
                        ident, stmt.name.value
                    );
                }
            }
            None => {
                panic!("Expected LetStatement, got={:?}", stmt.type_id())
            }
        }
    }

    fn check_parser_errors(parser: &mut Parser) {
        let errors = &parser.errors;
        if errors.len() == 0 {
            return;
        }

        println!("Parser has {} errors", errors.len());

        for err in errors {
            println!("parser error: {}", err);
        }

        panic!("Failed to parse input");
    }
}
