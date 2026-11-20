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
  let cssWidth = $state(800);
  let cssHeight = $state(600);
  let dpr = $state(typeof window !== "undefined" ? window.devicePixelRatio || 1 : 1);

  // Transient CSS transform for snappy panning/zooming without immediate re-sampling
  let isInteracting = $state(false);
  let transformStyle = $state("");
  let debounceTimer: number | null = null;

  // Track base view state when interaction starts
  let baseScale = 0;
  let baseOffsetX = 0;
  let baseOffsetY = 0;

  // ResizeObserver to track container sizing dynamically
  onMount(() => {
    if (!canvasEl) return;
    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        if (width > 0 && height > 0) {
          cssWidth = width;
          cssHeight = height;
        }
      }
    });
    observer.observe(canvasEl);
    return () => observer.disconnect();
  });

  // Detect scale/offset changes to handle transient CSS transform vs precise redraw
  $effect(() => {
    // If scale or offset changes, trigger debounce
    if (!isInteracting) {
      isInteracting = true;
      baseScale = scale;
      baseOffsetX = offsetX;
      baseOffsetY = offsetY;
    }

    // Compute relative transform matrix string for GPU acceleration during gesture
    const k = scale / baseScale;
    const dx = offsetX - baseOffsetX * k;
    const dy = offsetY - baseOffsetY * k;
    transformStyle = `matrix(${k}, 0, 0, ${k}, ${dx}, ${dy})`;

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = window.setTimeout(() => {
      isInteracting = false;
      transformStyle = "";
      baseScale = scale;
      baseOffsetX = offsetX;
      baseOffsetY = offsetY;
    }, 120);
  });

  // Main canvas render loop
  $effect(() => {
    if (!canvasEl || cssWidth <= 0 || cssHeight <= 0) return;
    const ctx = canvasEl.getContext("2d");
    if (!ctx) return;

    // Adjust backing store resolution for DPR
    const backingWidth = Math.round(cssWidth * dpr);
    const backingHeight = Math.round(cssHeight * dpr);
    if (canvasEl.width !== backingWidth || canvasEl.height !== backingHeight) {
      canvasEl.width = backingWidth;
      canvasEl.height = backingHeight;
    }

    ctx.save();
    ctx.scale(dpr, dpr);

    // Read current CSS theme colors dynamically
    const style = getComputedStyle(document.documentElement);
    const bgCol = style.getPropertyValue("--bg").trim() || (darkMode ? "#0f172a" : "#ffffff");
    const gridMinorCol = style.getPropertyValue("--grid").trim() || (darkMode ? "#1e293b" : "#e2e8f0");
    const gridMajorCol = style.getPropertyValue("--grid-major").trim() || (darkMode ? "#334155" : "#cbd5e1");
    const axisCol = style.getPropertyValue("--axis").trim() || (darkMode ? "#94a3b8" : "#475569");
    const mutedCol = style.getPropertyValue("--muted").trim() || (darkMode ? "#94a3b8" : "#64748b");

    // Clear background
    ctx.fillStyle = bgCol;
    ctx.fillRect(0, 0, cssWidth, cssHeight);

    // Draw Grid & Axes
    if (showGrid) {
      // Minor/Major vertical grid lines
      const xLines = generateGridLines(scale, offsetX, cssWidth);
      for (const line of xLines) {
        ctx.beginPath();
        ctx.strokeStyle = line.isMajor ? gridMajorCol : gridMinorCol;
        ctx.lineWidth = line.isMajor ? 1.5 : 1;
        ctx.moveTo(line.pos, 0);
        ctx.lineTo(line.pos, cssHeight);
        ctx.stroke();

        if (line.isMajor) {
          ctx.fillStyle = mutedCol;
          ctx.font = "bold 11px sans-serif";
          ctx.textAlign = "center";
          ctx.textBaseline = "alphabetic";
          const labelY = offsetY < 15 ? 15 : offsetY > cssHeight - 15 ? cssHeight - 15 : offsetY - 5;
          ctx.fillText(String(line.label), line.pos, labelY);
        }
      }

      // Minor/Major horizontal grid lines
      const yLines = generateGridLines(scale, offsetY, cssHeight);
      for (const line of yLines) {
        ctx.beginPath();
        ctx.strokeStyle = line.isMajor ? gridMajorCol : gridMinorCol;
        ctx.lineWidth = line.isMajor ? 1.5 : 1;
        ctx.moveTo(0, line.pos);
        ctx.lineTo(cssWidth, line.pos);
        ctx.stroke();

        if (line.isMajor) {
          ctx.fillStyle = mutedCol;
          ctx.font = "bold 11px sans-serif";
          ctx.textAlign = offsetX < 25 ? "left" : offsetX > cssWidth - 15 ? "right" : "left";
          ctx.textBaseline = "middle";
          const labelX = offsetX < 25 ? 25 : offsetX > cssWidth - 15 ? cssWidth - 15 : offsetX + 5;
          ctx.fillText(String(-line.label), labelX, line.pos);
        }
      }
    }

    // Main Axes
    ctx.beginPath();
    ctx.strokeStyle = axisCol;
    ctx.lineWidth = 1.5;
    // X axis (horizontal line at y = offsetY)
    ctx.moveTo(0, offsetY);
    ctx.lineTo(cssWidth, offsetY);
    // Y axis (vertical line at x = offsetX)
    ctx.moveTo(offsetX, 0);
    ctx.lineTo(offsetX, cssHeight);
    ctx.stroke();

    // Sample and draw curves via Path2D
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
      if (path && expr.expression.trim()) {
        ctx.save();
        ctx.strokeStyle = expr.color;
        ctx.lineWidth = 2.5;
        ctx.stroke(path);
        ctx.restore();
      }
    }

    ctx.restore();
  });
</script>

<canvas
  bind:this={canvasEl}
  class="graph-canvas"
  style:transform={transformStyle}
  style:transform-origin="0px 0px"
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
    will-change: transform;
  }
  .graph-canvas:active {
    cursor: grabbing;
  }
</style>
