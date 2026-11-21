<script lang="ts">
  let {
    showGrid = $bindable(true),
    darkMode = $bindable(false),
    zoomStep = $bindable(1.12),
    visible = $bindable(false),
    onResetView,
  }: {
    showGrid?: boolean;
    darkMode?: boolean;
    zoomStep?: number;
    visible?: boolean;
    onResetView?: () => void;
  } = $props();
</script>

{#if visible}
  <div
    class="settings-overlay"
    onclick={() => (visible = false)}
    role="presentation"
  ></div>
  <div class="settings-card">
    <div class="settings-header">
      <h3>Graph Settings</h3>
      <button
        class="close-btn"
        onclick={() => (visible = false)}
        aria-label="Close settings">×</button
      >
    </div>

    <label class="toggle-row">
      <span>Show grid</span>
      <input type="checkbox" bind:checked={showGrid} />
    </label>

    <label class="toggle-row">
      <span>Dark mode</span>
      <input type="checkbox" bind:checked={darkMode} />
    </label>

    <div class="setting-group">
      <div class="label-row">
        <span>Zoom Sensitivity</span>
        <span class="value">{zoomStep.toFixed(2)}x</span>
      </div>
      <input
        type="range"
        min="1.01"
        max="1.5"
        step="0.01"
        bind:value={zoomStep}
        onmousedown={(e) => e.stopPropagation()}
        onmousemove={(e) => e.stopPropagation()}
        onpointerdown={(e) => e.stopPropagation()}
        onpointermove={(e) => e.stopPropagation()}
        class="range-input"
      />
    </div>

    {#if typeof onResetView === "function"}
      <button class="reset-btn" onclick={() => onResetView()}>Reset view</button
      >
    {/if}
  </div>
{/if}

<style>
  .settings-overlay {
    position: fixed;
    inset: 0;
    z-index: 40;
  }
  .settings-card {
    position: absolute;
    top: 3.5rem;
    right: 1rem;
    z-index: 50;
    width: 220px;
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.9rem 1rem;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.18);
    color: var(--fg);
  }
  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.6rem;
  }
  .settings-header h3 {
    margin: 0;
    font-size: 0.95rem;
  }
  .close-btn {
    background: none;
    border: none;
    font-size: 1.3rem;
    line-height: 1;
    cursor: pointer;
    color: var(--muted);
  }
  .toggle-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.4rem 0;
    font-size: 0.9rem;
    cursor: pointer;
    border-top: 1px solid var(--row-border);
  }
  .toggle-row input {
    width: 16px;
    height: 16px;
    cursor: pointer;
  }
  .setting-group {
    padding: 0.6rem 0;
    border-top: 1px solid var(--row-border);
  }
  .label-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.9rem;
    margin-bottom: 0.4rem;
  }
  .label-row .value {
    color: var(--muted);
    font-variant-numeric: tabular-nums;
  }
  .range-input {
    width: 100%;
    cursor: pointer;
  }
  .reset-btn {
    margin-top: 0.6rem;
    width: 100%;
    padding: 0.4rem;
    border: 1px solid var(--border);
    background: var(--panel-2);
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    color: var(--fg);
  }
  .reset-btn:hover {
    background: var(--border);
  }
</style>
