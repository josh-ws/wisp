use crate::reader::Sexp;

pub type Builtin = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Vec<Sexp>,
}

#[derive(Debug)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Builtin(Builtin),
    Lambda(Lambda),
}
