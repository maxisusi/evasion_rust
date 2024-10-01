use core::panic;
use std::{fmt::Display, ops::Deref, usize};

mod bytecode_test;

#[derive(Clone, Debug)]
pub struct Instruction(pub Vec<u8>);

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut index = 0;

        while index < self.0.len() {
            let definition = Definition::lookup(&self.0[index].try_into().unwrap());

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

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    OpConstant = 0,
    OpAdd,
    OpPop,
    OpSub,
    OpMul,
    OpDiv,
    OpTrue,
    OpFalse,
    OpEqual,
    OpNotEqual,
    OpGreaterThan,
    OpMinus,
    OpBang,
    OpJumpNotTruthy,
    OpJump,
    OpNull,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::OpConstant => write!(f, "OpConstant"),
            OpCode::OpAdd => write!(f, "OpAdd"),
            OpCode::OpPop => write!(f, "OpPop"),
            OpCode::OpSub => write!(f, "OpSub"),
            OpCode::OpMul => write!(f, "OpMul"),
            OpCode::OpDiv => write!(f, "OpDiv"),
            OpCode::OpTrue => write!(f, "OpTrue"),
            OpCode::OpFalse => write!(f, "OpFalse"),
            OpCode::OpEqual => write!(f, "OpEqual"),
            OpCode::OpNotEqual => write!(f, "OpNotEqual"),
            OpCode::OpGreaterThan => write!(f, "OpGreaterThan"),
            OpCode::OpMinus => write!(f, "OpMinus"),
            OpCode::OpBang => write!(f, "OpBang"),
            OpCode::OpJumpNotTruthy => write!(f, "OpJumpNotTruthy"),
            OpCode::OpJump => write!(f, "OpJump"),
            OpCode::OpNull => write!(f, "OpNull"),
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> OpCode {
        unsafe { std::mem::transmute::<u8, OpCode>(value) }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
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
    pub fn lookup(opcode: &OpCode) -> Option<Self> {
        let opcode = opcode.clone().into();
        match opcode {
            OpCode::OpConstant => Some(Definition::new(OpCode::OpConstant.to_string(), [2])),
            OpCode::OpAdd => Some(Definition::new(OpCode::OpAdd.to_string(), [])),
            OpCode::OpPop => Some(Definition::new(OpCode::OpPop.to_string(), [])),
            OpCode::OpSub => Some(Definition::new(OpCode::OpSub.to_string(), [])),
            OpCode::OpMul => Some(Definition::new(OpCode::OpMul.to_string(), [])),
            OpCode::OpDiv => Some(Definition::new(OpCode::OpDiv.to_string(), [])),
            OpCode::OpTrue => Some(Definition::new(OpCode::OpTrue.to_string(), [])),
            OpCode::OpFalse => Some(Definition::new(OpCode::OpFalse.to_string(), [])),
            OpCode::OpEqual => Some(Definition::new(OpCode::OpEqual.to_string(), [])),
            OpCode::OpGreaterThan => Some(Definition::new(OpCode::OpGreaterThan.to_string(), [])),
            OpCode::OpNotEqual => Some(Definition::new(OpCode::OpNotEqual.to_string(), [])),
            OpCode::OpMinus => Some(Definition::new(OpCode::OpMinus.to_string(), [])),
            OpCode::OpBang => Some(Definition::new(OpCode::OpBang.to_string(), [])),
            OpCode::OpJumpNotTruthy => {
                Some(Definition::new(OpCode::OpJumpNotTruthy.to_string(), [2]))
            }
            OpCode::OpJump => Some(Definition::new(OpCode::OpJump.to_string(), [2])),
            OpCode::OpNull => Some(Definition::new(OpCode::OpNull.to_string(), [])),
        }
    }
}

pub fn make(opcode: &OpCode, operands: &Vec<usize>) -> Option<Instruction> {
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
            let witdh = definition
                .operands_width
                .get(idx)
                .expect(format!("Couldn't find width for instruction: {}", opcode).as_str());

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
/** Read two bytes from the given instruction */
pub fn read_unit16(instruction: &[u8]) -> u16 {
    u16::from_be_bytes([instruction[0], instruction[1]])
}
