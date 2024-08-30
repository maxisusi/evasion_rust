use crate::{
    ast::Nodes,
    bytecode::{Instruction, Instructions},
    object::ObjectType,
};
mod compile_test;

struct Compiler {
    instruction: Option<Instruction>,
    constant: Vec<ObjectType>,
}

struct Bytecode {
    instruction: Instruction,
    constant: Vec<ObjectType>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            instruction: None,
            constant: Vec::new(),
        }
    }

    fn compile(&mut self, nodes: Vec<Nodes>) -> Result<(), ()> {
        todo!()
    }

    fn bytecode(&self) -> Bytecode {
        Bytecode {
            constant: self.constant.clone(),
            instruction: self.instruction.clone().unwrap(),
        }
    }
}
