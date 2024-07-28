#[cfg(test)]

mod tests {
    use core::panic;
    use std::any::Any;

    use crate::{
        ast::{Node, Statements},
        lexer::Lexer,
        parser::Parser,
    };

    // LET STATMENTS

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
            for identif in expected_identifier {
                let stmt = program.statments.pop_back().unwrap();

                h_test_let_statments(stmt, identif);
            }
        } else {
            panic!("parse_program() return null")
        }
    }
    fn h_test_let_statments(stmt: Statements, ident: &str) {
        match &stmt {
            Statements::LetStatement { name, .. } => {
                if stmt.token_litteral() != "let" {
                    panic!("token_litteral() not 'let', got={}", stmt.token_litteral());
                }

                if name.to_string() != ident {
                    panic!("Identifier.value is not={} got={}", ident, name);
                }

                if name.token_litteral() != ident {
                    panic!(
                        "Identifier.name.token_litteral() is not={} got={}",
                        ident, name
                    );
                }

                //TODO: Check the expression as well (value)
            }
            _ => panic!("Expected a Let statement but got={}", stmt),
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

    // RETURN STATMENTS

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
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
            for _ in expected_identifier {
                let stmt = program.statments.pop_back().unwrap();
                match &stmt {
                    Statements::ReturnStatement { .. } => {
                        if stmt.token_litteral() != "return" {
                            panic!(
                                "token_litteral() not 'return', got={}",
                                stmt.token_litteral()
                            );
                        }
                        // TODO: Check the expression as well
                    }
                    _ => {
                        panic!("Expected ReturnStatement, got={}", stmt)
                    }
                }
            }
        } else {
            panic!("parse_program() return null")
        }
    }
}
