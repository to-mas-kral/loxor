use core::slice::Iter;
use std::iter::Enumerate;

pub mod opcodes;

type Bytecode = u8;

// TODO: challenge - run-length line number encoding
// TODO: add support for more constants (32 bit index)

pub struct Chunk {
    pub code: Vec<Bytecode>,
    pub constants: Vec<f64>,
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

    pub fn add_constant(&mut self, numerical_value: f64, line: usize) {
        let index = self.constants.len();
        if index < 255 {
            self.constants.push(numerical_value);
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
                opcodes::ADD => println!("ADD"),
                opcodes::SUBTRACT => println!("SUBTRACT"),
                opcodes::MULTIPLY => println!("MULTIPLY"),
                opcodes::DIVIDE => println!("DIVIDE"),
                opcodes::NEGATE => println!("NEGATE"),
                _ => unimplemented!("opcode {} is unimplemented", opcode),
            }
        }
    }

    fn constant(&self, code: &mut Enumerate<Iter<u8>>) {
        if let Some((_, index)) = code.next() {
            println!(
                "CONSTANT    c[{}] = {}",
                index, self.constants[*index as usize]
            )
        } else {
            panic!("Constant is missing the index");
        }
    }
}
