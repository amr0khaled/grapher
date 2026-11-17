pub mod parser;
pub mod eval;
mod sampling;
mod utils;

pub use crate::eval::{SymbolTable, eval_expr};
pub use crate::parser::{parse_assignment, parse_expr};

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
pub fn sample(expr: &str, scale: f64, offset_x: f64, offset_y: f64, width: u32, height: u32) -> Vec<f64> {
    crate::sampling::sample(expr, scale, offset_x, offset_y, width, height)
}
