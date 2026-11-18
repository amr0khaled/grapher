# Grapher

High-performance, natively cross-platform Desmos alternative with advanced mathematical capabilities and signal processing operations.

## Mission
Create a blazing-fast graphing calculator surpassing web-only alternatives through a Rust WebAssembly (WASM) math engine and modern Svelte 5 frontend, designed for seamless cross-platform execution (Web and Desktop via Tauri).

## Specification (Nutshell)
- **Frontend:** Svelte 5 runes, Vite, MathLive `<math-field>` equation editor with LaTeX rendering.
- **Math Engine:** Rust (`wasm-bindgen`, `wasm-pack`) using zero-serde fast FFI with flat `Float64Array` buffers.
- **Evaluation & Symbols:** Recursive-descent parser supporting assignments (`m = 5`), reactive dependent expressions (`n = m/2`), and implicit function sampling.
- **Grid & View:** Adaptive 1/2/5 nice-number grid generation with anti-flash density thresholds and zoom stabilization.

## Current Features
- Sidebar equation list with live LaTeX rendering and dynamic line addition/removal.
- Rust-powered variable and symbol table resolution (`F = 1/T`, `y = F^x`).
- Index-aligned batch expression sampling.
- Smooth pan, mouse wheel zoom, touch pinch-zoom, and auto-centering.
- Light and dark mode support.
- Stable adaptive grid rendering with clean integer and nice-step labels.

## Planned Features & Roadmap
- Signal processing operations (Fourier transforms, filtering, convolution, impulse responses).
- Implicit function graphing ($f(x,y)=0$ via marching squares).
- Complex numbers and vector/matrix operations.
- Sliders, tables, animations, and regression analysis.
- Full Tauri desktop shell integration.
