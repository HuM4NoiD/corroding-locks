use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::value::{VmValue, VmValueArray};

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<i32>,
    pub value_array: VmValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![], lines: vec![], value_array: VmValueArray::new() }
    }

    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn add_constant(&mut self, value: VmValue) -> usize {
        self.value_array.add(value);
        self.value_array.values.len() - 1
    }
}
