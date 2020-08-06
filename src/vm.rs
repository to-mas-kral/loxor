use std::{
    collections::{HashMap, HashSet},
    mem, ptr,
};

use super::{
    bytecode::{opcodes, Bytecode, Chunk},
    runtime_val::{Obj, ObjTyp, RuntimeValue, StringObj},
};

const STACK_SIZE: usize = 0xFF;

type RuntimeResult = Result<(), LoxRuntimeErr>;

pub struct Vm<'s> {
    chunk: Chunk,
    ip: usize,
    sp: usize,

    stack: [RuntimeValue; STACK_SIZE],

    strings: HashSet<&'s StringObj>,
    objects: *mut Obj,
}

macro_rules! binary_op {
    ($name:ident, $op:tt, $typ:ident) => {
        #[inline]
        fn $name(&mut self) -> RuntimeResult {
            let first = self.peek(2)?;
            let second = self.peek(1)?;

            match (first, second) {
                (RuntimeValue::Number(n1), RuntimeValue::Number(n2)) => {
                    self.stack[self.sp - 2] = RuntimeValue::$typ(n1 $op n2);
                    self.sp -= 1;
                    Ok(())
                }
                _ => {
                    eprintln!(
                        "runtime error at line {}: cannot apply '{}' to {} and {}",
                        self.chunk.get_line_at_ip(self.ip),
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

impl<'s> Vm<'s> {
    pub fn new(chunk: Chunk) -> Vm<'s> {
        let mut vm = Vm {
            chunk,
            ip: 0,
            sp: 0,

            stack: [RuntimeValue::Nil; STACK_SIZE],

            strings: HashSet::new(),
            objects: ptr::null_mut(),
        };

        vm.link_objects();
        vm.intern_strings();

        vm
    }

    pub fn execute(&mut self) -> RuntimeResult {
        loop {
            let opcode = self.chunk.code[self.ip];
            self.ip += 1;

            match opcode {
                opcodes::CONSTANT => self.constant()?,
                opcodes::NIL => self.push(RuntimeValue::Nil)?,
                opcodes::TRUE => self.push(RuntimeValue::Bool(true))?,
                opcodes::FALSE => self.push(RuntimeValue::Bool(false))?,
                opcodes::POP => {
                    self.pop()?;
                }
                opcodes::GET_GLOBAL => {
                    unimplemented!("global variable expressions are unimplemented")
                }
                opcodes::DEFINE_GLOBAL => {
                    unimplemented!("global variable definitions are unimplemented")
                }
                opcodes::EQUAL => self.equal()?,
                opcodes::GREATER => self.greater()?,
                opcodes::LESS => self.less()?,
                opcodes::ADD => self.add()?,
                opcodes::SUBTRACT => self.subtract()?,
                opcodes::MULTIPLY => self.multiply()?,
                opcodes::DIVIDE => self.divide()?,
                opcodes::NOT => self.not()?,
                opcodes::NEGATE => self.negate()?,
                opcodes::PRINT => self.print()?,
                opcodes::RETURN => return Ok(()),

                opcodes::CONSTANT_LONG => self.constant_long()?,
                _ => panic!("Invalid or unimplemented opcode: {}", opcode),
            };
        }
    }

    #[inline(never)]
    fn push(&mut self, val: RuntimeValue) -> RuntimeResult {
        if self.sp < STACK_SIZE {
            self.stack[self.sp] = val;
            self.sp += 1;
            Ok(())
        } else {
            Err(LoxRuntimeErr::StackOverflow)
        }
    }

    #[inline(never)]
    fn pop(&mut self) -> Result<RuntimeValue, LoxRuntimeErr> {
        if self.sp >= 1 {
            self.sp -= 1;
            Ok(self.stack[self.sp])
        } else {
            Err(LoxRuntimeErr::StackUnderflow)
        }
    }

    #[inline(never)]
    fn peek_mut(&mut self, distance: usize) -> Result<&mut RuntimeValue, LoxRuntimeErr> {
        if self.sp.checked_sub(distance).is_some() {
            Ok(&mut self.stack[self.sp - distance])
        } else {
            Err(LoxRuntimeErr::StackUnderflow)
        }
    }

    #[inline(never)]
    fn peek(&self, distance: usize) -> Result<&RuntimeValue, LoxRuntimeErr> {
        if self.sp.checked_sub(distance).is_some() {
            Ok(&self.stack[self.sp - distance])
        } else {
            Err(LoxRuntimeErr::StackUnderflow)
        }
    }

    #[inline(never)]
    fn read_byte(&mut self) -> Bytecode {
        let val = self.chunk.code[self.ip];
        self.ip += 1;
        val
    }

    #[inline(never)]
    fn constant(&mut self) -> RuntimeResult {
        let index = self.read_byte();
        let value = self.chunk.constants[index as usize];

        self.push(value)?;
        Ok(())
    }

    #[inline(never)]
    fn constant_long(&mut self) -> RuntimeResult {
        let mut bytes = [0; 4];

        for i in 0..3 {
            bytes[i] = self.read_byte();
        }

        let index = u32::from_le_bytes(bytes);
        let value = self.chunk.constants[index as usize];

        self.push(value)?;
        Ok(())
    }

    #[inline(never)]
    fn add(&mut self) -> RuntimeResult {
        let first = self.peek(2)?;
        let second = self.peek(1)?;

        match (first, second) {
            (RuntimeValue::Number(n1), RuntimeValue::Number(n2)) => {
                self.stack[self.sp - 2] = RuntimeValue::Number(n1 + n2);
                self.sp -= 1;
                Ok(())
            }
            (RuntimeValue::String(s1), RuntimeValue::String(s2)) => unsafe {
                let new_str_ptr = (**s1).concat(*s2);
                //println!("concated: {}", (*new_str_ptr).as_str());
                // FIXME: not great, add string to linked list
                (*new_str_ptr).next_obj = self.objects;
                self.objects = (*new_str_ptr).as_obj_ptr();

                self.stack[self.sp - 2] = RuntimeValue::String(new_str_ptr);
                self.sp -= 1;
                // TODO: string concatenation could return an error
                Ok(())
            },
            _ => {
                eprintln!(
                    "Runtime error at line {}: cannot apply 'add' to {} and {}",
                    self.chunk.get_line_at_ip(self.ip),
                    first.type_repr(),
                    second.type_repr()
                );
                Err(LoxRuntimeErr::InvalidType)
            }
        }
    }

    binary_op!(subtract, -, Number);
    binary_op!(multiply, *, Number);
    binary_op!(divide, /, Number);

    binary_op!(greater, >, Bool);
    binary_op!(less, <, Bool);

    #[inline(never)]
    fn not(&mut self) -> RuntimeResult {
        let peeked = self.peek_mut(1)?;
        *peeked = RuntimeValue::Bool(Vm::is_falsy(*peeked));
        Ok(())
    }

    #[inline(never)]
    fn equal(&mut self) -> RuntimeResult {
        // TODO: execute equal in-place
        let equal = Vm::values_equal(self.pop()?, self.pop()?);
        self.push(RuntimeValue::Bool(equal))?;
        Ok(())
    }

    #[inline(never)]
    fn negate(&mut self) -> RuntimeResult {
        let peeked = self.peek_mut(1)?;

        match peeked {
            RuntimeValue::Bool(_) | RuntimeValue::String(_) => Err(LoxRuntimeErr::InvalidType),
            RuntimeValue::Number(n) => {
                *peeked = RuntimeValue::Number(-*n);
                Ok(())
            }
            RuntimeValue::Nil => Err(LoxRuntimeErr::MissingOperand),
        }
    }

    #[inline(never)]
    fn print(&mut self) -> RuntimeResult {
        let val = self.pop()?;
        println!("{}", val);
        Ok(())
    }

    #[inline(never)]
    fn values_equal(val1: RuntimeValue, val2: RuntimeValue) -> bool {
        match (val1, val2) {
            (RuntimeValue::Bool(b1), RuntimeValue::Bool(b2)) => b1 == b2,
            (RuntimeValue::Number(n1), RuntimeValue::Number(n2)) => n1 == n2,
            (RuntimeValue::String(s1), RuntimeValue::String(s2)) => unsafe {
                (*s1).as_str() == (*s2).as_str()
            },
            (RuntimeValue::Nil, RuntimeValue::Nil) => true,
            _ => false,
        }
    }

    #[inline(never)]
    fn is_falsy(val: RuntimeValue) -> bool {
        match val {
            RuntimeValue::Nil | RuntimeValue::Bool(false) => true,
            _ => false,
        }
    }

    fn link_objects(&mut self) {
        for v in &mut self.chunk.constants {
            match v {
                RuntimeValue::String(str_ptr) => unsafe {
                    (**str_ptr).next_obj = self.objects;
                    self.objects = (**str_ptr).as_obj_ptr();
                },
                _ => continue,
            }
        }
    }

    fn intern_strings(&mut self) {}
}

impl<'s> Drop for Vm<'s> {
    fn drop(&mut self) {
        // Free the runtime objects
        let mut next_obj = self.objects;
        while next_obj != ptr::null_mut() {
            unsafe {
                match (*next_obj).typ {
                    ObjTyp::String => {
                        let str_ptr = mem::transmute::<*mut Obj, *mut StringObj>(next_obj);
                        //println!("freeing {}", (*str_ptr).as_str());
                        let next_ptr = (*str_ptr).next_obj;
                        ptr::drop_in_place(str_ptr);
                        next_obj = next_ptr;
                    }
                }
            }
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
