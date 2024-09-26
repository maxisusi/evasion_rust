mod tests {
    use core::panic;
    use std::{isize, usize};

    use crate::bytecode::{make, Instruction, Instructions};

    use crate::ast;
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
        struct Test {
            input: String,
            expected_constant: Vec<String>,
            expected_instructions: Vec<Instruction>,
        }

        impl Test {
            fn new<E>(
                input: E,
                expected_constant: Vec<E>,
                expected_instructions: Vec<Instruction>,
            ) -> Self
            where
                E: Into<String>,
            {
                Self {
                    input: input.into(),
                    expected_constant: expected_constant.into_iter().map(|c| c.into()).collect(),
                    expected_instructions,
                }
            }
        }

        let tests = [
            Test::new(
                "1 + 2",
                vec!["1", "2"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpAdd, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "1; 2",
                vec!["1", "2"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "1 - 2",
                vec!["1", "2"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpSub, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "1 * 2",
                vec!["1", "2"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpMul, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "2 / 1",
                vec!["2", "1"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpDiv, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "true",
                vec!["true"],
                vec![
                    make(&Instructions::OpTrue, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "false",
                vec!["false"],
                vec![
                    make(&Instructions::OpFalse, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "1 > 2",
                vec!["1", "2"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpGreaterThan, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "1 < 2",
                vec!["2", "1"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpGreaterThan, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "1 == 2",
                vec!["1", "2"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpEqual, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "1 != 2",
                vec!["1", "2"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpConstant, &vec![1]).unwrap(),
                    make(&Instructions::OpNotEqual, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "true == false",
                vec!["true", "false"],
                vec![
                    make(&Instructions::OpTrue, &vec![]).unwrap(),
                    make(&Instructions::OpFalse, &vec![]).unwrap(),
                    make(&Instructions::OpEqual, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "-1",
                vec!["1"],
                vec![
                    make(&Instructions::OpConstant, &vec![0]).unwrap(),
                    make(&Instructions::OpMinus, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            Test::new(
                "!true",
                vec!["true"],
                vec![
                    make(&Instructions::OpTrue, &vec![]).unwrap(),
                    make(&Instructions::OpBang, &vec![]).unwrap(),
                    make(&Instructions::OpPop, &vec![]).unwrap(),
                ],
            ),
            // Test::new(
            //     "if (true) { 10 }; 3333;",
            //     vec!["10, 3333"],
            //     vec![
            //         // 000
            //         make(&Instructions::OpTrue, &vec![]).unwrap(),
            //         // 001
            //         make(&Instructions::OpJumpNotTruthy, &vec![7]).unwrap(),
            //         // 004
            //         make(&Instructions::OpConstant, &vec![0]).unwrap(),
            //         // 007
            //         make(&Instructions::OpPop, &vec![]).unwrap(),
            //         // 008
            //         make(&Instructions::OpConstant, &vec![1]).unwrap(),
            //         // 0011
            //         make(&Instructions::OpPop, &vec![]).unwrap(),
            //     ],
            // ),
        ];

        fn run_compiler_test(tests: &[Test]) {
            for test in tests {
                // Parsing
                let program = h_parse(&test.input);
                let mut compiler = Compiler::new();

                match compiler.compile_program(program.statments) {
                    Ok(..) => {
                        let bytecode = compiler.bytecode();
                        h_test_instruction(&test.expected_instructions, bytecode.instruction);
                        h_test_constant(&test.expected_constant, bytecode.constant);
                    }
                    Err(..) => {
                        panic!("Compile error: ");
                    }
                }
            }
        }
        run_compiler_test(&tests)
    }

    fn h_test_instruction(expected: &[Instruction], actual: &Instruction) {
        let concatted = concat_instruction(expected);

        if concatted.0.len() != actual.0.len() {
            panic!(
                "Wrong instructions length.\nwant={}\ngot={}",
                concatted, actual,
            )
        }

        for (idx, ins) in concatted.0.iter().enumerate() {
            if actual.0[idx] != *ins {
                panic!(
                    "Wrong instruction at {}.\nwant={}\ngot={}",
                    idx, actual, concatted
                )
            }
        }
    }

    fn h_parse(input: &str) -> ast::Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }
    fn concat_instruction(instruction: &[Instruction]) -> Instruction {
        instruction
            .into_iter()
            .map(|i| i.0.clone())
            .flatten()
            .collect()
    }

    fn h_test_constant(expected: &Vec<String>, actual: &Vec<ObjectType>) {
        if expected.len() != actual.len() {
            panic!(
                "Wrong number of constants. got={}, want={}",
                actual.len(),
                expected.len()
            )
        }

        for (idx, constant) in expected.into_iter().enumerate() {
            if let Ok(res) = constant.parse::<isize>() {
                h_test_integer_object(res, actual[idx])
            }
        }
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
