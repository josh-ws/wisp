use crate::env::Env;
use crate::reader::Sexp;
use crate::value::{Lambda, Value};

fn apply(env: &mut Env, f: &Value, args: &[Value]) -> Result<Value, String> {
    match f {
        Value::Builtin(f) => f(args),
        Value::Lambda(lambda) => {
            if lambda.params.len() != args.len() {
                let e = format!("expected {} args, got {}", lambda.params.len(), args.len());
                return Err(e);
            }

            let mut saved: Vec<(String, Option<Value>)> = Vec::new();
            for (p, arg) in lambda.params.iter().zip(args) {
                let prev = env.get(p).cloned();
                env.define(p.clone(), arg.clone());
                saved.push((p.clone(), prev));
            }

            let mut result = Err("lambda: empty body".into());
            for form in &lambda.body {
                result = eval(env, form);
                if result.is_err() {
                    break;
                }
            }

            for (param, prev) in saved {
                match prev {
                    Some(v) => env.define(param, v),
                    None => {
                        env.remove(&param);
                    }
                }
            }

            result
        }
        other => Err(format!("symbol is not callable: {}", other)),
    }
}

fn eval_define(env: &mut Env, args: &[Sexp]) -> Result<Value, String> {
    match args {
        [Sexp::Symbol(name), v] => {
            let res = eval(env, v)?;
            env.define(name.clone(), res.clone());
            Ok(res)
        }
        _ => Err("syntax: (define key value)".into()),
    }
}

fn eval_if(env: &mut Env, args: &[Sexp]) -> Result<Value, String> {
    match args {
        [cond, then, els] => match eval(env, cond)? {
            Value::Bool(false) => eval(env, els),
            _ => eval(env, then),
        },
        _ => Err("syntax: (if cond then else)".into()),
    }
}

pub fn eval_lambda(args: &[Sexp]) -> Result<Value, String> {
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
    }))
}

fn quote(sexp: &Sexp) -> Result<Value, String> {
    match sexp {
        Sexp::Number(n) => Ok(Value::Number(*n)),
        Sexp::Bool(b) => Ok(Value::Bool(*b)),
        Sexp::Symbol(s) => Ok(Value::Symbol(s.clone())),
        Sexp::List(l) => {
            let mut vals = Vec::new();
            for s in l {
                vals.push(quote(s)?);
            }
            Ok(Value::List(vals))
        }
    }
}

fn eval_quote(args: &[Sexp]) -> Result<Value, String> {
    match args {
        [t] => quote(t),
        _ => Err("syntax: (quote ...)".into()),
    }
}

pub fn eval(env: &mut Env, expr: &Sexp) -> Result<Value, String> {
    match expr {
        Sexp::Number(n) => Ok(Value::Number(*n)),
        Sexp::Bool(b) => Ok(Value::Bool(*b)),
        Sexp::Symbol(s) => env
            .get(s)
            .cloned()
            .ok_or_else(|| format!("undefined: {}", s)),
        Sexp::List(items) => {
            let (head, args) = items.split_first().ok_or("empty list")?;

            // Special forms
            if let Sexp::Symbol(sym) = head {
                match sym.as_str() {
                    "define" => return eval_define(env, args),
                    "if" => return eval_if(env, args),
                    "lambda" => return eval_lambda(args),
                    "quote" => return eval_quote(args),
                    _ => {}
                }
            }

            let func = eval(env, head)?;
            let argv: Vec<Value> = args
                .iter()
                .map(|a| eval(env, a))
                .collect::<Result<_, _>>()?;
            apply(env, &func, &argv)
        }
    }
}
