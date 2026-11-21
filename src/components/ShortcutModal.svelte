<script lang="ts">
  let { visible = $bindable(false) }: { visible?: boolean } = $props();
</script>

{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={() => (visible = false)}>
    <div class="modal-card" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>Keyboard Shortcuts</h3>
        <button class="close-btn" onclick={() => (visible = false)}>×</button>
      </div>
      <ul class="shortcuts-list">
        <li><span><kbd>Enter</kbd></span><span>New equation line</span></li>
        <li><span><kbd>+</kbd> / <kbd>-</kbd></span><span>Zoom in / out</span></li>
        <li><span><kbd>Arrow Keys</kbd></span><span>Pan view</span></li>
        <li><span><kbd>?</kbd></span><span>Toggle this help</span></li>
      </ul>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: grid;
    place-items: center;
    z-index: 100;
  }
  .modal-card {
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 1.2rem;
    width: 300px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
    color: var(--fg);
  }
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  .modal-header h3 {
    margin: 0;
    font-size: 1.1rem;
  }
  .close-btn {
    background: none;
    border: none;
    font-size: 1.4rem;
    cursor: pointer;
    color: var(--muted);
  }
  .shortcuts-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
  }
  .shortcuts-list li {
    display: flex;
    justify-content: space-between;
    font-size: 0.9rem;
  }
  kbd {
    background: var(--panel-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0.1rem 0.3rem;
    font-family: monospace;
    font-size: 0.8rem;
  }
</style>
