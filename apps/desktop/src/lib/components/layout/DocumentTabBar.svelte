<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  let fileInputRef: HTMLInputElement | undefined = $state();

  function openFileObjects(files: File[]) {
    if (files.length === 0) return;
    const targetIndex = appState.documents.length;
    appState.addDocuments(files);
    appState.selectDocument(targetIndex);
  }

  function handleFileInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      openFileObjects(Array.from(target.files));
      target.value = '';
    }
  }

  async function handleNewTab() {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        multiple: false,
        filters: [{ name: 'PDF Documents', extensions: ['pdf'] }]
      });
      if (selected && typeof selected === 'string') {
        const fileName = selected.split('/').pop() || selected.split('\\').pop() || 'document.pdf';
        try {
          // Tauri convertFileSrc
          const src = typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__?.convertFileSrc
            ? (window as any).__TAURI_INTERNALS__.convertFileSrc(selected)
            : selected;
          const response = await fetch(src);
          const blob = await response.blob();
          const file = new File([blob], fileName, { type: 'application/pdf' });
          Object.defineProperty(file, 'path', { value: selected, writable: false });
          openFileObjects([file]);
          return;
        } catch (fetchErr) {
          console.warn('convertFileSrc fetch failed, using direct file picker fallback', fetchErr);
        }
      }
    } catch (dialogErr) {
      console.warn('Native open dialog unavailable, using HTML file picker fallback', dialogErr);
    }
    // Fallback: trigger HTML file input directly
    fileInputRef?.click();
  }
</script>

<div class="tab-bar" role="tablist" aria-label="Open documents">
  <button
    class="home-tab-btn"
    class:active={appState.selectedDocumentIndex === null}
    onclick={() => appState.selectDocument(null)}
    title="Documents Library"
    id="tab-btn-home"
  >
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
      <polyline points="9 22 9 12 15 12 15 22"/>
    </svg>
    <span>Documents</span>
  </button>

  <div class="tabs-scroll-container">
    {#each appState.openDocIndices as docIndex (docIndex)}
      {@const doc = appState.documents[docIndex]}
      {#if doc}
        <div
          class="doc-tab"
          class:active={appState.selectedDocumentIndex === docIndex}
          role="tab"
          aria-selected={appState.selectedDocumentIndex === docIndex}
          id="tab-doc-{docIndex}"
          onclick={() => appState.selectDocument(docIndex)}
          onkeydown={(e) => e.key === 'Enter' && appState.selectDocument(docIndex)}
          tabindex="0"
        >
          <span class="tab-icon">📄</span>
          <span class="tab-title" title={doc.name}>{doc.name}</span>
          <button
            class="tab-close-btn"
            id="tab-close-{docIndex}"
            title="Close tab"
            aria-label="Close {doc.name}"
            onclick={(e) => {
              e.stopPropagation();
              appState.closeTab(docIndex);
            }}
          >
            ×
          </button>
        </div>
      {/if}
    {/each}
  </div>

  <button
    class="new-tab-btn"
    id="new-doc-tab-btn"
    title="Open new document"
    aria-label="Open new document"
    onclick={handleNewTab}
  >
    +
  </button>
  <input
    bind:this={fileInputRef}
    type="file"
    accept=".pdf"
    style="display: none;"
    onchange={handleFileInputChange}
  />
</div>

<style>
  .tab-bar {
    display: flex;
    align-items: center;
    background-color: var(--bg-surface, #1e1e24);
    border-bottom: 1px solid var(--border-color, #2a2a35);
    height: 38px;
    padding: 0 8px;
    /* Add padding-right so tabs don't overlap with the fixed window controls (46px * 3 = 138px) */
    padding-right: 146px;
    gap: 4px;
    user-select: none;
    flex-shrink: 0;
  }

  .home-tab-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    height: 28px;
    border-radius: var(--border-radius-sm, 4px);
    background: transparent;
    border: none;
    color: var(--text-secondary, #9ca3af);
    font-size: 12px;
    cursor: pointer;
    transition: all var(--transition-fast, 0.15s ease);
    flex-shrink: 0;
  }

  .home-tab-btn:hover {
    color: var(--text-primary, #ffffff);
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
  }

  .home-tab-btn.active {
    color: var(--accent-primary, #5e6ad2);
    background-color: rgba(94, 106, 210, 0.15);
    font-weight: 500;
  }

  .tabs-scroll-container {
    display: flex;
    align-items: center;
    gap: 4px;
    overflow-x: auto;
    scrollbar-width: none;
    flex: 1;
  }

  .tabs-scroll-container::-webkit-scrollbar {
    display: none;
  }

  .doc-tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    height: 28px;
    border-radius: var(--border-radius-sm, 4px);
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary, #9ca3af);
    font-size: 12px;
    cursor: pointer;
    transition: all var(--transition-fast, 0.15s ease);
    max-width: 180px;
    flex-shrink: 0;
  }

  .doc-tab:hover {
    color: var(--text-primary, #ffffff);
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
  }

  .doc-tab.active {
    color: var(--text-primary, #ffffff);
    background-color: var(--bg-secondary, #13131a);
    border-color: var(--border-color, #2a2a35);
    font-weight: 500;
  }

  .tab-icon {
    font-size: 12px;
  }

  .tab-title {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 12px;
  }

  .tab-close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: none;
    background: transparent;
    color: var(--text-muted, #6b7280);
    font-size: 13px;
    cursor: pointer;
    line-height: 1;
    padding: 0;
    transition: all var(--transition-fast, 0.15s ease);
  }

  .tab-close-btn:hover {
    background-color: rgba(255, 255, 255, 0.15);
    color: var(--text-primary, #ffffff);
  }

  .new-tab-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border-radius: var(--border-radius-sm, 4px);
    border: none;
    background: transparent;
    color: var(--text-muted, #6b7280);
    font-size: 16px;
    cursor: pointer;
    transition: all var(--transition-fast, 0.15s ease);
    flex-shrink: 0;
  }

  .new-tab-btn:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    color: var(--text-primary, #ffffff);
  }
</style>
