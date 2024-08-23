mod tests {
    use crate::{
        ast::{Expressions, Node, Nodes, Statements},
        lexer::Lexer,
        parser::Parser,
        token,
    };
    use std::{ops::Deref, usize};

    #[test]
    fn test_let_statments() {
        struct LetStmtTest {
            input: String,
            expected_ident: String,
            expected_value: String,
        }

        impl LetStmtTest {
            fn new<T>(input: T, expected_ident: T, expected_value: T) -> Self
            where
                T: Into<String>,
            {
                Self {
                    input: input.into(),
                    expected_value: expected_value.into(),
                    expected_ident: expected_ident.into(),
                }
            }
        }

        let inputs = vec![
            LetStmtTest::new("let x = 5;", "x", "5"),
            LetStmtTest::new("let y = 10;", "y", "10"),
            LetStmtTest::new("let foobar = 838383", "foobar", "838383"),
        ];
        for input in inputs {
            let lexer = Lexer::new(&input.input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            let statement: &Statements = &program.statments[0].clone().try_into().unwrap();
            match statement {
                Statements::Let { token, name, value } => {
                    h_test_let_statments(statement, input.expected_ident.as_str());
                    h_test_integer_litteral(value, input.expected_value.clone());
                }
                _ => panic!("Expected Let Statement, got={}", statement.display_type()),
            }
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

                if name.token_litteral() != ident.trim() {
                    panic!(
                        "Identifier.name.token_litteral() is not={} got={}",
                        ident, name
                    );
                }
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

    #[test]
    fn test_return_statements() {
        struct RetStmtTest {
            input: String,
            expected_value: String,
        }

        impl RetStmtTest {
            fn new<T>(input: T, expected_value: T) -> Self
            where
                T: Into<String>,
            {
                Self {
                    input: input.into(),
                    expected_value: expected_value.into(),
                }
            }
        }

        let inputs = vec![
            RetStmtTest::new("return 5;", "5"),
            RetStmtTest::new("return 10;", "10"),
        ];

        for input in inputs {
            let lexer = Lexer::new(&input.input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            let statement: &Statements = &program.statments[0].clone().try_into().unwrap();
            match statement {
                Statements::Return { token, value } => {
                    if statement.token_litteral() != "return" {
                        panic!(
                            "token_litteral() not 'return', got={}",
                            statement.token_litteral()
                        );
                    }
                    h_test_integer_litteral(value, input.expected_value.clone());
                }
                _ => panic!(
                    "Expected Return Statement, got={}",
                    statement.display_type()
                ),
            }
        }
    }

    #[test]
    fn test_identifier() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if program.statments.len() != 1 {
            panic!(
                "program.statments doesn't contain 1 statments, got={}",
                program.statments.len()
            )
        }
        let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();
        h_test_identifier(expression, "foobar".to_string());
    }

    #[test]
    fn test_integer_litteral() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if program.statments.len() != 1 {
            panic!(
                "program.statments doesn't contain 1 statments, got={}",
                program.statments.len()
            )
        }

        let expression = &program.statments[0].clone().try_into().unwrap();
        h_test_integer_litteral(expression, "5".to_string())
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

        let inputs = vec![
            PrefixTest::new("!5", "!", "5"),
            PrefixTest::new("-15", "-", "15"),
            PrefixTest::new("!true", "!", "true"),
            PrefixTest::new("!false", "!", "false"),
        ];

        for input in inputs {
            let lexer = Lexer::new(input.input.as_str());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();

            match expression {
                Expressions::Prefix {
                    token: _token,
                    right,
                    operator,
                } => {
                    let right = right.deref();

                    if let Some(operhand) = input.operhand.parse::<u64>().ok() {
                        h_test_integer_litteral(right, input.operhand.parse().unwrap());
                    } else {
                        if let Some(operhand) = input.operhand.parse::<bool>().ok() {
                            h_test_boolean(right, operhand);
                        } else {
                            panic!("Couldn't convert value to u64 nor boolean");
                        }
                    }
                    if *operator != input.operator {
                        panic!("Expected {}, got={}", input.operator, operator);
                    }
                }
                _ => {
                    panic!(
                        "Expected Prefix Expression, got={}",
                        expression.display_type()
                    )
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
        let inputs = vec![
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

        for input in inputs {
            let lexer = Lexer::new(input.input.as_str());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();

            match expression {
                Expressions::Infix { .. } => h_test_infix_expression(
                    expression,
                    input.left.clone(),
                    input.op.clone(),
                    input.right.clone(),
                ),
                _ => panic!(
                    "Expected Infix Expression, got={}",
                    expression.display_type()
                ),
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
                if let Some(t_left_p) = t_left.parse::<u64>().ok() {
                    h_test_integer_litteral(&left, t_left);
                    h_test_integer_litteral(&right, t_right);
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
        struct PrecedenceTest {
            input: String,
            expected: String,
        }
        impl PrecedenceTest {
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

        let inputs = vec![
            PrecedenceTest::new("-a * b", "((-a) * b)"),
            PrecedenceTest::new("!-a", "(!(-a))"),
            PrecedenceTest::new("a + b + c", "((a + b) + c)"),
            PrecedenceTest::new("a + b - c", "((a + b) - c)"),
            PrecedenceTest::new("a * b * c", "((a * b) * c)"),
            PrecedenceTest::new("a * b / c", "((a * b) / c)"),
            PrecedenceTest::new("a + b / c", "(a + (b / c))"),
            PrecedenceTest::new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            PrecedenceTest::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            PrecedenceTest::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            PrecedenceTest::new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            PrecedenceTest::new(
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            PrecedenceTest::new("true", "true"),
            PrecedenceTest::new("false", "false"),
            PrecedenceTest::new("3 > 5 == false", "((3 > 5) == false)"),
            PrecedenceTest::new("3 < 5 == true", "((3 < 5) == true)"),
            PrecedenceTest::new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            PrecedenceTest::new("(5 + 5) * 2", "((5 + 5) * 2)"),
            PrecedenceTest::new("2 / (5 + 5)", "(2 / (5 + 5))"),
            PrecedenceTest::new("-(5 + 5)", "(-(5 + 5))"),
            PrecedenceTest::new("!(true == true)", "(!(true == true))"),
            PrecedenceTest::new("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            PrecedenceTest::new(
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            PrecedenceTest::new(
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];

        for input in inputs {
            let lexer = Lexer::new(input.input.as_str());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            if input.expected != program.to_string() {
                panic!("Expected {}, got={}", input.expected, program.to_string())
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

        if program.statments.len() != 1 {
            panic!(
                "program.statments doesn't contain 1 statments, got={}",
                program.statments.len()
            )
        }

        let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();

        match expression {
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
                    Statements::BlockStatements { token, statements } => match &statements[0] {
                        Nodes::Expression(e) => {
                            h_test_identifier(&e, "x".to_string());
                        }
                        _ => panic!("Expected an Expession, got={}", statements[0]),
                    },
                    _ => panic!("Expected to find a BlockStatements, got={}", consequence),
                };

                // Check if there is no alternative
                if let Some(alt) = alternative {
                    panic!("Should not have an alternatice statement")
                }
            }

            _ => panic!(
                "was exprecting IntegerLiteral, got={}",
                expression.display_type()
            ),
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x > y) { x } else { y }";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if program.statments.len() != 1 {
            panic!(
                "program.statments doesn't contain 1 statments, got={}",
                program.statments.len()
            )
        }

        let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();

        match expression {
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
                        let expression: &Expressions = &statements[0].clone().try_into().unwrap();
                        h_test_identifier(&expression, "x".to_string());
                    }
                    _ => panic!("Expected to find a BlockStatements, got={}", consequence),
                };

                // Check if there is no alternative
                if let Some(alt) = alternative {
                    match alt.deref() {
                        Statements::BlockStatements { token, statements } => {
                            let expression: &Expressions =
                                &statements[0].clone().try_into().unwrap();
                            h_test_identifier(&expression, "y".to_string());
                        }
                        _ => panic!("Expected to find a BlockStatements, got={}", alt),
                    };
                }
            }

            _ => panic!(
                "was exprecting IntegerLiteral, got={}",
                expression.display_type()
            ),
        }
    }

    #[test]
    fn test_fn_expression() {
        let input = "fn(x, y) { x + y; }";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if program.statments.len() != 1 {
            panic!(
                "program.statments doesn't contain 1 statments, got={}",
                program.statments.len()
            )
        }

        let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();

        match expression {
            Expressions::FnExpression {
                token,
                body,
                parameters,
            } => {
                if expression.token_litteral() != "fn" {
                    panic!(
                        "Was expecting fn as token litteral, got={}",
                        expression.token_litteral()
                    );
                }

                h_test_identifier(&parameters[0], "x".to_string());
                h_test_identifier(&parameters[1], "y".to_string());

                // Testing consequence Block Statement
                let body = match body.deref() {
                    Statements::BlockStatements { token, statements } => {
                        let expression: &Expressions = &statements[0].clone().try_into().unwrap();
                        h_test_infix_expression(
                            expression,
                            "x".to_string(),
                            "+".to_string(),
                            "y".to_string(),
                        );
                    }
                    _ => panic!("Expected to find a BlockStatements, got={}", body),
                };
            }

            n => panic!("was exprecting IntegerLiteral, got={}", n.display_type()),
        }
    }

    #[test]
    fn test_fn_parameter() {
        struct FnParamsTest {
            input: String,
            expected: Vec<String>,
        }

        impl FnParamsTest {
            fn new<T>(input: T, expected: Vec<String>) -> Self
            where
                T: Into<String>,
            {
                Self {
                    input: input.into(),
                    expected,
                }
            }
        }

        let tests: Vec<FnParamsTest> = vec![
            FnParamsTest::new("fn() {}", vec![]),
            FnParamsTest::new("fn(x) {}", vec!["x".to_string()]),
            FnParamsTest::new(
                "fn(x, y, z) {}",
                vec!["x".to_string(), "y".to_string(), "z".to_string()],
            ),
        ];

        for test in tests {
            let lexer = Lexer::new(&test.input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();
            check_parser_errors(&mut parser);

            let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();

            match expression {
                Expressions::FnExpression {
                    token,
                    body,
                    parameters,
                } => {
                    for (idx, expected_param) in test.expected.iter().enumerate() {
                        h_test_identifier(&parameters[idx], expected_param.to_string());
                    }
                }
                _ => panic!(
                    "was exprecting FnExpression, got={}",
                    expression.display_type()
                ),
            }
        }
    }

    #[test]
    fn test_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5);";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if program.statments.len() != 1 {
            panic!(
                "program.statments doesn't contain 1 statments, got={}",
                program.statments.len()
            )
        }

        let expression: &Expressions = &program.statments[0].clone().try_into().unwrap();

        match expression {
            Expressions::CallExpression {
                token,
                function,
                arguments,
            } => {
                h_test_identifier(&function, "add".to_string());

                if arguments.len() != 3 {
                    panic!("Expected 3 arguments, got={}", arguments.len())
                }
                h_test_integer_litteral(&arguments[0], "1".to_string());
                h_test_infix_expression(
                    &arguments[1],
                    "2".to_string(),
                    "*".to_string(),
                    "3".to_string(),
                );
                h_test_infix_expression(
                    &arguments[2],
                    "4".to_string(),
                    "+".to_string(),
                    "5".to_string(),
                )
            }
            n => panic!("was exprecting CallExpression, got={}", n.display_type()),
        }
    }

    fn h_test_boolean(expression: &Expressions, expected_bool: bool) {
        match expression {
            Expressions::Boolean { token, value } => {
                if *value != expected_bool {
                    panic!("Was expecting {}, got={}", value, expected_bool)
                }
            }
            _ => panic!("Was expecting integer litteral, got={}", expression),
        }
    }

    fn h_test_identifier(expression: &Expressions, expected_identifier: String) {
        match expression {
            Expressions::Identifier { .. } => {
                if expression.token_litteral() != expected_identifier {
                    panic!(
                        "token_litteral() not '{}', got={}",
                        expected_identifier,
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

    fn h_test_integer_litteral(expression: &Expressions, expected_integer: String) {
        return match expression {
            Expressions::IntegerLiteral { .. } => {
                if expression.token_litteral() != expected_integer {
                    panic!(
                        "token_litteral() not '5', got={}",
                        expression.token_litteral()
                    );
                }
            }
            _ => panic!(
                "was exprecting IntegerLiteral, got={}",
                expression.display_type()
            ),
        };
    }
}
