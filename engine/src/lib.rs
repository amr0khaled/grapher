pub mod builtins;
pub mod eval;
pub mod parser;
pub mod sampling;
pub mod transforms;
pub mod utils;

pub use crate::eval::SymbolTable;
pub use crate::parser::{parse_assignment, parse_expr, parse_function_definition};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eval(expr: &str, x: f64, y: f64) -> Result<f64, JsError> {
    let res = crate::eval::eval_expr(expr, x, y);
    if res.is_nan() {
        return Err(JsError::new("Invalid expression"));
    }
    Ok(res)
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn sample(
    lines: Vec<String>,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    width: u32,
    height: u32,
) -> js_sys::Array {
    let paths = crate::sampling::sample(lines, scale, offset_x, offset_y, width, height);
    let js_array = js_sys::Array::new();
    for points in paths {
        let arr = js_sys::Float64Array::from(&points[..]);
        js_array.push(&arr);
    }
    js_array
}

#[wasm_bindgen]
pub fn get_symbol_values(lines: Vec<String>) -> js_sys::Object {
    let symbols = crate::sampling::build_symbols(&lines);
    let vals = symbols.symbol_values(0.0, 0.0);
    let obj = js_sys::Object::new();
    for (k, v) in vals {
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str(&k.to_string()),
            &JsValue::from_f64(v),
        );
    }
    obj
}

#[wasm_bindgen]
pub fn is_drawable(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    if !s.contains('=') {
        return false;
    }
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return false;
    }
    let lhs = parts[0].trim().to_lowercase();
    if lhs == "x" || lhs == "y" || lhs.contains('(') {
        return true;
    }
    if lhs.len() == 1 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_drawable() {
        assert!(is_drawable("y=x"));
        assert!(!is_drawable("a=5"));
        assert!(is_drawable("x^2+y^2=1"));
    }
}
