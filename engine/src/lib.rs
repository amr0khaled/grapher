pub mod eval;
pub mod parser;
pub mod sampling;
pub mod utils;
pub mod builtins;

pub use crate::eval::SymbolTable;
pub use crate::parser::{parse_assignment, parse_expr, parse_function_definition};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eval(expr: &str, x: f64, y: f64) -> f64 {
    crate::eval::eval_expr(expr, x, y)
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
pub fn get_symbol_values(_lines: Vec<String>) -> js_sys::Object {
    let symbols = SymbolTable::new();
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
