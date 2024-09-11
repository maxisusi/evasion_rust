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

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip = 0; // Instruction pointer
        while ip < self.instructions.0.len() {
            let op = Instructions::try_from(self.instructions.0[ip])?;

            match op {
                Instructions::OpConstant => {
                    let index_from_object_pool =
                        bytecode::read_unit16(&self.instructions.0[(ip + 1)..]);
                    let constant = self.constants[index_from_object_pool as usize];
                    ip += 2; // Increment by two because we read the contant

                    if let Err(err) = self.push(constant) {
                        return Err(
                            "An error occured while pushing to the stack: {err}".to_string()
                        );
                    }
                }
                Instructions::OpAdd
                | Instructions::OpDiv
                | Instructions::OpMul
                | Instructions::OpSub => {
                    let result = self.execute_binary_operation(op);
                    self.push(result);
                }
                Instructions::OpPop => {
                    self.pop();
                }
                Instructions::OpTrue => {
                    self.push(ObjectType::Boolean(true));
                }
                Instructions::OpFalse => {
                    self.push(ObjectType::Boolean(false));
                }
                Instructions::OpEqual | Instructions::OpGreaterThan | Instructions::OpNotEqual => {
                    let result = self.execute_binary_operation(op);
                    self.push(result);
                }
            }

            ip += 1; // Increment Instruction Pointer in order to loop at the next instruction
        }

        Ok(())
    }

    fn execute_binary_operation(&mut self, operation: Instructions) -> ObjectType {
        let right_obj = self.pop();
        let left_obj = self.pop();

        match (left_obj, right_obj) {
            (ObjectType::Integer(left), ObjectType::Integer(right)) => {
                self.execute_integer_operation(operation, left, right)
            }

            (ObjectType::Boolean(left), ObjectType::Boolean(right)) => {
                self.execute_boolean_operation(operation, left, right)
            }
            _ => panic!("Add only accepts Integer Object"),
        }
    }

    fn execute_integer_operation(
        &mut self,
        op: Instructions,
        left: usize,
        right: usize,
    ) -> ObjectType {
        match op {
            Instructions::OpAdd => ObjectType::Integer((left + right)),
            Instructions::OpDiv => ObjectType::Integer((left / right)),
            Instructions::OpMul => ObjectType::Integer((left * right)),
            Instructions::OpSub => ObjectType::Integer((left - right)),
            Instructions::OpEqual => ObjectType::Boolean(left == right),
            Instructions::OpNotEqual => ObjectType::Boolean(left != right),
            Instructions::OpGreaterThan => ObjectType::Boolean(left > right),
            _ => panic!("Not supported instruction"),
        }
    }

    fn execute_boolean_operation(
        &mut self,
        op: Instructions,
        left: bool,
        right: bool,
    ) -> ObjectType {
        match op {
            Instructions::OpEqual => ObjectType::Boolean(left == right),
            Instructions::OpNotEqual => ObjectType::Boolean(left != right),
            Instructions::OpGreaterThan => ObjectType::Boolean(left > right),
            _ => panic!("Not supported instruction"),
        }
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

    pub fn last_popped_stack_elem(&self) -> ObjectType {
        self.stack[self.sp]
    }
}
