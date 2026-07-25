use std::io::{self, BufRead, Write};

use wisp::builtins::bootstrap_env;
use wisp::eval::eval;
use wisp::reader::read;

fn main() {
    let mut arena = bootstrap_env();
    let root = arena.root();

    let mut lines = io::stdin().lock().lines();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match lines.next() {
            Some(Ok(line)) => match read(&line) {
                Ok(exprs) => {
                    for expr in exprs {
                        match eval(&mut arena, root, &expr) {
                            Ok(v) => println!("{}", v),
                            Err(e) => eprintln!("error: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("parse error: {}", e),
            },
            Some(Err(e)) => {
                eprintln!("input error: {}", e);
                break;
            }
            None => break,
        }
    }
}
