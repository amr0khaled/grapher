<script lang="ts">
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";

  type SnapPoint = "peek" | "half" | "full";

  let {
    activeSnap = $bindable<SnapPoint>("peek"),
    children,
  }: {
    activeSnap?: SnapPoint;
    children: Snippet;
  } = $props();
  const SHEET_HEIGHT = 600; // fixed max sheet height px
  const TOP_MARGIN = 24;

  let dragging = $state(false);
  let dragStartY = 0;
  let dragStartTranslateY = 0;
  let currentTranslateY = $state(0);
  let keyboardOffset = $state(0);
  let viewportH = $state(window.innerHeight);
  const minTranslateY = $derived(
    Math.max(0, SHEET_HEIGHT - (viewportH - keyboardOffset - TOP_MARGIN)),
  );
  const getMvk = () => (window as any).mathVirtualKeyboard;
  function getSnapTranslateY(snap: SnapPoint): number {
    switch (snap) {
      case "peek":
        return SHEET_HEIGHT - 40;
      case "half":
        return SHEET_HEIGHT * 0.45;
      case "full":
        return 0;
    }
  }

  function nearestSnap(translateY: number): SnapPoint {
    const snaps: { snap: SnapPoint; y: number }[] = [
      { snap: "peek", y: getSnapTranslateY("peek") },
      { snap: "half", y: Math.max(getSnapTranslateY("half"), minTranslateY) },
      { snap: "full", y: Math.max(getSnapTranslateY("full"), minTranslateY) },
    ];
    let best = snaps[0];
    let minDiff = Infinity;
    for (const s of snaps) {
      const diff = Math.abs(translateY - s.y);
      if (diff < minDiff) {
        minDiff = diff;
        best = s;
      }
    }
    return best.snap;
  }

  let lastY = 0;
  let lastTime = 0;
  let velocity = 0;
  let totalDragDistance = 0;

  function onPointerDown(e: PointerEvent) {
    e.stopPropagation();
    const startY = computedTranslateY;
    dragStartTranslateY = startY;
    currentTranslateY = startY;
    dragStartY = e.clientY;
    dragging = true;
    lastY = e.clientY;
    lastTime = e.timeStamp;
    velocity = 0;
    totalDragDistance = 0;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
  }

  function onPointerMove(e: PointerEvent) {
    e.stopPropagation();
    if (!dragging) return;
    const dy = e.clientY - dragStartY;
    totalDragDistance = Math.abs(dy);
    const targetY = dragStartTranslateY + dy;
    const minY = minTranslateY; // full
    const maxY = SHEET_HEIGHT - 40; // peek
    currentTranslateY = Math.max(minY, Math.min(maxY, targetY));

    const dt = e.timeStamp - lastTime;
    if (dt > 0) {
      velocity = (e.clientY - lastY) / dt;
    }
    lastY = e.clientY;
    lastTime = e.timeStamp;
  }
  const haptic = () => navigator.vibrate?.(10);
  function onPointerUp(e: PointerEvent) {
    e.stopPropagation();
    if (!dragging) return;
    dragging = false;

    // a tap: the click handler on the handle deals with it
    if (totalDragDistance <= 6) {
      if (totalDragDistance <= 1) {
        handleHandleClick(e);
      }
      return;
    }

    // no recent movement means the finger rested before release, so it isn't a flick
    if (e.timeStamp - lastTime > 80) velocity = 0;

    const VELOCITY_THRESHOLD = 0.5;
    let target: SnapPoint;

    if (Math.abs(velocity) > VELOCITY_THRESHOLD) {
      if (velocity > 0) {
        target = activeSnap === "full" ? "half" : "peek";
      } else {
        target = activeSnap === "peek" ? "half" : "full";
      }
    } else {
      target = nearestSnap(currentTranslateY);
    }

    if (activeSnap !== target) haptic();
    activeSnap = target;
  }

  function handleHandleClick(_: MouseEvent) {
    if (totalDragDistance > 6) return;
    haptic();
    if (activeSnap === "peek") {
      activeSnap = "half";
    } else if (activeSnap === "half") {
      activeSnap = "peek";
    }
  }
  let blurTimer = 0;
  let bodyEl: HTMLElement;
  let focusedEl = $state<HTMLElement | null>(null);
  let focusTimer = 0;
  function onFocusIn(e: FocusEvent) {
    clearTimeout(blurTimer);
    focusedEl = e.target as HTMLElement;
    if (activeSnap === "peek") activeSnap = "half";

    syncKeyboard();
    // the keyboard may not have its final size yet when focus fires
    requestAnimationFrame(() => syncKeyboard());
    focusTimer = setTimeout(syncKeyboard, 100);
  }

  function onFocusOut(e: FocusEvent) {
    const next = e.relatedTarget as Node | null;
    if (next && bodyEl.contains(next)) return; // moving to another field
    // relatedTarget can be null inside shadow DOM, so check again after a tick
    blurTimer = window.setTimeout(() => {
      if (!bodyEl.contains(document.activeElement)) focusedEl = null;
    }, 150);
  }
  function revealInBody(el: HTMLElement) {
    const pad = 16;
    const b = bodyEl.getBoundingClientRect();
    const r = el.getBoundingClientRect();
    const visibleBottom = Math.min(
      b.bottom,
      window.innerHeight - keyboardOffset,
    );

    if (r.top < b.top + pad) {
      bodyEl.scrollBy({ top: r.top - b.top - pad, behavior: "smooth" });
    } else if (r.bottom > visibleBottom - pad) {
      bodyEl.scrollBy({
        top: r.bottom - visibleBottom + pad,
        behavior: "smooth",
      });
    }
  }
  let hideTimer = 0;
  function syncKeyboard(notifyHidden = false) {
    clearTimeout(hideTimer);
    if (getMvk()?.visible) {
      keyboardOffset = getMvk()?.boundingRect.height;
    } else {
      // only treat as closed if it stays hidden (avoids flicker between fields)
      hideTimer = window.setTimeout(() => {
        if (getMvk()?.visible) return;
        keyboardOffset = 0;
        if (notifyHidden) onKeyboardHidden();
      }, 150);
    }
  }

  $effect(() => {
    const el = focusedEl;
    keyboardOffset; // dependency: keyboard moved/resized
    activeSnap; // dependency: sheet changed size
    if (!el) return;

    const align = () => {
      revealInBody(el);
      window.scrollTo(0, 0);
    };
    const raf = requestAnimationFrame(align);
    const t = setTimeout(align, 250); // again after the sheet/keyboard animation
    return () => {
      cancelAnimationFrame(raf);
      clearTimeout(t);
    };
  });
  function onKeyboardHidden() {
    // whatever you want to happen when it closes, for example:
    focusedEl = null;
    if (activeSnap === "full") activeSnap = "half"; // optional
  }
  onMount(() => {
    const onResize = () => (viewportH = window.innerHeight);

    const onToggle = () => syncKeyboard(true);
    getMvk()?.addEventListener("virtual-keyboard-toggle", onToggle);
    getMvk()?.addEventListener("geometrychange", onToggle); // also fires on resize or layout change
    window.addEventListener("resize", onResize);

    return () => {
      clearTimeout(hideTimer);
      clearTimeout(focusTimer);
      getMvk()?.removeEventListener("virtual-keyboard-toggle", onToggle);
      getMvk()?.removeEventListener("geometrychange", onToggle);
      window.removeEventListener("resize", onResize);
    };
  });

  let computedTranslateY = $derived(
    Math.max(
      dragging ? currentTranslateY : getSnapTranslateY(activeSnap),
      minTranslateY,
    ),
  );
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="bottom-sheet"
  class:dragging
  class:peek-mode={activeSnap === "peek"}
  style:height="{SHEET_HEIGHT}px"
  style:transform="translateY({computedTranslateY}px)"
  style:bottom="{keyboardOffset}px"
  onpointerdown={(e) => e.stopPropagation()}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ore a11y_no_static_element_interactions -->
  <div
    class="drag-handle"
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
  >
    <div class="handle-bar"></div>
  </div>
  <div
    bind:this={bodyEl}
    class="sheet-body"
    style:padding-bottom="{computedTranslateY}px"
    onfocusin={onFocusIn}
    onfocusout={onFocusOut}
  >
    {@render children()}
  </div>
</div>

<style>
  .bottom-sheet {
    position: fixed;
    left: 0;
    right: 0;
    z-index: 30;
    background: var(--panel);
    border-radius: 16px 16px 0 0;
    box-shadow: 0 -2px 12px rgba(0, 0, 0, 0.15);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    will-change: transform;
    transition: transform 300ms cubic-bezier(0.2, 0, 0, 1);
    max-height: calc(100dvh - var(--kb, 0px) - env(safe-area-inset-top));
    translate: 0 calc(-1 * var(--kb, 0px));
    transition:
      transform 300ms cubic-bezier(0.2, 0, 0, 1),
      bottom 300ms cubic-bezier(0.2, 0, 0, 1);
  }
  @media (prefers-reduced-motion: reduce) {
    .bottom-sheet {
      transition: none !important;
    }
  }
  .bottom-sheet.dragging {
    transition: none;
  }
  .bottom-sheet.peek-mode .sheet-body {
    pointer-events: none;
  }
  .drag-handle {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 12px 0 8px;
    cursor: grab;
    touch-action: none;
    flex-shrink: 0;
  }
  .drag-handle:active {
    cursor: grabbing;
  }
  .handle-bar {
    width: 36px;
    height: 4px;
    border-radius: 2px;
    background: var(--muted);
    opacity: 0.5;
  }
  .sheet-body {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    overscroll-behavior: contain;
    padding: 0 12px 12px;
    -webkit-overflow-scrolling: touch;
    touch-action: pan-y;
    min-height: 0;
    transition: padding-bottom 300ms cubic-bezier(0.2, 0, 0, 1);
  }
  .sheet-body :global(math-field) {
    scroll-margin-block: 16px;
  }
  @media (pointer: coarse) {
    :global(html),
    :global(body) {
      height: 100%;
      overflow: hidden;
      overscroll-behavior: none;
    }
  }
</style>
