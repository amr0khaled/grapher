# Grapher Project

**Mission:** Desmos alternative, faster + natively cross-platform.
**Stack:** Svelte + Vite front, Rust→WASM math engine (wasm-bindgen, bundled by wasm-pack), later Tauri desktop shell.
**Memory:** see `.claude/memory/` (project state, gaps, decisions).

## Commands
- dev: `bun run dev` (vite)
- build: `bun run build` (vite)
- preview: `bun run preview`
- check: `bun run check` (svelte-check)
- wasm build: `cargo build --release` in `engine/` + copy `engine/target/release/*.wasm` to `src/wasm/` (WASM prebuilt)

## MVP (done)
- Sidebar: line-by-line equations (add/remove/edit) via `src/components/EquationList.svelte` + `src/components/EquationInput.svelte`.
- Main view: 2D graph area via `src/components/GraphCanvas.svelte`.
- Math core: `y=f(x)` graphing with LaTeX parse + render via MathLive `<math-field>`.
- View: pan/zoom, adaptive sample.
- Symbol assignment & reactive dependent evaluation (`m = 5`, `n = m/2`) via `src/utils/symbols.ts` (`evalAssignments`, `evalExpr`, `parseAssignment`).
- Evaluated value badge bottom-right of equation rows when unambiguous.
- Enter-key new line creation + focus; Arrow Up/Down line navigation.

## Later (roadmap)
- Complex numbers, vectors/matrices, 3D implicit, regressions, actions, animations, tables, sliders, full Desmos parity.

## Hard parts (Desmos-grade)
1. Implicit f(x,y)=0 (marching squares + interval arithmetic + adaptive refine).
2. LaTeX parser/evaluator (Rust or pre-parser).
3. Fast FFI: batch calls, typed arrays, arena alloc, no serde/json on hot path.
4. Asymptotes, sin(1/x), tan poles, domain holes, near-tangencies.
5. Incremental redraw, viewport culling, worker offload.

## Current architecture
- `src/App.svelte`: shell — layout grid (sidebar + graph), view state (scale/offset), pan/zoom handlers, `engineReady` gate.
- `src/layouts/Sidebar.svelte`: renders equation list.
- `src/components/EquationInput.svelte`: MathLive `<math-field>` editor, `onInput` edits sync `value` via `$bindable`, `onMount` attaches field, `enter` handler triggers `onAddLine`.
- `src/components/GraphCanvas.svelte`: SVG grid/axes/paths, samples each expression via `sampleToPath` → Rust WASM `sample()`.
- `src/utils/math.ts`: `sampleToPath` (WASM sample → SVG path `M`/`L`, NaN breaks segments), `generateGridLines` (1/2/5 nice grid).
- `src/utils/symbols.ts`: `SymbolTable`, `parseAssignment`, `evalExpr` (plain infix only — LaTeX braces rejected), implicit multiplication insertion, recursive-descent parser.
- `src/utils/latex.ts`: `toLatex` (plain→KaTeX: `^`→`^{}`, `/`→`\frac`, `sqrt`/`sum`/`int`/`infinity`, `*`→`\cdot`).
- `src/wasm/`: prebuilt Rust WASM engine (`grapher_engine_bg.js` + `.wasm`).
- `engine/src/lib.rs`: Rust math engine (tokenizer, recursive descent, `sqrt`/`sin`/`cos`/`tan`/`pi`, `sample()` FFI).
- `src/types.ts`: `ExpressionLine`, `GridLine`.

## Git workflow
- `main` is release branch.
- New features: create feature branch (`git checkout -b feature/<name>`), PR/merge into `main` when verified.
