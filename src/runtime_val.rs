use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    mem::{size_of, transmute},
    ptr::copy_nonoverlapping,
    slice, str,
};
#[derive(Clone, Copy)]
pub enum RuntimeValue {
    Nil,
    Bool(bool),
    Number(f64),
    String(*mut StringObj),
}

extern "C" {
    pub type opaque_slice;
}

#[repr(C)]
pub struct StringObj {
    pub len: usize,
    pub contents: opaque_slice,
}

// TODO: consult StringObj implementation with more experienced devs

impl StringObj {
    pub fn new(contents: &str) -> *mut StringObj {
        let size = size_of::<usize>() + contents.len();
        let layout = Layout::from_size_align(size, size_of::<usize>()).expect("shouldn't happen");
        unsafe {
            let ptr = alloc(layout);
            let string_ptr = transmute::<*mut u8, *mut StringObj>(ptr);
            (*string_ptr).len = contents.len();

            let contents_ptr = &mut (*string_ptr).contents as *mut opaque_slice as *mut u8;
            copy_nonoverlapping::<u8>(contents.as_ptr(), contents_ptr, contents.len());

            string_ptr
        }
    }

    // TODO: refactor string concat
    pub fn concat(&mut self, other: *mut StringObj) -> *mut StringObj {
        let size = size_of::<usize>() + self.len;
        let prev_layout =
            Layout::from_size_align(size, size_of::<usize>()).expect("shouldn't happen");
        unsafe {
            let prev_len = self.len;
            self.len += (*other).len;
            let new_size = size_of::<usize>() + self.len;

            let self_ptr = self as *mut StringObj as *mut u8;
            let new_ptr = realloc(self_ptr, prev_layout, new_size);

            let contents_ptr_dst = &mut self.contents as *mut opaque_slice as *mut u8;
            let contents_ptr_src = &mut (*other).contents as *mut opaque_slice as *mut u8;

            copy_nonoverlapping(
                contents_ptr_src,
                contents_ptr_dst.add(prev_len),
                (*other).len,
            );

            transmute::<*mut u8, *mut StringObj>(new_ptr)
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let data_ptr = &self.contents as *const opaque_slice as *const u8;
            let slice = slice::from_raw_parts(data_ptr, self.len);
            str::from_utf8_unchecked(slice)
        }
    }

    pub fn _free(&mut self) {
        todo!();
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
