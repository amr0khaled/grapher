use crate::utils::{insert_implicit_mul, preprocess_latex};

// --- Tokenizer ---

// Builtin function names containing digits; tokenizer reads [a-zA-Z]+ first,
// so these need longest-match handling or they split into name + number.
const DIGIT_BUILTINS: &[&str] = &["atan2", "log10", "log2"];

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tok {
    Num(f64),
    VarX,
    VarY,
    Var(String),
    Fn(String),
    Op(char),
    LParen,
    RParen,
    Comma,
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
            // Longest match for digit-bearing builtins: atan2, log2, log10
            let mut matched: Option<&str> = None;
            for name in DIGIT_BUILTINS {
                let nb = name.as_bytes();
                if b.len() - i >= nb.len() && b[i..i + nb.len()].eq_ignore_ascii_case(nb) {
                    let end = i + nb.len();
                    // require word boundary after digit-bearing builtin name
                    if end < b.len() && (b[end].is_ascii_alphanumeric() || b[end] == b'_') {
                        continue;
                    }
                    let mut j = end;
                    while j < b.len() && b[j].is_ascii_whitespace() { j += 1; }
                    if (j < b.len() && b[j] == b'(') || crate::builtins::is_one_arg(name) {
                        matched = Some(*name);
                        break;
                    }
                }
            }
            if let Some(name) = matched {
                let mut j = i + name.len();
                while j < b.len() && b[j].is_ascii_whitespace() { j += 1; }
                out.push(Tok::Fn(name.to_string()));
                i = j;
                continue;
            }
            let start = i;
            while i < b.len() && b[i].is_ascii_alphabetic() {
                i += 1;
            }
            // optional _subscript: word or word_digits (e.g. x_2, a_t) -> one symbol
            if i < b.len() && b[i] == b'_'
                && i + 1 < b.len()
                && (b[i + 1].is_ascii_alphanumeric() || b[i + 1] == b'_')
            {
                i += 1;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
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
                    if (j < b.len() && b[j] == b'(') || crate::builtins::is_one_arg(word) {
                        out.push(Tok::Fn(word.to_string()));
                    } else {
                        out.push(Tok::Var(word.to_string()));
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
        } else if c == b',' {
            out.push(Tok::Comma);
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
                if let Some(Tok::LParen) = self.peek() {
                    self.bump();
                    let mut args = Vec::new();
                    if let Some(Tok::RParen) = self.peek() {
                        return Err(format!("expected argument for fn {}", name));
                    }
                    loop {
                        args.push(self.parse_expr()?);
                        match self.peek() {
                            Some(Tok::Comma) => {
                                self.bump();
                            }
                            Some(Tok::RParen) => {
                                self.bump();
                                break;
                            }
                            _ => return Err(format!("expected , or ) in fn {}", name)),
                        }
                    }
                    Ok(Node::Fn(name, args))
                } else {
                    let arg = self.parse_factor()?;
                    Ok(Node::Fn(name, vec![arg]))
                }
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
    Var(String),
    Neg(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Fn(String, Vec<Node>),
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

pub fn parse_function_definition(s: &str) -> Result<(String, String, Node), String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 {
        return Err("invalid function definition".to_string());
    }
    let lhs = parts[0].trim();
    let inner = lhs.strip_suffix(')').ok_or("expected )")?;
    let open = inner.find('(').ok_or("expected (")?;
    let name = inner[..open].trim();
    let arg = inner[open + 1..].trim();
    if name.is_empty() {
        return Err("function name must not be empty".to_string());
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("function name must be alphanumeric".to_string());
    }
    if arg.is_empty() {
        return Err("function arg must not be empty".to_string());
    }
    if !arg
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("function arg must be alphanumeric".to_string());
    }
    let body = parse_expr(parts[1])?;
    Ok((name.to_string(), arg.to_string(), body))
}

pub fn parse_assignment(s: &str) -> Result<(String, Node), String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 {
        return Err("invalid assignment".to_string());
    }
    let name = parts[0].trim();
    if name.is_empty() {
        return Err("invalid symbol name".to_string());
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("symbol name must be alphanumeric (or _)".to_string());
    }
    let node = parse_expr(parts[1])?;
    Ok((name.to_string(), node))
}
