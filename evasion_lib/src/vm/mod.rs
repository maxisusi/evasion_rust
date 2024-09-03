use crate::{bytecode, compiler, object};

mod vm_test;

pub struct VirtualMachine {}

impl VirtualMachine {
    fn new(bytecode_object: compiler::Bytecode) -> Self {
        todo!()
    }

    fn run(&self) -> Result<(), &'static str> {
        todo!()
    }

    fn stack_top(&self) -> object::ObjectType {
        todo!()
    }
}
