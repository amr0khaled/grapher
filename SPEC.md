# Grapher Project Specification (SPEC.md)

**Mission:** High-performance, natively cross-platform Desmos alternative.

## Architecture & Stack
- **Frontend:** Svelte 5 (runes `$state`, `$derived`, `$props`), Vite, MathLive `<math-field>` editor.
- **Math Engine:** Rust (`wasm-bindgen`, `wasm-pack`), compiled to WASM.
- **FFI & Evaluation:** Index-aligned batch sampling (`Vec<Vec<f64>>` -> `Float64Array[]`) with zero serde/json overhead on hot sampling loop; symbol values exported via `get_symbol_values` for reactive badge display.
- **Parsing & Symbols:** Pure Rust recursive-descent parser and symbol table supporting assignments (`m = 5`), reactive dependent symbols, implicit multiplication, multi-arg and signal built-ins, and LaTeX-to-infix sanitization (`toPlainInfix`). Legacy TypeScript symbol table is superseded by engine WASM FFI.
- **View & Grid:** Graph canvas with pan/zoom state (`scale`, `offsetX`, `offsetY`), discrete 1/2/5 nice steps (`targetPx = 80`), conditional minor sub-grid rendering, and stable major grid thresholds preventing label jumping and DOM flashing.

## Current Status (MVP Completed + Symbol Migration + Canvas & Responsive Roadmap)
- Sidebar equation input with MathLive and evaluated value badges.
- Rust WASM math engine handling expression evaluation, symbol tables (`get_symbol_values`), and batch sampling.
- LaTeX translation (`toLatex` and `toPlainInfix`).
- Stable adaptive grid with 1/2/5 nice-number sequences and anti-flash thresholds.
- Active plan: HTML5 Canvas renderer with DPI backing store, debounced resampling with CSS stretch transform, tablet/phone responsive drawer, and full WASM symbol badge wiring.
