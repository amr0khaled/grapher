use crate::utils::{insert_implicit_mul, preprocess_latex};

// --- Tokenizer ---

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tok {
    Num(f64),
    VarX,
    VarY,
    Var(char),
    Fn(String),
    Op(char),
    LParen,
    RParen,
}

pub(crate) fn tokenize(s: &str) -> Result<Vec<Tok>, String> {
    let b = s.as_bytes();
    let mut i = 0;
    let mut out = Vec::new();
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || c == b'.' {
            let start = i;
            while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.') {
                i += 1;
            }
            if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
                let mut j = i + 1;
                if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
                    j += 1;
                }
                if j < b.len() && b[j].is_ascii_digit() {
                    while j < b.len() && b[j].is_ascii_digit() {
                        j += 1;
                    }
                    i = j;
                }
            }
            let n: f64 = std::str::from_utf8(&b[start..i])
                .unwrap()
                .parse()
                .map_err(|_| "bad number".to_string())?;
            out.push(Tok::Num(n));
        } else if c.is_ascii_alphabetic() {
            let start = i;
            while i < b.len() && b[i].is_ascii_alphabetic() {
                i += 1;
            }
let word = std::str::from_utf8(&b[start..i]).unwrap();
            match word {
                "x" | "X" => out.push(Tok::VarX),
                "y" | "Y" => out.push(Tok::VarY),
                "pi" | "PI" | "π" => out.push(Tok::Num(std::f64::consts::PI)),
                "e" | "E" => out.push(Tok::Num(std::f64::consts::E)),
                _ => {
                    let mut j = i;
                    while j < b.len() && b[j].is_ascii_whitespace() {
                        j += 1;
                    }
                    if j < b.len() && b[j] == b'(' {
                        out.push(Tok::Fn(word.to_string()));
                    } else {
                        word.chars().for_each(|e| out.push(Tok::Var(e)));
                    }
                }
            }
        } else if c == b'+' || c == b'-' || c == b'*' || c == b'/' || c == b'^' {
            out.push(Tok::Op(c as char));
            i += 1;
        } else if c == b'(' {
            out.push(Tok::LParen);
            i += 1;
        } else if c == b')' {
            out.push(Tok::RParen);
            i += 1;
        } else {
            return Err(format!("bad char: {}", c as char));
        }
    }
    Ok(out)
}

// --- Parser ---

struct Parser {
    toks: Vec<Tok>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }
    fn bump(&mut self) -> Option<Tok> {
        let t = self.toks.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Node, String> {
        let mut left = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Tok::Op('+')) => {
                    self.bump();
                    let right = self.parse_term()?;
                    left = Node::Add(Box::new(left), Box::new(right));
                }
                Some(Tok::Op('-')) => {
                    self.bump();
                    let right = self.parse_term()?;
                    left = Node::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Node, String> {
        let mut left = self.parse_factor()?;
        loop {
            match self.peek() {
                Some(Tok::Op('*')) => {
                    self.bump();
                    let right = self.parse_factor()?;
                    left = Node::Mul(Box::new(left), Box::new(right));
                }
                Some(Tok::Op('/')) => {
                    self.bump();
                    let right = self.parse_factor()?;
                    left = Node::Div(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Node, String> {
        let left = self.parse_unary()?;
        if let Some(Tok::Op('^')) = self.peek() {
            self.bump();
            let right = self.parse_factor()?;
            return Ok(Node::Pow(Box::new(left), Box::new(right)));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, String> {
        if let Some(Tok::Op('-')) = self.peek() {
            self.bump();
            let inner = self.parse_unary()?;
            return Ok(Node::Neg(Box::new(inner)));
        }
        if let Some(Tok::Op('+')) = self.peek() {
            self.bump();
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Node, String> {
        match self.bump() {
            Some(Tok::Num(n)) => Ok(Node::Num(n)),
            Some(Tok::VarX) => Ok(Node::VarX),
            Some(Tok::VarY) => Ok(Node::VarY),
            Some(Tok::Var(c)) => Ok(Node::Var(c)),
            Some(Tok::Fn(name)) => {
                match self.bump() {
                    Some(Tok::LParen) => {}
                    _ => return Err(format!("expected ( after fn {}", name)),
                }
                let arg = self.parse_expr()?;
                match self.bump() {
                    Some(Tok::RParen) => {}
                    _ => return Err(format!("expected ) after fn {}", name)),
                }
                Ok(Node::Fn(name, Box::new(arg)))
            }
            Some(Tok::LParen) => {
                let inner = self.parse_expr()?;
                match self.bump() {
                    Some(Tok::RParen) => Ok(inner),
                    _ => Err("expected )".to_string()),
                }
            }
            _ => Err("unexpected token".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Num(f64),
    VarX,
    VarY,
    Var(char),
    Neg(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Fn(String, Box<Node>),
}

pub fn parse_expr(s: &str) -> Result<Node, String> {
    let cleaned = preprocess_latex(s);
    let toks = insert_implicit_mul(tokenize(&cleaned)?);
    let mut p = Parser { toks, pos: 0 };
    let tree = p.parse_expr()?;
    if p.pos != p.toks.len() {
        return Err("trailing tokens".to_string());
    }
    Ok(tree)
}

pub fn parse_assignment(s: &str) -> Result<(char, Node), String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 { return Err("invalid assignment".to_string()); }
    let name = parts[0].trim();
    if name.len() != 1 { return Err("invalid symbol name".to_string()); }
    let c = name.chars().next().unwrap();
    let node = parse_expr(parts[1])?;
    Ok((c, node))
}
