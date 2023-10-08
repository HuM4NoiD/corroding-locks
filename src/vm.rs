use crate::{
    chunk::{Chunk, OpCode},
    compiler::Parser,
    debug::disassemble_instruction,
    value::VmValue,
};

macro_rules! binary_op {
    ($self:ident, $op:tt) => {{
        let b = $self.pop().unwrap();
        let a = $self.pop().unwrap();
        let c = a $op b;
        $self.push(c)
    }}
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct VM {
    stack: Vec<VmValue>,
    top: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![],
            top: 1,
        }
    }

    pub fn reset(&mut self) {
        self.top = 1;
    }

    pub fn push(&mut self, value: VmValue) {
        self.stack.push(value);
        self.top = self.stack.len();
    }

    pub fn pop(&mut self) -> Option<VmValue> {
        self.stack.pop()
    }

    pub fn interpret(&mut self, source: String) -> Result<(), InterpretError> {
        let mut chunk = Chunk::new();
        let mut parser = Parser::new(source);
        let compilation_result = parser.compile(&mut chunk);
        if let Err(_) = compilation_result {
            Err(InterpretError::CompileError)
        } else {
            self.run(&chunk)
        }
    }

    pub fn run(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
        let mut index: usize = 0;
        while index < chunk.code.len() {
            if cfg!(feature = "DEBUG_TRACE_EXECUTION") {
                print!("        ");
                for value in &self.stack {
                    print!("[ {} ]", value);
                }
                println!("");
                disassemble_instruction(chunk, index);
            }
            if let Ok(code) = OpCode::try_from(chunk.code[index]) {
                index = index + 1;
                use OpCode as OC;
                match code {
                    OC::OpConstant => {
                        let value_index = chunk.code[index] as usize;
                        let constant = chunk.value_array.values[value_index];
                        self.push(constant);
                        index = index + 1;
                    }
                    OC::OpAdd => binary_op!(self, +),
                    OC::OpSubtract => binary_op!(self, -),
                    OC::OpMultiply => binary_op!(self, *),
                    OC::OpDivide => binary_op!(self, /),
                    OC::OpNegate => {
                        if let Some(value) = self.pop() {
                            self.push(-value);
                        }
                    }
                    OC::OpReturn => {
                        let value = self.pop();
                        println!("{:?}", value);
                        return Ok(());
                    }
                }
            } else {
                return Err(InterpretError::RuntimeError);
            };
        }
        return Ok(());
    }
}
