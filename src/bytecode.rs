use core::slice::Iter;
use std::iter::Enumerate;

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    Return = 1,
    Add,
}

type ByteCode = u8;

pub struct Chunk {
    pub code: Vec<ByteCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<u32>,
}

type Value = f64;

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn disassemble(&self) {
        let mut opcodes = self.code.iter().enumerate();

        println!("OFFSET     LINE     OPCODE      OTHER INFO");
        println!("==========================================");

        while let Some((offset, next_opcode)) = opcodes.next() {
            let opcode = unsafe { std::mem::transmute(*next_opcode) };
            print!("0x{:4X}     {:4}     ", offset, self.lines[offset]);
            match opcode {
                OpCode::Return => println!("RETURN"),
                OpCode::Constant => self.disassemble_constant(&mut opcodes),
                _ => unimplemented!(),
            }
        }
    }

    pub fn disassemble_constant(&self, code: &mut Enumerate<Iter<u8>>) {
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
