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
    let src = "(define (square x) (* x x)) (print (square 5))";

    println!("{}\n", src);

    match read(src) {
        Err(e) => eprintln!("parse error: {}", e),
        Ok(s) => {
            for expr in s {
                println!("{:?}", expr)
            }
        }
    }
}
