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
    fn lookup_func(&self, name: char) -> Option<(Vec<char>, Node)> {
        self.funcs.get(&name).cloned()
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
            Node::Fn(name, args) => {
                let name_char = name.chars().next().unwrap_or(' ');
                if let Some((f_args, body)) = st.funcs.get(&name_char) {
                    let arg_vals: Vec<f64> = args
                        .iter()
                        .map(|a| a.eval_with_symbols_rec(x, y, st, visited))
                        .collect();
                    // Params named 'x'/'y' bind to body's positional vars; others via symbol table.
                    let mut body_x = x;
                    let mut body_y = y;
                    let mut sub_st = st.clone();
                    for (i, (f_arg, val)) in f_args.iter().zip(arg_vals.iter()).enumerate() {
                        if *f_arg == 'x' {
                            body_x = *val;
                        } else if *f_arg == 'y' {
                            body_y = *val;
                        } else {
                            sub_st.set_expr(*f_arg, Node::Num(*val));
                        }
                    }
                    return body.eval_with_symbols_rec(body_x, body_y, &sub_st, visited);
                }
                let arg_vals: Vec<f64> = args.iter().map(|a| a.eval_with_symbols_rec(x, y, st, visited)).collect();
                crate::builtins::eval_scalar(name.as_str(), &arg_vals).unwrap_or(f64::NAN)
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
