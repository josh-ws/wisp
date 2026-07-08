use crate::reader::Sexp;

pub type Builtin = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Vec<Sexp>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Builtin(Builtin),
    Lambda(Lambda),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            _ => false,
        }
    }
}
