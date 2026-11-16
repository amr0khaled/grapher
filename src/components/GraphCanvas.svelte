<script lang="ts">
  import { sampleToPath, generateGridLines } from "../utils/math";
  import { substituteSymbols } from "../utils/symbols";
  import type { ExpressionLine } from "../types.ts";

  let {
    expressions,
    symbols = {},
    scale,
    offsetX,
    offsetY,
    showGrid,
    darkMode,
  }: {
    expressions: ExpressionLine[];
    symbols?: Record<string, number>;
    scale: number;
    offsetX: number;
    offsetY: number;
    showGrid: boolean;
    darkMode: boolean;
  } = $props();
</script>

<svg class="graph-canvas" viewBox="0 0 800 600" data-dark={darkMode}>
  {#if showGrid}
    {#each generateGridLines(scale, offsetX, 800) as line (line.pos)}
      {#if line.isMajor}
        <g>
          <line
            x1={line.pos}
            y1="0"
            x2={line.pos}
            y2="600"
            class="grid-major"
          />
          <text
            x={line.pos}
            y={offsetY < 15 ? 15 : offsetY > 585 ? 585 : offsetY - 5}
            class="axis-label"
            text-anchor="middle">{line.label}</text
          >
        </g>
      {:else}
        <line x1={line.pos} y1="0" x2={line.pos} y2="600" class="grid-minor" />
      {/if}
    {/each}
    {#each generateGridLines(scale, offsetY, 600) as line (line.pos)}
      {#if line.isMajor}
        <g>
          <line
            x1="0"
            y1={line.pos}
            x2="800"
            y2={line.pos}
            class="grid-major"
          />
          <text
            x={offsetX < 25 ? 25 : offsetX > 775 ? 775 : offsetX + 5}
            y={line.pos + 4}
            class="axis-label"
            dominant-baseline="middle">{-line.label}</text
          >
        </g>
      {:else}
        <line x1="0" y1={line.pos} x2="800" y2={line.pos} class="grid-minor" />
      {/if}
    {/each}
  {/if}
  <line x1="0" y1={offsetY} x2="800" y2={offsetY} class="axis" />
  <line x1={offsetX} y1="0" x2={offsetX} y2="600" class="axis" />

  {#each expressions as expr, i (i)}
    {@const d = sampleToPath(expr.expression, scale, offsetX, offsetY)}
    {#if d}
      <path {d} fill="none" stroke={expr.color} stroke-width="2.5" />
    {/if}
  {/each}
</svg>

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
  .grid-minor {
    stroke: var(--grid);
    stroke-width: 1;
  }
  .grid-major {
    stroke: var(--grid-major);
    stroke-width: 1.5;
  }
  .axis {
    stroke: var(--axis);
    stroke-width: 1.5;
  }
  .axis-label {
    fill: var(--muted);
    font-size: 11px;
    font-family: "Cambria Math", "MathJax_Math", "Times New Roman", serif;
    pointer-events: none;
  }
</style>
