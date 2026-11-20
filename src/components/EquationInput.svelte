<script lang="ts">
  import { onMount } from "svelte";
  import "mathlive";
  import { isDrawable } from "../utils/math";

  let {
    value = $bindable(""),
    color = $bindable("#388c46"),
    visible = $bindable(true),
    index = 0,
    onAddLine,
    onRemoveLine,
    onUpdateExpression,
    onUpdateColor,
    onToggleVisibility,
    symbols = {},
  }: {
    value?: string;
    color?: string;
    visible?: boolean;
    index?: number;
    onAddLine?: (afterIndex: number) => void;
    onRemoveLine?: (index: number) => void;
    onUpdateExpression?: (index: number, value: string) => void;
    onUpdateColor?: (index: number, color: string) => void;
    onToggleVisibility?: (index: number) => void;
    symbols?: Record<string, number>;
  } = $props();

  let mathFieldEl:
    | (HTMLElement & { value?: string; focus?: () => void })
    | null = null;
  let colorInputEl: HTMLInputElement | null = null;
  let longPressTimer: any = null;

  let drawable = $derived(isDrawable(value));

  function handleColorClick() {
    if (!drawable) return;
    visible = !visible;
    onToggleVisibility?.(index);
  }

  function handleColorDblClick() {
    colorInputEl?.click();
  }

  function handleTouchStart() {
    if (!drawable) return;
    longPressTimer = setTimeout(() => {
      colorInputEl?.click();
    }, 600);
  }

  function handleTouchEnd() {
    if (longPressTimer) clearTimeout(longPressTimer);
  }

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
    const assignment = value.match(/^\s*([a-zA-Z_]\w*)\s*=\s*(.+)$/);
    if (assignment) {
      const varName = assignment[1];
      const rhsExpr = assignment[2].trim();
      // If RHS is a pure number or resolvable symbol
      if (/^[+-]?\d+(\.\d+)?$/.test(rhsExpr)) {
        return parseFloat(rhsExpr);
      }
      return symbols[varName] !== undefined ? symbols[varName] : symbols[rhsExpr];
    }
    return null;
  });
</script>

<div class="equation-row">
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <div
    class="color-picker-wrapper"
    class:hidden={!drawable}
    style="background-color: {visible ? color : 'transparent'}; border: 2px solid {color};"
    onclick={handleColorClick}
    ondblclick={handleColorDblClick}
    ontouchstart={handleTouchStart}
    ontouchend={handleTouchEnd}
    role="button"
    tabindex="0"
    onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') handleColorClick(); }}
    aria-label="Toggle visibility or change color"
  >
    <input
      bind:this={colorInputEl}
      type="color"
      bind:value={color}
      class="native-color-picker"
      oninput={() => onUpdateColor?.(index, color)}
    />
  </div>
  <span class="eq-number">{index + 1}</span>
  <!-- svelte-ignore a11y_no_redundant_roles -->
  <math-field
    bind:this={mathFieldEl}
    class="eq-input"
    smart-fence="true"
    math-virtual-keyboard-policy="off"
    virtual-keyboard-name="numeric-only"
  ></math-field>
  {#if badgeValue !== undefined && badgeValue !== null && !isNaN(badgeValue)}
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
  .color-picker-wrapper {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    cursor: pointer;
    flex-shrink: 0;
    position: relative;
    overflow: hidden;
    transition: background-color 0.15s;
  }
  .color-picker-wrapper.hidden {
    visibility: hidden;
    pointer-events: none;
  }
  .native-color-picker {
    position: absolute;
    opacity: 0;
    width: 100%;
    height: 100%;
    cursor: pointer;
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

