use std::collections::{HashMap, HashSet};
use crate::parser::Node;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    exprs: HashMap<char, Node>,
    funcs: HashMap<char, (Vec<char>, Node)>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            exprs: HashMap::new(),
            funcs: HashMap::new(),
        }
    }
    pub fn set_expr(&mut self, name: char, expr: Node) {
        self.exprs.insert(name, expr);
    }
    pub fn set_func(&mut self, name: char, args: Vec<char>, body: Node) {
        self.funcs.insert(name, (args, body));
    }
    pub fn eval_symbol(&self, name: char, x: f64, y: f64) -> f64 {
        let mut visited = HashSet::new();
        self.eval_rec(name, x, y, &mut visited)
    }
    pub fn symbol_values(&self, x: f64, y: f64) -> Vec<(String, f64)> {
        let mut out = Vec::new();
        for (name, _) in &self.exprs {
            let v = self.eval_symbol(*name, x, y);
            out.push((name.to_string(), v));
        }
        out
    }
    fn eval_rec(&self, name: char, x: f64, y: f64, visited: &mut HashSet<char>) -> f64 {
        if !visited.insert(name) { return f64::NAN; }
        let res = match self.exprs.get(&name) {
            Some(node) => node.eval_with_symbols_rec(x, y, self, visited),
            None => f64::NAN,
        };
        visited.remove(&name);
        res
    }
}

impl Node {
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        let st = SymbolTable::new();
        let mut visited = HashSet::new();
        self.eval_with_symbols_rec(x, y, &st, &mut visited)
    }

    pub fn eval_with_symbols(&self, x: f64, y: f64, st: &SymbolTable) -> f64 {
        let mut visited = HashSet::new();
        self.eval_with_symbols_rec(x, y, st, &mut visited)
    }

    pub fn eval_with_symbols_rec(&self, x: f64, y: f64, st: &SymbolTable, visited: &mut HashSet<char>) -> f64 {
        match self {
            Node::Num(n) => *n,
            Node::VarX => x,
            Node::VarY => y,
            Node::Var(c) => st.eval_rec(*c, x, y, visited),
            Node::Neg(a) => -a.eval_with_symbols_rec(x, y, st, visited),
            Node::Add(a, b) => a.eval_with_symbols_rec(x, y, st, visited) + b.eval_with_symbols_rec(x, y, st, visited),
            Node::Sub(a, b) => a.eval_with_symbols_rec(x, y, st, visited) - b.eval_with_symbols_rec(x, y, st, visited),
            Node::Mul(a, b) => a.eval_with_symbols_rec(x, y, st, visited) * b.eval_with_symbols_rec(x, y, st, visited),
            Node::Div(a, b) => {
                let d = b.eval_with_symbols_rec(x, y, st, visited);
                if d == 0.0 { f64::NAN } else { a.eval_with_symbols_rec(x, y, st, visited) / d }
            }
            Node::Pow(a, b) => a.eval_with_symbols_rec(x, y, st, visited).powf(b.eval_with_symbols_rec(x, y, st, visited)),
            Node::Fn(name, a) => {
                let name_char = name.chars().next().unwrap_or(' ');
                if let Some((args, body)) = st.funcs.get(&name_char) {
                    let arg_val = a.eval_with_symbols_rec(x, y, st, visited);
                    if args[0] == 'x' {
                        return body.eval_with_symbols_rec(arg_val, y, st, visited);
                    }
                    if args[0] == 'y' {
                        return body.eval_with_symbols_rec(x, arg_val, st, visited);
                    }
                    let mut sub_st = st.clone();
                    sub_st.set_expr(args[0], Node::Num(arg_val));
                    return body.eval_with_symbols_rec(x, y, &sub_st, visited);
                }
                let v = a.eval_with_symbols_rec(x, y, st, visited);
                match name.as_str() {
                    "sin" => v.sin(),
                    "cos" => v.cos(),
                    "tan" => v.tan(),
                    "sqrt" => if v < 0.0 { f64::NAN } else { v.sqrt() },
                    _ => f64::NAN,
                }
            }
        }
    }
}

pub fn eval_expr(expr: &str, x: f64, y: f64) -> f64 {
    match crate::parser::parse_expr(expr) {
        Ok(tree) => {
            let v = tree.eval(x, y);
            if v.is_finite() { v } else { f64::NAN }
        }
        Err(_) => f64::NAN,
    }
}
