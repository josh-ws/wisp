#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    Atom(String),
}

fn lex(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            _ => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || "()".contains(c) {
                        break;
                    }
                    s.push(c);
                    chars.next();
                }
                tokens.push(Token::Atom(s));
            }
        }
    }
    tokens
}

#[derive(Debug, Clone)]
enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    fn parse(&mut self) -> Result<Sexp, String> {
        let tok = self.next().ok_or("unexpected eof")?;
        match tok {
            Token::LParen => self.parse_list(),
            Token::RParen => Err("unexpected ')'".to_string()),
            Token::Atom(a) => Ok(Sexp::Atom(a)),
        }
    }

    fn parse_list(&mut self) -> Result<Sexp, String> {
        let mut items = Vec::new();
        loop {
            match self.peek() {
                Some(Token::RParen) => {
                    self.next();
                    return Ok(Sexp::List(items));
                }
                None => return Err("unexpected eof".to_string()),
                _ => items.push(self.parse()?),
            }
        }
    }
}

type Builtin = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug)]
struct Lambda {
    params: Vec<String>,
    body: Vec<Sexp>,
}

#[derive(Debug)]
enum Value {
    Number(f64),
    Bool(bool),
    Builtin(Builtin),
    Lambda(Lambda),
}

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

fn eval(expr: &Sexp) -> Result<Value, String> {
    match expr {
        Sexp::Atom(a) => eval_atom(a),
        Sexp::List(_) => todo!(),
    }
}

fn read(src: &str) -> Result<Vec<Sexp>, String> {
    let tokens = lex(src);
    let mut parser = Parser::new(tokens);
    let mut exprs = Vec::new();
    while parser.peek().is_some() {
        exprs.push(parser.parse()?);
    }
    Ok(exprs)
}

fn main() {
    // let src = "(define (square x) (* x x)) (print (square 5))";

    for src in ["5", "#t", "#f", "no"] {
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
