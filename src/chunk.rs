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
    pub lines: Vec<u32>,
    pub value_array: VmValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![], lines: vec![], value_array: VmValueArray::new() }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
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

    pub fn get_line(&self, chunk_index: usize) -> Option<u32> {
        if self.lines.len() <= chunk_index {
            None
        } else {
            Some(self.lines[chunk_index])
        }
    }
}
