mod builtins;
mod env;
mod eval;
mod reader;
mod value;

use crate::builtins::bootstrap_env;
use crate::env::Env;
use crate::eval::eval;
use crate::reader::read;

fn main() {
    // let src = "(define (square x) (* x x)) (print (square 5))";
    let mut env = bootstrap_env();

    for src in ["5", "#t", "#f", "no", "(+ 1 2)", "(+ 1 (+ 2 3))"] {
        match read(src) {
            Ok(e) => {
                for expr in e {
                    println!("{} => {:?}", src, eval(&mut env, &expr));
                }
            }
            Err(err) => eprintln!("parse err: {}", err),
        }
    }
}
