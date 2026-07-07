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

pub fn eval(env: &mut Env, expr: &Sexp) -> Result<Value, String> {
    match expr {
        Sexp::Atom(a) => eval_atom(env, a),
        Sexp::List(items) => {
            let (head, args) = items.split_first().ok_or("empty list")?;
            let func = eval(env, head)?;
            let argv: Vec<Value> = args
                .iter()
                .map(|a| eval(env, a))
                .collect::<Result<_, _>>()?;
            apply(&func, &argv)
        }
    }
}
