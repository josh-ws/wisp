#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    Quote,
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
            '\'' => {
                chars.next();
                tokens.push(Token::Quote);
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
pub enum Sexp {
    Number(f64),
    Bool(bool),
    Symbol(String),
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
            Token::Atom(a) => Ok(classify(a)),
            Token::Quote => {
                let inner = self.parse()?;
                Ok(Sexp::List(vec![Sexp::Symbol("quote".to_string()), inner]))
            }
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

fn classify(a: String) -> Sexp {
    match a.as_str() {
        "#t" => Sexp::Bool(true),
        "#f" => Sexp::Bool(false),
        _ => match a.parse::<f64>() {
            Ok(n) => Sexp::Number(n),
            Err(_) => Sexp::Symbol(a),
        },
    }
}

pub fn read(src: &str) -> Result<Vec<Sexp>, String> {
    let tokens = lex(src);
    let mut parser = Parser::new(tokens);
    let mut exprs = Vec::new();
    while parser.peek().is_some() {
        exprs.push(parser.parse()?);
    }
    Ok(exprs)
}
