use std::collections::HashMap;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Parser,
    debug::disassemble_instruction,
    obj::Obj,
    value::Value,
};

macro_rules! binary_op {
    ($self:ident, +) => {{
        let b = $self.pop().unwrap();
        let a = $self.pop().unwrap();
        match (a, b) {
            (Value::Obj(x), Value::Obj(y)) => {
                match (*x, *y) {
                    (Obj::Str(p), Obj::Str(q)) => {
                        let mut c = String::new();
                        c.push_str(&p);
                        c.push_str(&q);
                        $self.push(Value::Obj(Box::new(Obj::from(c))));
                    },
                    _ => {
                        $self.runtime_error("Operands must be strings.");
                        return Err(InterpretError::RuntimeError);
                    }
                }
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
                $self.push(Value::from(c));
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
        $self.push(Value::from(a $op b));
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
    chunk: Chunk,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![],
            top: 1,
            ip: 0,
            chunk: Chunk::new(),
            globals: HashMap::new(),
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

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        return self.chunk.code[self.ip - 1];
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte();
        self.chunk.value_array.values[index as usize].clone()
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
                    OC::OpTrue => self.push(Value::from(true)),
                    OC::OpFalse => self.push(Value::from(false)),
                    OC::OpPop => {
                        self.pop();
                    }
                    OC::OpGetGlobal => {
                        let constant: Value = self.read_constant();
                        if let Value::Obj(a) = constant {
                            if let Obj::Str(name) = *a {
                                if !self.globals.contains_key(&name) {
                                    self.runtime_error(&format!("Undefined variable '{}'", &name));
                                    return Err(InterpretError::RuntimeError);
                                }
                                let value = self.globals.get(&name).unwrap();
                                self.push(value.clone());
                            }
                        }
                    }
                    OC::OpDefineGlobal => {
                        let constant: Value = self.read_constant();
                        if let Value::Obj(a) = constant {
                            if let Obj::Str(name) = *a {
                                let value = self.pop();
                                self.globals.insert(name.clone(), value.unwrap());
                            }
                        }
                    }
                    OC::OpSetGlobal => {
                        let constant: Value = self.read_constant();
                        if let Value::Obj(a) = constant {
                            if let Obj::Str(name) = *a {
                                let top = self.peek(0).unwrap().clone();
                                let previous_value = self.globals.insert(name.clone(), top);
                                // If previous value did not exist
                                // i.e. the variable was not defined
                                if previous_value.is_none() {
                                    self.globals.remove(&name);
                                    self.runtime_error(&format!("Undefined variable '{}'", &name));
                                    return Err(InterpretError::RuntimeError);
                                }
                            }
                        }
                    }
                    OC::OpEqual => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(Value::from(b == a))
                    }
                    OC::OpGreater => compare!(self, >),
                    OC::OpLess => compare!(self, <),
                    OC::OpAdd => binary_op!(self, +),
                    OC::OpSubtract => binary_op!(self, -),
                    OC::OpMultiply => binary_op!(self, *),
                    OC::OpDivide => binary_op!(self, /),
                    OC::OpNot => {
                        if let Some(b) = self.pop() {
                            self.push(Value::from(b.is_falsey()));
                        }
                    }
                    OC::OpNegate => {
                        if let Some(value) = self.pop() {
                            if let Value::Number(num) = value {
                                self.push(Value::from(-num));
                            } else {
                                self.runtime_error("Operand must be a number.");
                                return Err(InterpretError::RuntimeError);
                            }
                        }
                    }
                    OC::OpPrint => {
                        let value = self.pop();
                        println!("{}", value.unwrap());
                    }
                    OC::OpReturn => {
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
