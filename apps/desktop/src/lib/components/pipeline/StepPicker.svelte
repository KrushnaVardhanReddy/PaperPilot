<script lang="ts">
  import { pipelineState, OPERATION_REGISTRY, type OperationId } from '$lib/state/pipeline.svelte';

  let { onclose } = $props<{ onclose: () => void }>();

  function pick(opId: OperationId) {
    pipelineState.addStep(opId);
    onclose();
  }

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
      <h3>Add Pipeline Step</h3>
      <button class="close-btn" id="close-step-picker" onclick={onclose}>×</button>
    </div>
    <div class="picker-grid">
      {#each Object.entries(OPERATION_REGISTRY) as [opId, def]}
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
    </div>
  </div>
</div>

<style>
  .picker-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .picker-dialog {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-lg, 12px);
    padding: 24px;
    width: 540px;
    max-height: 70vh;
    overflow-y: auto;
  }

  .picker-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .picker-header h3 {
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.4rem;
    color: var(--text-muted);
    cursor: pointer;
    line-height: 1;
    padding: 0;
  }

  .picker-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .op-tile {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 16px 8px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
    text-align: center;
  }

  .op-tile:hover {
    border-color: var(--accent-primary);
    background: var(--bg-surface-hover);
  }

  .op-icon {
    font-size: 1.8rem;
  }

  .op-label {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .op-desc {
    font-size: 0.72rem;
    color: var(--text-muted);
    line-height: 1.3;
  }
</style>
