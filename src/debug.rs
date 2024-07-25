use crate::chunk::{Chunk, OpCode};

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
    let op_code = OpCode::try_from(inst_code);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }
    use OpCode as OC;
    if let Ok(code) = op_code {
        match code {
            OC::OpConstant => constant_instruction("OpConstant", chunk, offset),
            OC::OpNil => simple_instruction("OpNil", offset),
            OC::OpTrue => simple_instruction("OpTrue", offset),
            OC::OpFalse => simple_instruction("OpFalse", offset),
            OC::OpPop => simple_instruction("OpPop", offset),
            OC::OpGetLocal => byte_instruction("OpGetLocal", chunk, offset),
            OC::OpSetLocal => byte_instruction("OpSetLocal", chunk, offset),
            OC::OpGetGlobal => constant_instruction("OpGetGlobal", chunk, offset),
            OC::OpDefineGlobal => constant_instruction("OpDefineGlobal", chunk, offset),
            OC::OpSetGlobal => constant_instruction("OpSetGlobal", chunk, offset),
            OC::OpEqual => simple_instruction("OpEqual", offset),
            OC::OpGreater => simple_instruction("OpGreater", offset),
            OC::OpLess => simple_instruction("OpLess", offset),
            OC::OpAdd => simple_instruction("OpAdd", offset),
            OC::OpSubtract => simple_instruction("OpSubtract", offset),
            OC::OpMultiply => simple_instruction("OpMultiply", offset),
            OC::OpDivide => simple_instruction("OpDivide", offset),
            OC::OpNot => simple_instruction("OpNot", offset),
            OC::OpNegate => simple_instruction("OpNegate", offset),
            OC::OpPrint => simple_instruction("OpPrint", offset),
            OC::OpJump => jump_instruction("OpJump", 1, chunk, offset),
            OC::OpJumpIfFalse => jump_instruction("OpJumpIfFalse", 1, chunk, offset),
            OC::OpLoop => jump_instruction("OpLoop", -1, chunk, offset),
            OC::OpReturn => simple_instruction("OpReturn", offset),
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

pub fn byte_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let slot = chunk.code[offset + 1];
    println!("{} {:4}", name, slot);
    return offset + 2;
}

pub fn jump_instruction(name: &str, sign: i8, chunk: &Chunk, offset: usize) -> usize {
    let jump = (((chunk.code[offset + 1] as u16) << 8) | chunk.code[offset + 2] as u16) as usize;
    let target = if sign > 0 {
        offset + 3 + jump
    } else {
        offset + 3 - jump
    };
    println!("{} {:4} -> {}", name, offset, target);
    return offset + 3;
}

pub fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let value = &chunk.value_array.values[constant as usize];
    println!("{} {:4} '{}'", name, constant, value);
    offset + 2
}
