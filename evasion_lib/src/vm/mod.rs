use core::panic;
use std::{isize, usize};

use crate::{
    bytecode::{self, Instruction, OpCode},
    compiler,
    object::{self, ObjectType},
};

mod vm_test;

const STACK_SIZE: usize = 2048;
pub const GLOBAL_SIZE: usize = 6625;

pub struct VirtualMachine<'a> {
    constants: &'a Vec<object::ObjectType>,
    instructions: &'a Instruction,
    stack: [ObjectType; STACK_SIZE],
    global: &'a mut [ObjectType; GLOBAL_SIZE],
    sp: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(
        bytecode_object: compiler::Bytecode<'a>,
        global: &'a mut [ObjectType; GLOBAL_SIZE],
    ) -> Self {
        Self {
            instructions: bytecode_object.instruction,
            constants: bytecode_object.constant,
            sp: 0,
            stack: [ObjectType::default(); STACK_SIZE],
            global,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip = 0; // Instruction pointer
        while ip < self.instructions.0.len() {
            let op = OpCode::from(self.instructions.0[ip]);

            match op {
                OpCode::OpConstant => {
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
                OpCode::OpAdd | OpCode::OpDiv | OpCode::OpMul | OpCode::OpSub => {
                    let result = self.execute_binary_operation(op);
                    self.push(result);
                }
                OpCode::OpPop => {
                    self.pop();
                }
                OpCode::OpTrue => {
                    self.push(ObjectType::Boolean(true));
                }
                OpCode::OpFalse => {
                    self.push(ObjectType::Boolean(false));
                }
                OpCode::OpEqual | OpCode::OpGreaterThan | OpCode::OpNotEqual => {
                    let result = self.execute_binary_operation(op);
                    self.push(result);
                }
                OpCode::OpBang => {
                    let operhand = self.pop();

                    match operhand {
                        ObjectType::Boolean(value) => {
                            if value == true {
                                self.push(ObjectType::Boolean(false))
                            } else {
                                self.push(ObjectType::Boolean(true))
                            }
                        }
                        ObjectType::Null => self.push(ObjectType::Boolean(false)),
                        ObjectType::Integer(..) => self.push(ObjectType::Boolean(false)),
                        _ => self.push(ObjectType::Boolean(false)),
                    };
                }
                OpCode::OpMinus => {
                    let operhand = self.pop();

                    match operhand {
                        ObjectType::Integer(value) => self.push(ObjectType::Integer(-value)),
                        _ => panic!("Unhandled operhand"),
                    };
                }
                OpCode::OpJump => {
                    let position = bytecode::read_unit16(&self.instructions.0[ip + 1..]);
                    ip = (position as usize) - 1;
                }
                OpCode::OpJumpNotTruthy => {
                    let position = bytecode::read_unit16(&self.instructions.0[ip + 1..]);
                    ip += 2;

                    let condition = self.pop();

                    if !self.is_truthy(condition) {
                        ip = (position as usize) - 1;
                    }
                }
                OpCode::OpNull => self.push(ObjectType::Null)?,
                OpCode::OpSetGlobal => {
                    let position = bytecode::read_unit16(&self.instructions.0[ip + 1..]);
                    ip += 2;

                    self.global[position as usize] = self.pop();
                }
                OpCode::OpGetGlobal => {
                    let position = bytecode::read_unit16(&self.instructions.0[ip + 1..]);
                    ip += 2;
                    let get_object = self.global.get(position as usize);
                    if let Some(obj) = get_object {
                        self.push(*obj);
                    }
                }
            }

            ip += 1; // Increment Instruction Pointer in order to loop at the next instruction
        }

        Ok(())
    }

    fn is_truthy(&self, obj: ObjectType) -> bool {
        match obj {
            ObjectType::Boolean(value) => value,
            ObjectType::Null => false,
            _ => true,
        }
    }

    fn execute_binary_operation(&mut self, operation: OpCode) -> ObjectType {
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

    fn execute_integer_operation<T>(&mut self, op: OpCode, left: T, right: T) -> ObjectType
    where
        T: Into<isize>,
    {
        let left: isize = left.into();
        let right: isize = right.into();
        match op {
            OpCode::OpAdd => ObjectType::Integer((left + right)),
            OpCode::OpDiv => ObjectType::Integer((left / right)),
            OpCode::OpMul => ObjectType::Integer((left * right)),
            OpCode::OpSub => ObjectType::Integer((left - right)),
            OpCode::OpEqual => ObjectType::Boolean(left == right),
            OpCode::OpNotEqual => ObjectType::Boolean(left != right),
            OpCode::OpGreaterThan => ObjectType::Boolean(left > right),
            _ => panic!("Not supported instruction"),
        }
    }

    fn execute_boolean_operation(&mut self, op: OpCode, left: bool, right: bool) -> ObjectType {
        match op {
            OpCode::OpEqual => ObjectType::Boolean(left == right),
            OpCode::OpNotEqual => ObjectType::Boolean(left != right),
            OpCode::OpGreaterThan => ObjectType::Boolean(left > right),
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
