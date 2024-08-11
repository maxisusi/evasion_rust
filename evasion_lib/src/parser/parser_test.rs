mod tests {
    use core::panic;
    use std::{ops::Deref, usize};

    use crate::{
        ast::{Expressions, Node, Nodes, Statements},
        lexer::Lexer,
        parser::Parser,
        token,
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

        if let Some(program) = program {
            if program.statments.len() != 3 {
                panic!(
                    "program.statments doesn't contain 3 statments, got={}",
                    program.statments.len()
                )
            }

            let expected_identifier = ["x", "y", "foobar"];

            let mut program_iter = program.statments.iter();

            for identif in expected_identifier {
                if let Some(nodes) = program_iter.next() {
                    match &nodes {
                        Nodes::Statement(stmt) => {
                            h_test_let_statments(stmt, identif);
                        }
                        n => panic!("Expected Statement but got={}", n),
                    }
                } else {
                    break;
                }
            }
        } else {
            panic!("parse_program() return null")
        }
    }
    fn h_test_let_statments(stmt: &Statements, ident: &str) {
        match &stmt {
            Statements::Let { name, .. } => {
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
            _ => panic!("Expected a Let statement but got={}", stmt.display_type()),
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

        if let Some(program) = program {
            if program.statments.len() != 3 {
                panic!(
                    "program.statments doesn't contain 3 statments, got={}",
                    program.statments.len()
                )
            }

            let expected_identifier = ["x", "y", "foobar"];

            let mut program_iter = program.statments.iter();

            for _ in expected_identifier {
                if let Some(node) = program_iter.next() {
                    match &node {
                        Nodes::Statement(stmts) => {
                            match stmts {
                                Statements::Return { token, .. } => {
                                    if stmts.token_litteral() != "return" {
                                        panic!(
                                            "token_litteral() not 'return', got={}",
                                            stmts.token_litteral()
                                        );
                                    }
                                }
                                n => panic!("Expected ReturnStatement, got={}", n.display_type()),
                            }
                            // TODO: Check the expression as well
                        }
                        n => {
                            panic!("Expected Statement, got={}", n)
                        }
                    }
                } else {
                    break;
                }
            }
        } else {
            panic!("parse_program() return null")
        }
    }

    // EXPRESSIONS

    #[test]
    fn test_identifier() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if let Some(program) = program {
            if program.statments.len() != 1 {
                panic!(
                    "program.statments doesn't contain 1 statments, got={}",
                    program.statments.len()
                )
            }

            let node = &program.statments[0];

            match node {
                Nodes::Expression(stmt) => match stmt {
                    Expressions::Identifier { .. } => {
                        if stmt.token_litteral() != "foobar" {
                            panic!(
                                "token_litteral() not 'foobar', got={}",
                                stmt.token_litteral()
                            );
                        }
                    }
                    _ => panic!("was expecting Identifier, got={}", stmt.display_type()),
                },
                n => panic!("was exprecting Expression, got={}", n),
            };
        }
    }

    #[test]
    fn test_integer_litteral() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if let Some(program) = program {
            if program.statments.len() != 1 {
                panic!(
                    "program.statments doesn't contain 1 statments, got={}",
                    program.statments.len()
                )
            }

            let stmt = &program.statments[0];

            match stmt {
                Nodes::Expression(stmt) => match stmt {
                    Expressions::IntegerLiteral { .. } => {
                        if stmt.token_litteral() != "5" {
                            panic!("token_litteral() not '5', got={}", stmt.token_litteral());
                        }
                    }

                    n => panic!("was exprecting IntegerLiteral, got={}", n.display_type()),
                },
                n => panic!("was exprecting Expression, got={}", n),
            };
        }
    }

    #[test]
    fn test_prefix_expression() {
        struct PrefixTest {
            input: String,
            operator: String,
            interger_value: u64,
        }
        impl PrefixTest {
            fn new<T>(input: T, operator: T, interger_value: u64) -> Self
            where
                T: Into<String>,
            {
                Self {
                    operator: operator.into(),
                    interger_value,
                    input: input.into(),
                }
            }
        }

        let tests = vec![
            PrefixTest::new("!5", "!", 5),
            PrefixTest::new("-15", "-", 15),
        ];

        for test in tests.iter() {
            let lexer = Lexer::new(test.input.as_str());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            if let Some(stmt) = program {
                let stmt = &stmt.statments[0];

                match stmt {
                    Nodes::Expression(stmt) => match stmt {
                        Expressions::Prefix {
                            token: _token,
                            right,
                            operator,
                        } => {
                            // Test if the expression is infix
                            let right = right.deref();
                            match right {
                                Expressions::IntegerLiteral { .. } => {
                                    h_test_interger(right, test.interger_value);
                                    if *operator != test.operator {
                                        panic!("Expected {}, got={}", test.operator, operator);
                                    }
                                }
                                _ => panic!("Expected prefix, got={}", right.display_type()),
                            }
                        }
                        _ => {
                            panic!("Expected Prefix Expression, got={}", stmt.display_type())
                        }
                    },
                    _ => {
                        panic!("Expected an expression, got={}", stmt)
                    }
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_parsing_infix_expression() {
        struct InfixTests {
            input: String,
            left: usize,
            op: String,
            right: usize,
        }
        impl InfixTests {
            fn new<T>(input: T, left: usize, op: T, right: usize) -> Self
            where
                T: Into<String>,
            {
                Self {
                    input: input.into(),
                    left,
                    right,
                    op: op.into(),
                }
            }
        }
        let tests = vec![
            InfixTests::new("5 + 5", 5, "+", 5),
            InfixTests::new("5 - 5", 5, "-", 5),
            InfixTests::new("5 * 5", 5, "*", 5),
            InfixTests::new("5 / 5", 5, "/", 5),
            InfixTests::new("5 < 5", 5, "<", 5),
            InfixTests::new("5 > 5", 5, ">", 5),
            InfixTests::new("5 == 5", 5, "==", 5),
            InfixTests::new("5 != 5", 5, "!=", 5),
        ];

        for test in tests.iter() {
            let lexer = Lexer::new(test.input.as_str());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            if let Some(stmt) = program {
                let stmt = &stmt.statments[0];

                match stmt {
                    Nodes::Expression(stmt) => match stmt {
                        Expressions::Generic { expression, .. } => {
                            // Test if the expression is infix
                            let expr = expression.deref();
                            match expr {
                                Expressions::Infix {
                                    token: _token,
                                    left,
                                    operator,
                                    right,
                                } => {
                                    // h_test_interger(left.token_litteral().to_string(), test.left);
                                    // h_test_interger(right.token_litteral().to_string(), test.left);
                                    // if *operator != test.op {
                                    //     panic!("Expected {}, got={}", test.op, operator);
                                    // }
                                }
                                _ => panic!("Expected infix, got={}", expr.display_type()),
                            }
                        }
                        _ => {
                            panic!("Expected Generic expression, got={}", stmt.display_type())
                        }
                    },
                    _ => {
                        panic!("Expected an expression, got={}", stmt)
                    }
                }
            }
        }
    }

    fn h_test_interger(elem: &Expressions, exp: u64) {
        match elem {
            Expressions::IntegerLiteral { token, value } => {
                if *value != exp {
                    panic!("Expected {}, got={}", exp, value)
                }
            }
            _ => {
                panic!("Expected an Integer Literal, got={}", elem.display_type())
            }
        }
    }
}
