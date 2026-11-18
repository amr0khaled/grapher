# Grapher

**Stop waiting for your browser to compute math.**

Grapher is a high-performance, natively cross-platform Desmos alternative with advanced mathematical capabilities and signal processing operations. Built to surpass web-only alternatives through raw Rust WASM speed and a modern Svelte 5 frontend.

## ⚡ Technology Stack & Architecture
- **Frontend:** Svelte 5 runes, Vite, MathLive `<math-field>` equation editor with live LaTeX rendering.
- **Math Engine:** Rust (`wasm-bindgen`, `wasm-pack`) using zero-serde fast FFI with flat `Float64Array` buffers for batched expression evaluation.
- **Evaluation & Symbols:** Recursive-descent parser supporting assignments (`m = 5`), reactive dependent expressions (`n = m/2`), and index-aligned batch sampling.
- **Grid & View:** Adaptive 1/2/5 nice-number grid generation with anti-flash density thresholds, dynamic zoom stabilization, and auto-centering.

## 🛠 Current Features
- Sidebar equation line editing with live LaTeX rendering and dynamic line addition/removal.
- Rust-powered variable and symbol table resolution (`F = 1/T`, `y = F^x`).
- Implicit function graphing support.
- Smooth pan, mouse wheel zoom, touch pinch-zoom, and auto-centering.
- Light and dark mode support.
- Stable adaptive grid rendering with clean integer and nice-step labels.

## 🚀 Planned Features & Roadmap
- Signal processing operations (Fourier transforms, filtering, convolution, impulse responses).
- Complex numbers and vector/matrix operations.
- Sliders, tables, animations, and regression analysis.
- Full Tauri desktop shell integration.

## 💡 Vision: Beyond Basic Plotting
Standard graphing tools fall apart when you need to *process* data. Grapher is building the next-gen environment for mathematical exploration and real-time signal analysis.
