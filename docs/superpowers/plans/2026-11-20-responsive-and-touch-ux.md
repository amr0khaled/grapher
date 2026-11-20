# Responsive Touch & UX Improvements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform Grapher into a mobile-first, responsive touch graphing calculator with unified pointer gestures, adaptive layout modes (Desktop sidebar, Tablet overlay, Mobile bottom sheet), continuous canvas rendering, and rich UX features without desktop regression.

**Architecture:** 
- Unified `ViewportController` managing multi-pointer pan/pinch/double-tap with hardware-accelerated transforms and RAF debounce.
- Layout manager switching between Desktop (>=1024px split), Tablet (640-1023px overlay drawer), and Mobile (<640px full-screen canvas + 3-stage draggable bottom sheet) leveraging viewport dimensions and `(hover: none) and (pointer: coarse)`.
- Responsive Canvas pipeline integrating `ResizeObserver`, window `visualViewport`, safe-area insets (`env(safe-area-inset-*)`), and edge-pinned axis labels.
- Fast analytical/discrete Point-Of-Interest (POI) detection and curve tracing computed on Web Worker / sampled buffers with debounced RAF presentation.

**Tech Stack:** Svelte 5 (Runes `$state`, `$derived`, `$effect`), TypeScript, Vite, HTML5 Canvas 2D, MathLive, Web Pointer Events API, VisualViewport API, Web Storage API.

**Spec:** Responsive Touch & UX Requirements (user specification supplied in prompt).

## Global Constraints

- Svelte 5 Runes (`$state`, `$derived`, `$props`, `$effect`) must be used for all state reactivity; no Svelte 4 legacy stores or `export let`.
- Zero new runtime NPM dependencies for gestures or layouts; all gestures use standard DOM `PointerEvent` and native CSS container/media queries.
- Minimum tap target size >= 44x44px on touch devices (`pointer: coarse`).
- Desktop pointer/mouse experience, shortcuts, and wheel behaviors must not regress.
- All curve sampling remains offloaded or batched through WASM engine via `sampleToPath2D`.
- Strict type safety with `bun run check` and production verification with `bun run build`.

## Review Focus

1. **Simultaneous touch pan + pinch gesture transition:** Transitioning from 1-finger pan to 2-finger pinch must not create view jumps or discontinuous scale jumps.
2. **Virtual keyboard resize vs bottom sheet:** Mobile OS virtual keyboard popup changes `visualViewport.height` without affecting window height; bottom sheet must sit above the keyboard without squishing the canvas.
3. **Axis labels pinned at screen boundaries:** When origin `(0,0)` is panned outside the viewport, axis coordinates must remain pinned to screen margins with crisp contrast.
4. **Sign-change POI detection at asymptotes:** Asymptotes and poles (e.g., `tan(x)`, `1/x`) must not produce false root or extremum markers when signs invert across infinities.
5. **Touch sheet drag conflicting with curve tracing:** Dragging the bottom sheet header or body must not accidentally trigger canvas pan/pinch or math-field blur.

---

### Phase 1: Canvas Robustness & Coordinate Resizing

#### Task 1: Viewport Preserving ResizeObserver and Safe Area Handling

**Files:**
- Modify: `src/components/GraphCanvas.svelte`
- Modify: `src/App.svelte`
- Create: `src/utils/viewport.ts`
- Test: `src/utils/viewport.test.ts`

**Interfaces:**
- Consumes: `scale`, `offsetX`, `offsetY`, `cssWidth`, `cssHeight`
- Produces: `preserveCenterOnResize(prevW, prevH, newW, newH, scale, offsetX, offsetY) -> { offsetX, offsetY }`

- [ ] **Step 1: Write failing unit test for `preserveCenterOnResize`**

```typescript
// src/utils/viewport.test.ts
import { describe, it, expect } from "vitest";
import { preserveCenterOnResize } from "./viewport";

describe("preserveCenterOnResize", () => {
  it("keeps world center fixed when canvas dimensions change", () => {
    // Initial 800x600 with center at (400, 300) in screen coords
    const initial = { prevW: 800, prevH: 600, newW: 1000, newH: 800, scale: 50, offsetX: 400, offsetY: 300 };
    const result = preserveCenterOnResize(
      initial.prevW, initial.prevH, initial.newW, initial.newH,
      initial.scale, initial.offsetX, initial.offsetY
    );
    // New center should be at (500, 400) screen coords, maintaining world coordinate (0, 0) at center
    expect(result.offsetX).toBe(500);
    expect(result.offsetY).toBe(400);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bunx vitest run src/utils/viewport.test.ts`
Expected: FAIL with module `./viewport` not found.

- [ ] **Step 3: Implement `preserveCenterOnResize` in `src/utils/viewport.ts`**

Calculate center shift `deltaX = (newW - prevW) / 2`, `deltaY = (newH - prevH) / 2` and return shifted offsets.

- [ ] **Step 4: Run test to verify it passes**

Run: `bunx vitest run src/utils/viewport.test.ts`
Expected: PASS.

- [ ] **Step 5: Integrate dynamic resizing and safe-area margins in `src/components/GraphCanvas.svelte`**

Connect `ResizeObserver` to update `offsetX` and `offsetY` via `preserveCenterOnResize`, apply `dvh` units, and respect safe areas.

- [ ] **Step 6: Commit**

```bash
git add src/utils/viewport.ts src/utils/viewport.test.ts src/components/GraphCanvas.svelte src/App.svelte
git commit -m "feat(canvas): preserve center during resize and support dynamic viewport units"
```

---

#### Task 2: Pinned Edge Axis Labels and Collision-Free Grid

**Files:**
- Modify: `src/utils/math.ts`
- Modify: `src/components/GraphCanvas.svelte`
- Test: `src/utils/grid.test.ts`

**Interfaces:**
- Consumes: `generateGridLines(scale, offset, max)`
- Produces: `getPinnedAxisPositions(offsetX, offsetY, width, height, padding) -> { axisX, axisY, labelAlignX, labelAlignY }`

- [ ] **Step 1: Write failing test for pinned axis coordinates**

```typescript
// src/utils/grid.test.ts
import { describe, it, expect } from "vitest";
import { getPinnedAxisPositions } from "./math";

describe("getPinnedAxisPositions", () => {
  it("pins axes to screen borders when origin is offscreen", () => {
    // Canvas 800x600, origin at (-100, 1000)
    const result = getPinnedAxisPositions(-100, 1000, 800, 600, 20);
    expect(result.axisX).toBe(20); // pinned to left margin
    expect(result.axisY).toBe(580); // pinned to bottom margin (600 - 20)
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bunx vitest run src/utils/grid.test.ts`
Expected: FAIL with `getPinnedAxisPositions is not a function`.

- [ ] **Step 3: Implement `getPinnedAxisPositions` in `src/utils/math.ts`**

Clamp axis positions to `[padding, dimension - padding]` and return orientation alignment flags for text rendering.

- [ ] **Step 4: Run test to verify it passes**

Run: `bunx vitest run src/utils/grid.test.ts`
Expected: PASS.

- [ ] **Step 5: Update `drawGridAndAxes` in `src/components/GraphCanvas.svelte`**

Render pinned axis lines and floating edge coordinates when axes pan off-screen.

- [ ] **Step 6: Commit**

```bash
git add src/utils/math.ts src/utils/grid.test.ts src/components/GraphCanvas.svelte
git commit -m "feat(grid): pin axis labels to viewport edges when origin is offscreen"
```

---

### Phase 2: Unified Touch Gestures & Viewport Controller

#### Task 3: Unified PointerEvent Viewport Controller (Pan, Pinch, Double-Tap)

**Files:**
- Create: `src/utils/gestureController.ts`
- Test: `src/utils/gestureController.test.ts`
- Modify: `src/App.svelte`

**Interfaces:**
- Consumes: Native DOM `PointerEvent` stream (`pointerdown`, `pointermove`, `pointerup`, `pointercancel`)
- Produces: `GestureController` class dispatching `{ type: 'pan', dx, dy }`, `{ type: 'pinch', center: {x, y}, factor }`, `{ type: 'doubletap', point: {x, y} }`

- [ ] **Step 1: Write unit tests for multi-pointer tracking**

```typescript
// src/utils/gestureController.test.ts
import { describe, it, expect } from "vitest";
import { GestureTracker } from "./gestureController";

describe("GestureTracker", () => {
  it("tracks two pointers and computes midpoint and distance", () => {
    const tracker = new GestureTracker();
    tracker.addPointer(1, 100, 200);
    tracker.addPointer(2, 300, 200);

    const pinch = tracker.getPinchInfo();
    expect(pinch.center).toEqual({ x: 200, y: 200 });
    expect(pinch.distance).toBe(200);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bunx vitest run src/utils/gestureController.test.ts`
Expected: FAIL with `GestureTracker not found`.

- [ ] **Step 3: Implement `GestureTracker` and pointer state machine in `src/utils/gestureController.ts`**

Support single pointer pan, dual pointer pinch zoom with midpoint anchor, and double-tap detection (tap interval < 300ms, distance < 20px).

- [ ] **Step 4: Run test to verify it passes**

Run: `bunx vitest run src/utils/gestureController.test.ts`
Expected: PASS.

- [ ] **Step 5: Replace ad-hoc touch and mouse handlers in `src/App.svelte` with `GestureTracker`**

Bind single pointer events on `<main class="graph-main">`, ensure `touch-action: none`, and maintain 1:1 pinch zoom factor without applying wheel sensitivity.

- [ ] **Step 6: Commit**

```bash
git add src/utils/gestureController.ts src/utils/gestureController.test.ts src/App.svelte
git commit -m "feat(input): unify mouse and touch gestures under pointer event controller"
```

---

### Phase 3: Layout Modes (Desktop, Tablet, Landscape & Mobile)

#### Task 4: Responsive Layout Manager and Media Queries

**Files:**
- Create: `src/utils/device.svelte.ts`
- Modify: `src/App.svelte`
- Modify: `src/layouts/Sidebar.svelte`
- Test: `src/utils/device.test.ts`

**Breakpoints & Media Query Justification:**
- **Desktop (`>= 1024px` AND `(hover: hover)`):** Persistent side-by-side grid layout (sidebar 340px, canvas 1fr). Large screen with mouse precision.
- **Tablet (`640px - 1023px` OR `(pointer: coarse) and (min-width: 640px)`):** Overlay modal/drawer for sidebar; maximizes canvas for touch interaction while retaining desktop-like expression cards.
- **Mobile (`< 640px`):** Full-screen canvas + draggable bottom sheet. In portrait mode, expressions cannot fit horizontally alongside canvas without ruining graph visibility.
- **Landscape Phone (`< 900px` AND `orientation: landscape`):** Compact collapsible floating drawer instead of tall bottom sheet so the vertical graph height remains usable.

- [ ] **Step 1: Write test for layout mode resolution**

```typescript
// src/utils/device.test.ts
import { describe, it, expect } from "vitest";
import { resolveLayoutMode } from "./device.svelte";

describe("resolveLayoutMode", () => {
  it("resolves desktop for wide screen with hover", () => {
    expect(resolveLayoutMode(1200, 800, true, true)).toBe("desktop");
  });
  it("resolves tablet for medium screen", () => {
    expect(resolveLayoutMode(800, 600, false, false)).toBe("tablet");
  });
  it("resolves mobile for narrow screen", () => {
    expect(resolveLayoutMode(400, 800, false, false)).toBe("mobile");
  });
  it("resolves landscape-mobile for wide short touch device", () => {
    expect(resolveLayoutMode(800, 380, false, false)).toBe("landscape-mobile");
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bunx vitest run src/utils/device.test.ts`
Expected: FAIL.

- [ ] **Step 3: Implement `resolveLayoutMode` in `src/utils/device.svelte.ts`**

Return `"desktop" | "tablet" | "mobile" | "landscape-mobile"` considering viewport width, height, and pointer fine/coarse capabilities.

- [ ] **Step 4: Run test to verify it passes**

Run: `bunx vitest run src/utils/device.test.ts`
Expected: PASS.

- [ ] **Step 5: Wire responsive mode in `src/App.svelte` and `src/layouts/Sidebar.svelte`**

Expose current layout mode via rune state; conditionally display layout panels.

- [ ] **Step 6: Commit**

```bash
git add src/utils/device.svelte.ts src/utils/device.test.ts src/App.svelte src/layouts/Sidebar.svelte
git commit -m "feat(layout): implement responsive layout manager for desktop, tablet, and mobile"
```

---

### Phase 4: Draggable Bottom Sheet & Touch UI

#### Task 5: Mobile Draggable Bottom Sheet with Snap Points

**Files:**
- Create: `src/components/BottomSheet.svelte`
- Modify: `src/App.svelte`
- Modify: `src/components/EquationList.svelte`

**Interfaces:**
- Consumes: `snapPoints: ['peek', 'half', 'full']`, `activeSnap: 'peek' | 'half' | 'full'`, `keyboardHeight: number`
- Produces: `onSnapChange(snap: 'peek' | 'half' | 'full')`, `onCanvasTap()`

- [ ] **Step 1: Build `BottomSheet.svelte` component structure**

Define snap points:
- `peek`: 72px (shows active expression bar + drag handle).
- `half`: 45vh (shows top 2-3 equations + canvas visible above).
- `full`: calc(100dvh - 56px) (full list management).
Add drag handle with touch events (`pointerdown`, `pointermove`, `pointerup`).

- [ ] **Step 2: Add canvas tap backdrop dismiss**

Tapping the canvas while sheet is `half` or `full` snaps sheet down to `peek`.

- [ ] **Step 3: VisualViewport keyboard adjustment**

Adjust sheet bottom offset using `window.visualViewport.height` so sheet sits directly above the virtual keyboard without being pushed off-screen.

- [ ] **Step 4: Test in `App.svelte` mobile view**

Render `<BottomSheet>` when `layoutMode === 'mobile'`.

- [ ] **Step 5: Commit**

```bash
git add src/components/BottomSheet.svelte src/App.svelte src/components/EquationList.svelte
git commit -m "feat(mobile): add 3-stage draggable bottom sheet for mobile expressions"
```

---

#### Task 6: Touch-Friendly Equation Rows (Tap Targets, Actions, Virtual Keyboard Quick Tabs)

**Files:**
- Modify: `src/components/EquationInput.svelte`
- Modify: `src/components/EquationList.svelte`
- Create: `src/components/RowActionMenu.svelte`

**Interfaces:**
- Row tap targets >= 44px.
- Color dot toggles curve visibility.
- Long-press or context menu button opens `RowActionMenu` (Duplicate, Line Style: solid/dashed, Color palette, Delete).
- MathLive configuration: suppression of native software keyboard on mobile, custom quick action bar for built-ins (`rect`, `tri`, `step`, `sinc`, `heaviside`).

- [ ] **Step 1: Increase tap target sizes to >=44px**

Update `.color-picker`, `.eq-number`, and `.close-btn` CSS in `EquationInput.svelte` and `SettingsCard.svelte`.

- [ ] **Step 2: Add curve visibility toggle to color dot**

Add `visible: boolean` property to `ExpressionLine` in `src/types.ts`. Clicking color dot toggles visibility without deleting expression.

- [ ] **Step 3: Add `RowActionMenu.svelte`**

Modal popover with duplicate, line style (dashed / solid), color picker, and delete row options.

- [ ] **Step 4: MathLive built-in quick tabs**

Attach custom toolbar tab bar above MathLive with quick insertion for: `sinc(x)`, `rect(x)`, `tri(x)`, `step(x)`, `u(x)`.

- [ ] **Step 5: Commit**

```bash
git add src/components/EquationInput.svelte src/components/EquationList.svelte src/components/RowActionMenu.svelte src/types.ts
git commit -m "feat(ui): add 44px tap targets, row action menu, and math quick tabs"
```

---

### Phase 5: Viewport UX (Trace, Reset, Fit-to-Curves, Error State)

#### Task 7: Curve Tracing and Reset / Fit-to-Curves

**Files:**
- Create: `src/utils/curveTrace.ts`
- Test: `src/utils/curveTrace.test.ts`
- Modify: `src/components/GraphCanvas.svelte`
- Modify: `src/App.svelte`

**Interfaces:**
- Consumes: Sampled curve flat buffers `Float64Array`, touch pointer position `(px, py)`
- Produces: `findNearestCurvePoint(flatBuffers, px, py, scale, offsetX, offsetY, maxDistPx) -> { x, y, screenX, screenY, exprIndex } | null`

- [ ] **Step 1: Write test for `findNearestCurvePoint`**

```typescript
// src/utils/curveTrace.test.ts
import { describe, it, expect } from "vitest";
import { findNearestCurvePoint } from "./curveTrace";

describe("findNearestCurvePoint", () => {
  it("finds closest point on sampled polyline within threshold", () => {
    // Single curve with points at (100, 100), (105, 110), (110, 120)
    const buffer = new Float64Array([100, 100, 105, 110, 110, 120]);
    const hit = findNearestCurvePoint([buffer], 106, 112, 50, 400, 300, 15);
    expect(hit).not.toBeNull();
    expect(hit?.screenX).toBe(105);
    expect(hit?.screenY).toBe(110);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bunx vitest run src/utils/curveTrace.test.ts`
Expected: FAIL.

- [ ] **Step 3: Implement `findNearestCurvePoint` in `src/utils/curveTrace.ts`**

Efficiently iterate through flat coordinate segments; compute minimum Euclidean distance to touch point.

- [ ] **Step 4: Run test to verify it passes**

Run: `bunx vitest run src/utils/curveTrace.test.ts`
Expected: PASS.

- [ ] **Step 5: Connect curve trace tooltip and "Fit to Curves" in `GraphCanvas.svelte` and `App.svelte`**

Render coordinate badge `(x, y)` at traced position; compute bounding box of valid sampled curves for "Fit to Curves" button.

- [ ] **Step 6: Commit**

```bash
git add src/utils/curveTrace.ts src/utils/curveTrace.test.ts src/components/GraphCanvas.svelte src/App.svelte
git commit -m "feat(ux): implement curve tracing tooltip and fit-to-curves view bounding"
```

---

#### Task 8: Non-Breaking Inline Expression Error Indicators

**Files:**
- Modify: `src/components/EquationInput.svelte`
- Modify: `src/utils/math.ts`
- Modify: `src/App.svelte`

**Interfaces:**
- Retain previously valid `Path2D` when current expression syntax has errors or fails to evaluate.
- Display gentle warning badge on invalid expression row.

- [ ] **Step 1: Update `sampleToPath2D` to return status metadata**

Return `{ path: Path2D | null, isError: boolean }` for each expression line.

- [ ] **Step 2: Cache last valid path in `GraphCanvas.svelte`**

When an expression becomes invalid during typing, keep rendering its last valid path at reduced opacity (0.5) until corrected or cleared.

- [ ] **Step 3: Show error warning icon in `EquationInput.svelte`**

Show small non-blocking tooltip when syntax error occurs.

- [ ] **Step 4: Commit**

```bash
git add src/utils/math.ts src/components/EquationInput.svelte src/components/GraphCanvas.svelte
git commit -m "feat(ux): retain last valid curve during syntax errors and show inline error indicator"
```

---

### Phase 6: Persistence, Sharing & Export

#### Task 9: LocalStorage Autosave, URL Hash Sharing, and PNG Export

**Files:**
- Create: `src/utils/storage.ts`
- Test: `src/utils/storage.test.ts`
- Modify: `src/App.svelte`
- Modify: `src/components/SettingsCard.svelte`

**Interfaces:**
- `saveState(state: AppState): void`
- `loadState(): AppState | null`
- `encodeShareUrl(state: AppState): string`
- `decodeShareUrl(hash: string): AppState | null`
- `exportCanvasPng(canvas: HTMLCanvasElement): Promise<void>`

- [ ] **Step 1: Write test for URL hash serialization and parsing**

```typescript
// src/utils/storage.test.ts
import { describe, it, expect } from "vitest";
import { encodeShareUrl, decodeShareUrl } from "./storage";

describe("URL state encoding", () => {
  it("encodes and decodes expression state without loss", () => {
    const state = {
      expressions: [{ expression: "y=\\sin(x)", color: "#10b981", visible: true }],
      scale: 50,
      offsetX: 400,
      offsetY: 300,
    };
    const hash = encodeShareUrl(state);
    const decoded = decodeShareUrl(hash);
    expect(decoded?.expressions[0].expression).toBe("y=\\sin(x)");
    expect(decoded?.scale).toBe(50);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bunx vitest run src/utils/storage.test.ts`
Expected: FAIL.

- [ ] **Step 3: Implement `storage.ts`**

Base64 / URL-safe compressed JSON encoding with fallback for corrupt `localStorage` access (e.g. incognito Safari).

- [ ] **Step 4: Run test to verify it passes**

Run: `bunx vitest run src/utils/storage.test.ts`
Expected: PASS.

- [ ] **Step 5: Add PNG export with Web Share API in `SettingsCard.svelte`**

Download PNG or invoke `navigator.share({ files })` if supported on mobile.

- [ ] **Step 6: Commit**

```bash
git add src/utils/storage.ts src/utils/storage.test.ts src/App.svelte src/components/SettingsCard.svelte
git commit -m "feat(storage): add localStorage autosave, URL sharing, and PNG export"
```

---

### Phase 7: Points of Interest (POI) Detection

#### Task 10: Lightweight Debounced Points of Interest (Roots, Intercepts, Extrema)

**Files:**
- Create: `src/utils/poi.ts`
- Test: `src/utils/poi.test.ts`
- Modify: `src/components/GraphCanvas.svelte`

**POI Algorithm:**
- Sign-change scanning on sampled points in visible screen bounds `[0, width]`.
- For roots ($y = 0$): Detect $y_i \cdot y_{i+1} < 0$, filter out vertical asymptotes where $|y_{i+1} - y_i| > \text{height} / 2$.
- For extrema ($dy/dx = 0$): Detect slope sign change $(y_i - y_{i-1}) \cdot (y_{i+1} - y_i) < 0$.
- For y-intercept: Evaluate at $x = 0$ if $x=0$ is within screen bounds.
- Debounced at 150ms after pan/zoom interaction ceases to prevent frame drops on mobile.

- [ ] **Step 1: Write test for sign-change root detection**

```typescript
// src/utils/poi.test.ts
import { describe, it, expect } from "vitest";
import { findRootsFromPoints } from "./poi";

describe("findRootsFromPoints", () => {
  it("detects roots from sign transitions excluding asymptotes", () => {
    // Points crossing zero from y=-2 to y=2 around x=10
    const points = new Float64Array([9, -2, 11, 2]);
    const roots = findRootsFromPoints(points, 50, 400, 300, 800, 600);
    expect(roots.length).toBe(1);
    expect(roots[0].x).toBeCloseTo(-6); // world coordinate
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bunx vitest run src/utils/poi.test.ts`
Expected: FAIL.

- [ ] **Step 3: Implement `poi.ts`**

Write `findRootsFromPoints` and `findExtremaFromPoints` with asymptotic discontinuity guards.

- [ ] **Step 4: Run test to verify it passes**

Run: `bunx vitest run src/utils/poi.test.ts`
Expected: PASS.

- [ ] **Step 5: Render pinnable POI markers in `GraphCanvas.svelte`**

Draw discrete dots at roots and extrema on `renderPrecise`; clicking/tapping pins the value callout.

- [ ] **Step 6: Commit**

```bash
git add src/utils/poi.ts src/utils/poi.test.ts src/components/GraphCanvas.svelte
git commit -m "feat(poi): implement debounced root, intercept, and extremum point detection"
```

---

### Phase 8: Polish, Accessibility & Testing

#### Task 11: Accessibility, Haptics, and Keyboard Navigation

**Files:**
- Modify: `src/App.svelte`
- Modify: `src/components/BottomSheet.svelte`
- Modify: `src/components/SettingsCard.svelte`
- Create: `src/components/ShortcutModal.svelte`

- [ ] **Step 1: Desktop shortcut modal and keyboard navigation**

Add `?` shortcut to open keyboard cheat sheet (`+`/`-` zoom, arrow keys pan, `Enter` new line, `Delete` remove).

- [ ] **Step 2: Haptic feedback**

Trigger `navigator.vibrate?.(10)` on snap point changes and POI pin taps (feature detected).

- [ ] **Step 3: Reduced-motion query**

Honor `@media (prefers-reduced-motion: reduce)` by disabling bottom sheet sliding animations.

- [ ] **Step 4: Commit**

```bash
git add src/App.svelte src/components/BottomSheet.svelte src/components/SettingsCard.svelte src/components/ShortcutModal.svelte
git commit -m "feat(a11y): add keyboard shortcuts modal, haptic feedback, and reduced-motion support"
```

---

#### Task 12: Playwright Viewport Tests & Real-Device Checklist

**Files:**
- Create: `playwright.config.ts`
- Create: `tests/e2e/responsive.spec.ts`
- Create: `tests/manual-device-checklist.md`

- [ ] **Step 1: Install Playwright test runner**

Run: `bun add -d @playwright/test`

- [ ] **Step 2: Write responsive viewport E2E tests in `tests/e2e/responsive.spec.ts`**

Test:
- Desktop viewport (1280x800): Sidebar visible, bottom sheet absent.
- Tablet viewport (768x1024): Drawer toggle visible, canvas full width.
- Mobile viewport (390x844): Bottom sheet visible, dragging expands sheet, tapping canvas collapses.
- Landscape Mobile (844x390): Compact floating panel layout.

- [ ] **Step 3: Run Playwright tests**

Run: `bunx playwright test`
Expected: PASS.

- [ ] **Step 4: Create `tests/manual-device-checklist.md`**

Checklist for real touch testing (iOS Safari URL bar expansion, Android Chrome virtual keyboard layout, 2-finger hardware pinch smoothness).

- [ ] **Step 5: Commit**

```bash
git add playwright.config.ts tests/e2e/responsive.spec.ts tests/manual-device-checklist.md package.json
git commit -m "test(e2e): add Playwright viewport tests and real-device testing checklist"
```

---

## Performance Risks & Low-End Phone Mitigations

1. **POI Detection Computation Overhead:**
   - *Risk:* Scanning thousands of sample points for roots/extrema on every frame during pan/pinch will stutter UI on budget mobile CPUs.
   - *Mitigation:* POI detection is completely disabled during active interaction and only runs in a single debounced pass (150ms delay) after gesture release.
2. **Curve Tracing Hit-Testing:**
   - *Risk:* Linear search through multiple expressions on every touch move.
   - *Mitigation:* Spatial bounding box early-exit check; only curves whose bounding box contains the touch point are scanned.
3. **Canvas Re-allocation on Mobile URL Bar Hide/Show:**
   - *Risk:* Mobile browser dynamic toolbar resize repeatedly triggers canvas re-allocation and memory pressure.
   - *Mitigation:* Debounce canvas buffer dimension resizing by 100ms or until gesture completion; maintain cached backing buffer.

---

## Clarifying Questions

1. **Math Keyboard:** MathLive has its own built-in touch virtual keyboard. Should we configure MathLive's virtual keyboard with custom layouts (algebra, signals/built-ins), or do you want an entirely custom Svelte-rendered math keyboard? *(Plan assumes configuring MathLive's virtual keyboard policy with custom quick tabs).*
2. **Landscape Phone Behavior:** In landscape on small phones (e.g. 390px tall), the bottom sheet would cover the entire graph. Is a compact floating left/right toolbar preferred over a bottom sheet in landscape? *(Plan specifies compact floating panel in landscape).*
3. **Undo/Redo Scope:** Should Undo/Redo apply only to equation deletions and text edits, or also view pan/zoom state changes? *(Recommended: equation list mutations only, avoiding jarring view shifts).*
