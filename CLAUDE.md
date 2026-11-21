# Grapher Project

**Mission:** Desmos alternative, fast, natively cross-platform.
**Stack:** Svelte + Vite, Rust→WASM (wasm-bindgen, wasm-pack), Tauri later.
**Memory:** see `.claude/memory/` (state, gaps, decisions).

## Commands
- dev: `bun run dev`
- build: `bun run build`
- preview: `bun run preview`
- check: `bun run check`
- wasm build: `cargo build --release` in `engine/` + copy `engine/target/release/*.wasm` to `src/wasm/`

## MVP (done)
- Sidebar: equation list, `src/components/EquationList.svelte`, `src/components/EquationInput.svelte`.
- Main view: 2D graph, `src/components/GraphCanvas.svelte`.
- Math core: `y=f(x)`, LaTeX parse + render via MathLive `<math-field>`.
- View: pan/zoom, adaptive sample.
- Symbols: native Rust eval (`m = 5`, `n = m/2`) handled natively in Rust (`engine/src/eval.rs`, `engine/src/sampling.rs`, WASM `get_symbol_values`).
- Sampling: `sampleAllToPaths`, flat Float64Arrays.
- LaTeX: `toPlainInfix` (`src/utils/latex.ts`), support `F^x`.
- Grid: 1/2/5 generation, 80px target, anti-flash.
- Badges: evaluated value badge bottom-right of equation rows when unambiguous (wired to WASM `get_symbol_values`).
- UX: Enter (new line), Arrow (navigate).

## Later (roadmap)
- Complex, vectors/matrices, 3D implicit, regressions, actions, animations, tables, sliders, Desmos parity.

## Hard parts (Desmos-grade)
1. Implicit f(x,y)=0 (marching squares, interval arithmetic, adaptive refine).
2. LaTeX parser/evaluator.
3. Fast FFI: batch, typed arrays, arena alloc, no serde/json.
4. Asymptotes, sin(1/x), tan poles, domain holes, near-tangencies.
5. Incremental redraw, viewport culling, worker offload.

## Current architecture
- `src/App.svelte`: layout shell, view state, pan/zoom, `engineReady`.
- `src/layouts/Sidebar.svelte`: equation list render.
- `src/components/EquationInput.svelte`: MathLive `<math-field>` editor, `onInput` edits sync `value` via `$bindable`, `onMount` attaches field, `enter` handler triggers `onAddLine`.
- `src/components/GraphCanvas.svelte`: SVG grid/axes/paths, samples each expression via `sampleToPath` → Rust WASM `sample()`.
- `src/utils/math.ts`: `sampleToPath` (WASM sample → SVG path `M`/`L`, NaN breaks segments), `generateGridLines`, `evaluateSymbols` (wraps WASM `get_symbol_values`).
- `src/utils/symbols.ts`: [Deprecated] legacy TS symbol evaluator superseded by Rust engine WASM `get_symbol_values`.
- `src/utils/latex.ts`: `toLatex` (plain→KaTeX: `^`→`^{}`, `/`→`\frac`, `sqrt`/`sum`/`int`/`infinity`, `*`→`\cdot`).
- `src/wasm/`: prebuilt Rust WASM (`grapher_engine_bg.js` + `.wasm`).
- `engine/src/lib.rs`: Rust math engine (tokenizer, recursive descent, `sqrt`/`sin`/`cos`/`tan`/`pi`, `sample()` FFI).
- `src/types.ts`: `ExpressionLine`, `GridLine`.

## Git workflow
- `main`: release.
- Features: `git checkout -b feature/<name>` → PR/merge to `main`.