mod tests {

    use core::panic;
    use std::isize;

    use crate::ast;
    use crate::compiler;
    use crate::object::ObjectType;
    use crate::vm::VirtualMachine;
    use crate::{
        ast::{Expressions, Node, Nodes, Statements},
        lexer::Lexer,
        parser::Parser,
        token,
    };

    #[test]
    fn test_vm() {
        struct Test {
            input: String,
            expected: String,
        }

        impl Test {
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
            Test::new("1", "1"),
            Test::new("2", "2"),
            Test::new("1 + 2", "3"),
            Test::new("1 * 2", "2"),
            Test::new("4 / 2", "2"),
            Test::new("50 / 2 * 2  + 10 - 5", "55"),
            Test::new("5 + 5 + 5 + 5 - 10", "10"),
            Test::new("2 * 2 * 2 * 2 * 2", "32"),
            Test::new("5 * 2 + 10", "20"),
            Test::new("5 + 2 * 10", "25"),
            Test::new("5 * (2 + 10)", "60"),
            Test::new("true", "true"),
            Test::new("false", "false"),
            Test::new("1 < 2", "true"),
            Test::new("1 > 2", "false"),
            Test::new("1 < 1", "false"),
            Test::new("1 > 1", "false"),
            Test::new("1 == 1", "true"),
            Test::new("1 != 1", "false"),
            Test::new("1 == 2", "false"),
            Test::new("1 != 2", "true"),
            Test::new("true == true", "true"),
            Test::new("false == false", "true"),
            Test::new("true == false", "false"),
            Test::new("true != false", "true"),
            Test::new("(1 < 2) == true", "true"),
            Test::new("(1 < 2) == false", "false"),
            Test::new("(1 > 2) == true", "false"),
            Test::new("(1 > 2) == false", "true"),
            Test::new("-5", "-5"),
            Test::new("-10", "-10"),
            Test::new("-50 + 100 + -50", "0"),
            Test::new("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
            Test::new("!true", "false"),
            Test::new("!false", "true"),
            Test::new("!5", "false"),
            Test::new("!!true", "true"),
            Test::new("!!false", "false"),
            Test::new("!!5", "true"),
        ];

        for test in tests {
            let program = h_parse(&test.input);

            let mut compiler = compiler::Compiler::new();
            compiler.compile_program(program.statments);
            let bytecode = compiler.bytecode();

            let mut vm = VirtualMachine::new(bytecode);

            if let Err(err) = vm.run() {
                panic!("An error occured: {err}")
            }

            let stack_top = vm.last_popped_stack_elem();

            h_test_expected_object(test.expected, stack_top);
        }
    }

    fn h_test_expected_object<T>(expected: T, actual: ObjectType)
    where
        T: Into<String>,
    {
        // Get the type of the string
        let expected: String = expected.into();
        if let Ok(integer) = expected.parse::<isize>() {
            h_test_integer_object(integer, actual)
        } else if Ok(true) == expected.parse::<bool>() || Ok(false) == expected.parse::<bool>() {
            match actual {
                ObjectType::Boolean(boolean_value) => {
                    if boolean_value != expected.parse::<bool>().unwrap() {
                        panic!("Wrong value. got={}, want={}", actual, expected)
                    }
                }
                _ => panic!(
                    "Unexpected value, expected Boolean Object, got={:?}",
                    actual
                ),
            }
        } else {
            panic!(
                "Unexpected value, expected Integer Object or Boolean, got={:?}",
                actual
            )
        }
    }

    fn h_parse(input: &str) -> ast::Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }

    fn h_test_integer_object(value: isize, actual: ObjectType) {
        match actual {
            ObjectType::Integer(integer_value) => {
                if integer_value != value {
                    panic!("Wrong value. got={}, want={}", actual, value)
                }
            }
            _ => panic!(
                "Unexpected value, expected Integer Object, got={:?}",
                actual
            ),
        }
    }
}
