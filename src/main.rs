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
#[macro_use]
pub mod vm;

use std::result;

use crate::{
    chunk::{ Chunk, OpCode::* }, 
    debug::disassemble_chunk,
    value::{ VmValue },
    vm::{ InterpretError, VM }
};


fn main() {
    let mut chunk = Chunk::new();
    let mut vm = VM::new();

    let constant_index = chunk.add_constant(1.2);
    chunk.write(OpConstant.into(), 123);
    chunk.write(constant_index.try_into().unwrap(), 123);

    let constant_index = chunk.add_constant(3.4);
    chunk.write(OpConstant.into(), 123);
    chunk.write(constant_index.try_into().unwrap(), 123);

    chunk.write(OpAdd.into(), 123);

    let constant_index = chunk.add_constant(5.6);
    chunk.write(OpConstant.into(), 123);
    chunk.write(constant_index.try_into().unwrap(), 123);

    chunk.write(OpDivide.into(), 123);

    chunk.write(OpNegate.into(), 123);
    chunk.write(OpReturn.into(), 123);

//    disassemble_chunk(&chunk, "test chunk");
    let result = vm.interpret(&chunk);
    match result {
        Ok(_) => println!("Interpret Ok"),
        Err(e) => println!("Interpret Error, {:?}", e),
    };
}

