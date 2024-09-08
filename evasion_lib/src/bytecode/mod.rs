use core::panic;
use std::{fmt::Display, ops::Deref, usize};

mod bytecode_test;

#[derive(Clone, Debug)]
pub struct Instruction(pub Vec<u8>);

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut index = 0;

        while index < self.0.len() {
            let definition = Definition::lookup(&self.0[index].into());

            if let Some(def) = &definition {
                let operhand = &self.0[index + 1..];

                let (read_op, length_op) = read_operhands(def, operhand);

                write!(f, "{:04} ", index); // Print index

                let count_op = def.operands_width.len();
                if operhand.len() != operhand.len() {
                    panic!("Error, operan len doesn't match defined {}", count_op);
                }

                match count_op {
                    1 => write!(f, "{} {}\n", def.name, read_op[0]),
                    0 => write!(f, "{}\n", def.name),
                    _ => panic!("Error, unhandled operhand count {}", def.name),
                };

                index += 1 + length_op
            } else {
                panic!("Couldn't find definition")
            }
        }
        Ok(())
    }
}

impl FromIterator<u8> for Instruction {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut item = Vec::new();

        for bit in iter {
            item.push(bit)
        }

        Instruction(item)
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Instructions {
    OpConstant,
    OpAdd,
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::OpConstant => write!(f, "OpConstant"),
            Instructions::OpAdd => write!(f, "OpAdd"),
        }
    }
}

impl From<u8> for Instructions {
    fn from(value: u8) -> Self {
        match value {
            0 => Instructions::OpConstant,
            1 => Instructions::OpAdd,
            _ => todo!(),
        }
    }
}

impl Into<u8> for Instructions {
    fn into(self) -> u8 {
        match self {
            Instructions::OpConstant => 0,
            Instructions::OpAdd => 1,
        }
    }
}

pub struct Definition {
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
    pub fn lookup(opcode: &Instructions) -> Option<Self> {
        let opcode = opcode.clone().into();
        match opcode {
            Instructions::OpConstant => {
                Some(Definition::new(Instructions::OpConstant.to_string(), [2]))
            }
            Instructions::OpAdd => Some(Definition::new(Instructions::OpAdd.to_string(), [])),
        }
    }
}

pub fn make(opcode: &Instructions, operands: &Vec<usize>) -> Option<Instruction> {
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
                    // Get the last two bytes
                    let op = &operand.to_be_bytes()[std::mem::size_of::<usize>() - 2..];
                    for byte in op.into_iter() {
                        instruction.push(*byte);
                    }
                }
                _ => todo!(),
            }
            offset += witdh;
        }
        Some(Instruction(instruction))
    } else {
        None
    }
}

pub fn read_operhands(definition: &Definition, instruction: &[u8]) -> (Vec<usize>, usize) {
    let mut operhand: Vec<usize> = Vec::with_capacity(definition.operands_width.len());
    let mut offset = 0;

    for (idx, width) in definition.operands_width.iter().enumerate() {
        match width {
            2 => {
                let instruction = &instruction[offset..];
                let op = read_unit16(instruction) as usize;
                operhand.push(op);
            }
            _ => todo!(),
        }

        offset += (*width as usize);
    }

    (operhand, offset)
}

pub fn read_unit16(instruction: &[u8]) -> u16 {
    u16::from_be_bytes([instruction[0], instruction[1]])
}
