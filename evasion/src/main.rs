use evasion_lib::{
    lexer::Lexer,
    parser::Parser,
    token::{Token, TokenTypes},
};

use std::io::{stdin, stdout, Write};

fn main() {
    // print_lexer();
    print_parser();
}

fn print_parser() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut user_input = String::new();

        stdin().read_line(&mut user_input).unwrap();

        let lexer = Lexer::new(&user_input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        if let Some(program) = program {
            println!("{}", program);
        }
    }
}

fn print_lexer() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        let mut tokens: Vec<Token> = Vec::new();

        stdin().read_line(&mut user_input).unwrap();
        print!("{user_input}");

        let mut lexer = Lexer::new(&user_input);

        loop {
            let tok = lexer.next_token();
            if tok.token_type == TokenTypes::EOF {
                break;
            } else {
                tokens.push(tok);
            }
        }

        for tok in tokens.iter() {
            println!("{:?}", tok);
        }
    }
}
