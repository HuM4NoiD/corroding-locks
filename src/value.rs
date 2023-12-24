use std::fmt::{Debug, Display};

use crate::{
    obj::Obj
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    Obj(Box<Obj>)
}



impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Boolean(b) => !b,
            Self::Number(_) => false,
            Self::Obj(_) => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "{}", "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(float) => write!(f, "{}", *float),
            Value::Obj(o) => std::fmt::Display::fmt(&o, f)
        }
    }
}

pub type VmValue = f64;

#[derive(Debug)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: vec![] }
    }

    pub fn add(&mut self, value: Value) {
        self.values.push(value);
    }
}
