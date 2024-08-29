mod tests {

    use core::panic;
    use std::usize;

    use crate::bytecode::{make, Instructions};

    #[test]
    fn test_make() {
        #[derive(PartialEq)]
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
                if instruction.len() != test.expected.len() {
                    panic!(
                        "Expectect instruction length of {}, got={}",
                        test.expected.len(),
                        instruction.len()
                    )
                }

                for (idx, bit) in test.expected.iter().enumerate() {
                    if instruction[idx] != test.expected[idx] {
                        panic!(
                        "Wrong token at positon={} while parsing the instruction, expected={}, got={}",idx,
                        test.expected[idx], instruction[idx]
                    )
                    }
                }
            } else {
                panic!("Couldn't find an instruction for opcode={}", test.opcode)
            }
        }
    }
}
