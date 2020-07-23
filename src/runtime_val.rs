use std::mem::transmute;
#[derive(Clone, Copy)]
pub enum RuntimeValue {
    Nil,
    Bool(bool),
    Number(f64),
    Obj(*mut ObjType),
}
#[derive(Clone, Copy)]
pub enum ObjType {
    String,
    //Function,
}

#[repr(C)]
pub struct StringObjFam <T: ?Sized> {
    pub typ: ObjType,
    pub len: usize,
    pub contents: T,
}

#[repr(C)]
pub struct StringObj {
    typ: ObjType,
    pub contents: String,
}

impl StringObj {
    pub fn new(chars: &str) -> StringObj {
        StringObj {
            typ: ObjType::String,
            contents: String::from(chars),
        }
    }
}
/*
#[repr(C)]
pub struct FunctionObj {
    name: String,
} */

impl core::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            RuntimeValue::Bool(b) => match b {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            RuntimeValue::Number(n) => write!(f, "{}", n.to_string().as_str()),
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Obj(typ_ptr) => unsafe {
                match **typ_ptr {
                    ObjType::String => {
                        let string_ptr = transmute::<*mut ObjType, *mut StringObj>(*typ_ptr);
                        write!(f, "{}", (*string_ptr).contents)
                    } /* ObjType::Function => {
                          let function_ptr = transmute::<*mut ObjType, *mut FunctionObj>(*typ_ptr);
                          write!(f, "{}", (*function_ptr).name)
                      } */
                }
            },
        }
    }
}

impl RuntimeValue {
    pub fn type_repr(&self) -> &str {
        match self {
            RuntimeValue::Nil => "",
            RuntimeValue::Bool(_) => "bool",
            RuntimeValue::Number(_) => "number",
            RuntimeValue::Obj(typ_ptr) => unsafe {
                match **typ_ptr {
                    ObjType::String => "string",
                    /* ObjType::Function => "function", */
                }
            },
        }
    }
}
