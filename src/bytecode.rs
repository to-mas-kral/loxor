use core::slice::Iter;
use std::iter::Enumerate;

use crate::runtime_val::RuntimeValue;

pub struct Chunk {
    pub code: Vec<Bytecode>,
    pub constants: Vec<RuntimeValue>,

    pub lines: Vec<(usize, usize)>,
    line_index: usize,
    current_line: usize,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),

            lines: Vec::new(),
            current_line: 0,
            line_index: 0,
        }
    }

    pub fn add_constant(&mut self, val: RuntimeValue, line: usize) {
        let index = self.constants.len();
        self.constants.push(val);

        if index < 0xFF {
            self.code.push(opcodes::CONSTANT);
            self.code.push(index as u8);
            self.at_line(line, 2);
        } else if index < 0xFF_FFFF {
            self.code.push(opcodes::CONSTANT_LONG);
            let bytes = index.to_le_bytes();
            self.code.push(bytes[0]);
            self.code.push(bytes[1]);
            self.code.push(bytes[2]);
            self.at_line(line, 4);
        } else {
            // TODO: error handling
            panic!("Too many constants");
        }
    }

    pub fn add_opocode(&mut self, opcode: Bytecode, line: usize) {
        self.code.push(opcode);
        self.at_line(line, 1);
    }

    pub fn disassemble(&self) {
        let mut opcodes = self.code.iter().enumerate();

        println!("OFFSET     LINE     OPCODE      OTHER INFO");
        println!("==========================================");

        while let Some((offset, opcode)) = opcodes.next() {
            print!("0x{:4X}     {:4}     ", offset, self.get_line_at_ip(offset));
            match *opcode {
                opcodes::RETURN => println!("RETURN"),
                opcodes::CONSTANT => self.constant(&mut opcodes),
                opcodes::NIL => println!("NIL"),
                opcodes::TRUE => println!("TRUE"),
                opcodes::FALSE => println!("FALSE"),
                opcodes::POP => println!("POP"),
                opcodes::EQUAL => println!("EQUAL"),
                opcodes::GREATER => println!("GREATER"),
                opcodes::LESS => println!("LESS"),
                opcodes::ADD => println!("ADD"),
                opcodes::SUBTRACT => println!("SUBTRACT"),
                opcodes::MULTIPLY => println!("MULTIPLY"),
                opcodes::DIVIDE => println!("DIVIDE"),
                opcodes::NOT => println!("NOT"),
                opcodes::NEGATE => println!("NEGATE"),
                opcodes::PRINT => println!("PRINT"),

                opcodes::CONSTANT_LONG => self.constant_long(&mut opcodes),
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

    fn constant_long(&self, code: &mut Enumerate<Iter<u8>>) {
        let mut bytes = [0; 4];

        for i in 0..3 {
            if let Some((_, index_byte)) = code.next() {
                bytes[i] = *index_byte;
            } else {
                panic!("Constant long is missing the index");
            }
        }

        let index = u32::from_le_bytes(bytes) as usize;
        let val = self.constants[index];

        println!("CONSTANT_LONG    c[{}] = {}", index, val);
    }

    /*
    Lines are encoded using run-length encoding.
    Every tuple in Chunk.lines is a tuple of two entries.
    The first one is the line number, the second one is the amount of
    instructions ("Bytecodes") in that line.
    */
    fn at_line(&mut self, line: usize, bytes: usize) {
        if self.current_line == line {
            // FIXME: fix subtract with overflow
            self.lines[self.line_index - 1].1 += bytes;
        } else {
            self.current_line = line;
            self.line_index += 1;
            self.lines.push((line, bytes));
        }
    }

    pub fn get_line_at_ip(&self, mut ip: usize) -> usize {
        for l in &self.lines {
            let len = l.1;
            let res = ip.overflowing_sub(len);
            ip = res.0;
            if res.1 {
                return l.0;
            } else {
                continue;
            }
        }

        0
    }
}

pub type Bytecode = u8;

pub(crate) mod opcodes {
    use super::Bytecode;

    pub const CONSTANT: Bytecode = 0;
    pub const NIL: Bytecode = 1;
    pub const TRUE: Bytecode = 2;
    pub const FALSE: Bytecode = 3;
    pub const POP: Bytecode = 4;
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
    pub const EQUAL: Bytecode = 15;
    pub const GREATER: Bytecode = 16;
    pub const LESS: Bytecode = 17;
    pub const ADD: Bytecode = 18;
    pub const SUBTRACT: Bytecode = 19;
    pub const MULTIPLY: Bytecode = 20;
    pub const DIVIDE: Bytecode = 21;
    pub const NOT: Bytecode = 22;
    pub const NEGATE: Bytecode = 23;
    pub const PRINT: Bytecode = 24;
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
    //pub const METHOD: Bytecode = 36;

    pub const CONSTANT_LONG: Bytecode = 37;
}
