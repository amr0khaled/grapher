# Grapher Project Specification (SPEC.md)

**Mission:** High-performance, natively cross-platform Desmos alternative.

## Architecture & Stack
- **Frontend:** Svelte 5 (runes `$state`, `$derived`, `$props`), Vite, MathLive `<math-field>` editor.
- **Math Engine:** Rust (`wasm-bindgen`, `wasm-pack`), compiled to WASM.
- **FFI / Sampling:** Index-aligned batch sampling (`Vec<Vec<f64>>` -> `Float64Array[]`), zero serde/json overhead on hot sampling loop.
- **Parsing & Symbols:** Recursive-descent parser in Rust supporting assignments (`m = 5`), reactive dependent symbols, implicit multiplication, and LaTeX-to-infix sanitization (`toPlainInfix`).
- **View & Grid:** SVG graph canvas with pan/zoom state (`scale`, `offsetX`, `offsetY`), discrete 1/2/5 nice steps (`targetPx = 80`), conditional minor sub-grid rendering, and stable major grid thresholds preventing label jumping and DOM flashing.

## Current Status (MVP Completed + Symbol Migration)
- Sidebar equation input with MathLive.
- Rust WASM math engine handling expression evaluation, symbol tables, and batch sampling.
- LaTeX translation (`toLatex` and `toPlainInfix`).
- Stable adaptive grid with 1/2/5 nice-number sequences and anti-flash thresholds.
