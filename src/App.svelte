<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "./layouts/Sidebar.svelte";
  import GraphCanvas from "./components/GraphCanvas.svelte";
  import SettingsCard from "./components/SettingsCard.svelte";
  import initEngine from "./wasm/grapher_engine.js";
  import { generateColor } from "./utils/colors";
  import type { ExpressionLine } from "./types.ts";

  let expressions = $state<ExpressionLine[]>([
    { expression: "", color: generateColor(0) },
    { expression: "", color: generateColor(1) },
  ]);

  // Symbol table derived from assignment lines (top-down evaluation).
  let symbols: Record<string, number> = {};

  // --- View state ---
  let scale = $state(100);
  let offsetX = $state(400);
  let offsetY = $state(300);
  let showGrid = $state(true);
  let darkMode = $state(true);
  let showSettings = $state(false);
  let engineReady = $state(false);

  // --- Tunables (acceleration / limits) ---
  const MIN_SCALE = 0.0005;
  const MAX_SCALE = 1e9;
  const ZOOM_STEP = 1.05; // wheel zoom factor per notch
  const ZOOM_BUTTON_STEP = 1.25; // +/- button factor
  const PAN_ACCEL = 1; // >1 faster pan, <1 slower; 1 = 1:1 with cursor
  const ZOOM_ACCEL = 1; // exponent on wheel delta (1 = normal, 2 = snappier)

  function clampScale(s: number) {
    return Math.min(MAX_SCALE, Math.max(MIN_SCALE, s));
  }

  function addLine(afterIndex: number) {
    expressions.splice(afterIndex + 1, 0, {
      expression: "",
      color: generateColor(expressions.length),
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

  function zoomAtPoint(clientX: number, clientY: number, factor: number) {
    const svg = document.querySelector(".graph-canvas");
    if (!svg) return;
    const rect = svg.getBoundingClientRect();
    const mx = ((clientX - rect.left) / rect.width) * 800;
    const my = ((clientY - rect.top) / rect.height) * 600;
    const f = Math.pow(factor, ZOOM_ACCEL);
    const ns = clampScale(scale * f);
    const k = ns / scale;
    offsetX = mx - (mx - offsetX) * k;
    offsetY = my - (my - offsetY) * k;
    scale = ns;
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    zoomAtPoint(e.clientX, e.clientY, e.deltaY < 0 ? ZOOM_STEP : 1 / ZOOM_STEP);
  }
  let initialZoom = {
    x: 0,
    y: 0,
  };
  function handleTouchZoomStart(e: TouchEvent) {
    if (e.touches.length == 2) {
      let x = 0,
        y = 0;
      x = Math.abs(
        (e.targetTouches.item(0)?.clientX ?? 0) -
          (e.targetTouches.item(1)?.clientX ?? 0),
      );
      y = Math.abs(
        (e.targetTouches.item(0)?.clientY ?? 0) -
          (e.targetTouches.item(1)?.clientY ?? 0),
      );
      initialZoom = {
        x,
        y,
      };
    }
  }
  function handleTouchZoom(e: TouchEvent) {
    if (e.touches.length == 2) {
      const t0 = e.targetTouches.item(0)!;
      const t1 = e.targetTouches.item(1)!;
      const mx = (t0.clientX + t1.clientX) / 2;
      const my = (t0.clientY + t1.clientY) / 2;
      const dx = t0.clientX - t1.clientX;
      const dy = t0.clientY - t1.clientY;
      const dist = Math.hypot(dx, dy);
      const zoomIn = dist > initialZoom.x;
      zoomAtPoint(mx, my, zoomIn ? ZOOM_STEP : 1 / ZOOM_STEP);
    }
  }
  function handleTouchEnd() {
    initialZoom = {
      x: 0,
      y: 0,
    };
  }

  let isPanning = $state(false);
  let panStart = { x: 0, y: 0 };
  let panStartOffset = { x: 0, y: 0 };

  function onPointerDown(e: MouseEvent) {
    isPanning = true;
    panStart = { x: e.clientX, y: e.clientY };
    panStartOffset = { x: offsetX, y: offsetY };
  }
  function onPointerMove(e: MouseEvent) {
    if (!isPanning) return;
    const svg = document.querySelector(".graph-canvas");
    const ratio = svg ? 800 / svg.getBoundingClientRect().width : 1;
    offsetX = panStartOffset.x + (e.clientX - panStart.x) * ratio * PAN_ACCEL;
    offsetY = panStartOffset.y + (e.clientY - panStart.y) * ratio * PAN_ACCEL;
  }
  function onPointerUp(e: MouseEvent) {
    isPanning = false;
  }

  function resetView() {
    scale = 50;
    offsetX = 400;
    offsetY = 300;
  }

  $effect(() => {
    document.documentElement.dataset.theme = darkMode ? "dark" : "light";
  });

  onMount(async () => {
    try {
      await initEngine();
      engineReady = true;
    } catch (e) {
      console.error("WASM engine failed to load", e);
    }
  });
</script>

<div class="grapher-layout" class:dark={darkMode}>
  <Sidebar
    {expressions}
    {addLine}
    {removeLine}
    {updateExpression}
    {updateColor}
    {symbols}
  ></Sidebar>

  <main
    class="graph-main"
    onwheel={handleWheel}
    ontouchstart={handleTouchZoomStart}
    ontouchmove={handleTouchZoom}
    ontouchend={handleTouchEnd}
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
    onpointerleave={onPointerUp}
  >
    {#if engineReady}
      <GraphCanvas
        {expressions}
        {scale}
        {offsetX}
        {offsetY}
        {showGrid}
        {darkMode}
      />
    {:else}
      <div class="loading">Loading Engine…</div>
    {/if}
    <div class="zoom-controls">
      <button
        class="settings-btn"
        onclick={() => (showSettings = !showSettings)}
        title="Settings"
        aria-label="Toggle settings">⚙️</button
      >
      <button onclick={() => zoomAtPoint(400, 300, ZOOM_BUTTON_STEP)}>+</button>
      <button onclick={() => zoomAtPoint(400, 300, 1 / ZOOM_BUTTON_STEP)}
        >-</button
      >
    </div>
    <SettingsCard
      bind:showGrid
      bind:darkMode
      bind:visible={showSettings}
      onResetView={resetView}
    />
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
  }
  .zoom-controls button:hover {
    background: var(--border);
  }

  .settings-btn {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: var(--fg);
  }
</style>
