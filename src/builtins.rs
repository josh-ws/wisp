use std::rc::Rc;

use crate::{env::Arena, value::Value};

// TODO:
// Special forms: quote, cond, lambda, define, and, or
// Builtins: atom, eq?, =, car, cdr, cons, +, -, *, /
// Prelude: not
// https://leinonen.ninja/posts/building-lisp-from-the-ground-up

// Checks for argument being an atom, i.e. non-list. Single argument
fn atom(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Pair(_)] => Ok(Value::Bool(false)),
        [_] => Ok(Value::Bool(true)),
        _ => Err(format!("atom? expects 1 argument, {} provided", args.len())),
    }
}

fn null(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Nil] => Ok(Value::Bool(true)),
        [_] => Ok(Value::Bool(false)),
        _ => Err(format!("null? expects 1 argument, {} provided", args.len())),
    }
}

// Quick check for same object, no recursion.
fn eq(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Number(x), Value::Number(y)] => Ok(Value::Bool(x == y)),
        [Value::Bool(x), Value::Bool(y)] => Ok(Value::Bool(x == y)),
        [Value::Nil, Value::Nil] => Ok(Value::Bool(true)),
        [Value::Pair(x), Value::Pair(y)] => Ok(Value::Bool(Rc::ptr_eq(x, y))),
        [Value::Symbol(x), Value::Symbol(y)] => Ok(Value::Bool(x == y)),
        [_, _] => Ok(Value::Bool(false)),
        _ => Err(format!("eq? expects 2 arguments, {} provided", args.len())),
    }
}

fn numeric_eq(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn numeric_add(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn numeric_sub(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn numeric_mul(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn numeric_div(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn numeric_lt(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn car(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Pair(p)] => Ok(p.car.clone()),
        [Value::Nil] => Err("car: empty list".to_string()),
        [other] => Err(format!("car expects a pair, got {}", other)),
        _ => Err(format!("car expects 1 argument, {} provided", args.len())),
    }
}

fn cdr(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Pair(p)] => Ok(p.cdr.clone()),
        [Value::Nil] => Err("cdr: empty list".to_string()),
        [other] => Err(format!("cdr expects a pair, got {}", other)),
        _ => Err(format!("cdr expects 1 argument, {} provided", args.len())),
    }
}

fn cons(args: &[Value]) -> Result<Value, String> {
    match args {
        [a, b] => Ok(Value::cons(a.clone(), b.clone())),
        _ => Err(format!("cons expects 2 arguments, {} provided", args.len())),
    }
}

// temporary, until we have the prelude
fn not(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Bool(false)] => Ok(Value::Bool(true)),
        [_] => Ok(Value::Bool(false)),
        _ => Err(format!("not expects 1 argument, {} provided", args.len())),
    }
}

// also temp
fn equal(args: &[Value]) -> Result<Value, String> {
    match args {
        [a, b] => Ok(Value::Bool(a == b)),
        _ => Err(format!(
            "equal? expects 2 arguments, {} provided",
            args.len()
        )),
    }
}

pub fn bootstrap_env() -> Arena {
    let mut a = Arena::new();
    let root = a.root();

    a.define(root, "atom?".to_string(), Value::Builtin(atom));
    a.define(root, "eq?".to_string(), Value::Builtin(eq));
    a.define(root, "not".to_string(), Value::Builtin(not));
    a.define(root, "equal?".to_string(), Value::Builtin(equal));
    a.define(root, "null?".to_string(), Value::Builtin(null));
    a.define(root, "=".to_string(), Value::Builtin(numeric_eq));
    a.define(root, "+".to_string(), Value::Builtin(numeric_add));
    a.define(root, "-".to_string(), Value::Builtin(numeric_sub));
    a.define(root, "*".to_string(), Value::Builtin(numeric_mul));
    a.define(root, "/".to_string(), Value::Builtin(numeric_div));
    a.define(root, "<".to_string(), Value::Builtin(numeric_lt));
    a.define(root, "car".to_string(), Value::Builtin(car));
    a.define(root, "cdr".to_string(), Value::Builtin(cdr));
    a.define(root, "cons".to_string(), Value::Builtin(cons));

    a
}
