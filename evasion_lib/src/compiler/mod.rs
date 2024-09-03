use std::usize;

use crate::{
    ast,
    bytecode::{self, Instruction},
    object,
};
mod compile_test;

pub struct Compiler {
    instruction: bytecode::Instruction,
    constant: Vec<object::ObjectType>,
}

pub struct Bytecode<'a> {
    instruction: &'a bytecode::Instruction,
    constant: &'a Vec<object::ObjectType>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instruction: Instruction(Vec::new()),
            constant: Vec::new(),
        }
    }

    pub fn compile_program(&mut self, nodes: Vec<ast::Nodes>) -> Result<(), ()> {
        for node in nodes {
            if let None = self.compile_node(node) {
                // Ignoring the null case for now
                // return Err(());
            }
        }
        Ok(())
    }

    fn compile_node(&mut self, node: ast::Nodes) -> Option<()> {
        match node {
            ast::Nodes::Expression(e) => match e {
                crate::ast::Expressions::Infix { left, right, .. } => {
                    let left = self.compile_node(ast::Nodes::from(*left));
                    let right = self.compile_node(ast::Nodes::from(*right));

                    Some(())
                }
                crate::ast::Expressions::IntegerLiteral { token, value } => {
                    let integer_contant = object::ObjectType::Integer(value);
                    let idx_in_constant_pool = &[self.add_constant(integer_contant)];
                    self.emit(bytecode::Instructions::OpConstant, idx_in_constant_pool);
                    Some(())
                }
                _ => todo!(),
            },
            ast::Nodes::Statement(s) => match s {
                _ => todo!(),
            },
        }
    }

    fn add_constant(&mut self, constant: object::ObjectType) -> usize {
        self.constant.push(constant);
        self.constant.len() - 1 // Returns the index from the constant pool
    }

    fn emit<const T: usize>(
        &mut self,
        opcode: bytecode::Instructions,
        operhands: &[usize; T],
    ) -> usize {
        let instruction = bytecode::make(&opcode, operhands).unwrap();
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
