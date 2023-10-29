use crate::{
    chunk::{Chunk, OpCode},
    compiler::Parser,
    debug::disassemble_instruction,
    value::Value,
};

macro_rules! binary_op {
    ($self:ident, +) => {{
        let b = $self.pop().unwrap();
        let a = $self.pop().unwrap();
        match (a, b) {
            (Value::Str(x), Value::Str(y)) => {
                let mut c = String::new();
                c.push_str(&x);
                c.push_str(&y);
                $self.push(Value::Str(Box::new(c)));
            }
            (Value::Number(x), Value::Number(y)) => {
                let c = x + y;
                $self.push(Value::Number(c));
            }
            _ => {
                $self.runtime_error("Operands must be numbers.");
                return Err(InterpretError::RuntimeError);
            }
        }
    }};
    ($self:ident, $op:tt) => {{
        let b = $self.pop().unwrap();
        let a = $self.pop().unwrap();
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => {
                let c = x $op y;
                $self.push(Value::Number(c));
            }
            _ => {
                $self.runtime_error("Operands must be numbers.");
                return Err(InterpretError::RuntimeError);
            }
        }
    }}
}

macro_rules! compare {
    ($self:ident, $op:tt) => {{
        let b = $self.pop().unwrap();
        let a = $self.pop().unwrap();
        $self.push(Value::Boolean(a $op b));
    }};
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct VM {
    stack: Vec<Value>,
    top: usize,
    ip: usize,
    chunk: Chunk
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![],
            top: 1,
            ip: 0,
            chunk: Chunk::new(),
        }
    }

    pub fn reset(&mut self) {
        self.top = 1;
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.top = self.stack.len();
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn peek(&self, depth: usize) -> Option<&Value> {
        self.stack.get(depth)
    }

    pub fn interpret(&mut self, source: String) -> Result<(), InterpretError> {
        let mut parser = Parser::new(source);
        let compilation_result = parser.compile(&mut self.chunk);
        if let Err(_) = compilation_result {
            Err(InterpretError::CompileError)
        } else {
            self.run()
        }
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        while self.ip < self.chunk.code.len() {
            if cfg!(feature = "DEBUG_TRACE_EXECUTION") {
                print!("        ");
                for value in &self.stack {
                    print!("[ {} ]", value);
                }
                println!("");
                disassemble_instruction(&mut self.chunk, self.ip);
            }
            if let Ok(code) = OpCode::try_from(self.chunk.code[self.ip]) {
                self.ip = self.ip + 1;
                use OpCode as OC;
                match code {
                    OC::OpConstant => {
                        let value_index = self.chunk.code[self.ip] as usize;
                        let constant = self.chunk.value_array.values[value_index].clone();
                        self.push(constant);
                        self.ip = self.ip + 1;
                    }
                    OC::OpNil => self.push(Value::Nil),
                    OC::OpTrue => self.push(Value::Boolean(true)),
                    OC::OpFalse => self.push(Value::Boolean(false)),
                    OC::OpEqual => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(Value::Boolean(b == a))
                    },
                    OC::OpGreater => compare!(self, >),
                    OC::OpLess => compare!(self, <),
                    OC::OpAdd => binary_op!(self, +),
                    OC::OpSubtract => binary_op!(self, -),
                    OC::OpMultiply => binary_op!(self, *),
                    OC::OpDivide => binary_op!(self, /),
                    OC::OpNot => {
                        if let Some(b) = self.pop() {
                            self.push(Value::Boolean(b.is_falsey()));
                        }
                    }
                    OC::OpNegate => {
                        if let Some(value) = self.pop() {
                            if let Value::Number(num) = value {
                                self.push(Value::Number(-num));
                            } else {
                                self.runtime_error("Operand must be a number.");
                                return Err(InterpretError::RuntimeError);
                            }
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

    fn runtime_error(&mut self, message: &str) {
        eprintln!("{}", message);
        eprintln!("[line {}] in script", self.chunk.lines[self.ip - 1])
    }
}
