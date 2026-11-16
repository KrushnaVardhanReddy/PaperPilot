<script lang="ts">
  import { pipelineState, OPERATION_REGISTRY, type OperationId } from '$lib/state/pipeline.svelte';

  let { onclose } = $props<{ onclose: () => void }>();

  let searchQuery = $state('');

  function pick(opId: OperationId) {
    pipelineState.addStep(opId);
    onclose();
  }

  // Filter operations dynamically by search query
  let filteredOps = $derived(
    Object.entries(OPERATION_REGISTRY).filter(([opId, def]) => {
      if (!searchQuery.trim()) return true;
      const q = searchQuery.toLowerCase();
      return (
        opId.toLowerCase().includes(q) ||
        def.label.toLowerCase().includes(q) ||
        def.description.toLowerCase().includes(q)
      );
    })
  );

  // Close when clicking outside
  function handleBackdropClick(e: MouseEvent) {
    if ((e.target as HTMLElement).classList.contains('picker-backdrop')) {
      onclose();
    }
  }
</script>

<div 
  class="picker-backdrop" 
  onclick={handleBackdropClick} 
  onkeydown={(e) => e.key === 'Escape' && onclose()}
  role="presentation"
  id="step-picker-modal">
  <div class="picker-dialog" role="dialog" aria-label="Pick an operation">
    <div class="picker-header">
      <div class="header-title">
        <h3>Add Pipeline Step</h3>
        <span class="count-badge">{filteredOps.length} / {Object.keys(OPERATION_REGISTRY).length} tools</span>
      </div>
      <button class="close-btn" id="close-step-picker" onclick={onclose}>×</button>
    </div>

    <div class="search-bar">
      <input
        type="text"
        placeholder="Search 90 tools (e.g. merge, compress, ocr, docx, bates)..."
        bind:value={searchQuery}
        id="step-picker-search"
        autofocus
      />
    </div>

    <div class="picker-grid">
      {#each filteredOps as [opId, def]}
        <button
          class="op-tile"
          id={`op-tile-${opId}`}
          onclick={() => pick(opId as OperationId)}
        >
          <span class="op-icon">{def.icon}</span>
          <span class="op-label">{def.label}</span>
          <span class="op-desc">{def.description}</span>
        </button>
      {/each}
      {#if filteredOps.length === 0}
        <div class="no-results">
          No matching tools found for "{searchQuery}"
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .picker-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .picker-dialog {
    background: var(--bg-secondary, #1e1e24);
    border: 1px solid var(--border-color, #333);
    border-radius: var(--border-radius-lg, 12px);
    padding: 24px;
    width: 680px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.5);
  }

  .picker-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .header-title {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .picker-header h3 {
    font-size: 1.15rem;
    margin: 0;
    color: var(--text-primary, #ffffff);
  }

  .count-badge {
    font-size: 0.75rem;
    padding: 2px 8px;
    background: rgba(94, 106, 210, 0.2);
    border: 1px solid rgba(94, 106, 210, 0.4);
    border-radius: 12px;
    color: var(--accent-primary, #818cf8);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.4rem;
    color: var(--text-muted, #888);
    cursor: pointer;
    line-height: 1;
    padding: 0;
  }

  .search-bar {
    margin-bottom: 16px;
  }

  .search-bar input {
    width: 100%;
    padding: 10px 14px;
    border-radius: 8px;
    background: var(--bg-surface, #121216);
    border: 1px solid var(--border-color, #3a3a42);
    color: var(--text-primary, #fff);
    font-size: 0.9rem;
    outline: none;
    box-sizing: border-box;
  }

  .search-bar input:focus {
    border-color: var(--accent-primary, #5e6ad2);
  }

  .picker-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    overflow-y: auto;
    padding-right: 4px;
    max-height: 55vh;
  }

  .no-results {
    grid-column: 1 / -1;
    text-align: center;
    padding: 40px 12px;
    color: var(--text-muted, #888);
    font-size: 0.9rem;
  }

  .op-tile {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 14px 10px;
    background: var(--bg-surface, #141418);
    border: 1px solid var(--border-color, #2b2b34);
    border-radius: var(--border-radius-md, 8px);
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: center;
  }

  .op-tile:hover {
    border-color: var(--accent-primary, #5e6ad2);
    background: var(--bg-surface-hover, #202028);
    transform: translateY(-1px);
  }

  .op-icon {
    font-size: 1.6rem;
  }

  .op-label {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .op-desc {
    font-size: 0.72rem;
    color: var(--text-muted, #9ca3af);
    line-height: 1.3;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
