use evasion_lib::{compiler, lexer, object::ObjectType, parser, symbol_table::SymbolTable, vm};

use std::io::{stdin, stdout, Write};

fn main() {
    run();
}

fn run() {
    let mut symbol_table = Box::new(SymbolTable::new());
    let mut global = [ObjectType::default(); vm::GLOBAL_SIZE];

    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut user_input = String::new();

        stdin().read_line(&mut user_input).unwrap();

        let lexer = lexer::Lexer::new(&user_input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let mut compiler = compiler::Compiler::new(&mut *symbol_table);
        let bytecode = compiler
            .compile_program(program.statments)
            .unwrap()
            .bytecode();

        let mut vm = vm::VirtualMachine::new(bytecode, &mut global);

        if let Err(err) = vm.run() {
            panic!("An error occured: {err}")
        }

        let stack_top = vm.last_popped_stack_elem();
        println!("{stack_top}")
    }
}
