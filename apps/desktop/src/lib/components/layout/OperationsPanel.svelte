<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';

  let selectedOperation = $state('merge');

  function handleRunOperation() {
    if (selectedOperation === 'merge') {
      if (appState.documents.length < 2) {
        toastState.error('Need at least 2 documents to merge.');
        return;
      }
      toastState.success('Merge triggered');
    }
  }

  function moveUp(index: number) {
    if (index > 0) {
      appState.reorderDocuments(index, index - 1);
    }
  }

  function moveDown(index: number) {
    if (index < appState.documents.length - 1) {
      appState.reorderDocuments(index, index + 1);
    }
  }
</script>

<div class="operations-panel">
  <div class="panel-header">
    <h3>Operations</h3>
  </div>

  <div class="panel-content">
    <div class="operation-selector">
      <label for="opSelect">Select Action</label>
      <select id="opSelect" bind:value={selectedOperation} class="form-input">
        <option value="merge">Merge PDFs</option>
        <option value="split">Split PDF</option>
        <option value="compress">Compress</option>
      </select>
    </div>

    {#if selectedOperation === 'merge'}
      <div class="operation-config">
        <p class="section-desc">Reorder files to set the merge order:</p>

        {#if appState.documents.length === 0}
          <div class="empty-list">No documents added.</div>
        {:else}
          <div class="reorder-list">
            {#each appState.documents as doc, i}
              <div class="reorder-item">
                <span class="item-name" title={doc.name}>{doc.name}</span>
                <div class="reorder-controls">
                  <button
                    class="icon-btn"
                    disabled={i === 0}
                    onclick={() => moveUp(i)}
                    title="Move Up"
                  >
                    ⬆️
                  </button>
                  <button
                    class="icon-btn"
                    disabled={i === appState.documents.length - 1}
                    onclick={() => moveDown(i)}
                    title="Move Down"
                  >
                    ⬇️
                  </button>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}

    <div class="action-area">
      <button
        class="run-btn"
        disabled={appState.documents.length === 0}
        onclick={handleRunOperation}
      >
        Run {selectedOperation.charAt(0).toUpperCase() + selectedOperation.slice(1)}
      </button>
    </div>
  </div>
</div>

<style>
  .operations-panel {
    width: 320px;
    background-color: var(--bg-secondary);
    border-left: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .panel-header {
    padding: 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .panel-header h3 {
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .panel-content {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 24px;
    flex: 1;
    overflow-y: auto;
  }

  .operation-selector {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .operation-selector label {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .form-input {
    width: 100%;
    padding: 10px 12px;
    background-color: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    color: var(--text-primary);
    font-family: inherit;
    font-size: 0.95rem;
    transition: border-color var(--transition-fast);
  }

  .form-input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  select.form-input {
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23A1A7B3' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 12px center;
    padding-right: 40px;
  }

  .operation-config {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-desc {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .empty-list {
    padding: 16px;
    text-align: center;
    font-size: 0.9rem;
    color: var(--text-muted);
    background-color: var(--bg-primary);
    border: 1px dashed var(--border-color);
    border-radius: var(--border-radius-md);
  }

  .reorder-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    background-color: var(--bg-primary);
    padding: 12px;
    border-radius: var(--border-radius-md);
    border: 1px solid var(--border-color);
  }

  .reorder-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px;
    background-color: var(--bg-surface);
    border-radius: var(--border-radius-sm);
    border: 1px solid var(--border-color);
  }

  .item-name {
    font-size: 0.85rem;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    margin-right: 12px;
  }

  .reorder-controls {
    display: flex;
    gap: 4px;
  }

  .icon-btn {
    background: none;
    border: none;
    padding: 4px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.9rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--transition-fast);
  }

  .icon-btn:hover:not(:disabled) {
    background-color: var(--bg-surface-hover);
  }

  .icon-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .action-area {
    margin-top: auto;
    padding-top: 24px;
  }

  .run-btn {
    width: 100%;
    padding: 12px;
    background-color: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--border-radius-md);
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color var(--transition-fast);
  }

  .run-btn:hover:not(:disabled) {
    background-color: var(--accent-hover);
  }

  .run-btn:disabled {
    background-color: var(--bg-surface-hover);
    color: var(--text-muted);
    cursor: not-allowed;
  }
</style>
