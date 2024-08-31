mod tests {

    use core::panic;
    use std::usize;

    use crate::bytecode::{make, Instruction, Instructions};

    #[test]
    fn test_make() {
        struct Test<const T: usize, const U: usize> {
            opcode: Instructions,
            operand: [u16; T],
            expected: [u8; U],
        }
        impl<const T: usize, const U: usize> Test<T, U> {
            fn new(opcode: Instructions, operand: [u16; T], expected: [u8; U]) -> Self {
                Test {
                    opcode,
                    operand,
                    expected,
                }
            }
        }
        let tests = vec![Test::new(
            Instructions::OpConstant,
            [65534],
            [Instructions::OpConstant.into(), 255, 254],
        )];
        for test in tests {
            let instruction = make(&test.opcode, test.operand);

            if let Some(instruction) = instruction {
                if instruction.0.len() != test.expected.len() {
                    panic!(
                        "Expectect instruction length of {}, got={}",
                        test.expected.len(),
                        instruction.0.len()
                    )
                }

                for (idx, bit) in test.expected.iter().enumerate() {
                    if instruction.0[idx] != test.expected[idx] {
                        panic!(
                        "Wrong token at positon={} while parsing the instruction, expected={}, got={}",idx,
                        test.expected[idx], instruction.0[idx]
                    )
                    }
                }
            } else {
                panic!("Couldn't find an instruction for opcode={}", test.opcode)
            }
        }
    }

    #[test]
    fn test_instruction_string() {
        let mut instructions = vec![
            make(&Instructions::OpConstant, [1]),
            make(&Instructions::OpConstant, [2]),
            make(&Instructions::OpConstant, [3]),
        ];

        let expected = "0000 OpConstant 1\n0003 OpConstant 2\n0006 OpConstant 65535";

        let instr = Instruction(
            instructions
                .into_iter()
                .map(|f| f.unwrap().0)
                .flatten()
                .collect::<Vec<u8>>(),
        );

        if instr.to_string() != expected {
            panic!(
                "Instructions wrongly formatted.\nwant={}, got={}",
                expected, instr
            )
        }
    }
}
