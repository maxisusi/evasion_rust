mod tests {

    use core::panic;

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
        if let Ok(integer) = expected.parse::<usize>() {
            h_test_integer_object(integer, actual)
        } else {
            panic!("Couldn't parse expected value")
        }
    }

    fn h_parse(input: &str) -> ast::Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }

    fn h_test_integer_object(value: usize, actual: ObjectType) {
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
