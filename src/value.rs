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
