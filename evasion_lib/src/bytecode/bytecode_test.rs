mod tests {

    use core::panic;
    use std::usize;

    use lazy_static::initialize;

    use crate::bytecode::{self, make, Definition};

    #[test]
    fn test_make() {
        struct Test {
            opcode: bytecode::OpCode,
            operand: Vec<usize>,
            expected: Vec<u8>,
        }
        impl Test {
            fn new(opcode: bytecode::OpCode, operand: Vec<usize>, expected: Vec<u8>) -> Self {
                Test {
                    opcode,
                    operand,
                    expected,
                }
            }
        }
        let tests = vec![
            Test::new(
                bytecode::OpCode::OpConstant,
                vec![65534],
                vec![bytecode::OpCode::OpConstant.into(), 255.into(), 254.into()],
            ),
            Test::new(
                bytecode::OpCode::OpAdd,
                vec![],
                vec![bytecode::OpCode::OpAdd.into()],
            ),
            Test::new(
                bytecode::OpCode::OpPop,
                vec![],
                vec![bytecode::OpCode::OpPop.into()],
            ),
        ];
        for test in tests {
            let instruction = bytecode::make(&test.opcode, &test.operand);

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
            bytecode::make(&bytecode::OpCode::OpAdd, &vec![]),
            bytecode::make(&bytecode::OpCode::OpConstant, &vec![2]),
            bytecode::make(&bytecode::OpCode::OpConstant, &vec![65535]),
            bytecode::make(&bytecode::OpCode::OpPop, &vec![]),
        ];

        let expected = "0000 OpAdd\n0001 OpConstant 2\n0004 OpConstant 65535\n0007 OpPop\n";

        let instr = bytecode::Instruction(
            instructions
                .into_iter()
                .map(|f| f.unwrap().0)
                .flatten()
                .collect::<Vec<u8>>(),
        );

        if instr.to_string() != expected {
            panic!(
                "Instructions wrongly formatted.\nwant={}\ngot={}",
                expected, instr
            )
        }
    }

    #[test]
    fn test_read_operhands() {
        struct Test<const T: usize> {
            opcode: bytecode::OpCode,
            operand: [usize; T],
            bytes_read: usize,
        }
        impl<const T: usize> Test<T> {
            fn new(opcode: bytecode::OpCode, operand: [usize; T], bytes_read: usize) -> Self {
                Test {
                    opcode,
                    operand,
                    bytes_read,
                }
            }
        }
        let tests = vec![Test::new(bytecode::OpCode::OpConstant, [65534], 2)];

        for test in tests {
            let instruction = make(&test.opcode, &test.operand.to_vec()).unwrap();
            let definition = bytecode::Definition::lookup(&test.opcode);

            if let Some(def) = definition {
                // We remove the operator from the instruction in order to extract the operhands
                let (read, length) = bytecode::read_operhands(&def, &instruction.0[1..]);

                if length != test.bytes_read {
                    panic!(
                        "wrong length of bytes read\nwant={}, got={}",
                        test.bytes_read, length
                    )
                }
                for (idx, op) in test.operand.iter().enumerate() {
                    if read[idx] != test.operand[idx] {
                        panic!(
                            "wrong operand read at index={}\nwant={}, got={}",
                            idx, test.operand[idx], read[idx]
                        )
                    }
                }
            } else {
                panic!("Definition not found")
            }
        }
    }
}
