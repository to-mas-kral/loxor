use super::bytecode::{opcodes, Bytecode, Chunk};

const STACK_SIZE: usize = 0xFF;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    sp: usize,

    stack: [RuntimeValue; STACK_SIZE],
}

macro_rules! binary_op {
    ($name:ident, $op:tt) => {
        #[inline]
        fn $name(&mut self) -> Result<(), LoxRuntimeErr> {
            let first = self.peek(2)?;
            let second = self.peek(1)?;

            match (first, second) {
                (RuntimeValue::Number(n1), RuntimeValue::Number(n2)) => {
                    self.stack[self.sp - 2] = RuntimeValue::Number(n1 $op n2);
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
                opcodes::RETURN => {
                    println!("{}", self.pop()?);
                    return Ok(());
                }
                opcodes::CONSTANT => self.constant()?,
                opcodes::NIL => self.push(RuntimeValue::Nil)?,
                opcodes::TRUE => self.push(RuntimeValue::Bool(true))?,
                opcodes::FALSE => self.push(RuntimeValue::Bool(false))?,
                opcodes::ADD => self.add()?,
                opcodes::SUBTRACT => self.subtract()?,
                opcodes::MULTIPLY => self.multiply()?,
                opcodes::DIVIDE => self.divide()?,
                opcodes::NEGATE => self.negate()?,
                _ => unreachable!(),
            }
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

    binary_op!(add, +);
    binary_op!(subtract, -);
    binary_op!(multiply, *);
    binary_op!(divide, /);

    #[inline]
    fn negate(&mut self) -> Result<(), LoxRuntimeErr> {
        let peeked = self.peek_mut(1)?;

        match peeked {
            RuntimeValue::Bool(_) => Err(LoxRuntimeErr::InvalidType),
            RuntimeValue::Number(n) => {
                *peeked = RuntimeValue::Number(-*n);
                Ok(())
            }
            RuntimeValue::Nil => Err(LoxRuntimeErr::MissingOperand),
        }
    }
}

#[derive(Clone, Copy)]
pub enum RuntimeValue {
    Nil,
    Bool(bool),
    Number(f64),
}

impl core::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            RuntimeValue::Bool(b) => match b {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            RuntimeValue::Number(n) => write!(f, "{}", n.to_string().as_str()),
            RuntimeValue::Nil => write!(f, ""),
        }
    }
}

impl RuntimeValue {
    pub fn type_repr(&self) -> &str {
        match self {
            RuntimeValue::Nil => "",
            RuntimeValue::Bool(_) => "bool",
            RuntimeValue::Number(_) => "number",
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
