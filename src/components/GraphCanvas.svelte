<script lang="ts">
  import { sampleAllToPaths, generateGridLines } from "../utils/math";
  import type { ExpressionLine } from "../types.ts";

  let {
    expressions,
    scale,
    offsetX,
    offsetY,
    innerWidth,
    innerHeight,
    showGrid,
    darkMode,
  }: {
    expressions: ExpressionLine[];
    scale: number;
    offsetX: number;
    offsetY: number;
    innerHeight: number;
    innerWidth: number;
    showGrid: boolean;
    darkMode: boolean;
  } = $props();

  let sampledPaths = $derived(
    sampleAllToPaths(
      expressions.map((e) => e.expression),
      scale,
      offsetX,
      offsetY,
    ),
  );
</script>

<svg
  class="graph-canvas"
  width={outerWidth}
  height={outerHeight}
  viewBox={`0 0 ${innerWidth} ${innerHeight}`}
  preserveAspectRatio="xMidYMid meet"
  data-dark={darkMode}
>
  {#if showGrid}
    {#each generateGridLines(scale, offsetX, innerWidth) as line (line.pos)}
      {#if line.isMajor}
        <g>
          <line
            x1={line.pos}
            y1="0"
            x2={line.pos}
            y2={innerHeight}
            class="grid-major"
          />
          <text
            x={line.pos}
            y={offsetY < 15
              ? 15
              : offsetY > innerHeight - 15
                ? innerHeight - 15
                : offsetY - 5}
            class="axis-label"
            font-size="50"
            font-weight="900"
            text-anchor="middle">{line.label}</text
          >
        </g>
      {:else}
        <line
          x1={line.pos}
          y1="0"
          x2={line.pos}
          y2={innerHeight}
          class="grid-minor"
        />
      {/if}
    {/each}
    {#each generateGridLines(scale, offsetY, innerHeight) as line (line.pos)}
      {#if line.isMajor}
        <g>
          <line
            x1="0"
            y1={line.pos}
            x2={innerWidth}
            y2={line.pos}
            class="grid-major"
          />
          <text
            x={offsetX < 25
              ? 25
              : offsetX > innerWidth - 15
                ? innerWidth - 15
                : offsetX + 5}
            y={line.pos + 4}
            class="axis-label"
            font-weight="900"
            dominant-baseline="middle">{-line.label}</text
          >
        </g>
      {:else}
        <line
          x1="0"
          y1={line.pos}
          x2={innerWidth}
          y2={line.pos}
          class="grid-minor"
        />
      {/if}
    {/each}
  {/if}
  <line x1="0" y1={offsetY} x2={innerWidth} y2={offsetY} class="axis" />
  <line x1={offsetX} y1="0" x2={offsetX} y2={innerHeight} class="axis" />

  {#each expressions as expr, i (i)}
    {#if sampledPaths[i] && expr.expression.trim()}
      <path
        d={sampledPaths[i]}
        fill="none"
        stroke={expr.color}
        stroke-width="2.5"
      />
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
