use crate::runtime_val::{RuntimeValue, StringObj};

pub fn hash_str(_str: &str) -> u32 {
    let mut hash: u32 = 2166136261;

    for b in _str.bytes() {
        hash ^= b as u32;
        hash = hash.overflowing_mul(16777619).0;
    }

    hash
}

pub struct Table<'s> {
    entries: Vec<Entry<'s>>,
}

pub struct Entry<'s> {
    key: &'s StringObj,
    value: RuntimeValue,
}
