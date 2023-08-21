use crate::chunk::{ Chunk, OpCode };

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    
    let mut offset: usize = 0;
    while offset < chunk.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:4} ", offset);

    let inst_code: u8 = chunk.code[offset];
    let op_code = OpCode::try_from(
        inst_code
    );

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }
    if let Ok(code) = op_code {
        match code {
            OpCode::OpReturn => simple_instruction("OpReturn", offset),
            OpCode::OpConstant => constant_instruction("OpConstant", chunk, offset)
        }
    } else {
        println!("Unknown opcode {}", inst_code);
        offset + 1
    }
}

pub fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

pub fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let value = chunk.value_array.values[constant as usize];
    println!("{} {:4} '{}'", name, constant, value);
    offset + 2
}

