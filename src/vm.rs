use std::mem::transmute;

use super::{
    bytecode::{opcodes, Bytecode, Chunk},
    runtime_val::{ObjType, RuntimeValue, StringObj},
};

const STACK_SIZE: usize = 0xFF;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    sp: usize,

    stack: [RuntimeValue; STACK_SIZE],
}

macro_rules! binary_op {
    ($name:ident, $op:tt, $typ:ident) => {
        #[inline]
        fn $name(&mut self) -> Result<(), LoxRuntimeErr> {
            let first = self.peek(2)?;
            let second = self.peek(1)?;

            match (first, second) {
                (RuntimeValue::Number(n1), RuntimeValue::Number(n2)) => {
                    self.stack[self.sp - 2] = RuntimeValue::$typ(n1 $op n2);
                    self.sp -= 1;
                    Ok(())
                }
                (RuntimeValue::Nil, _) | (_, RuntimeValue::Nil) => {
                    Err(LoxRuntimeErr::MissingOperand)
                }
                _ => {
                    eprintln!(
                        "runtime error at line {}: cannot apply '{}' to {} and {}",
                        self.chunk.lines[self.ip],
                        std::stringify!($name),
                        first.type_repr(),
                        second.type_repr()
                    );
                    Err(LoxRuntimeErr::InvalidType)
                }
            }
        }
    };
}

impl Vm {
    pub fn new(chunk: Chunk) -> Vm {
        Vm {
            chunk,
            ip: 0,
            sp: 0,

            stack: [RuntimeValue::Nil; STACK_SIZE],
        }
    }

    pub fn execute(&mut self) -> Result<(), LoxRuntimeErr> {
        loop {
            let opcode = self.chunk.code[self.ip];
            self.ip += 1;

            match opcode {
                opcodes::CONSTANT => self.constant()?,
                opcodes::NIL => self.push(RuntimeValue::Nil)?,
                opcodes::TRUE => self.push(RuntimeValue::Bool(true))?,
                opcodes::FALSE => self.push(RuntimeValue::Bool(false))?,
                opcodes::EQUAL => self.equal()?,
                opcodes::GREATER => self.greater()?,
                opcodes::LESS => self.less()?,
                opcodes::ADD => self.add()?,
                opcodes::SUBTRACT => self.subtract()?,
                opcodes::MULTIPLY => self.multiply()?,
                opcodes::DIVIDE => self.divide()?,
                opcodes::NOT => self.not()?,
                opcodes::NEGATE => self.negate()?,
                opcodes::RETURN => {
                    println!("{}", self.pop()?);
                    return Ok(());
                }
                _ => panic!("Invalid or unimplemented opcode: {}", opcode),
            };
        }
    }

    #[inline]
    fn push(&mut self, val: RuntimeValue) -> Result<(), LoxRuntimeErr> {
        if self.sp < STACK_SIZE {
            self.stack[self.sp] = val;
            self.sp += 1;
            Ok(())
        } else {
            Err(LoxRuntimeErr::StackOverflow)
        }
    }

    #[inline]
    fn pop(&mut self) -> Result<RuntimeValue, LoxRuntimeErr> {
        if self.sp >= 1 {
            self.sp -= 1;
            Ok(self.stack[self.sp])
        } else {
            Err(LoxRuntimeErr::StackUnderflow)
        }
    }

    #[inline]
    fn peek_mut(&mut self, distance: usize) -> Result<&mut RuntimeValue, LoxRuntimeErr> {
        if self.sp.checked_sub(distance).is_some() {
            Ok(&mut self.stack[self.sp - distance])
        } else {
            Err(LoxRuntimeErr::StackUnderflow)
        }
    }

    #[inline]
    fn peek(&self, distance: usize) -> Result<&RuntimeValue, LoxRuntimeErr> {
        if self.sp.checked_sub(distance).is_some() {
            Ok(&self.stack[self.sp - distance])
        } else {
            Err(LoxRuntimeErr::StackUnderflow)
        }
    }

    #[inline]
    fn read_byte(&mut self) -> Bytecode {
        let val = self.chunk.code[self.ip];
        self.ip += 1;
        val
    }

    #[inline]
    fn constant(&mut self) -> Result<(), LoxRuntimeErr> {
        let index = self.read_byte();
        let value = self.chunk.constants[index as usize];

        self.push(value)?;
        Ok(())
    }

    binary_op!(add, +, Number);
    binary_op!(subtract, -, Number);
    binary_op!(multiply, *, Number);
    binary_op!(divide, /, Number);

    binary_op!(greater, >, Bool);
    binary_op!(less, <, Bool);

    #[inline]
    fn not(&mut self) -> Result<(), LoxRuntimeErr> {
        let peeked = self.peek_mut(1)?;
        *peeked = RuntimeValue::Bool(Vm::is_falsy(*peeked));
        Ok(())
    }

    #[inline]
    fn equal(&mut self) -> Result<(), LoxRuntimeErr> {
        // TODO: execute equal in-place
        let equal = Vm::values_equal(self.pop()?, self.pop()?);
        self.push(RuntimeValue::Bool(equal))?;
        Ok(())
    }

    #[inline]
    fn negate(&mut self) -> Result<(), LoxRuntimeErr> {
        let peeked = self.peek_mut(1)?;

        match peeked {
            RuntimeValue::Bool(_) | RuntimeValue::Obj(_) => Err(LoxRuntimeErr::InvalidType),
            RuntimeValue::Number(n) => {
                *peeked = RuntimeValue::Number(-*n);
                Ok(())
            }
            RuntimeValue::Nil => Err(LoxRuntimeErr::MissingOperand),
        }
    }

    #[inline]
    fn values_equal(val1: RuntimeValue, val2: RuntimeValue) -> bool {
        match (val1, val2) {
            (RuntimeValue::Bool(b1), RuntimeValue::Bool(b2)) => b1 == b2,
            (RuntimeValue::Number(n1), RuntimeValue::Number(n2)) => n1 == n2,
            (RuntimeValue::Obj(typ_ptr_1), RuntimeValue::Obj(typ_ptr_2)) => unsafe {
                match (*typ_ptr_1, *typ_ptr_2) {
                    (ObjType::String, ObjType::String) => {
                        let string_ptr_1 = transmute::<*mut ObjType, *mut StringObj>(typ_ptr_1);
                        let string_ptr_2 = transmute::<*mut ObjType, *mut StringObj>(typ_ptr_2);

                        (*string_ptr_1).contents == (*string_ptr_2).contents
                    }
                }
            },
            (RuntimeValue::Nil, RuntimeValue::Nil) => true,
            _ => false,
        }
    }

    #[inline]
    fn is_falsy(val: RuntimeValue) -> bool {
        match val {
            RuntimeValue::Nil | RuntimeValue::Bool(false) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum LoxRuntimeErr {
    InvalidType,
    MissingOperand,
    StackOverflow,
    StackUnderflow,
}
