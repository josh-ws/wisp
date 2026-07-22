use std::iter::zip;
use std::rc::Rc;

use crate::env::{Arena, Env};
use crate::reader::Sexp;
use crate::value::{Lambda, Value};

fn apply(arena: &mut Arena, f: &Value, args: &[Value]) -> Result<Value, String> {
    match f {
        Value::Builtin(f) => f(args),
        Value::Lambda(lambda) => {
            if lambda.params.len() != args.len() {
                let e = format!("expected {} args, got {}", lambda.params.len(), args.len());
                return Err(e);
            }

            let child = arena.child(lambda.env);
            for (param, arg) in zip(&lambda.params, args) {
                arena.define(child, param.clone(), arg.clone());
            }

            let mut result = Err("lambda: empty body".into());
            for form in &lambda.body {
                result = eval(arena, child, form);
                if result.is_err() {
                    break;
                }
            }
            result
        }
        other => Err(format!("symbol is not callable: {}", other)),
    }
}

fn eval_define(arena: &mut Arena, env: Env, args: &[Sexp]) -> Result<Value, String> {
    match args {
        [Sexp::Symbol(name), v] => {
            let res = eval(arena, env, v)?;
            arena.define(env, name.clone(), res.clone());
            Ok(res)
        }
        _ => Err("syntax: (define key value)".into()),
    }
}

fn eval_if(arena: &mut Arena, env: Env, args: &[Sexp]) -> Result<Value, String> {
    match args {
        [cond, then, els] => match eval(arena, env, cond)? {
            Value::Bool(false) => eval(arena, env, els),
            _ => eval(arena, env, then),
        },
        _ => Err("syntax: (if cond then else)".into()),
    }
}

pub fn eval_lambda(env: Env, args: &[Sexp]) -> Result<Value, String> {
    let (p, body) = args.split_first().ok_or("empty list")?;
    let Sexp::List(params) = p else {
        return Err("syntax: (lambda (params...) body...)".into());
    };
    if body.is_empty() {
        return Err("lambda: empty body".into());
    }
    let mut v = Vec::new();
    for param in params {
        match param {
            Sexp::Symbol(name) => v.push(name.clone()),
            _ => {
                return Err("lambda: params must be symbols".into());
            }
        }
    }
    Ok(Value::Lambda(Lambda {
        params: v,
        body: body.to_vec(),
        env,
    }))
}

fn quote(sexp: &Sexp) -> Result<Value, String> {
    match sexp {
        Sexp::Number(n) => Ok(Value::Number(*n)),
        Sexp::Bool(b) => Ok(Value::Bool(*b)),
        Sexp::Symbol(s) => Ok(Value::Symbol(s.clone())),
        Sexp::List(l) => {
            let mut acc = Value::Nil;
            for s in l.iter().rev() {
                acc = Value::cons(quote(s)?, acc);
            }
            Ok(acc)
        }
    }
}

fn eval_quote(args: &[Sexp]) -> Result<Value, String> {
    match args {
        [t] => quote(t),
        _ => Err("syntax: (quote ...)".into()),
    }
}

pub fn eval(arena: &mut Arena, env: Env, expr: &Sexp) -> Result<Value, String> {
    match expr {
        Sexp::Number(n) => Ok(Value::Number(*n)),
        Sexp::Bool(b) => Ok(Value::Bool(*b)),
        Sexp::Symbol(s) => arena.get(env, s).ok_or_else(|| format!("undefined: {}", s)),
        Sexp::List(items) => {
            let (head, args) = items.split_first().ok_or("empty list")?;

            // Special forms
            if let Sexp::Symbol(sym) = head {
                match sym.as_str() {
                    "define" => return eval_define(arena, env, args),
                    "if" => return eval_if(arena, env, args),
                    "lambda" => return eval_lambda(env, args),
                    "quote" => return eval_quote(args),
                    _ => {}
                }
            }

            let func = eval(arena, env, head)?;
            let argv: Vec<Value> = args
                .iter()
                .map(|a| eval(arena, env, a))
                .collect::<Result<_, _>>()?;
            apply(arena, &func, &argv)
        }
    }
}
