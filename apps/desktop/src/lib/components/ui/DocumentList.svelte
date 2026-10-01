<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
</script>

<div class="document-list">
  {#if appState.documents.length === 0}
    <div class="empty-state">
      <p>No documents added yet.</p>
    </div>
  {:else}
    <div class="list-container">
      {#each appState.documents as doc, index}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="document-item {appState.selectedDocumentIndex === index ? 'selected' : ''}"
          onclick={() => appState.selectDocument(index)}
        >
          <div class="doc-icon">📄</div>
          <div class="doc-info">
            <span class="doc-name">{doc.name}</span>
            <div class="doc-meta">
              <span class="doc-size">{formatFileSize(doc.size)}</span>
              <span class="doc-pages">{(doc as any).pageCount ? `${(doc as any).pageCount} pages` : 'PDF'}</span>
            </div>
          </div>
          <button
            class="view-btn"
            id="view-doc-{index}"
            title="View document"
            aria-label="View {doc.name}"
            onclick={(e) => { e.stopPropagation(); appState.selectDocument(index); }}
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2"
                 stroke-linecap="round" stroke-linejoin="round">
              <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
              <circle cx="12" cy="12" r="3"/>
            </svg>
          </button>
          <button class="remove-btn" onclick={(e) => { e.stopPropagation(); appState.removeDocument(index); }} title="Remove Document">
            ❌
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .document-list {
    width: 100%;
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .empty-state {
    padding: 24px;
    text-align: center;
    color: var(--text-muted);
    background-color: var(--bg-surface);
    border-radius: var(--border-radius-md);
    border: 1px dashed var(--border-color);
  }

  .list-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .document-item {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    transition: all var(--transition-fast);
    cursor: pointer;
  }

  .document-item:hover {
    background-color: var(--bg-surface-hover);
    border-color: var(--text-muted);
  }

  .document-item:hover .view-btn,
  .document-item:hover .remove-btn {
    opacity: 1;
  }

  .document-item.selected {
    background-color: rgba(94, 106, 210, 0.1);
    border-color: var(--accent-primary);
  }

  .doc-icon {
    font-size: 1.5rem;
    margin-right: 16px;
    opacity: 0.9;
  }

  .doc-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .doc-name {
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-bottom: 4px;
  }

  .doc-meta {
    display: flex;
    gap: 12px;
    font-size: 0.8rem;
    color: var(--text-secondary);
  }

  .view-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    padding: 8px;
    margin-left: 4px;
    border-radius: var(--border-radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-fast);
    opacity: 0.6;
  }

  .view-btn:hover {
    opacity: 1;
    color: var(--accent-primary);
    background-color: rgba(94, 106, 210, 0.1);
  }

  .remove-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1rem;
    color: var(--text-muted);
    padding: 8px;
    margin-left: 8px;
    border-radius: var(--border-radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-fast);
    opacity: 0.6;
  }

  .remove-btn:hover {
    opacity: 1;
    background-color: rgba(255, 0, 0, 0.1);
  }
</style>
