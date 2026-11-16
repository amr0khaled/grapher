<script lang="ts">
  import EquationList from "../components/EquationList.svelte";
  import type { ExpressionLine } from "../types.ts";

  let {
    expressions,
    addLine,
    removeLine,
    updateExpression,
    updateColor,
    symbols,
  }: {
    expressions: ExpressionLine[];
    addLine: (afterIndex: number) => void;
    removeLine: (index: number) => void;
    updateExpression: (index: number, value: string) => void;
    updateColor: (index: number, color: string) => void;
    symbols: Record<string, number>;
  } = $props();
</script>

<aside class="sidebar">
  <header class="sidebar-header">
    <h2>Grapher</h2>
  </header>
  <EquationList
    {expressions}
    {addLine}
    {removeLine}
    {updateExpression}
    {updateColor}
    {symbols}
  />
</aside>

<style>
  .sidebar {
    border-right: 1px solid var(--border);
    background: var(--panel);
    color: var(--fg);
    padding: 1rem;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    box-sizing: border-box;
  }
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }
  .sidebar-header h2 {
    margin: 0;
    font-size: 1.2rem;
    font-family: 'Cambria Math', 'MathJax_Math', 'Times New Roman', serif;
    font-weight: bold;
  }
</style>
