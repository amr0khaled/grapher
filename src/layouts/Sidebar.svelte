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
    showSidebar,
    errorStates = [],
    onClose,
  }: {
    expressions: ExpressionLine[];
    addLine: (afterIndex: number) => void;
    removeLine: (index: number) => void;
    updateExpression: (index: number, value: string) => void;
    updateColor: (index: number, color: string) => void;
    symbols: Record<string, number>;
    showSidebar: boolean;
    errorStates?: boolean[];
    onClose: () => void;
  } = $props();
</script>

<aside class="sidebar" class:open={showSidebar}>
  <header class="sidebar-header">
    <h2>Grapher</h2>
    <button class="close-drawer-btn" onclick={onClose} aria-label="Close sidebar">✕</button>
  </header>
  <EquationList
    {expressions}
    {addLine}
    {removeLine}
    {updateExpression}
    {updateColor}
    {symbols}
    errors={errorStates}
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
  .close-drawer-btn {
    display: none;
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: var(--fg);
  }

  @media (max-width: 900px) {
    .sidebar {
      position: fixed;
      top: 0;
      left: 0;
      bottom: 0;
      width: 85vw;
      max-width: 340px;
      z-index: 20;
      transform: translateX(-100%);
      transition: transform 200ms ease;
      box-shadow: 2px 0 12px rgba(0, 0, 0, 0.25);
    }
    .sidebar.open {
      transform: translateX(0);
    }
    .close-drawer-btn {
      display: inline-block;
    }
  }
</style>
