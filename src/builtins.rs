use std::rc::Rc;

use crate::{
    env::{Arena, Env},
    eval::eval,
    reader::read,
    value::Value,
};

// TODO:
// Special forms: quote, cond, lambda, define, and, or
// Builtins: atom, eq?, =, car, cdr, cons, +, -, *, /
// Prelude: not
// https://leinonen.ninja/posts/building-lisp-from-the-ground-up

const PRELUDE: &str = include_str!("prelude.scm");

fn load_prelude(arena: &mut Arena, root: Env) {
    let forms = read(PRELUDE).expect("parse error loading prelude");
    for form in &forms {
        if let Err(e) = eval(arena, root, form) {
            panic!("prelude: failed to eval {:?}\n{}", form, e);
        }
    }
}

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
    match args {
        [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a == b)),
        [_, _] => Err("= expects numeric arguments".to_string()),
        _ => Err(format!("= expects 2 arguments, {} provided", args.len())),
    }
}

fn numeric_lt(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a < b)),
        [_, _] => Err("< expects numeric arguments".to_string()),
        _ => Err(format!("< expects 2 arguments, {} provided", args.len())),
    }
}

fn numeric_add(args: &[Value]) -> Result<Value, String> {
    Ok(Value::Number(as_numbers(args)?.iter().sum()))
}

fn numeric_sub(args: &[Value]) -> Result<Value, String> {
    let nums = as_numbers(args)?;
    match nums.as_slice() {
        [] => Err("- expects at least 1 argument".to_string()),
        [x] => Ok(Value::Number(-x)),
        [first, rest @ ..] => Ok(Value::Number(rest.iter().fold(*first, |acc, n| acc - n))),
    }
}

fn numeric_mul(args: &[Value]) -> Result<Value, String> {
    Ok(Value::Number(as_numbers(args)?.iter().product()))
}

fn numeric_div(args: &[Value]) -> Result<Value, String> {
    let nums = as_numbers(args)?;
    match nums.as_slice() {
        [] => Err("/ expects at least 1 argument".to_string()),
        [x] => Ok(Value::Number(1.0 / x)),
        [first, rest @ ..] => Ok(Value::Number(rest.iter().fold(*first, |acc, n| acc / n))),
    }
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

fn display(args: &[Value]) -> Result<Value, String> {
    match args {
        [a] => {
            println!("{}", a);
            Ok(Value::Nil)
        }
        _ => Err(format!("display expects 1 argument, {} given", args.len())),
    }
}

fn as_numbers(args: &[Value]) -> Result<Vec<f64>, String> {
    let mut f = Vec::new();
    for arg in args {
        match arg {
            Value::Number(x) => f.push(*x),
            _ => return Err(format!("expected number, received {}", arg)),
        }
    }
    Ok(f)
}

pub fn bootstrap_env() -> Arena {
    let mut a = Arena::new();
    let root = a.root();

    a.define(root, "atom?".to_string(), Value::Builtin(atom));
    a.define(root, "eq?".to_string(), Value::Builtin(eq));
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
    a.define(root, "display".to_string(), Value::Builtin(display));
    load_prelude(&mut a, root);
    a
}
