use std::{
    fmt::{self, Display},
    rc::Rc,
};

use crate::{env::Env, reader::Sexp};

pub type Builtin = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Vec<Sexp>,
    pub env: Env,
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub car: Value,
    pub cdr: Value,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.car == other.car && self.cdr == other.cdr
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}", self.car)?;

        let mut cdr = &self.cdr;
        loop {
            match cdr {
                Value::Nil => break,
                Value::Pair(next) => {
                    write!(f, " {}", next.car)?;
                    cdr = &next.cdr;
                }
                tail => {
                    write!(f, " . {}", tail)?;
                    break;
                }
            }
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Pair(Rc<Pair>),
    Symbol(String),
    Builtin(Builtin),
    Lambda(Lambda),
}

impl Value {
    pub fn cons(car: Value, cdr: Value) -> Value {
        Value::Pair(Rc::new(Pair { car, cdr }))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Nil, Value::Nil) => true,
            (Value::Pair(x), Value::Pair(y)) => x == y,
            (Value::Symbol(x), Value::Symbol(y)) => x == y,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(true) => write!(f, "#t"),
            Value::Bool(false) => write!(f, "#f"),
            Value::Symbol(sym) => write!(f, "{}", sym),
            Value::Builtin(_) => write!(f, "#<builtin>"),
            Value::Lambda(_) => write!(f, "#<lambda>"),
            Value::Nil => write!(f, "()"),
            Value::Pair(p) => write!(f, "{}", p),
        }
    }
}
