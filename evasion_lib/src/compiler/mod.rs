use std::usize;

use crate::{
    ast::{self, Expressions},
    bytecode::{self, Instruction, Instructions},
    object,
};
mod compile_test;

pub struct Compiler {
    instruction: bytecode::Instruction,
    constant: Vec<object::ObjectType>,
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
                    self.emit(Instructions::OpPop, vec![]);
                }
                Some(())
            }
            ast::Nodes::Statement(s) => match s {
                _ => todo!(),
            },
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
                let left = self.compile_expression(*left);
                let right = self.compile_expression(*right);

                match operator.as_str() {
                    "+" => self.emit(Instructions::OpAdd, vec![]),
                    "-" => self.emit(Instructions::OpSub, vec![]),
                    "*" => self.emit(Instructions::OpMul, vec![]),
                    "/" => self.emit(Instructions::OpDiv, vec![]),
                    _ => panic!("Unknown operator: {}", operator),
                };
                Some(())
            }
            crate::ast::Expressions::IntegerLiteral { token, value } => {
                let integer_object = object::ObjectType::Integer(value);
                let idx_in_constant_pool = &[self.add_constant(integer_object)];
                self.emit(
                    bytecode::Instructions::OpConstant,
                    idx_in_constant_pool.to_vec(),
                );
                Some(())
            }
            _ => None,
        }
    }

    fn add_constant(&mut self, constant: object::ObjectType) -> usize {
        self.constant.push(constant);
        self.constant.len() - 1 // Returns the index from the constant pool
    }

    fn emit(
        &mut self,
        opcode: bytecode::Instructions,
        op_index_from_obj_pool: Vec<usize>,
    ) -> usize {
        let instruction = bytecode::make(&opcode, &op_index_from_obj_pool).unwrap();
        let pos = self.add_instruction(instruction);
        return pos;
    }

    fn add_instruction(&mut self, instruction: bytecode::Instruction) -> usize {
        let pos_new_instruction = instruction.0.len();

        for bit in instruction.0 {
            self.instruction.0.push(bit);
        }
        pos_new_instruction
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            constant: &self.constant,
            instruction: &self.instruction,
        }
    }
}
