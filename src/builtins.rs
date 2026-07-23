use crate::{
    env::{Arena, Env},
    value::Value,
};

// TODO:
// Special forms: quote, cond, lambda, define, and, or
// Builtins: atom, eq?, =, car, cdr, cons, +, -, *, /
// Prelude: not
// https://leinonen.ninja/posts/building-lisp-from-the-ground-up

// Checks for argument being an atom, i.e. non-list. Single argument
fn atom(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

// Quick check for same object, no recursion.
fn eq(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
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
    Err("not implemented".to_string())
}

fn cdr(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn cons(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

// temporary, until we have the prelude
fn not(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

// also temp
fn equal(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
}

fn null(args: &[Value]) -> Result<Value, String> {
    Err("not implemented".to_string())
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
