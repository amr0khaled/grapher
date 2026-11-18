# Canvas Renderer & Trigger Resampling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the SVG graph renderer with an HTML5 `<canvas>` renderer that fills the available viewport container (no 800×600 letterbox), leverages devicePixelRatio backing stores, implements trigger-based re-sampling with a CSS-transform stretch during pan/zoom gestures, and wire the engine's symbol evaluator to equation row badges while deleting the dead TypeScript symbol evaluator.

**Architecture:** `src/utils/math.ts` replaces `sampleAllToPaths` (SVG `d` string builder) with `sampleToPath2D` returning `Path2D[]`, and exports `evaluateSymbols` wrapping WASM `get_symbol_values`. `GraphCanvas.svelte` renders grid, axes, labels, and curves imperatively onto a DPI-scaled `<canvas>`. `App.svelte` drops hardcoded 800×600 dimensions and reactively derives `symbols`. `EquationInput.svelte` displays evaluated values in the row badge. `src/utils/symbols.ts` is deleted. Pan/zoom interactions apply a cheap GPU CSS `transform` on the canvas element while a 120ms debounce schedules precise re-sampling and redrawing.

**Tech Stack:** Svelte 5 (runes), HTML5 Canvas 2D API, ResizeObserver, existing Rust→WASM math engine (`grapher_engine`).

**Spec:** User decisions (2026-11-18): CSS-transform transient scaling (Google Maps style), full container fill, y-axis coordinates unchanged.

## Global Constraints

- Engine changes: scoped strictly to connecting `lines` into `build_symbols` in `get_symbol_values` and rebuilding WASM (`bun run build:rs`). Engine oversamples 4× internally; canvas draws in CSS-pixel units, scaled by `devicePixelRatio`.
- Dead code removal: `src/utils/symbols.ts` is deleted completely; no references remain in frontend or tests.
- No SVG graph elements remaining. Grid, axes, labels, and curves are canvas-drawn.
- Theme colors read dynamically from `getComputedStyle(document.documentElement)` at draw time (`--bg`, `--grid`, `--grid-major`, `--axis`, `--muted`, `--fg`).
- Empty/assignment lines produce no path; NaN points break paths (`moveTo`), matching prior behavior.
- `bun run check` must pass cleanly.
- `.graph-canvas` CSS selector must remain on the canvas element (App.svelte queries it for rect math).

## Review Focus

1. **Resize edge cases:** Canvas sized to `0×0` on initial mount before ResizeObserver fires. Guard: skip draw until `cssWidth > 0`.
2. **DPI scale changes:** Moving window between monitors with different DPR. Guard: ResizeObserver + backing store resize.
3. **NaN segment continuity in Path2D:** Missing finite points can connect across asymptotes if `Path2D` receives `lineTo` on first valid point after `NaN`. Guard: track `hasMoved` boolean per subpath in `sampleToPath2D`.
4. **Theme switching reactivity:** Ensuring dark/light toggle immediately redraws canvas. Guard: `$effect` tracking `darkMode` and CSS vars.
5. **Transform cleanup on debounce:** Stale CSS transforms sticking after rapid pan/zoom stop. Guard: reset `transform` explicitly in `drawPrecise`.

---

### Task 0: Commit existing changes and push to GitHub

**Files:**
- Modify: git worktree status (all staged/unstaged changes on `feature/signal-builtins`)

- [ ] **Step 1: Check git status and stage changes**

Run: `git status`
Expected: shows modified engine, tests, src/wasm, vite.config.ts, and docs/.

- [ ] **Step 2: Commit current progress**

Run:
```bash
git add engine/ src/wasm/ src/utils/latex.ts vite.config.ts docs/
git commit -m "$(cat <<'EOF'
feat(engine): signal built-ins, subscript symbols, and production wasm inline

- Complete math, signal waveforms, Fourier transforms, and convolution built-ins
- Support multi-arg function calls and multi-char/subscripted symbols (e.g. a_2)
- Inline WASM as base64 data URI in Vite production build for file:// support
EOF
)"
```

- [ ] **Step 3: Push to remote branch**

Run: `git push -u origin feature/signal-builtins`
Expected: branch successfully pushed to remote.

---

### Task 1: Path2D sampling utility

**Files:**
- Modify: `src/utils/math.ts`

**Interfaces:**
- Consumes: WASM `sample` output (`Float64Array` per line).
- Produces: `sampleToPath2D(expressions: string[], scale: number, offsetX: number, offsetY: number, width: number, height: number): (Path2D | null)[]`

- [ ] **Step 1: Replace SVG path builder with `Path2D` builder in `src/utils/math.ts`**

Edit `src/utils/math.ts` to remove `pathFromFlat` and `sampleAllToPaths`, and add:
```ts
function buildPath2D(flat: Float64Array): Path2D | null {
  if (!flat || flat.length < 2) return null;
  const p = new Path2D();
  let hasPoints = false;
  let startedSubpath = false;
  for (let i = 0; i + 1 < flat.length; i += 2) {
    const x = flat[i];
    const y = flat[i + 1];
    if (!Number.isFinite(x) || !Number.isFinite(y)) {
      startedSubpath = false;
      continue;
    }
    hasPoints = true;
    if (!startedSubpath) {
      p.moveTo(x, y);
      startedSubpath = true;
    } else {
      p.lineTo(x, y);
    }
  }
  return hasPoints ? p : null;
}

export function sampleToPath2D(
  expressions: string[],
  scale: number,
  offsetX: number,
  offsetY: number,
  width: number,
  height: number,
): (Path2D | null)[] {
  const plain = expressions.map((e) => toPlainInfix(e));
  const arrays = sample(plain, scale, offsetX, offsetY, width, height) as unknown[];
  const out: (Path2D | null)[] = [];
  for (let i = 0; i < arrays.length; i++) {
    const flat = arrays[i] as Float64Array;
    out.push(buildPath2D(flat));
  }
  return out;
}
```

- [ ] **Step 2: Run type check**

Run: `bun run check`
Expected: compilation errors in `GraphCanvas.svelte` (since it still calls `sampleAllToPaths`), confirming interface boundary.

- [ ] **Step 3: Commit**

```bash
git add src/utils/math.ts
git commit -m "refactor(math): replace SVG path string builder with Path2D builder"
```

---

### Task 2: Canvas GraphCanvas component

**Files:**
- Modify: `src/components/GraphCanvas.svelte`

**Interfaces:**
- Consumes: `sampleToPath2D`, `generateGridLines` from `src/utils/math.ts`, props `expressions`, `scale`, `offsetX`, `offsetY`, `showGrid`, `darkMode`.
- Produces: `<canvas class="graph-canvas">` with imperative 2D rendering loop.

- [ ] **Step 1: Rewrite `src/components/GraphCanvas.svelte`**

Implement canvas sizing via `ResizeObserver`, backing store setup with `devicePixelRatio`, theme reading via `getComputedStyle`, and full grid/axis/label/path rendering.

- [ ] **Step 2: Run type check**

Run: `bun run check`
Expected: no errors in GraphCanvas.

- [ ] **Step 3: Commit**

```bash
git add src/components/GraphCanvas.svelte
git commit -m "feat(canvas): replace SVG GraphCanvas with DPI-aware HTML5 canvas renderer"
```

---

### Task 3: App.svelte size refactor

**Files:**
- Modify: `src/App.svelte`

**Interfaces:**
- Consumes: `GraphCanvas` without `innerWidth`/`innerHeight` props.
- Produces: view state calculations anchored to canvas element bounding rect.

- [ ] **Step 1: Remove hardcoded `innerWidth`/`innerHeight` consts from `src/App.svelte` and update zoom/pan/reset handlers to query canvas rect**

- [ ] **Step 2: Run type check and build**

Run: `bun run check && bun run build`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add src/App.svelte
git commit -m "refactor(app): remove fixed viewport dimensions, anchor view math to canvas rect"
```

---

### Task 4: Trigger-based resampling and CSS-stretch transient

**Files:**
- Modify: `src/components/GraphCanvas.svelte`

**Interfaces:**
- Consumes: view state changes (`scale`, `offsetX`, `offsetY`).
- Produces: instant CSS matrix scaling during gestures + debounced 120ms precise re-sampling.

- [ ] **Step 1: Add transient CSS transform and debounce logic to `GraphCanvas.svelte`**

- [ ] **Step 2: Verify build and type check**

Run: `bun run check && bun run build`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add src/components/GraphCanvas.svelte
git commit -m "perf(canvas): add trigger-based resampling with GPU CSS transform transient scaling"
```

---

### Task 5: Mobile and tablet responsiveness

**Files:**
- Modify: `src/App.svelte` (layout grid, toggle state, floating control styles, scrim styles)
- Modify: `src/layouts/Sidebar.svelte` (drawer positioning, close affordance)

**Interfaces:**
- Consumes: canvas renderer from Tasks 2/4 (`.graph-canvas` rect already drives view math, so drawer open/close resize flows through ResizeObserver automatically).
- Produces: `showSidebar` `$state` in App.svelte (default `false` under 900px breakpoint), toggle button in `.zoom-controls`, `.sidebar-scrim` element, media queries at 900px (tablet) and 600px (phone).

**Decisions (locked):**
- Sidebar becomes a fixed-position left drawer below 900px with a scrim backdrop; static column above 900px. Toolbar stays visible on top of canvas.
- Drawer state persists only for the session; no localStorage.
- Toggle button placed in existing `.zoom-controls` stack (top-right) with `☰` icon, visible only under 900px.
- >=48px tap targets for zoom/settings/toggle buttons under 600px.
- No changes to engine, `grapher_engine`, or `math.ts`.

- [ ] **Step 1: Add `showSidebar` state and toggle button**

In `App.svelte`: add `let showSidebar = $state(false);` near other `$state` declarations. Add a `☰` toggle button inside `.zoom-controls`, first in the stack, with `aria-label="Toggle sidebar"`, `onclick={() => (showSidebar = !showSidebar)}`, and a new class `sidebar-toggle` (hidden by default via `display: none` in desktop styles).

- [ ] **Step 2: Pass drawer state into Sidebar**

Change `<Sidebar ...>` to receive `{showSidebar}` and `{closeSidebar}` props: `onClose={() => (showSidebar = false)}`.

- [ ] **Step 3: Add scrim element in `<main>`**

```svelte
{#if showSidebar}
  <div class="sidebar-scrim" onclick={() => (showSidebar = false)}></div>
{/if}
```

Positioned absolute, full-size, `background: rgba(0,0,0,0.4)`, `z-index: 9`, only rendered when drawer visible; never rendered on desktop.

- [ ] **Step 4: Add media queries in `App.svelte` `<style>`**

```css
@media (max-width: 900px) {
  .grapher-layout { grid-template-columns: 1fr; }
  .sidebar-toggle { display: flex; }
  .sidebar-scrim { position: absolute; inset: 0; background: rgba(0,0,0,0.4); z-index: 9; }
}
@media (max-width: 600px) {
  .zoom-controls button { width: 48px; height: 48px; font-size: 1.5rem; }
  .sidebar-header h2 { font-size: 1rem; }
}
```

The desktop `.grapher-layout` keeps `grid-template-columns: 320px 1fr` untouched; media query overrides it.

- [ ] **Step 5: Sidebar drawer CSS in `Sidebar.svelte`**

```css
@media (max-width: 900px) {
  .sidebar {
    position: fixed;
    top: 0; left: 0; bottom: 0;
    width: 85vw;
    max-width: 340px;
    z-index: 20;
    transform: translateX(-100%);
    transition: transform 200ms ease;
    box-shadow: 2px 0 12px rgba(0,0,0,0.25);
  }
  .sidebar.open { transform: translateX(0); }
}
```

Apply `class:open={showSidebar}` to `<aside class="sidebar">`. Desktop layout untouched outside the media query.

- [ ] **Step 6: Verify**

Run: `bun run check && bun run build`
Expected: PASS.
Manual: resize window across 901/899/601/599px; drawer slides in/out; scrim closes it; canvas remains interactive in static 320px column on desktop (no regression).

- [ ] **Step 7: Commit**

```bash
git add src/App.svelte src/layouts/Sidebar.svelte
git commit -m "feat(mobile): collapsible sidebar drawer and touch-friendly controls for tablet/phone"
```

---

### Task 6: Wire WASM symbol evaluation to equation badge & delete legacy symbols.ts

**Files:**
- Modify: `engine/src/sampling.rs` (export `build_symbols` to crate)
- Modify: `engine/src/lib.rs` (pass `lines` to `build_symbols` in `get_symbol_values`)
- Modify: `engine/tests/symbol_evaluation.rs` (add unit test verifying `symbol_values` output)
- Modify: `src/wasm/` (rebuilt via `bun run build:rs`)
- Modify: `src/utils/math.ts` (export `evaluateSymbols(expressions: string[])`)
- Modify: `src/App.svelte` (reactively derive `symbols` from expressions via `evaluateSymbols`)
- Modify: `src/components/EquationInput.svelte` (parse assignment name and look up `symbols[name]`)
- Delete: `src/utils/symbols.ts`
- Modify: `CLAUDE.md` (remove `src/utils/symbols.ts` reference and document WASM symbol badge wiring)
- Modify: `SPEC.md` (update architecture and current status for WASM symbol evaluation)

**Interfaces:**
- Consumes: WASM `get_symbol_values(lines: string[]): Record<string, number>`.
- Produces: `evaluateSymbols(expressions: string[]): Record<string, number>` in `src/utils/math.ts`.
- Reactivity: `App.svelte` derives `symbols = $derived(engineReady ? evaluateSymbols(expressions.map(e => e.expression)) : {})` and passes it through `Sidebar` → `EquationList` → `EquationInput`.
- Badge: `EquationInput.svelte` matches assignment variable name from `toPlainInfix(value)` and renders formatted number in `.eq-badge`.

- [ ] **Step 1: Wire `build_symbols` into `get_symbol_values` in the Rust engine**

In `engine/src/sampling.rs`:
Change `fn build_symbols(lines: &[String]) -> SymbolTable` to `pub(crate) fn build_symbols(lines: &[String]) -> SymbolTable`.

In `engine/src/lib.rs`:
Update `get_symbol_values`:
```rust
#[wasm_bindgen]
pub fn get_symbol_values(lines: Vec<String>) -> js_sys::Object {
    let symbols = crate::sampling::build_symbols(&lines);
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
```

- [ ] **Step 2: Add engine unit test for symbol table values**

In `engine/tests/symbol_evaluation.rs`:
Add a test verifying that `SymbolTable` evaluates dependent assignments to expected values:
```rust
#[test]
fn test_symbol_values_extraction() {
    let mut st = SymbolTable::new();
    st.set_expr("m".to_string(), parse_expr("5").unwrap());
    st.set_expr("n".to_string(), parse_expr("m * 2 + 1").unwrap());
    let vals: std::collections::HashMap<_, _> = st.symbol_values(0.0, 0.0).into_iter().collect();
    assert_eq!(vals.get("m"), Some(&5.0));
    assert_eq!(vals.get("n"), Some(&11.0));
}
```

Run: `cargo test --manifest-path engine/Cargo.toml`
Expected: PASS.

- [ ] **Step 3: Rebuild WASM engine**

Run: `bun run build:rs`
Expected: `src/wasm/grapher_engine_bg.wasm` and JS bindings updated with the new `get_symbol_values(lines)` implementation.

- [ ] **Step 4: Export `evaluateSymbols` helper in `src/utils/math.ts`**

In `src/utils/math.ts`:
Import `get_symbol_values` from `../wasm/grapher_engine.js`:
```ts
import initEngine, { get_symbol_values, sample } from '../wasm/grapher_engine.js';
```
And export:
```ts
export function evaluateSymbols(expressions: string[]): Record<string, number> {
  const plain = expressions.map((e) => toPlainInfix(e));
  return get_symbol_values(plain) as Record<string, number>;
}
```

- [ ] **Step 5: Derive `symbols` reactively in `src/App.svelte`**

In `src/App.svelte`:
Import `evaluateSymbols` from `./utils/math`:
```ts
import { evaluateSymbols, sampleToPath2D, generateGridLines } from "./utils/math";
```
Replace the static `let symbols: Record<string, number> = {};` with:
```ts
let symbols = $derived.by(() => {
  if (!engineReady) return {};
  return evaluateSymbols(expressions.map((e) => e.expression));
});
```

- [ ] **Step 6: Update `EquationInput.svelte` badge resolution**

In `src/components/EquationInput.svelte`:
Import `toPlainInfix` from `../utils/latex`:
```ts
import { toPlainInfix } from "../utils/latex";
```
Update `badgeValue`:
```ts
let badgeValue = $derived.by(() => {
  const plain = toPlainInfix(value);
  const m = plain.match(/^\s*([a-zA-Z_]\w*)\s*=/);
  if (!m) return null;
  const name = m[1].toLowerCase();
  if (name === "x" || name === "y") return null;
  const val = symbols[m[1]];
  return typeof val === "number" && Number.isFinite(val) ? val : null;
});
```

- [ ] **Step 7: Delete `src/utils/symbols.ts` and update `CLAUDE.md` & `SPEC.md`**

Delete `src/utils/symbols.ts`:
```bash
rm src/utils/symbols.ts
```

In `CLAUDE.md` and `SPEC.md`:
Remove `- `src/utils/symbols.ts`...` from the architecture section, and note that symbol evaluation is natively handled by Rust WASM `get_symbol_values` and `evaluateSymbols` in `src/utils/math.ts`.

- [ ] **Step 8: Type check and test build**

Run: `bun run check && bun run build`
Expected: PASS with 0 errors.

- [ ] **Step 9: Commit**

```bash
git add engine/src/sampling.rs engine/src/lib.rs engine/tests/symbol_evaluation.rs src/wasm/ src/utils/math.ts src/App.svelte src/components/EquationInput.svelte CLAUDE.md SPEC.md
git rm src/utils/symbols.ts
git commit -m "feat(symbols): wire WASM get_symbol_values to equation badges and delete legacy symbols.ts"
```

