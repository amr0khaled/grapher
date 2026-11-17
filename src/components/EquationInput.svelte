<script lang="ts">
  import { onMount } from "svelte";
  import "mathlive";

  let {
    value = $bindable(""),
    color = $bindable("#388c46"),
    index = 0,
    onAddLine,
    onRemoveLine,
    onUpdateExpression,
    onUpdateColor,
    symbols = {},
  }: {
    value?: string;
    color?: string;
    index?: number;
    onAddLine?: (afterIndex: number) => void;
    onRemoveLine?: (index: number) => void;
    onUpdateExpression?: (index: number, value: string) => void;
    onUpdateColor?: (index: number, color: string) => void;
    symbols?: Record<string, number>;
  } = $props();

  let mathFieldEl:
    | (HTMLElement & { value?: string; focus?: () => void })
    | null = null;

  onMount(() => {
    if (mathFieldEl) {
      mathFieldEl.value = value;
      mathFieldEl.addEventListener("input", () => {
        if (mathFieldEl && "value" in mathFieldEl) {
          value = mathFieldEl.value || "";
          onUpdateExpression?.(index, value);
        }
      });
      mathFieldEl.addEventListener("keydown", (e: KeyboardEvent) => {
        if (e.key === "Enter") {
          e.preventDefault();
          onAddLine?.(index);
          setTimeout(() => {
            const fields = document.querySelectorAll("math-field");
            const next = fields[index + 1] as HTMLElement & {
              focus?: () => void;
            };
            if (next && next.focus) next.focus();
          }, 0);
        }
      });
    }
  });

  $effect(() => {
    if (mathFieldEl && mathFieldEl.value !== value) {
      mathFieldEl.value = value;
    }
  });

  let badgeValue = $derived.by(() => {
    const assignment = value.match(/^\s*[a-zA-Z_]\w*\s*=\s*(.+)$/);
    if (assignment) {
      const m = value.match(/^\s*([a-zA-Z_]\w*)/);
      return m ? symbols[m[1]] : null;
    }
    return null;
  });
</script>

<div class="equation-row">
  <input
    type="color"
    bind:value={color}
    class="color-picker"
    aria-label="Line color"
    oninput={() => onUpdateColor?.(index, color)}
  />
  <span class="eq-number">{index + 1}</span>
  <!-- svelte-ignore a11y_no_redundant_roles -->
  <math-field
    bind:this={mathFieldEl}
    class="eq-input"
    smart-fence="true"
    math-virtual-keyboard-policy="off"
    virtual-keyboard-name="numeric-only"
  ></math-field>
  {#if badgeValue !== undefined && badgeValue !== null}
    <span class="eq-badge">{+badgeValue.toFixed(6)}</span>
  {/if}
</div>

<style>
  math-field::part(menu-toggle) {
    display: none;
  }
  math-field::part(virtual-keyboard-toggle) {
    height: 16px;
  }
  .equation-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0;
    min-height: 2.4rem;
  }
  .color-picker {
    width: 22px;
    height: 22px;
    border: none;
    border-radius: 50%;
    padding: 0;
    cursor: pointer;
    background: none;
    flex-shrink: 0;
  }
  .eq-number {
    width: 18px;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--muted);
    text-align: right;
    flex-shrink: 0;
  }
  .eq-input {
    flex: 1;
    border: none;
    border-bottom: 1px solid var(--border);
    padding: 0.3rem 0.2rem;
    font-size: 1.05rem;
    background: transparent;
    color: var(--fg);
    min-width: 60px;
    outline: none;
  }
  .eq-input:focus-within {
    border-bottom-color: var(--accent);
  }
  .eq-badge {
    align-self: flex-end;
    font-size: 0.75rem;
    color: var(--muted);
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0 0.35rem;
    line-height: 1.5;
    font-variant-numeric: tabular-nums;
    flex-shrink: 0;
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

