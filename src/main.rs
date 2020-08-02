#![feature(test)]
#![feature(extern_types)]

use std::{env, fs::File, io::prelude::*};

mod bytecode;
mod compiler;
mod lexer;
mod runtime_val;
mod table;
mod token;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.len() {
        1 => "test.lox",
        2 => &args[1],
        _ => panic!("too many arguments"),
    };

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let text = contents.as_str();

    let mut compiler = compiler::Compiler::new(text);

    match compiler.compile() {
        Ok(()) => compiler.dump_bytecode(),
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    }

    let mut vm = vm::Vm::new(compiler.bytecode);

    match vm.execute() {
        Ok(()) => return,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    }
}
