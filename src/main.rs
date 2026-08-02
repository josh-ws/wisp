use std::fs;
use std::io::{self, BufRead, Write};

use wisp::builtins::bootstrap_env;
use wisp::env::{Arena, Env};
use wisp::eval::eval;
use wisp::reader::read;

fn main() {
    let mut arena = bootstrap_env();
    let root = arena.root();

    let files: Vec<String> = std::env::args().skip(1).collect();
    if files.is_empty() {
        repl(&mut arena, root)
    } else {
        for file in &files {
            match run_file(&mut arena, root, file) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("error in {}: {}", file, err);
                    break;
                }
            }
        }
    }
}

fn run_file(arena: &mut Arena, root: Env, path: &str) -> Result<(), String> {
    let src = match fs::read_to_string(path) {
        Ok(src) => src,
        Err(e) => return Err(format!("could not open {}: {}", path, e)),
    };

    let exprs = match read(&src) {
        Ok(exprs) => exprs,
        Err(e) => return Err(format!("parse error in {}: {}", path, e)),
    };

    for expr in &exprs {
        match eval(arena, root, expr) {
            Ok(v) => println!("{}", v),
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

fn repl(arena: &mut Arena, root: Env) {
    let mut lines = io::stdin().lock().lines();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match lines.next() {
            Some(Ok(line)) => match read(&line) {
                Ok(exprs) => {
                    for expr in exprs {
                        match eval(arena, root, &expr) {
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
