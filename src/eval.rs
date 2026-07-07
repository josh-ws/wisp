use crate::reader::Sexp;
use crate::value::Value;

const TRUE_TOKEN: &str = "#t";
const FALSE_TOKEN: &str = "#f";

fn eval_atom(atom: &str) -> Result<Value, String> {
    match atom {
        TRUE_TOKEN => Ok(Value::Bool(true)),
        FALSE_TOKEN => Ok(Value::Bool(false)),
        _ => atom
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|_| format!("cannot eval {}", atom)),
    }
}

fn apply(op: &str, args: &[Value]) -> Result<Value, String> {
    match op {
        "+" => {
            let mut sum = 0.0;
            for arg in args {
                match arg {
                    Value::Number(n) => sum += n,
                    other => return Err(format!("invalid symbol {:?}", other)),
                }
            }
            Ok(Value::Number(sum))
        }
        _ => Err(format!("unsupported: {}", op)),
    }
}

pub fn eval(expr: &Sexp) -> Result<Value, String> {
    match expr {
        Sexp::Atom(a) => eval_atom(a),
        Sexp::List(items) => {
            let (head, args) = items.split_first().ok_or("empty list")?;
            let argv: Vec<Value> = args.iter().map(eval).collect::<Result<_, _>>()?;

            match head {
                Sexp::Atom(a) => apply(a, &argv),
                Sexp::List(_) => Err(format!("cannot call: {:?}", head)),
            }
        }
    }
}
