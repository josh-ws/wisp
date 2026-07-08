use crate::env::Env;
use crate::reader::Sexp;
use crate::value::Value;

const TRUE_TOKEN: &str = "#t";
const FALSE_TOKEN: &str = "#f";

fn apply(f: &Value, args: &[Value]) -> Result<Value, String> {
    match f {
        Value::Builtin(f) => f(args),
        other => Err(format!("symbol is not callable: {:?}", other)),
    }
}

fn eval_atom(env: &Env, atom: &str) -> Result<Value, String> {
    match atom {
        TRUE_TOKEN => Ok(Value::Bool(true)),
        FALSE_TOKEN => Ok(Value::Bool(false)),
        _ => match atom.parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(_) => env
                .get(atom)
                .cloned()
                .ok_or_else(|| format!("undefined: {}", atom)),
        },
    }
}

fn eval_define(env: &mut Env, args: &[Sexp]) -> Result<Value, String> {
    match args {
        [Sexp::Atom(name), v] => {
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

pub fn eval(env: &mut Env, expr: &Sexp) -> Result<Value, String> {
    match expr {
        Sexp::Atom(a) => eval_atom(env, a),
        Sexp::List(items) => {
            let (head, args) = items.split_first().ok_or("empty list")?;

            // Special forms
            if let Sexp::Atom(sym) = head {
                match sym.as_str() {
                    "define" => return eval_define(env, args),
                    "if" => return eval_if(env, args),
                    _ => {}
                }
            }

            let func = eval(env, head)?;
            let argv: Vec<Value> = args
                .iter()
                .map(|a| eval(env, a))
                .collect::<Result<_, _>>()?;
            apply(&func, &argv)
        }
    }
}
