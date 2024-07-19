use evasion_lib::{
    lexer::Lexer,
    token::{Token, TokenType},
};

use std::io::{stdin, stdout, Write};

fn main() {
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
            if tok.token_type == TokenType::EOF {
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
