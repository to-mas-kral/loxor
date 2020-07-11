#![feature(test)]

use std::fs::File;
use std::io::prelude::*;

mod bytecode;

mod lexer;
mod token;

use token::TokenType;

fn main() {
    /* let mut chunk = Chunk::new();
    chunk.code.push(OpCode::Return as u8);
    chunk.code.push(OpCode::Constant as u8);
    chunk.code.push(0);
    chunk.constants.push(1.2);
    chunk.lines.push(56);
    chunk.lines.push(56);

    chunk.disassemble(); */

    let mut file = File::open("test.lox").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    debug_assert!(contents.is_ascii());

    let mut text = contents.as_str();

    let mut lexer = lexer::Lexer::new();
    //let mut tokens = Vec::new();

    while let Some(t) = lexer.next_token(text) {
        println!("{}", t.lexeme);
        println!("{:?}", t);

        let typ = t.typ;

        text = &text[t.lexeme.len()..];
        //tokens.push(t);
    }

    /*  for t in tokens {
        println!("{:?}", t);
    } */
}
