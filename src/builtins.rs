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
            other => return Err(format!("+ expects numbers, got {}", other)),
        }
    }
    Ok(Value::Number(sum))
}

fn equal(args: &[Value]) -> Result<Value, String> {
    match args {
        [a, b] => Ok(Value::Bool(a == b)),
        _ => Err("eq expects two args".into()),
    }
}

fn null(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Nil] => Ok(Value::Bool(true)),
        [_] => Ok(Value::Bool(false)),
        _ => Err("null? expects a single argument".into()),
    }
}

fn car(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Pair(p)] => Ok(p.car.clone()),
        [Value::Nil] => Err("car: empty list".into()),
        [other] => Err(format!("car expects a list, got {}", other)),
        _ => Err("car expects one arg".into()),
    }
}

fn cdr(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Pair(p)] => Ok(p.cdr.clone()),
        [Value::Nil] => Err("cdr: empty list".into()),
        [other] => Err(format!("cdr expects a list, got {}", other)),
        _ => Err("cdr expects one arg".into()),
    }
}

fn cons(args: &[Value]) -> Result<Value, String> {
    match args {
        [a, b] => Ok(Value::cons(a.clone(), b.clone())),
        _ => Err("cons expects two args".into()),
    }
}

pub fn bootstrap_env() -> Env {
    let mut e = Env::new();
    e.define("+".to_string(), Value::Builtin(add));
    e.define("equal?".to_string(), Value::Builtin(equal));
    e.define("null?".to_string(), Value::Builtin(null));
    e.define("car".to_string(), Value::Builtin(car));
    e.define("cdr".to_string(), Value::Builtin(cdr));
    e.define("cons".to_string(), Value::Builtin(cons));
    e
}
