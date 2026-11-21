<script lang="ts">
  import Warning from "./Warning.svelte";
  import { onMount } from "svelte";
  import "mathlive";
  import { isDrawable, getEvaluatedValue } from "../utils/math";
  import { _eval } from "../wasm/grapher_engine.js";

  let {
    value = $bindable(""),
    color = $bindable("#388c46"),
    visible = $bindable(true),
    index = 0,
    isError = false,
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
    isError?: boolean;
    onAddLine?: (afterIndex: number) => void;
    onRemoveLine?: (index: number) => void;
    onUpdateExpression?: (index: number, value: string) => void;
    onUpdateColor?: (index: number, color: string) => void;
    onToggleVisibility?: (index: number) => void;
    symbols?: Record<string, number>;
  } = $props();

  let showMathKeyboard = $state(false);
  let mathFieldEl:
    | (HTMLElement & {
        value?: string;
        focus?: () => void;
        inlineShortcuts?: Record<string, string>;
      })
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
    window.mathVirtualKeyboard.layouts = ["numeric", "symbols", "alphabetic"];
    // Intercept back button when MathLive virtual keyboard is open
    const handlePopState = (e: PopStateEvent) => {
      // @ts-ignore
      if (window.mathVirtualKeyboard?.visible) {
        // @ts-ignore
        window.mathVirtualKeyboard.hide();
      }
    };
    window.addEventListener("popstate", handlePopState);

    if (mathFieldEl) {
      if ("macros" in mathFieldEl) {
        mathFieldEl.macros = {
          ...mathFieldEl.macros,
          sinc: "\\operatorname{sinc}",
        };
      }
      if ("inlineShortcuts" in mathFieldEl) {
        mathFieldEl.inlineShortcuts = {
          ...mathFieldEl["inlineShortcuts"],
          sinc: "\\operatorname{sinc}",
        };
      }
      mathFieldEl.value = value;
      mathFieldEl.addEventListener("focus", () => {
        // @ts-ignore
        if (showMathKeyboard) {
          navigator?.virtualKeyboard?.hide?.();
          window.mathVirtualKeyboard?.show?.();
        } else {
          navigator?.virtualKeyboard?.show?.();
          window.mathVirtualKeyboard?.hide?.();
        }
      });
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

    return () => {
      window.removeEventListener("popstate", handlePopState);
    };
  });

  $effect(() => {
    if (mathFieldEl && mathFieldEl.value !== value) {
      mathFieldEl.value = value;
    }
  });
  let evaluation = $derived.by(() => {
    const simpleVal = getEvaluatedValue(value, symbols);
    if (simpleVal !== null) {
      return { value: simpleVal, error: null };
    }

    try {
      return { value: _eval(value, 0, 0), error: null };
    } catch (e) {
      return {
        value: null,
        error: e instanceof Error ? e.message : "Evaluation error",
      };
    }
  });
  let badgeValue = $derived(evaluation.value);
  let errorMessage = $derived<string | null>(evaluation.error);
</script>

<div class="equation-row">
  <span class="eq-number">{index + 1}</span>
  {#if isError}
    <div class="error-wrapper">
      <Warning class="eq-error-badge" size={20} />
      <div class="error-popover">Invalid expression or formula syntax</div>
    </div>
  {:else if badgeValue !== undefined && badgeValue !== null}
    <span class="eq-badge">{+badgeValue.toFixed(6)}</span>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
      class="color-picker-wrapper"
      class:hidden={!drawable}
      style="background-color: {visible
        ? color
        : 'transparent'}; border: 2px solid {color};"
      onclick={handleColorClick}
      ondblclick={handleColorDblClick}
      ontouchstart={handleTouchStart}
      ontouchend={handleTouchEnd}
      role="button"
      tabindex="0"
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " ") handleColorClick();
      }}
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
  {/if}
  <!-- svelte-ignore a11y_no_redundant_roles -->
  <math-field
    bind:this={mathFieldEl}
    class="eq-input"
    class:eq-error={isError}
    smart-fence="true"
    math-virtual-keyboard-policy="manual"
    virtualkeyboardpolicy="manual"
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
    padding: 0.2rem 0;
    min-height: 20px;
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
  .color-picker::-webkit-color-swatch-wrapper {
    padding: 0;
  }
  .color-picker::-webkit-color-swatch {
    border: none;
    border-radius: 50%;
  }
  .color-picker::-moz-color-swatch {
    border: none;
    border-radius: 50%;
  }
  .eq-number {
    min-width: 24px;
    min-height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--muted);
    opacity: 0.6;
    flex-shrink: 0;
  }
  .eq-input {
    flex: 1;
    border: none;
    border-bottom: 1px solid var(--border);
    font-size: 1.45rem;
    padding-inline-start: 0.2rem;
    background: transparent;
    color: var(--fg);
    min-width: 60px;
    outline: none;
  }
  .eq-input:focus-within {
    border-bottom-color: var(--accent);
  }
  .error-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    cursor: help;
  }
  .error-popover {
    visibility: hidden;
    position: absolute;
    bottom: 125%;
    left: 50%;
    transform: translateX(0%);
    background: var(--panel);
    color: var(--fg);
    border: 1px solid var(--border);
    padding: 0.3rem 0.6rem;
    border-radius: 4px;
    font-size: 0.75rem;
    white-space: nowrap;
    z-index: 50;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }
  .error-wrapper:hover .error-popover,
  .error-wrapper:active .error-popover {
    visibility: visible;
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
