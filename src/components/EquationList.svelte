<script lang="ts">
  import EquationInput from "./EquationInput.svelte";
  import type { ExpressionLine } from "../types.ts";

  let {
    expressions,
    addLine,
    removeLine,
    updateExpression,
    updateColor,
    symbols,
    errors = [],
  }: {
    expressions: ExpressionLine[];
    addLine: (afterIndex: number) => void;
    removeLine: (index: number) => void;
    updateExpression: (index: number, value: string) => void;
    updateColor: (index: number, color: string) => void;
    symbols: Record<string, number>;
    errors?: boolean[];
  } = $props();
</script>

<div class="equation-list">
  {#each expressions as expr, i (i)}
    <EquationInput
      value={expr.expression}
      color={expr.color}
      index={i}
      isError={errors[i] ?? false}
      onAddLine={addLine}
      onRemoveLine={removeLine}
      onUpdateExpression={updateExpression}
      onUpdateColor={updateColor}
      {symbols}
    />
  {/each}
</div>

<style>
  .equation-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
</style>
