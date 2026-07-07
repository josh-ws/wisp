mod eval;
mod reader;
mod value;

use crate::eval::eval;
use crate::reader::read;

fn main() {
    // let src = "(define (square x) (* x x)) (print (square 5))";

    for src in ["5", "#t", "#f", "no", "(+ 1 2)", "(+ 1 (+ 2 3))"] {
        match read(src) {
            Ok(e) => {
                for expr in e {
                    println!("{} => {:?}", src, eval(&expr));
                }
            }
            Err(err) => eprintln!("parse err: {}", err),
        }
    }
}
