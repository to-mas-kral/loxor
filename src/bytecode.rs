use core::slice::Iter;
use std::iter::Enumerate;

use crate::vm::RuntimeValue;

// TODO: challenge - run-length line number encoding
// TODO: add support for more constants (32 bit index)

pub struct Chunk {
    pub code: Vec<Bytecode>,
    pub constants: Vec<RuntimeValue>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, val: RuntimeValue, line: usize) {
        let index = self.constants.len();
        if index < 255 {
            self.constants.push(val);
            self.code.push(opcodes::CONSTANT);
            self.code.push(index as u8);
            self.lines.push(line);
            self.lines.push(line);
        } else {
            panic!("Too many constants");
        }
    }

    pub fn add_opocode(&mut self, opcode: Bytecode, line: usize) {
        self.code.push(opcode);
        self.lines.push(line);
    }

    pub fn disassemble(&self) {
        let mut opcodes = self.code.iter().enumerate();

        println!("OFFSET     LINE     OPCODE      OTHER INFO");
        println!("==========================================");

        while let Some((offset, opcode)) = opcodes.next() {
            print!("0x{:4X}     {:4}     ", offset, self.lines[offset]);
            match *opcode {
                opcodes::RETURN => println!("RETURN"),
                opcodes::CONSTANT => self.constant(&mut opcodes),
                opcodes::NIL => println!("NIL"),
                opcodes::TRUE => println!("TRUE"),
                opcodes::FALSE => println!("FALSE"),
                opcodes::ADD => println!("ADD"),
                opcodes::SUBTRACT => println!("SUBTRACT"),
                opcodes::MULTIPLY => println!("MULTIPLY"),
                opcodes::DIVIDE => println!("DIVIDE"),
                opcodes::NEGATE => println!("NEGATE"),
                _ => unreachable!(),
            }
        }
    }

    fn constant(&self, code: &mut Enumerate<Iter<u8>>) {
        if let Some((_, index)) = code.next() {
            let val = self.constants[*index as usize];

            println!("CONSTANT    c[{}] = {}", index, val);
        } else {
            panic!("Constant is missing the index");
        }
    }
}

pub type Bytecode = u8;

pub mod opcodes {
    use super::Bytecode;

    pub const CONSTANT: Bytecode = 0;
    pub const NIL: Bytecode = 1;
    pub const TRUE: Bytecode = 2;
    pub const FALSE: Bytecode = 3;
    //pub const POP: Bytecode = 4;
    //pub const GET_LOCAL: Bytecode = 5;
    //pub const SET_LOCAL: Bytecode = 6;
    //pub const GET_GLOBAL: Bytecode = 7;
    //pub const DEFINE_GLOBAL: Bytecode = 8;
    //pub const SET_GLOBAL: Bytecode = 9;
    //pub const GET_UPVALUE: Bytecode = 10;
    //pub const SET_UPVALUE: Bytecode = 11;
    //pub const GET_PROPERTY: Bytecode = 12;
    //pub const SET_PROPERTY: Bytecode = 13;
    //pub const GET_SUPER: Bytecode = 14;
    //pub const EQUAL: Bytecode = 15;
    //pub const GREATER: Bytecode = 16;
    //pub const LESS: Bytecode = 17;
    pub const ADD: Bytecode = 18;
    pub const SUBTRACT: Bytecode = 19;
    pub const MULTIPLY: Bytecode = 20;
    pub const DIVIDE: Bytecode = 21;
    //pub const NOT: Bytecode = 22;
    pub const NEGATE: Bytecode = 23;
    //pub const PRINT: Bytecode = 24;
    //pub const JUMP: Bytecode = 25;
    //pub const JUMP_IF_FALSE: Bytecode = 26;
    //pub const LOOP: Bytecode = 27;
    //pub const CALL: Bytecode = 28;
    //pub const INVOKE: Bytecode = 29;
    //pub const SUPER_INVOKE: Bytecode = 30;
    //pub const CLOSURE: Bytecode = 31;
    //pub const CLOSE_UPVALUE: Bytecode = 32;
    pub const RETURN: Bytecode = 33;
    //pub const CLASS: Bytecode = 34;
    //pub const INHERIT: Bytecode = 35;
    //pub const METHO: Bytecode = 36;
}
