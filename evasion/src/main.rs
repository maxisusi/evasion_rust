use evasion_lib::{compiler, lexer, parser, vm};

use std::io::{stdin, stdout, Write};

fn main() {
    run();
}

fn run() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut user_input = String::new();

        stdin().read_line(&mut user_input).unwrap();

        let lexer = lexer::Lexer::new(&user_input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let mut compiler = compiler::Compiler::new();
        let bytecode = compiler
            .compile_program(program.statments)
            .unwrap()
            .bytecode();

        let mut vm = vm::VirtualMachine::new(bytecode);

        if let Err(err) = vm.run() {
            panic!("An error occured: {err}")
        }

        let stack_top = vm.stack_top().unwrap();
        println!("{stack_top}")
    }
}
