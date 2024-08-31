mod tests {
    use core::panic;
    use std::usize;

    use crate::bytecode::{make, Instruction, Instructions};

    use crate::compiler::Compiler;
    use crate::object::ObjectType;
    use crate::{
        ast::{Expressions, Node, Nodes, Statements},
        lexer::Lexer,
        parser::Parser,
        token,
    };

    #[test]
    fn test_compiler() {
        struct Test<'a, const T: usize, const U: usize> {
            input: String,
            expected_constant: [&'a str; T],
            expected_instructions: [Instruction; U],
        }

        impl<'a, const T: usize, const U: usize> Test<'a, T, U> {
            fn new<E>(
                input: E,
                expected_constant: [&'a str; T],
                expected_instructions: [Instruction; U],
            ) -> Self
            where
                E: Into<String>,
            {
                Self {
                    input: input.into(),
                    expected_constant,
                    expected_instructions,
                }
            }
        }

        let tests = [Test::new(
            "1 + 2",
            ["1", "2"],
            [
                make(&Instructions::OpConstant, [1]).unwrap(),
                make(&Instructions::OpConstant, [2]).unwrap(),
            ],
        )];

        fn run_compiler_test<const T: usize, const U: usize>(tests: &[Test<T, U>]) {
            for test in tests {
                // Parsing
                let lexer = Lexer::new(&test.input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();

                let mut compiler = Compiler::new();

                match compiler.compile(program.statments) {
                    Ok(..) => {
                        let bytecode = compiler.bytecode();
                        h_test_instruction(&test.expected_instructions, &bytecode.instruction);
                        h_test_constant(&test.expected_constant, &bytecode.constant);
                    }
                    Err(..) => {
                        panic!("Compile error: ");
                    }
                }
            }
        }
        run_compiler_test(&tests)
    }

    fn h_test_instruction<const U: usize>(expected: &[Instruction; U], actual: &Instruction) {
        let concatted = concat_instruction(expected);

        if concatted.0.len() != actual.0.len() {
            panic!(
                "Wrong instructions length. want={}, got={}",
                concatted.0.len(),
                actual.0.len(),
            )
        }

        for (idx, ins) in concatted.0.iter().enumerate() {
            if actual.0[idx] != *ins {
                panic!(
                    "Wrong instruction at {}, want={:?}, got={:?}",
                    idx, concatted, actual
                )
            }
        }
    }
    fn concat_instruction<const U: usize>(instruction: &[Instruction; U]) -> Instruction {
        instruction
            .clone()
            .into_iter()
            .map(|i| i.0)
            .flatten()
            .collect()
    }

    fn h_test_constant<const T: usize>(expected: &[&str; T], actual: &Vec<ObjectType>) {
        if expected.len() != actual.len() {
            panic!(
                "Wrong number of constants. got={}, want={}",
                actual.len(),
                expected.len()
            )
        }

        for (idx, constant) in expected.into_iter().enumerate() {
            if let Ok(res) = constant.parse::<usize>() {
                h_test_integer_object(res, actual[idx])
            }
        }
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
