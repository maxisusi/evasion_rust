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
                Nodes::Expression(stmt) => h_test_identifier(stmt, "foobar".to_string()),
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
            operhand: String,
        }
        impl PrefixTest {
            fn new<T>(input: T, operator: T, operhand: T) -> Self
            where
                T: Into<String>,
            {
                Self {
                    operator: operator.into(),
                    operhand: operhand.into(),
                    input: input.into(),
                }
            }
        }

        let tests = vec![
            PrefixTest::new("!5", "!", "5"),
            PrefixTest::new("-15", "-", "15"),
            PrefixTest::new("!true", "!", "true"),
            PrefixTest::new("!false", "!", "false"),
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
                            let right = right.deref();

                            if let Some(operhand) = test.operhand.parse::<u64>().ok() {
                                h_test_interger(right, operhand);
                            } else {
                                if let Some(operhand) = test.operhand.parse::<bool>().ok() {
                                    h_test_boolean(right, operhand);
                                } else {
                                    panic!("Couldn't convert value to u64 nor boolean");
                                }
                            }
                            if *operator != test.operator {
                                panic!("Expected {}, got={}", test.operator, operator);
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
    fn test_parsing_infix_expression() {
        struct InfixTests {
            input: String,
            left: String,
            op: String,
            right: String,
        }
        impl InfixTests {
            fn new<T>(input: T, left: T, op: T, right: T) -> Self
            where
                T: Into<String>,
            {
                Self {
                    input: input.into(),
                    left: left.into(),
                    right: right.into(),
                    op: op.into(),
                }
            }
        }
        let tests = vec![
            InfixTests::new("5 + 4;", "5", "+", "4"),
            InfixTests::new("5 - 5;", "5", "-", "5"),
            InfixTests::new("5 * 5;", "5", "*", "5"),
            InfixTests::new("5 / 5;", "5", "/", "5"),
            InfixTests::new("5 < 5;", "5", "<", "5"),
            InfixTests::new("5 > 5;", "5", ">", "5"),
            InfixTests::new("5 == 5;", "5", "==", "5"),
            InfixTests::new("5 != 5;", "5", "!=", "5"),
            InfixTests::new("true != true;", "true", "!=", "true"),
            InfixTests::new("false == false;", "false", "==", "false"),
            InfixTests::new("true == false;", "true", "==", "false"),
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
                        Expressions::Infix { .. } => h_test_infix_expression(
                            stmt,
                            test.left.clone(),
                            test.op.clone(),
                            test.right.clone(),
                        ),
                        _ => {}
                    },
                    _ => {
                        panic!("Expected an expression, got={}", stmt)
                    }
                }
            }
        }
    }

    fn h_test_infix_expression(
        exp_infix: &Expressions,
        t_left: String,
        t_op: String,
        t_right: String,
    ) {
        match exp_infix {
            Expressions::Infix {
                token: _token,
                left,
                operator,
                right,
            } => {
                if let Some(t_left) = t_left.parse::<u64>().ok() {
                    h_test_interger(&left, t_left);
                    h_test_interger(&right, t_right.parse().unwrap());
                } else {
                    if let Some(t_left) = t_left.parse::<bool>().ok() {
                        h_test_boolean(&left, t_left);
                        h_test_boolean(&right, t_right.parse().unwrap());
                    } else {
                        // Test Identifier
                        h_test_identifier(&left, t_left.to_string());
                        h_test_identifier(&right, t_right.to_string());
                    }
                }
            }
            _ => panic!("Expected an expression, got={}", exp_infix),
        }
    }

    #[test]
    fn test_precedence() {
        struct Tests {
            input: String,
            expected: String,
        }
        impl Tests {
            fn new<T>(input: T, expected: T) -> Self
            where
                T: Into<String>,
            {
                Self {
                    input: input.into(),
                    expected: expected.into(),
                }
            }
        }

        let tests = vec![
            Tests::new("-a * b", "((-a) * b)"),
            Tests::new("!-a", "(!(-a))"),
            Tests::new("a + b + c", "((a + b) + c)"),
            Tests::new("a + b - c", "((a + b) - c)"),
            Tests::new("a * b * c", "((a * b) * c)"),
            Tests::new("a * b / c", "((a * b) / c)"),
            Tests::new("a + b / c", "(a + (b / c))"),
            Tests::new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            Tests::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            Tests::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            Tests::new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            Tests::new(
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            Tests::new("true", "true"),
            Tests::new("false", "false"),
            Tests::new("3 > 5 == false", "((3 > 5) == false)"),
            Tests::new("3 < 5 == true", "((3 < 5) == true)"),
            Tests::new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            Tests::new("(5 + 5) * 2", "((5 + 5) * 2)"),
            Tests::new("2 / (5 + 5)", "(2 / (5 + 5))"),
            Tests::new("-(5 + 5)", "(-(5 + 5))"),
            Tests::new("!(true == true)", "(!(true == true))"),
        ];

        for test in tests.iter() {
            let lexer = Lexer::new(test.input.as_str());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            if let Some(program) = program {
                if test.expected != program.to_string() {
                    panic!("Expected {}, got={}", test.expected, program.to_string())
                }
            }
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x > y) { x }";

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
                    Expressions::IfExpression {
                        token,
                        alternative,
                        consequence,
                        condition,
                    } => {
                        // Testing conditions
                        h_test_infix_expression(
                            condition,
                            "x".to_string(),
                            ">".to_string(),
                            "y".to_string(),
                        );

                        // Testing consequence Block Statement
                        let consequence = match consequence.deref() {
                            Statements::BlockStatements { token, statements } => {
                                match &statements[0] {
                                    Nodes::Expression(e) => {
                                        h_test_identifier(&e, "x".to_string());
                                    }
                                    _ => panic!("Expected an Expession, got={}", statements[0]),
                                }
                            }
                            _ => panic!("Expected to find a BlockStatements, got={}", consequence),
                        };

                        // Check if there is no alternative
                        if let Some(alt) = alternative {
                            panic!("Should not have an alternatice statement")
                        }
                    }

                    n => panic!("was exprecting IntegerLiteral, got={}", n.display_type()),
                },
                n => panic!("was exprecting Expression, got={}", n),
            };
        }
    }

    fn h_test_interger(expression: &Expressions, exp: u64) {
        match expression {
            Expressions::IntegerLiteral { token, value } => {
                if *value != exp {
                    panic!("Was expecting {}, got={}", exp, value)
                }
            }
            _ => panic!("Was expecting integer litteral, got={}", expression),
        }
    }

    fn h_test_boolean(expression: &Expressions, exp: bool) {
        match expression {
            Expressions::Boolean { token, value } => {
                if *value != exp {
                    panic!("Was expecting {}, got={}", value, exp)
                }
            }
            _ => panic!("Was expecting integer litteral, got={}", expression),
        }
    }

    fn h_test_identifier(expression: &Expressions, expected: String) {
        match expression {
            Expressions::Identifier { .. } => {
                if expression.token_litteral() != expected {
                    panic!(
                        "token_litteral() not 'foobar', got={}",
                        expression.token_litteral()
                    );
                }
            }
            _ => panic!(
                "was expecting Identifier, got={}",
                expression.display_type()
            ),
        }
    }
}
