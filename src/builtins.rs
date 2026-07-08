use crate::{env::Env, value::Value};

// quote, cond, lambda, define
// atom, eq, car, cdr, cons
// +, *, -, /
// and, or, not
// https://leinonen.ninja/posts/building-lisp-from-the-ground-up

fn add(args: &[Value]) -> Result<Value, String> {
    let mut sum = 0.0;
    for arg in args {
        match arg {
            Value::Number(n) => sum += n,
            other => return Err(format!("+ expects numbers, got {:?}", other)),
        }
    }
    Ok(Value::Number(sum))
}

pub fn bootstrap_env() -> Env {
    let mut e = Env::new();
    e.define("+".to_string(), Value::Builtin(add));
    e
}
