# Grapher

**Stop waiting for your browser to compute math.**

Grapher is a high-performance, natively cross-platform graphing engine designed to replace clunky web calculators with raw Rust speed. Whether you're mapping functions or performing real-time signal analysis, Grapher gets out of your way.

---

## ⚡ The Stack
* **Engine:** Pure Rust compiled to WASM. Batch processing, zero-serialization FFI, native symbol resolution.
* **Interface:** Svelte 5 (Runes) + MathLive. LaTeX input that actually feels snappy.
* **Adaptive UX:** Infinite zoom with stable, flicker-free grids using nice-number dynamic scaling.

## 🛠 Capabilities
* **Current:** 2D function plotting, real-time symbol table evaluation (`F = 1/T`, `y = F^x`), buttery pan/zoom, implicit function sampling.
* **Roadmap:** Signal processing operations (Fourier transforms, convolution, impulse responses), complex matrix algebra, desktop integration via Tauri.

## 🚀 Vision: Beyond Basic Plotting
Standard graphing tools fall apart when you need to *process* data. We are building the next-gen environment for mathematical exploration and signal analysis.
