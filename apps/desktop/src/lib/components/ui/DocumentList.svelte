<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { documentFilters, matchesDocument } from '$lib/utils/documentFilters';
  import type { DocumentFilter } from '$lib/utils/documentFilters';
  let activeFilter = $state<DocumentFilter>('all');
  let counts = $derived(Object.fromEntries(documentFilters.map(f => [f.id, appState.documents.filter(doc => matchesDocument(f.id, appState.inspections.get(doc))).length])));
  let visible = $derived(appState.documents.map((doc, index) => ({ doc, index })).filter(({ doc }) => matchesDocument(activeFilter, appState.inspections.get(doc))));
  let pending = $derived([...appState.inspections.values()].filter(s => s.status === 'pending').length);
  let unavailable = $derived([...appState.inspections.values()].filter(s => s.status === 'error' || s.status === 'unavailable').length);
  import { getFileTypeInfo } from '$lib/utils/fileType';

  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
</script>

<div class="document-list">
  <div class="list-header">
    <div class="filter-pills" role="group" aria-label="Document filters">
      {#each documentFilters as filter}<button type="button" class:active={activeFilter === filter.id} aria-pressed={activeFilter === filter.id} onclick={() => activeFilter = filter.id}>{filter.label} ({counts[filter.id] ?? 0})</button>{/each}
    </div>
    {#if appState.documents.length > 0}
      <button
        type="button"
        class="clear-all-btn"
        title="Remove all documents"
        onclick={() => appState.clearAllDocuments()}
      >
        Clear All
      </button>
    {/if}
  </div>
  {#if pending}<p role="status">Inspecting {pending} document{pending === 1 ? '' : 's'}…</p>{/if}
  {#if unavailable}<p role="status">Details unavailable for {unavailable} document{unavailable === 1 ? '' : 's'}. These remain in All documents.</p>{/if}
  {#if appState.documents.length === 0}
    <div class="empty-state">
      <p>No documents added yet.</p>
    </div>
  {:else}
    {#if visible.length === 0}<p role="status">No documents match this filter.</p>{/if}
    <div class="list-container">
      {#each visible as { doc, index }}
        {@const fileInfo = getFileTypeInfo(doc.name)}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="document-item {appState.selectedDocumentIndex === index ? 'selected' : ''}"
          onclick={() => appState.selectDocument(index)}
        >
          <div class="doc-icon">{fileInfo.icon}</div>
          <div class="doc-info">
            <span class="doc-name">{doc.name}</span>
            <div class="doc-meta">
              <span class="doc-size">{formatFileSize(doc.size)}</span>
              <span class="doc-pages">{fileInfo.isPdf ? (appState.inspections.get(doc)?.facts?.pageCount ? `${appState.inspections.get(doc)?.facts?.pageCount} pages` : 'PDF') : fileInfo.label.split(' ')[0]}</span>
            </div>
          </div>
          <div class="doc-actions">
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
              ✕
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .list-header {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .filter-pills { display: flex; flex-wrap: wrap; gap: 6px; }
  .filter-pills button {
    border: 1px solid var(--border-color);
    background: var(--bg-surface);
    color: var(--text-secondary);
    border-radius: 20px;
    padding: 5px 11px;
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }
  .filter-pills button:hover {
    color: var(--text-primary);
    background: var(--bg-surface-hover);
  }
  .filter-pills button.active {
    border-color: var(--accent-primary);
    background: rgba(99, 102, 241, 0.12);
    color: var(--accent-primary);
    font-weight: 600;
  }
  .filter-pills button:focus-visible { outline: 2px solid var(--accent-primary); outline-offset: 2px; }
  .clear-all-btn {
    align-self: flex-start;
    background: transparent;
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
    font-size: 11px;
    font-weight: 500;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
  }
  .clear-all-btn:hover {
    background: rgba(239, 68, 68, 0.15);
    border-color: #ef4444;
  }

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
    min-width: 0;
  }

  .doc-name {
    font-size: 0.88rem;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-bottom: 3px;
  }

  .doc-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.75rem;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .doc-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-left: 8px;
    flex-shrink: 0;
  }

  .view-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    padding: 6px;
    border-radius: var(--border-radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-fast);
    opacity: 0.7;
  }

  .view-btn:hover {
    opacity: 1;
    color: var(--accent-primary);
    background-color: var(--bg-surface-hover);
  }

  .remove-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.85rem;
    color: var(--text-muted);
    padding: 6px;
    border-radius: var(--border-radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-fast);
    opacity: 0.7;
  }

  .remove-btn:hover {
    opacity: 1;
    color: #ef4444;
    background-color: rgba(239, 68, 68, 0.1);
  }
</style>
