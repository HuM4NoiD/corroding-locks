#[macro_use]
extern crate paste;

#[macro_use]
pub mod ast_macros;
pub mod ast;
pub mod error;
pub mod lox;
pub mod scanner;
pub mod token;
pub mod value;
pub mod chunk;
pub mod debug;

use crate::{
    chunk::{ Chunk, OpCode::* }, 
    debug::disassemble_chunk,
    value::{ VmValue }
};


fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpReturn.into(), 123);

    let constant_index = chunk.add_constant(1.2);
    chunk.write(OpConstant.into(), 123);
    chunk.write(constant_index.try_into().unwrap(), 123);

    disassemble_chunk(&chunk, "test chunk");
}

