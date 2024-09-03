use core::panic;
use std::usize;

use crate::{
    bytecode::{self, Instruction, Instructions},
    compiler,
    object::{self, ObjectType},
};

mod vm_test;

pub struct VirtualMachine<'a> {
    constants: &'a Vec<object::ObjectType>,
    instructions: &'a Instruction,
    stack: Vec<object::ObjectType>,
    sp: usize,
}

const STACK_SIZE: usize = 2048;

impl<'a> VirtualMachine<'a> {
    fn new(bytecode_object: compiler::Bytecode<'a>) -> Self {
        Self {
            instructions: bytecode_object.instruction,
            constants: bytecode_object.constant,
            sp: 0,
            stack: vec![ObjectType::default(); STACK_SIZE],
        }
    }

    fn run(&mut self) -> Result<(), &'static str> {
        let mut ip = 0;

        while ip < self.instructions.0.len() {
            let op = Instructions::from(self.instructions.0[ip]);

            match op {
                Instructions::OpConstant => {
                    let constant_index = bytecode::read_unit16(&self.instructions.0[(ip + 1)..]);
                    let constant = self.constants[constant_index as usize];
                    ip += 2;

                    if let Err(err) = self.push(constant) {
                        return Err("An error occured: {err}");
                    }
                    ip += 1;
                }
            }
        }

        Ok(())
    }

    fn push(&mut self, object: ObjectType) -> Result<(), &'static str> {
        if self.sp > STACK_SIZE {
            return Err("Stack overflow");
        }
        self.stack[self.sp] = object;
        self.sp += 1;
        Ok(())
    }

    fn stack_top(&self) -> Option<object::ObjectType> {
        if self.sp == 0 {
            return None;
        }
        Some(self.stack[self.sp - 1])
    }
}
