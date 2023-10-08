use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil(),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil() => write!(f, "{}", "nil"),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(float) => write!(f, "{}", *float),
        }
    }
}

pub type VmValue = f64;

#[derive(Debug)]
pub struct VmValueArray {
    pub values: Vec<VmValue>,
}

impl VmValueArray {
    pub fn new() -> VmValueArray {
        VmValueArray { values: vec![] }
    }

    pub fn add(&mut self, value: VmValue) {
        self.values.push(value);
    }
}
