use core::panic;
use std::usize;

use crate::{
    bytecode::{self, Instruction, Instructions},
    compiler,
    object::{self, ObjectType},
};

mod vm_test;

const STACK_SIZE: usize = 2048;
pub struct VirtualMachine<'a> {
    constants: &'a Vec<object::ObjectType>,
    instructions: &'a Instruction,
    stack: [ObjectType; STACK_SIZE],
    sp: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(bytecode_object: compiler::Bytecode<'a>) -> Self {
        Self {
            instructions: bytecode_object.instruction,
            constants: bytecode_object.constant,
            sp: 0,
            stack: [ObjectType::default(); STACK_SIZE],
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        let mut ip = 0; // Instruction pointer
        while ip < self.instructions.0.len() {
            let op = Instructions::from(self.instructions.0[ip]);

            match op {
                Instructions::OpConstant => {
                    let index_from_object_pool =
                        bytecode::read_unit16(&self.instructions.0[(ip + 1)..]);
                    let constant = self.constants[index_from_object_pool as usize];
                    ip += 2; // Increment by two because we read the contant

                    if let Err(err) = self.push(constant) {
                        return Err("An error occured while pushing to the stack: {err}");
                    }
                }
                Instructions::OpAdd => {
                    let right = self.pop();
                    let left = self.pop();
                    let result = match (right, left) {
                        (ObjectType::Integer(right), ObjectType::Integer(left)) => {
                            let result = right + left;
                            ObjectType::Integer(result)
                        }
                        _ => panic!("Add only accepts Integer Object"),
                    };

                    self.push(result);
                }
            }

            ip += 1; // Increment Instruction Pointer in order to loop at the next instruction
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

    fn pop(&mut self) -> ObjectType {
        let obj = self.stack[self.sp - 1];
        self.sp -= 1;
        obj
    }

    pub fn stack_top(&self) -> Option<object::ObjectType> {
        if self.sp == 0 {
            return None;
        }
        Some(self.stack[self.sp - 1])
    }
}
