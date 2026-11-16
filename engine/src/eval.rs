use std::collections::HashMap;

use crate::parser::Node;

impl Node {
    pub(crate) fn eval(&self, x: f64, y: f64) -> f64 {
        match self {
            Node::Num(n) => *n,
            Node::VarX => x,
            Node::VarY => y,
            Node::Var(c) => 1.0,
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

pub(crate) fn eval_expr(expr: &str, x: f64, y: f64) -> f64 {
    match crate::parser::parse_expr(expr) {
        Ok(tree) => {
            let v = tree.eval(x, y);
            if v.is_finite() {
                v
            } else {
                f64::NAN
            }
        }
        Err(_) => f64::NAN,
    }
}
