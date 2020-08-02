use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    mem::{size_of, transmute},
    ptr::{copy_nonoverlapping, null_mut},
    slice, str,
};

use crate::table::hash_str;

// TODO: challenge - add support for "constant" strings

#[derive(Clone, Copy)]
pub enum RuntimeValue {
    Nil,
    Bool(bool),
    Number(f64),
    String(*mut StringObj),
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum ObjTyp {
    String,
}

#[repr(C)]
pub struct Obj {
    pub typ: ObjTyp,
}

extern "C" {
    pub type opaque_slice;
}

#[repr(C)]
pub struct StringObj {
    obj_typ: ObjTyp,
    hash: u32,
    len: usize,
    pub next_obj: *mut Obj,
    contents: opaque_slice,
}

// TODO: consult StringObj implementation with more experienced devs

const BASE_STRING_OBJ_SIZE: usize =
    size_of::<ObjTyp>() + size_of::<u32>() + size_of::<usize>() + size_of::<*mut Obj>();

impl StringObj {
    pub fn new(contents: &str) -> *mut StringObj {
        // Create an allocation layout and allocate enough memory
        let size = BASE_STRING_OBJ_SIZE + contents.len();
        let layout = Layout::from_size_align(size, size_of::<usize>()).unwrap();

        unsafe {
            let ptr = alloc(layout);

            // Initialize fields of the new object
            let string_ptr = transmute::<*mut u8, *mut StringObj>(ptr);
            (*string_ptr).obj_typ = ObjTyp::String;
            (*string_ptr).hash = hash_str(contents);
            (*string_ptr).len = contents.len();
            (*string_ptr).next_obj = null_mut();

            // Copy the string contents
            let contents_ptr = &mut (*string_ptr).contents as *mut opaque_slice as *mut u8;
            copy_nonoverlapping(contents.as_ptr(), contents_ptr, contents.len());

            string_ptr
        }
    }

    // TODO: allocate bigger chunks less often
    pub fn concat(&mut self, other: *mut StringObj) -> *mut StringObj {
        // Create a layout for realloc

        unsafe {
            let size = BASE_STRING_OBJ_SIZE + self.len + (*other).len;
            let prev_layout = Layout::from_size_align(size, size_of::<usize>()).unwrap();

            // Allocate new space for the string
            let new_ptr = alloc(prev_layout);
            let new_ptr = transmute::<*mut u8, *mut StringObj>(new_ptr);

            let contents_ptr_dst = &mut (*new_ptr).contents as *mut opaque_slice as *mut u8;

            let contents_self_ptr = &mut self.contents as *mut opaque_slice as *mut u8;
            let contents_other_ptr = &mut (*other).contents as *mut opaque_slice as *mut u8;

            // Copy contents of self to the new location
            copy_nonoverlapping(contents_self_ptr, contents_ptr_dst, self.len);

            // Copy contents of other to the new location
            copy_nonoverlapping(
                contents_other_ptr,
                contents_ptr_dst.add(self.len),
                (*other).len,
            );

            // Initialize the other fields
            (*new_ptr).obj_typ = ObjTyp::String;
            (*new_ptr).len = self.len + (*other).len;
            (*new_ptr).hash = hash_str((*new_ptr).as_str());
            (*new_ptr).next_obj = null_mut();

            new_ptr
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let data_ptr = &self.contents as *const opaque_slice as *const u8;
            let slice = slice::from_raw_parts(data_ptr, self.len);
            str::from_utf8_unchecked(slice)
        }
    }

    pub fn as_obj_ptr(&mut self) -> *mut Obj {
        unsafe { transmute::<*mut StringObj, *mut Obj>(self as *mut StringObj) }
    }
}

impl Drop for StringObj {
    fn drop(&mut self) {
        let size = BASE_STRING_OBJ_SIZE + self.len;
        let layout = Layout::from_size_align(size, size_of::<usize>()).unwrap();

        unsafe {
            dealloc(self as *mut StringObj as *mut u8, layout);
        }
    }
}

impl core::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            RuntimeValue::Bool(b) => match b {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            RuntimeValue::Number(n) => write!(f, "{}", n.to_string().as_str()),
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::String(string_ptr) => unsafe { write!(f, "{}", (**string_ptr).as_str()) },
        }
    }
}

impl RuntimeValue {
    pub fn type_repr(&self) -> &str {
        match self {
            RuntimeValue::Nil => "",
            RuntimeValue::Bool(_) => "bool",
            RuntimeValue::Number(_) => "number",
            RuntimeValue::String(_) => "string",
        }
    }
}
