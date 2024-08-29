use std::{fmt::Display, ops::Deref, usize};

mod bytecode_test;

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Instructions {
    OpConstant,
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::OpConstant => write!(f, "OpConstant"),
        }
    }
}

impl From<u8> for Instructions {
    fn from(value: u8) -> Self {
        match value {
            0 => Instructions::OpConstant,
            _ => todo!(),
        }
    }
}

impl Into<u8> for Instructions {
    fn into(self) -> u8 {
        match self {
            Instructions::OpConstant => 0,
        }
    }
}

struct Definition {
    name: String,
    operands_width: Vec<u8>,
}

impl Definition {
    fn new<T, const U: usize>(name: T, operands_width: [u8; U]) -> Self
    where
        T: Into<String>,
    {
        Definition {
            name: name.into(),
            operands_width: operands_width.to_vec(),
        }
    }
}

impl Definition {
    fn lookup(opcode: &Instructions) -> Option<Self> {
        let opcode = opcode.clone().into();
        match opcode {
            0 => Some(Definition::new("OpCode", [2])),
            _ => None,
        }
    }
}

pub fn make<const T: usize>(opcode: &Instructions, operands: [u16; T]) -> Option<Vec<u8>> {
    let defintion = Definition::lookup(opcode);

    if let Some(definition) = defintion {
        let mut instruction_len = 1; // 1 for the opcode

        for op_width in definition.operands_width.iter().enumerate() {
            instruction_len += 1
        }

        let mut instruction = Vec::with_capacity(instruction_len);
        instruction.push(opcode.clone().into());

        let mut offset = 1;
        for (idx, operand) in operands.iter().enumerate() {
            let witdh = definition.operands_width[idx];

            match witdh {
                2 => {
                    let op = operand.to_be_bytes();
                    for byte in op.into_iter() {
                        instruction.push(byte);
                    }
                }
                _ => todo!(),
            }
            offset += witdh;
        }
        Some(instruction)
    } else {
        None
    }
}
