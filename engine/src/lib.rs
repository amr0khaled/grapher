use wasm_bindgen::prelude::*;

// --- LaTeX-ish preprocessing ---
// Translates a small LaTeX subset into the token language the tokenizer already speaks.
// Runs once per parse, never on the sample() hot path.

fn find_braced_group(b: &[u8], i: usize) -> Option<(usize, usize)> {
    // b[i] == b'{'; returns (start, end_exclusive) of contents
    let mut depth = 0usize;
    for j in i..b.len() {
        match b[j] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((i + 1, j));
                }
            }
            _ => {}
        }
    }
    None
}

fn replace_command_args(
    s: &str,
    prefix: &str,
    n: usize,
    make: impl Fn(&[&str]) -> String,
) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(idx) = rest.find(prefix) {
        let mut args: Vec<&str> = Vec::with_capacity(n);
        let mut scan = idx + prefix.len();
        let mut ok = true;
        for _ in 0..n {
            while scan < rest.len() && rest.as_bytes()[scan].is_ascii_whitespace() {
                scan += 1;
            }
            if scan >= rest.len() || rest.as_bytes()[scan] != b'{' {
                ok = false;
                break;
            }
            match find_braced_group(rest.as_bytes(), scan) {
                Some((a, b2)) => {
                    args.push(&rest[a..b2]);
                    scan = b2 + 1;
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break;
        }
        out.push_str(&rest[..idx]);
        out.push_str(&make(&args));
        rest = &rest[scan..];
    }
    out.push_str(rest);
    out
}

fn preprocess_latex(s: &str) -> String {
    let mut r = s.to_string();

    // \frac{a}{b} -> ((a)/(b))
    r = replace_command_args(&r, "\\frac", 2, |args| {
        format!("(({})/({}))", args[0], args[1])
    });
    // \sqrt{x} -> sqrt(x)
    r = replace_command_args(&r, "\\sqrt", 1, |args| format!("sqrt({})", args[0]));

    // Single-arg function commands -> name(x)
    for cmd in [
        "\\sin", "\\cos", "\\tan", "\\ln", "\\exp", "\\min", "\\max", "\\abs",
    ] {
        let name = &cmd[1..];
        r = replace_command_args(&r, cmd, 1, |args| format!("{}({})", name, args[0]));
    }
    // \log{x} -> log(x)  (matches our base-10 convention)
    r = replace_command_args(&r, "\\log", 1, |args| format!("log({})", args[0]));
    // \floor{x}, \ceil{x} if present
    r = replace_command_args(&r, "\\floor", 1, |args| format!("floor({})", args[0]));
    r = replace_command_args(&r, "\\ceil", 1, |args| format!("ceil({})", args[0]));

    // Symbol replacements
    r = r
        .replace("\\pi", "pi")
        .replace("\\cdot", "*")
        .replace("\\times", "*")
        .replace("\\left(", "(")
        .replace("\\right(", ")")
        .replace("\\left[", "(")
        .replace("\\right]", ")")
        .replace("\\left|", "abs(")
        .replace("\\right|", ")")
        .replace("\\{", "(")
        .replace("\\}", ")");

    // Collapse stray whitespace around operators created by replacements
    r = r.replace('\u{00a0}', " ");
    r
}

// Implicit multiplication: 2x -> 2*x, 2( -> 2*(, )( -> )*(, )x -> )*x, x( -> x*(
// between adjacent tokens where left is Num|Var|RParen|Fn-name and right is Num|Var|LParen|Fn
fn insert_implicit_mul(tokens: Vec<Tok>) -> Vec<Tok> {
    fn starts_expr(t: &Tok) -> bool {
        match t {
            Tok::Num(_) | Tok::VarX | Tok::VarY | Tok::LParen | Tok::Fn(_) => true,
            _ => false,
        }
    }
    fn ends_expr(t: &Tok) -> bool {
        match t {
            Tok::Num(_) | Tok::VarX | Tok::VarY | Tok::RParen => true,
            _ => false,
        }
    }
    let mut out: Vec<Tok> = Vec::with_capacity(tokens.len() * 2);
    for t in tokens {
        if let Some(prev) = out.last() {
            if ends_expr(prev) && starts_expr(&t) {
                out.push(Tok::Op('*'));
            }
        }
        out.push(t);
    }
    out
}

// --- Tokenizer ---

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(f64),
    VarX,
    VarY,
    Fn(&'static str),
    Op(char),
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Result<Vec<Tok>, String> {
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
                "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sqrt" | "abs" | "floor"
                | "ceil" | "round" | "log" | "ln" | "exp" => out.push(Tok::Fn(
                    match word {
                        "sin" => "sin",
                        "cos" => "cos",
                        "tan" => "tan",
                        "asin" => "asin",
                        "acos" => "acos",
                        "atan" => "atan",
                        "sqrt" => "sqrt",
                        "abs" => "abs",
                        "floor" => "floor",
                        "ceil" => "ceil",
                        "round" => "round",
                        "log" => "log",
                        "ln" => "ln",
                        "exp" => "exp",
                        _ => unreachable!(),
                    },
                )),
                _ => return Err(format!("unknown name: {}", word)),
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

    fn parse_expr(&mut self) -> Result<Node, String> {
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

#[derive(Debug)]
enum Node {
    Num(f64),
    VarX,
    VarY,
    Neg(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Fn(&'static str, Box<Node>),
}

impl Node {
    fn eval(&self, x: f64, y: f64) -> f64 {
        match self {
            Node::Num(n) => *n,
            Node::VarX => x,
            Node::VarY => y,
            Node::Neg(a) => -a.eval(x, y),
            Node::Add(a, b) => a.eval(x, y) + b.eval(x, y),
            Node::Sub(a, b) => a.eval(x, y) - b.eval(x, y),
            Node::Mul(a, b) => a.eval(x, y) * b.eval(x, y),
            Node::Div(a, b) => {
                let d = b.eval(x, y);
                if d == 0.0 {
                    f64::NAN
                } else {
                    a.eval(x, y) / d
                }
            }
            Node::Pow(a, b) => a.eval(x, y).powf(b.eval(x, y)),
            Node::Fn(name, a) => {
                let v = a.eval(x, y);
                match *name {
                    "sin" => v.sin(),
                    "cos" => v.cos(),
                    "tan" => v.tan(),
                    "asin" => v.asin(),
                    "acos" => v.acos(),
                    "atan" => v.atan(),
                    "sqrt" => {
                        if v < 0.0 {
                            f64::NAN
                        } else {
                            v.sqrt()
                        }
                    }
                    "abs" => v.abs(),
                    "floor" => v.floor(),
                    "ceil" => v.ceil(),
                    "round" => v.round(),
                    "log" => {
                        if v <= 0.0 {
                            f64::NAN
                        } else {
                            v.log10()
                        }
                    }
                    "ln" => {
                        if v <= 0.0 {
                            f64::NAN
                        } else {
                            v.ln()
                        }
                    }
                    "exp" => v.exp(),
                    _ => f64::NAN,
                }
            }
        }
    }
}

fn parse_expr(s: &str) -> Result<Node, String> {
    let cleaned = preprocess_latex(s);
    let toks = insert_implicit_mul(tokenize(&cleaned)?);
    let mut p = Parser { toks, pos: 0 };
    let tree = p.parse_expr()?;
    if p.pos != p.toks.len() {
        return Err("trailing tokens".to_string());
    }
    Ok(tree)
}

#[wasm_bindgen]
pub fn eval(expr: &str, x: f64, y: f64) -> f64 {
    match parse_expr(expr) {
        Ok(tree) => {
            let v = tree.eval(x, y);
            if v.is_finite() { v } else { f64::NAN }
        }
        Err(_) => f64::NAN,
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn sample(expr: &str, scale: f64, offset_x: f64, offset_y: f64, width: u32, height: u32) -> Vec<f64> {
    let mut clean = expr.trim();
    let mut is_x_eq = false;

    if let Some(idx) = clean.find('=') {
        let lhs = clean[..idx].trim().to_lowercase();
        let rhs = clean[idx + 1..].trim();
        if lhs == "x" {
            is_x_eq = true;
            clean = rhs;
        } else if lhs == "y" {
            clean = rhs;
        } else {
            // implicit or general expression like x^2 + y^2 = 4 -> treat as y = rhs - lhs or similar, or parse rhs
            clean = rhs;
        }
    }

    let tree = match parse_expr(clean) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let width_f = width as f64;
    let height_f = height as f64;
    let mut out: Vec<f64> = Vec::new();

    if is_x_eq {
        // Sample along py (screen y), computing x = f(y)
        let mut py = 0.0_f64;
        while py <= height_f {
            let y = (offset_y - py) / scale;
            let x = tree.eval(0.0, y);
            if x.is_finite() {
                let px = offset_x + x * scale;
                if px.is_finite() && px >= -width_f * 2.0 && px <= width_f * 3.0 {
                    out.push(px);
                    out.push(py);
                } else {
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                }
            } else {
                out.push(f64::NAN);
                out.push(f64::NAN);
            }
            py += 1.0;
        }
    } else {
        // Sample along px (screen x), computing y = f(x)
        let mut px = 0.0_f64;
        while px <= width_f {
            let x = (px - offset_x) / scale;
            let y = tree.eval(x, 0.0);
            if y.is_finite() {
                let py = offset_y - y * scale;
                if py.is_finite() && py > -height_f * 4.0 && py < height_f * 5.0 {
                    out.push(px);
                    out.push(py);
                } else {
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                }
            } else {
                out.push(f64::NAN);
                out.push(f64::NAN);
            }
            px += 1.5;
        }
    }
    out
}
