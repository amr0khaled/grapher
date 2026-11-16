# Grapher Project

**Mission:** Desmos alternative, faster + natively cross-platform.
**Stack:** Svelte + Vite front, Rust→WASM math engine (wasm-bindgen, bundled by wasm-pack), later Tauri desktop shell.
**Memory:** see `.claude/memory/` (project state, gaps, decisions).

## MVP (today)
- Sidebar: line-by-line equations (add/remove/edit).
- Main view: 2D graph area.
- Math core: y=f(x) graphing, LaTeX parse + render, interval arithmetic for implicit.
- View: pan/zoom, adaptive sample.

## Later (roadmap)
- Complex numbers, vectors/matrices to manipulate points+grid, 3D implicit, regressions, actions, animations, tables, sliders, full Desmos parity.

## Hard parts (Desmos-grade)
1. Implicit f(x,y)=0 (marching squares + interval arithmetic + adaptive refine).
2. LaTeX parser/evaluator (Rust or pre-parser).
3. Fast FFI: batch calls, typed arrays, arena alloc, no serde/json on hot path.
4. Asymptotes, sin(1/x), tan poles, domain holes, near-tangencies.
5. Incremental redraw, viewport culling, worker offload.

## Risks/gaps
- Interval arithmetic lib choice for Rust (eval vs custom).
- LaTeX parser in Rust (pulldown-cmark? mathjax-like? KaTeX bridge?).
- GPU: WebGL vs Canvas2D hybrid.
- Tauri shell later; keep wasm-bindgen portable.
- Build pipeline: vite + wasm-pack.

## Perf rules (non-negotiable)
- Rust resident data, batch crosses, reuse buffers, typed arrays, no per-field marshal.
- Avoid JSON on hot path; use flat buffers.
- Benchmark: many-tiny-calls vs 1-big-buffer vs serde path; pick per workload.

## Commands (fill when scaffolded)
- dev: `npm run dev` or `vite`
- wasm build: `wasm-pack build ...`
- test: `cargo test` + wasm tests
- lint: `cargo fmt/clippy`, svelte-check

## Files
- `src/lib/` Rust engine (parser, evaluator, implicit, sampling, FFI)
- `src/` Svelte UI (sidebar, graph, latex view)
- `public/` static assets
- `CLAUDE.md` this file
- `.claude/memory/` durable project memory (read on start, not echoed every turn)