<script lang="ts">
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";

  type SnapPoint = "peek" | "half" | "full";

  const SNAP_HEIGHTS: Record<SnapPoint, string> = {
    peek: "72px",
    half: "45vh",
    full: "calc(100dvh - 56px)",
  };

  const SNAP_ORDER: SnapPoint[] = ["peek", "half", "full"];

  let {
    activeSnap = $bindable<SnapPoint>("half"),
    onCanvasTap,
    children,
  }: {
    activeSnap?: SnapPoint;
    onCanvasTap?: () => void;
    children: Snippet;
  } = $props();

  let sheetEl: HTMLDivElement | null = null;
  let dragging = $state(false);
  let dragStartY = 0;
  let dragStartHeight = 0;
  let currentTranslateY = $state(0);
  let keyboardOffset = $state(0);

  function getSnapHeightPx(snap: SnapPoint): number {
    switch (snap) {
      case "peek": return 72;
      case "half": return window.innerHeight * 0.45;
      case "full": return window.innerHeight - 56;
    }
  }

  function nearestSnap(heightPx: number): SnapPoint {
    let best: SnapPoint = "peek";
    let bestDist = Infinity;
    for (const s of SNAP_ORDER) {
      const d = Math.abs(heightPx - getSnapHeightPx(s));
      if (d < bestDist) { bestDist = d; best = s; }
    }
    return best;
  }

  let lastPointerY = 0;
  let lastPointerTime = 0;
  let velocity = 0;

  function onPointerDown(e: PointerEvent) {
    dragging = true;
    dragStartY = e.clientY;
    dragStartHeight = getSnapHeightPx(activeSnap);
    lastPointerY = e.clientY;
    lastPointerTime = e.timeStamp;
    velocity = 0;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
  }

  function onPointerMove(e: PointerEvent) {
    if (!dragging) return;
    const dy = dragStartY - e.clientY;
    const newHeight = dragStartHeight + dy;
    const maxH = getSnapHeightPx("full");
    const clamped = Math.max(40, Math.min(maxH, newHeight));
    currentTranslateY = getSnapHeightPx(activeSnap) - clamped;

    const dt = e.timeStamp - lastPointerTime;
    if (dt > 0) {
      velocity = (lastPointerY - e.clientY) / dt;
    }
    lastPointerY = e.clientY;
    lastPointerTime = e.timeStamp;
  }

  function onPointerUp(_e: PointerEvent) {
    if (!dragging) return;
    dragging = false;

    const currentHeight = getSnapHeightPx(activeSnap) - currentTranslateY;
    const VELOCITY_THRESHOLD = 0.4;

    let target: SnapPoint;
    if (Math.abs(velocity) > VELOCITY_THRESHOLD) {
      // Fast swipe: snap in swipe direction
      const idx = SNAP_ORDER.indexOf(activeSnap);
      if (velocity > 0) {
        target = SNAP_ORDER[Math.min(idx + 1, SNAP_ORDER.length - 1)];
      } else {
        target = SNAP_ORDER[Math.max(idx - 1, 0)];
      }
    } else {
      target = nearestSnap(currentHeight);
    }

    currentTranslateY = 0;
    activeSnap = target;
  }

  // VisualViewport keyboard adjustment
  onMount(() => {
    const vv = window.visualViewport;
    if (!vv) return;

    function onResize() {
      const vv = window.visualViewport;
      if (!vv) return;
      const offset = window.innerHeight - vv.height - vv.offsetTop;
      keyboardOffset = Math.max(0, offset);
    }

    vv.addEventListener("resize", onResize);
    vv.addEventListener("scroll", onResize);
    return () => {
      vv.removeEventListener("resize", onResize);
      vv.removeEventListener("scroll", onResize);
    };
  });
</script>

<div
  bind:this={sheetEl}
  class="bottom-sheet"
  class:dragging
  data-snap={activeSnap}
  style:height={SNAP_HEIGHTS[activeSnap]}
  style:bottom="{keyboardOffset}px"
  style:transform="translateY({currentTranslateY}px)"
>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="drag-handle"
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
  >
    <div class="handle-bar"></div>
  </div>
  <div class="sheet-body">
    {@render children()}
  </div>
</div>

<style>
  .bottom-sheet {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 30;
    background: var(--panel);
    border-radius: 16px 16px 0 0;
    box-shadow: 0 -2px 12px rgba(0, 0, 0, 0.15);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    will-change: transform, height;
    transition: transform 300ms cubic-bezier(0.2, 0, 0, 1),
                height 300ms cubic-bezier(0.2, 0, 0, 1);
  }
  .bottom-sheet.dragging {
    transition: none;
  }
  .drag-handle {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 10px 0 6px;
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
