<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "./layouts/Sidebar.svelte";
  import GraphCanvas from "./components/GraphCanvas.svelte";
  import SettingsCard from "./components/SettingsCard.svelte";
  import BottomSheet from "./components/BottomSheet.svelte";
  import EquationList from "./components/EquationList.svelte";
  import initEngine from "./wasm/grapher_engine.js";
  import { generateColor } from "./utils/colors";
  import { evaluateSymbols } from "./utils/math";
  import { resolveLayoutMode } from "./utils/device.svelte";
  import type { ExpressionLine } from "./types.ts";
  import { GestureTracker } from "./utils/gestureController";

  let expressions = $state<ExpressionLine[]>([
    { expression: "", color: generateColor(0), visible: true },
    { expression: "", color: generateColor(1), visible: true },
  ]);

  // Symbol table derived from assignment lines (top-down evaluation via WASM).
  let symbols = $derived.by(() => {
    if (!engineReady) return {};
    return evaluateSymbols(expressions.map((e) => e.expression));
  });

  let errorStates = $state<boolean[]>([]);
  function handlePathsChange(paths: (Path2D | null)[]) {
    errorStates = expressions.map((expr, i) => {
      const text = expr.expression.trim();
      if (text === "") return false;
      const isAssignmentOrFunc = /^\s*[a-zA-Z_]\w*\s*=/.test(text) || /^\s*[a-zA-Z_]\w*\s*\(.*\)\s*=/.test(text);
      return !isAssignmentOrFunc && paths[i] === null;
    });
  }

  // --- View state ---
  let windowWidth = $state(typeof window !== "undefined" ? window.innerWidth : 800);
  let windowHeight = $state(typeof window !== "undefined" ? window.innerHeight : 600);

  onMount(() => {
    const handleResize = () => {
      windowWidth = window.innerWidth;
      windowHeight = window.innerHeight;
    };
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  });

  let layoutMode = $derived(
    resolveLayoutMode(windowWidth, windowHeight, typeof window !== "undefined" && window.matchMedia("(hover: hover)").matches, typeof window !== "undefined" && window.matchMedia("(pointer: fine)").matches)
  );

  let scale = $state(100);
  let offsetX = $state(400);
  let offsetY = $state(300);
  let showGrid = $state(true);
  let darkMode = $state(true);
  let showSettings = $state(false);
  let showSidebar = $state(false);
  let engineReady = $state(false);
  let zoomStep = $state(1.12);

  // --- Tunables (acceleration / limits) ---
  const MIN_SCALE = 0.0005;
  const MAX_SCALE = 1e9;
  const ZOOM_BUTTON_STEP = 1.25; // +/- button factor
  const PAN_ACCEL = 1; // >1 faster pan, <1 slower; 1 = 1:1 with cursor
  const ZOOM_ACCEL = 0.5; // smoother wheel zoom

  function clampScale(s: number) {
    return Math.min(MAX_SCALE, Math.max(MIN_SCALE, s));
  }

  function addLine(afterIndex: number) {
    expressions.splice(afterIndex + 1, 0, {
      expression: "",
      color: generateColor(expressions.length),
      visible: true,
    });
  }

  function removeLine(index: number) {
    if (expressions.length <= 1) return;
    expressions.splice(index, 1);
  }

  function updateExpression(index: number, value: string) {
    expressions[index].expression = value;
  }

  function updateColor(index: number, color: string) {
    expressions[index].color = color;
  }

  function toggleVisibility(index: number) {
    expressions[index].visible = !expressions[index].visible;
  }

  function zoomAtPoint(clientX: number, clientY: number, factor: number) {
    const svg = document.querySelector(".graph-canvas");
    if (!svg) return;
    const rect = svg.getBoundingClientRect();
    const width = rect.width;
    const height = rect.height;
    const mx = ((clientX - rect.left) / width) * width;
    const my = ((clientY - rect.top) / height) * height;
    const f = Math.pow(factor, ZOOM_ACCEL);
    const ns = clampScale(scale * f);
    const k = ns / scale;
    offsetX = mx - (mx - offsetX) * k;
    offsetY = my - (my - offsetY) * k;
    scale = ns;
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const normalized = Math.max(-100, Math.min(100, e.deltaY));
    const factor = Math.pow(zoomStep, -normalized / 100);
    zoomAtPoint(e.clientX, e.clientY, factor);
  }

  let tracker = new GestureTracker();
  let isPanning = $state(false);
  let lastPinchDist = 0;
  let singlePanStart = { x: 0, y: 0 };
  let singlePanStartOffset = { x: 0, y: 0 };

  function handlePointerDown(e: PointerEvent) {
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
    tracker.addPointer(e.pointerId, e.clientX, e.clientY);
    if (tracker.getPointerCount() === 1) {
      isPanning = true;
      singlePanStart = { x: e.clientX, y: e.clientY };
      singlePanStartOffset = { x: offsetX, y: offsetY };
    } else if (tracker.getPointerCount() === 2) {
      isPanning = false;
      const pinch = tracker.getPinchInfo();
      lastPinchDist = pinch.distance;
    }
  }

  function handlePointerMove(e: PointerEvent) {
    tracker.updatePointer(e.pointerId, e.clientX, e.clientY);
    const count = tracker.getPointerCount();
    if (count === 2) {
      const pinch = tracker.getPinchInfo();
      if (lastPinchDist > 0 && pinch.distance > 0) {
        const factor = pinch.distance / lastPinchDist;
        zoomAtPoint(pinch.center.x, pinch.center.y, 1 / Math.pow(factor, 0.5));
        lastPinchDist = pinch.distance;
      }
    } else if (count === 1 && isPanning) {
      offsetX = singlePanStartOffset.x + (e.clientX - singlePanStart.x) * PAN_ACCEL;
      offsetY = singlePanStartOffset.y + (e.clientY - singlePanStart.y) * PAN_ACCEL;
    }
  }

  function handlePointerUp(e: PointerEvent) {
    try {
      (e.target as HTMLElement).releasePointerCapture(e.pointerId);
    } catch {}
    tracker.removePointer(e.pointerId);
    if (tracker.getPointerCount() < 2) {
      lastPinchDist = 0;
    }
    if (tracker.getPointerCount() === 0) {
      isPanning = false;
    }
  }


  function resetView() {
    scale = 50;
    const canvas = document.querySelector(".graph-canvas");
    const rect = canvas?.getBoundingClientRect();
    offsetX = (rect?.width ?? 800) / 2;
    offsetY = (rect?.height ?? 600) / 2;
  }

  $effect(() => {
    document.documentElement.dataset.theme = darkMode ? "dark" : "light";
  });

  onMount(async () => {
    try {
      await initEngine();
      engineReady = true;
      resetView();
    } catch (e) {
      console.error("WASM engine failed to load", e);
    }
  });
</script>

<div class="grapher-layout" class:dark={darkMode} class:mobile={layoutMode === 'mobile'}>
  {#if layoutMode !== 'mobile'}
    {#if showSidebar}
      <div
        class="sidebar-scrim"
        role="button"
        tabindex="0"
        aria-label="Close sidebar"
        onclick={() => (showSidebar = false)}
        onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') showSidebar = false; }}
      ></div>
    {/if}

    <Sidebar
      {expressions}
      {addLine}
      {removeLine}
      {updateExpression}
      {updateColor}
      {symbols}
      {showSidebar}
      {errorStates}
      onClose={() => (showSidebar = false)}
    ></Sidebar>
  {/if}

  <main
    class="graph-main"
    onwheel={handleWheel}
    onpointerdown={handlePointerDown}
    onpointermove={handlePointerMove}
    onpointerup={handlePointerUp}
    onpointerleave={handlePointerUp}
  >
    {#if engineReady}
      <GraphCanvas
        {expressions}
        {scale}
        {offsetX}
        {offsetY}
        {showGrid}
        {darkMode}
        onPathsChange={handlePathsChange}
      />
    {:else}
      <div class="loading">Loading Engine…</div>
    {/if}
    <div class="zoom-controls">
      <button
        class="sidebar-toggle"
        onclick={() => (showSidebar = !showSidebar)}
        title="Toggle Sidebar"
        aria-label="Toggle Sidebar">☰</button
      >
      <button
        class="settings-btn"
        onclick={() => (showSettings = !showSettings)}
        title="Settings"
        aria-label="Toggle settings">⚙️</button
      >
      <button onclick={() => {
        const canvas = document.querySelector(".graph-canvas");
        const rect = canvas?.getBoundingClientRect();
        zoomAtPoint((rect?.left ?? 0) + (rect?.width ?? 800) / 2, (rect?.top ?? 0) + (rect?.height ?? 600) / 2, ZOOM_BUTTON_STEP);
      }}>+</button
      >
      <button onclick={() => {
        const canvas = document.querySelector(".graph-canvas");
        const rect = canvas?.getBoundingClientRect();
        zoomAtPoint((rect?.left ?? 0) + (rect?.width ?? 800) / 2, (rect?.top ?? 0) + (rect?.height ?? 600) / 2, 1 / ZOOM_BUTTON_STEP);
      }}>-</button
      >
    </div>
    <SettingsCard
      bind:showGrid
      bind:darkMode
      bind:zoomStep
      bind:visible={showSettings}
      onResetView={resetView}
    />
    {#if layoutMode === 'mobile'}
      <BottomSheet>
        <EquationList
          {expressions}
          {addLine}
          {removeLine}
          {updateExpression}
          {updateColor}
          {symbols}
          errors={errorStates}
        />
      </BottomSheet>
    {/if}
  </main>
</div>

<style>
  :global(:root) {
    --bg: #ffffff;
    --fg: #1e293b;
    --muted: #64748b;
    --border: #e2e8f0;
    --grid: #e2e8f0;
    --grid-major: #cbd5e1;
    --axis: #475569;
    --accent: #10b981;
    --panel: #ffffff;
    --panel-2: #f8fafc;
    --row-border: #f1f5f9;
  }
  :global([data-theme="dark"]) {
    --bg: #0f172a;
    --fg: #e2e8f0;
    --muted: #94a3b8;
    --border: #334155;
    --grid: #1e293b;
    --grid-major: #334155;
    --axis: #94a3b8;
    --accent: #34d399;
    --panel: #1e293b;
    --panel-2: #0f172a;
    --row-border: #1e293b;
  }

  .grapher-layout {
    display: grid;
    grid-template-columns: 320px 1fr;
    height: 100vh;
    box-sizing: border-box;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
    background: var(--bg);
    color: var(--fg);
    position: relative;
    overflow: hidden;
  }

  .graph-main {
    position: relative;
    overflow: hidden;
    background: var(--bg);
    touch-action: none;
    user-select: none;
  }

  .loading {
    display: grid;
    place-items: center;
    height: 100%;
    color: var(--muted);
  }

  .zoom-controls {
    position: absolute;
    top: 1rem;
    right: 1rem;
    display: flex;
    flex-direction: column;
    gap: 2px;
    z-index: 10;
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 4px;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
  }
  .zoom-controls button {
    width: 32px;
    height: 32px;
    background: none;
    border: none;
    font-size: 1.2rem;
    font-weight: bold;
    color: var(--fg);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .zoom-controls button:hover {
    background: var(--border);
  }

  .sidebar-toggle {
    display: none;
  }

  .settings-btn {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: var(--fg);
  }

  .sidebar-scrim {
    display: none;
  }

  @media (max-width: 900px) {
    .grapher-layout {
      grid-template-columns: 1fr;
    }
    .sidebar-toggle {
      display: flex;
    }
    .sidebar-scrim {
      display: block;
      position: absolute;
      inset: 0;
      background: rgba(0, 0, 0, 0.4);
      z-index: 15;
    }
  }

  @media (max-width: 600px) {
    .zoom-controls button {
      width: 48px;
      height: 48px;
      font-size: 1.5rem;
    }
  }
</style>
