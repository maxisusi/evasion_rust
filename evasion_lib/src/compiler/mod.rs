use core::panic;
use std::{isize, usize};

use crate::{
    ast::{self, Expressions, Statements},
    bytecode::{self, Instruction, OpCode},
    object,
};
mod compile_test;

pub struct Compiler {
    instruction: bytecode::Instruction,
    constant: Vec<object::ObjectType>,
    last_instruction: EmitterInstruction,
    previous_instruction: EmitterInstruction,
}
#[derive(Clone)]
struct EmitterInstruction {
    instruction: bytecode::Instruction,
    position: usize,
}

impl EmitterInstruction {
    fn new() -> Self {
        Self {
            instruction: Instruction(Vec::new()),
            position: 0,
        }
    }
}

pub struct Bytecode<'a> {
    pub instruction: &'a bytecode::Instruction,
    pub constant: &'a Vec<object::ObjectType>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instruction: Instruction(Vec::new()),
            constant: Vec::new(),
            last_instruction: EmitterInstruction::new(),
            previous_instruction: EmitterInstruction::new(),
        }
    }

    pub fn compile_program(&mut self, nodes: Vec<ast::Nodes>) -> Result<&Self, ()> {
        for node in nodes {
            if let None = self.compile_node(node) {
                // Ignoring the null case for now
                // return Err(());
            }
        }
        Ok(self)
    }

    fn compile_node(&mut self, node: ast::Nodes) -> Option<()> {
        match node {
            ast::Nodes::Expression(e) => {
                if let Some(expr) = self.compile_expression(e) {
                    self.emit(OpCode::OpPop, vec![]);
                    return Some(());
                }

                None
            }
            ast::Nodes::Statement(s) => {
                if let Some(()) = self.compile_statement(s) {
                    return Some(());
                }
                None
            }
        }
    }

    fn compile_statement(&mut self, statement: Statements) -> Option<()> {
        match statement {
            ast::Statements::BlockStatements { token, statements } => {
                for consequence in statements {
                    self.compile_node(consequence);
                    if self.is_last_instruction_pop() {
                        self.instruction.0 =
                            self.instruction.0[..self.last_instruction.position].to_vec();
                    }
                }

                Some(())
            }
            _ => todo!(),
        }
    }
    fn compile_expression(&mut self, expression: Expressions) -> Option<()> {
        match expression {
            crate::ast::Expressions::Infix {
                left,
                right,
                operator,
                ..
            } => {
                if operator.as_str() == "<" {
                    let right = self.compile_expression(*right);
                    let left = self.compile_expression(*left);
                    self.emit(OpCode::OpGreaterThan, vec![]);
                } else {
                    let left = self.compile_expression(*left);
                    let right = self.compile_expression(*right);

                    match operator.as_str() {
                        "+" => self.emit(OpCode::OpAdd, vec![]),
                        "-" => self.emit(OpCode::OpSub, vec![]),
                        "*" => self.emit(OpCode::OpMul, vec![]),
                        "/" => self.emit(OpCode::OpDiv, vec![]),
                        "==" => self.emit(OpCode::OpEqual, vec![]),
                        "!=" => self.emit(OpCode::OpNotEqual, vec![]),
                        "<" | ">" => self.emit(OpCode::OpGreaterThan, vec![]),
                        _ => return None,
                    };
                }
                Some(())
            }

            crate::ast::Expressions::IfExpression {
                token,
                condition,
                consequence,
                alternative,
            } => {
                let condition = self.compile_expression(*condition);

                // Emit OpJumpNotTruthy with wrong value
                // We will patch it after compiling if expression
                let jump_not_truthy_pos = self.emit(OpCode::OpJumpNotTruthy, vec![9999]);
                self.compile_statement(*consequence);

                let jump_pos = self.emit(OpCode::OpJump, vec![999]);
                let after_consequence_offset = self.instruction.0.len();
                self.change_operhand(jump_not_truthy_pos, after_consequence_offset);

                if let None = alternative {
                    self.emit(OpCode::OpNull, vec![]);
                } else {
                    self.compile_statement(*alternative.unwrap());
                }
                let after_alternative_offset = self.instruction.0.len();
                self.change_operhand(jump_pos, after_alternative_offset);

                Some(())
            }
            crate::ast::Expressions::Prefix {
                token,
                operator,
                right,
            } => {
                let right = self.compile_expression(*right);

                match operator.as_str() {
                    "!" => self.emit(bytecode::OpCode::OpBang, vec![]),
                    "-" => self.emit(bytecode::OpCode::OpMinus, vec![]),
                    _ => panic!("Unexpected operator founded for infix expression"),
                };

                Some(())
            }
            crate::ast::Expressions::IntegerLiteral { token, value } => {
                let integer_object = object::ObjectType::Integer(value as isize);
                let idx_in_constant_pool = &[self.add_constant(integer_object)];
                self.emit(bytecode::OpCode::OpConstant, idx_in_constant_pool.to_vec());
                Some(())
            }

            crate::ast::Expressions::Boolean { token, value } => {
                let boolean_object = object::ObjectType::Boolean(value);
                if value == true {
                    self.emit(bytecode::OpCode::OpTrue, vec![]);
                } else {
                    self.emit(bytecode::OpCode::OpFalse, vec![]);
                }
                Some(())
            }
            _ => None,
        }
    }

    fn add_constant(&mut self, constant: object::ObjectType) -> usize {
        self.constant.push(constant);
        self.constant.len() - 1 // Returns the index from the constant pool
    }

    fn emit(&mut self, opcode: bytecode::OpCode, op_index_from_obj_pool: Vec<usize>) -> usize {
        let instruction = bytecode::make(&opcode, &op_index_from_obj_pool).unwrap();
        let pos = self.add_instruction(instruction.clone());
        self.register_instruction(instruction, pos.clone());

        return pos;
    }

    fn remplace_instruction(&mut self, position: usize, instruction: bytecode::Instruction) {
        for (i, bit) in instruction.0.into_iter().enumerate() {
            self.instruction.0[position + i] = bit;
        }
    }

    fn change_operhand(&mut self, position: usize, operhand: usize) {
        let get_opcode = bytecode::OpCode::from(self.instruction.0.get(position).unwrap().clone());
        let new_instruction = bytecode::make(&get_opcode, &vec![operhand]);

        if let Some(instruction) = new_instruction {
            self.remplace_instruction(position, instruction)
        }
    }

    fn register_instruction(&mut self, instruction: bytecode::Instruction, position: usize) {
        let emitted_instruction = EmitterInstruction {
            instruction,
            position,
        };

        self.previous_instruction = self.last_instruction.clone();
        self.last_instruction = emitted_instruction;
    }

    fn add_instruction(&mut self, instruction: bytecode::Instruction) -> usize {
        let pos_new_instruction = self.instruction.0.len();

        for bit in instruction.0 {
            self.instruction.0.push(bit);
        }
        pos_new_instruction
    }
    fn is_last_instruction_pop(&self) -> bool {
        let opcode = self.last_instruction.instruction.0.get(0);
        if let Some(opcode) = opcode {
            return OpCode::from(opcode.clone()) == OpCode::OpPop;
        }
        return false;
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            constant: &self.constant,
            instruction: &self.instruction,
        }
    }
}
