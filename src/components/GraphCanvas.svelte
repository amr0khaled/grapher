<script lang="ts">
  import { onMount } from "svelte";
  import { sampleToPath2D, generateGridLines, getPinnedAxisPositions } from "../utils/math";
  import { preserveCenterOnResize } from "../utils/viewport";
  import type { ExpressionLine } from "../types.ts";

  let {
    expressions,
    scale,
    offsetX = $bindable(0),
    offsetY = $bindable(0),
    showGrid,
    darkMode,
  }: {
    expressions: ExpressionLine[];
    scale: number;
    offsetX: number;
    offsetY: number;
    showGrid: boolean;
    darkMode: boolean;
  } = $props();

  let canvasEl: HTMLCanvasElement | null = null;
  let pathCache = document.createElement("canvas");
  let cssWidth = $state(800);
  let cssHeight = $state(600);
  let dpr = $state(typeof window !== "undefined" ? window.devicePixelRatio || 1 : 1);

  let isInteracting = $state(false);
  let transform = { k: 1, dx: 0, dy: 0 };
  let debounceTimer: number | null = null;

  let baseScale = 1;
  let baseOffsetX = 0;
  let baseOffsetY = 0;

  onMount(() => {
    if (!canvasEl) return;
    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        if (width > 0 && height > 0) {
          const newOffsets = preserveCenterOnResize(cssWidth, cssHeight, width, height, scale, offsetX, offsetY);
          offsetX = newOffsets.offsetX;
          offsetY = newOffsets.offsetY;
          cssWidth = width;
          cssHeight = height;
          renderPrecise();
        }
      }
    });
    observer.observe(canvasEl);
    return () => observer.disconnect();
  });

  function drawGridAndAxes(
    ctx: CanvasRenderingContext2D,
    curScale: number,
    curOffsetX: number,
    curOffsetY: number
  ) {
    const style = getComputedStyle(document.documentElement);
    const bgCol = style.getPropertyValue("--bg").trim() || (darkMode ? "#0f172a" : "#ffffff");
    const gridMinorCol = style.getPropertyValue("--grid").trim() || (darkMode ? "#1e293b" : "#e2e8f0");
    const gridMajorCol = style.getPropertyValue("--grid-major").trim() || (darkMode ? "#334155" : "#cbd5e1");
    const axisCol = style.getPropertyValue("--axis").trim() || (darkMode ? "#94a3b8" : "#475569");
    const mutedCol = style.getPropertyValue("--muted").trim() || (darkMode ? "#94a3b8" : "#64748b");

    ctx.fillStyle = bgCol;
    ctx.fillRect(0, 0, cssWidth, cssHeight);

    if (showGrid) {
      const { axisX, axisY } = getPinnedAxisPositions(curOffsetX, curOffsetY, cssWidth, cssHeight, 20);

      const xLines = generateGridLines(curScale, curOffsetX, cssWidth);
      for (const line of xLines) {
        ctx.beginPath();
        ctx.strokeStyle = line.isMajor ? gridMajorCol : gridMinorCol;
        ctx.lineWidth = line.isMajor ? 1.5 : 1;
        ctx.moveTo(Math.floor(line.pos) + 0.5, 0);
        ctx.lineTo(Math.floor(line.pos) + 0.5, cssHeight);
        ctx.stroke();

        if (line.isMajor) {
          ctx.fillStyle = mutedCol;
          ctx.font = "bold 11px sans-serif";
          ctx.textAlign = "center";
          ctx.textBaseline = "alphabetic";
          const labelY = axisY < 25 ? 25 : axisY > cssHeight - 25 ? cssHeight - 15 : axisY - 5;
          ctx.fillText(String(line.label), line.pos, labelY);
        }
      }

      const yLines = generateGridLines(curScale, curOffsetY, cssHeight);
      for (const line of yLines) {
        ctx.beginPath();
        ctx.strokeStyle = line.isMajor ? gridMajorCol : gridMinorCol;
        ctx.lineWidth = line.isMajor ? 1.5 : 1;
        ctx.moveTo(0, Math.floor(line.pos) + 0.5);
        ctx.lineTo(cssWidth, Math.floor(line.pos) + 0.5);
        ctx.stroke();

        if (line.isMajor) {
          ctx.fillStyle = mutedCol;
          ctx.font = "bold 11px sans-serif";
          ctx.textAlign = axisX < 35 ? "left" : axisX > cssWidth - 35 ? "right" : "left";
          ctx.textBaseline = "middle";
          const labelX = axisX < 35 ? 30 : axisX > cssWidth - 35 ? cssWidth - 30 : axisX + 6;
          ctx.fillText(String(-line.label), labelX, line.pos);
        }
      }
    }

    ctx.beginPath();
    ctx.strokeStyle = axisCol;
    ctx.lineWidth = 1.5;
    const { axisX, axisY } = getPinnedAxisPositions(curOffsetX, curOffsetY, cssWidth, cssHeight, 0);
    ctx.moveTo(0, Math.floor(axisY) + 0.5);
    ctx.lineTo(cssWidth, Math.floor(axisY) + 0.5);
    ctx.moveTo(Math.floor(axisX) + 0.5, 0);
    ctx.lineTo(Math.floor(axisX) + 0.5, cssHeight);
    ctx.stroke();
  }

  function renderPrecise() {
    if (!canvasEl || cssWidth <= 0 || cssHeight <= 0) return;
    const ctx = pathCache.getContext("2d");
    if (!ctx) return;

    const backingWidth = Math.round(cssWidth * dpr);
    const backingHeight = Math.round(cssHeight * dpr);
    if (pathCache.width !== backingWidth || pathCache.height !== backingHeight) {
      pathCache.width = backingWidth;
      pathCache.height = backingHeight;
    }

    ctx.clearRect(0, 0, pathCache.width, pathCache.height);
    ctx.save();
    ctx.scale(dpr, dpr);

    const paths = sampleToPath2D(
      expressions.map((e) => e.expression),
      scale,
      offsetX,
      offsetY,
      cssWidth,
      cssHeight,
    );

    for (let i = 0; i < expressions.length; i++) {
      const expr = expressions[i];
      const path = paths[i];
      if (path && expr.expression.trim() && expr.visible !== false) {
        ctx.save();
        ctx.strokeStyle = expr.color;
        ctx.lineWidth = 2.5;
        ctx.lineJoin = "round";
        ctx.lineCap = "round";
        ctx.stroke(path);
        ctx.restore();
      }
    }

    ctx.restore();
    renderTransient();
  }

  function renderTransient() {
    if (!canvasEl) return;
    const ctx = canvasEl.getContext("2d");
    if (!ctx) return;

    const backingWidth = Math.round(cssWidth * dpr);
    const backingHeight = Math.round(cssHeight * dpr);
    if (canvasEl.width !== backingWidth || canvasEl.height !== backingHeight) {
      canvasEl.width = backingWidth;
      canvasEl.height = backingHeight;
    }

    ctx.clearRect(0, 0, canvasEl.width, canvasEl.height);
    ctx.save();
    ctx.scale(dpr, dpr);
    drawGridAndAxes(ctx, scale, offsetX, offsetY);
    ctx.restore();

    ctx.save();
    ctx.setTransform(transform.k, 0, 0, transform.k, transform.dx * dpr, transform.dy * dpr);
    ctx.drawImage(pathCache, 0, 0);
    ctx.restore();
  }

  $effect(() => {
    if (!isInteracting) {
      isInteracting = true;
      baseScale = scale;
      baseOffsetX = offsetX;
      baseOffsetY = offsetY;
    }

    transform.k = scale / baseScale;
    transform.dx = offsetX - baseOffsetX * transform.k;
    transform.dy = offsetY - baseOffsetY * transform.k;

    renderTransient();

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = window.setTimeout(() => {
      isInteracting = false;
      transform = { k: 1, dx: 0, dy: 0 };
      renderPrecise();
    }, 120);
  });
</script>

<canvas
  bind:this={canvasEl}
  class="graph-canvas"
  data-dark={darkMode}
></canvas>

<style>
  .graph-canvas {
    width: 100%;
    height: 100%;
    display: block;
    background: var(--bg);
    cursor: grab;
    touch-action: none;
    user-select: none;
  }
  .graph-canvas:active {
    cursor: grabbing;
  }
</style>
