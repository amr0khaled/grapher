<script lang="ts">
  import { onMount } from "svelte";
  import { sampleToPath2D, generateGridLines } from "../utils/math";
  import type { ExpressionLine } from "../types.ts";

  let {
    expressions,
    scale,
    offsetX,
    offsetY,
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
  let cacheCanvas = document.createElement("canvas");
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
          cssWidth = width;
          cssHeight = height;
          renderPrecise();
        }
      }
    });
    observer.observe(canvasEl);
    return () => observer.disconnect();
  });

  function renderPrecise() {
    if (!canvasEl || cssWidth <= 0 || cssHeight <= 0) return;
    const ctx = cacheCanvas.getContext("2d");
    if (!ctx) return;

    // 3x overscan to prevent blank edges during pan/zoom
    const padX = cssWidth;
    const padY = cssHeight;
    const overWidth = cssWidth + padX * 2;
    const overHeight = cssHeight + padY * 2;

    const backingWidth = Math.round(overWidth * dpr);
    const backingHeight = Math.round(overHeight * dpr);
    if (cacheCanvas.width !== backingWidth || cacheCanvas.height !== backingHeight) {
      cacheCanvas.width = backingWidth;
      cacheCanvas.height = backingHeight;
    }

    ctx.save();
    ctx.scale(dpr, dpr);

    // Offset drawing so current viewport is centered in 3x cache
    ctx.translate(padX, padY);

    const style = getComputedStyle(document.documentElement);
    const bgCol = style.getPropertyValue("--bg").trim() || (darkMode ? "#0f172a" : "#ffffff");
    const gridMinorCol = style.getPropertyValue("--grid").trim() || (darkMode ? "#1e293b" : "#e2e8f0");
    const gridMajorCol = style.getPropertyValue("--grid-major").trim() || (darkMode ? "#334155" : "#cbd5e1");
    const axisCol = style.getPropertyValue("--axis").trim() || (darkMode ? "#94a3b8" : "#475569");
    const mutedCol = style.getPropertyValue("--muted").trim() || (darkMode ? "#94a3b8" : "#64748b");

    ctx.fillStyle = bgCol;
    ctx.fillRect(-padX, -padY, overWidth, overHeight);

    if (showGrid) {
      // Sample grid lines for the full overscan range
      const xLines = generateGridLines(scale, offsetX, overWidth);
      for (const line of xLines) {
        const x = line.pos - padX; // Adjust for ctx.translate
        ctx.beginPath();
        ctx.strokeStyle = line.isMajor ? gridMajorCol : gridMinorCol;
        ctx.lineWidth = line.isMajor ? 1.5 : 1;
        ctx.moveTo(Math.floor(x) + 0.5, -padY);
        ctx.lineTo(Math.floor(x) + 0.5, cssHeight + padY);
        ctx.stroke();

        if (line.isMajor && x >= 0 && x <= cssWidth) {
          ctx.fillStyle = mutedCol;
          ctx.font = "bold 11px sans-serif";
          ctx.textAlign = "center";
          const labelY = offsetY < 15 ? 15 : offsetY > cssHeight - 15 ? cssHeight - 15 : offsetY - 5;
          ctx.fillText(String(line.label), x, labelY);
        }
      }

      const yLines = generateGridLines(scale, offsetY, overHeight);
      for (const line of yLines) {
        const y = line.pos - padY;
        ctx.beginPath();
        ctx.strokeStyle = line.isMajor ? gridMajorCol : gridMinorCol;
        ctx.lineWidth = line.isMajor ? 1.5 : 1;
        ctx.moveTo(-padX, Math.floor(y) + 0.5);
        ctx.lineTo(cssWidth + padX, Math.floor(y) + 0.5);
        ctx.stroke();

        if (line.isMajor && y >= 0 && y <= cssHeight) {
          ctx.fillStyle = mutedCol;
          ctx.font = "bold 11px sans-serif";
          ctx.textAlign = offsetX < 25 ? "left" : offsetX > cssWidth - 15 ? "right" : "left";
          ctx.textBaseline = "middle";
          const labelX = offsetX < 25 ? 25 : offsetX > cssWidth - 15 ? cssWidth - 15 : offsetX + 5;
          ctx.fillText(String(-line.label), labelX, y);
        }
      }
    }

    ctx.beginPath();
    ctx.strokeStyle = axisCol;
    ctx.lineWidth = 1.5;
    ctx.moveTo(-padX, Math.floor(offsetY) + 0.5);
    ctx.lineTo(cssWidth + padX, Math.floor(offsetY) + 0.5);
    ctx.moveTo(Math.floor(offsetX) + 0.5, -padY);
    ctx.lineTo(Math.floor(offsetX) + 0.5, cssHeight + padY);
    ctx.stroke();

    const paths = sampleToPath2D(
      expressions.map((e) => e.expression),
      scale,
      offsetX,
      offsetY,
      overWidth,
      overHeight,
    );

    for (let i = 0; i < expressions.length; i++) {
      const expr = expressions[i];
      const path = paths[i];
      if (path && expr.expression.trim()) {
        ctx.save();
        ctx.translate(-padX, -padY); // Sample covers overWidth, so align to top-left of overscan
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

    // Apply relative interaction transform
    ctx.translate(transform.dx, transform.dy);
    // Scale around current center? No, simpler: scale relative to base view
    ctx.scale(transform.k, transform.k);

    // Draw 3x cache so its original center aligns with viewport top-left
    // Original center was at (padX, padY) in cache coords.
    // We want that center to stay at (0,0) in viewport space if no pan happened.
    ctx.drawImage(cacheCanvas, -cssWidth, -cssHeight, cssWidth * 3, cssHeight * 3, -cssWidth, -cssHeight, cssWidth * 3, cssHeight * 3);

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
