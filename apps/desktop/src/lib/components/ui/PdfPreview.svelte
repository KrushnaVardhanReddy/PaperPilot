<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  const mockPages = Array.from({ length: 12 }, (_, i) => i + 1);
  let selectedPages = $state(new Set<number>());

  function togglePageSelection(pageNum: number) {
    const newSelection = new Set(selectedPages);
    if (newSelection.has(pageNum)) {
      newSelection.delete(pageNum);
    } else {
      newSelection.add(pageNum);
    }
    selectedPages = newSelection;
  }

  function clearSelection() {
    selectedPages = new Set();
  }

  function selectAll() {
    selectedPages = new Set(mockPages);
  }

  function handleBack() {
    appState.selectDocument(null);
  }
</script>

<div class="pdf-preview-container">
  <div class="preview-header">
    <div class="header-left">
      <button class="back-btn" onclick={handleBack} title="Back to List">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="19" y1="12" x2="5" y2="12"></line>
          <polyline points="12 19 5 12 12 5"></polyline>
        </svg>
      </button>
      <div class="doc-info">
        <h3>{appState.documents[appState.selectedDocumentIndex!]?.name}</h3>
        <span class="page-count">{mockPages.length} pages</span>
      </div>
    </div>

    <div class="header-actions">
      <button class="action-btn" onclick={selectAll}>Select All</button>
      <button class="action-btn" onclick={clearSelection} disabled={selectedPages.size === 0}>Clear Selection</button>
    </div>
  </div>

  <div class="preview-content">
    <div class="thumbnails-grid">
      {#each mockPages as page}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="thumbnail-wrapper {selectedPages.has(page) ? 'selected' : ''}"
          onclick={() => togglePageSelection(page)}
        >
          <div class="thumbnail">
            <span class="mock-content">PDF Content</span>
          </div>
          <span class="page-number">Page {page}</span>
          {#if selectedPages.has(page)}
            <div class="selection-indicator">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"></polyline>
              </svg>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .pdf-preview-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: var(--bg-primary);
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .back-btn {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px;
    border-radius: var(--border-radius-sm);
    transition: all var(--transition-fast);
  }

  .back-btn:hover {
    color: var(--text-primary);
    background-color: var(--bg-surface-hover);
  }

  .doc-info {
    display: flex;
    flex-direction: column;
  }

  .doc-info h3 {
    font-size: 1.1rem;
    color: var(--text-primary);
    margin: 0;
    margin-bottom: 2px;
  }

  .page-count {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .header-actions {
    display: flex;
    gap: 12px;
  }

  .action-btn {
    background-color: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    padding: 6px 12px;
    border-radius: var(--border-radius-md);
    font-size: 0.85rem;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .action-btn:hover:not(:disabled) {
    background-color: var(--bg-surface-hover);
    border-color: var(--text-muted);
  }

  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .preview-content {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }

  .thumbnails-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 24px;
    padding-bottom: 24px;
  }

  .thumbnail-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    position: relative;
    transition: transform var(--transition-fast);
  }

  .thumbnail-wrapper:hover {
    transform: translateY(-2px);
  }

  .thumbnail {
    width: 100%;
    aspect-ratio: 1 / 1.414; /* A4 aspect ratio */
    background-color: #ffffff;
    border-radius: 4px;
    box-shadow: var(--shadow-md);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid transparent;
    transition: all var(--transition-fast);
    position: relative;
    overflow: hidden;
  }

  .mock-content {
    color: #e0e0e0;
    font-size: 1.2rem;
    font-weight: bold;
    transform: rotate(-45deg);
    user-select: none;
  }

  .thumbnail-wrapper:hover .thumbnail {
    box-shadow: var(--shadow-lg);
  }

  .thumbnail-wrapper.selected .thumbnail {
    border-color: var(--accent-primary);
  }

  .page-number {
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .thumbnail-wrapper.selected .page-number {
    color: var(--accent-primary);
  }

  .selection-indicator {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 24px;
    height: 24px;
    background-color: var(--accent-primary);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 4px rgba(0,0,0,0.2);
  }
</style>
