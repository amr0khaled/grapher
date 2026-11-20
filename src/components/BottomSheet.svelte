<script lang="ts">
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";

  type SnapPoint = "peek" | "half" | "full";

  let {
    activeSnap = $bindable<SnapPoint>("half"),
    onCanvasTap,
    children,
  }: {
    activeSnap?: SnapPoint;
    onCanvasTap?: () => void;
    children: Snippet;
  } = $props();

  let dragging = $state(false);
  let dragStartY = 0;
  let dragStartTranslateY = 0;
  let currentTranslateY = $state(0);
  let keyboardOffset = $state(0);

  const SHEET_HEIGHT = 600; // fixed max sheet height px

  function getSnapTranslateY(snap: SnapPoint): number {
    switch (snap) {
      case "peek":
        return SHEET_HEIGHT - 72;
      case "half":
        return SHEET_HEIGHT * 0.45;
      case "full":
        return 0;
    }
  }

  function nearestSnap(translateY: number): SnapPoint {
    const snaps: { snap: SnapPoint; y: number }[] = [
      { snap: "peek", y: getSnapTranslateY("peek") },
      { snap: "half", y: getSnapTranslateY("half") },
      { snap: "full", y: getSnapTranslateY("full") },
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
    dragging = true;
    dragStartY = e.clientY;
    dragStartTranslateY = getSnapTranslateY(activeSnap);
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
    const minY = 0; // full
    const maxY = SHEET_HEIGHT - 72; // peek
    currentTranslateY = Math.max(minY, Math.min(maxY, targetY));

    const dt = e.timeStamp - lastTime;
    if (dt > 0) {
      velocity = (e.clientY - lastY) / dt;
    }
    lastY = e.clientY;
    lastTime = e.timeStamp;
  }

  function onPointerUp(e: PointerEvent) {
    e.stopPropagation();
    if (!dragging) return;
    dragging = false;

    if (totalDragDistance <= 6) return;

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

    currentTranslateY = 0;
    activeSnap = target;
  }

  function handleHandleClick(e: MouseEvent) {
    if (totalDragDistance > 6) return;
    if (activeSnap === "peek") {
      activeSnap = "half";
    } else if (activeSnap === "half") {
      activeSnap = "full";
    } else {
      activeSnap = "peek";
    }
  }

  function onFocusIn(e: FocusEvent) {
    const target = e.target as HTMLElement;
    if (target && target.closest('.equation-row')) {
      target.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    }
  }

  onMount(() => {
    const vv = window.visualViewport;
    if (!vv) return;
    function onResize() {
      const vv = window.visualViewport;
      if (!vv) return;
      keyboardOffset = Math.max(0, window.innerHeight - vv.height - vv.offsetTop);
    }
    vv.addEventListener("resize", onResize);
    vv.addEventListener("scroll", onResize);
    return () => {
      vv.removeEventListener("resize", onResize);
      vv.removeEventListener("scroll", onResize);
    };
  });

  let computedTranslateY = $derived(
    dragging ? currentTranslateY : getSnapTranslateY(activeSnap)
  );
</script>

<div
  class="bottom-sheet"
  class:dragging
  class:peek-mode={activeSnap === "peek"}
  style:height="{SHEET_HEIGHT}px"
  style:transform="translateY({computedTranslateY}px)"
  style:bottom="{keyboardOffset}px"
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="drag-handle"
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
    onclick={handleHandleClick}
  >
    <div class="handle-bar"></div>
  </div>
  <div class="sheet-body" onfocusin={onFocusIn}>
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
    padding: 0 12px 12px;
    -webkit-overflow-scrolling: touch;
  }
</style>
