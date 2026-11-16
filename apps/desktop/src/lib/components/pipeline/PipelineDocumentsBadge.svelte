<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let isOpen = $state(false);
  let fileInputRef = $state<HTMLInputElement | null>(null);

  function formatFileSize(bytes?: number): string {
    if (!bytes || bytes <= 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
  }

  async function openFilePicker() {
    if (appState.isMobile) {
      fileInputRef?.click();
      return;
    }

    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        multiple: true,
        filters: [
          { name: 'Supported Files', extensions: ['pdf', 'docx', 'doc', 'xlsx', 'xls', 'pptx', 'ppt', 'json', 'html', 'htm', 'md', 'markdown', 'txt'] },
          { name: 'PDF Files', extensions: ['pdf'] },
          { name: 'All Files', extensions: ['*'] }
        ]
      });

      if (!selected) return;
      const paths = Array.isArray(selected) ? selected : [selected];
      const filesArray: File[] = [];

      for (const path of paths) {
        const meta = await invoke<{ name: string; size_bytes: number; path: string }>('get_file_metadata', { path });
        const blob = new Blob([], { type: 'application/pdf' });
        const file = new File([blob], meta.name, { type: 'application/pdf' });
        Object.defineProperty(file, 'size', { value: meta.size_bytes, writable: false });
        (file as any)._localPath = meta.path;
        (file as any)._isLoaded = false;
        filesArray.push(file);
      }

      if (filesArray.length > 0) {
        appState.addDocuments(filesArray);
        toastState.success(`Added ${filesArray.length} document${filesArray.length > 1 ? 's' : ''}`);
      }
    } catch (err) {
      // Fallback to browser file input
      fileInputRef?.click();
    }
  }

  function handleFileInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const filesArray = Array.from(target.files).map(f => {
        (f as any)._localPath = (f as any).path || f.name;
        (f as any)._isLoaded = f.size > 0;
        return f;
      });
      appState.addDocuments(filesArray);
      toastState.success(`Added ${filesArray.length} document${filesArray.length > 1 ? 's' : ''}`);
    }
    target.value = '';
  }

  function clearAll() {
    while (appState.documents.length > 0) {
      appState.removeDocument(0);
    }
    isOpen = false;
    toastState.info('Cleared all input documents');
  }
</script>

<div class="pipeline-documents-badge-container">
  <div class="badge-pill">
    <button
      class="badge-summary-btn"
      onclick={() => (isOpen = !isOpen)}
      title="Click to view and manage input documents"
    >
      <span class="doc-icon">{appState.documents.length > 1 ? '📚' : '📄'}</span>
      <span class="doc-title">
        {#if appState.documents.length === 1}
          {appState.documents[0].name}
        {:else}
          {appState.documents.length} Documents (Batch)
        {/if}
      </span>
      <span class="caret" class:open={isOpen}>▾</span>
    </button>

    <button
      class="add-quick-btn"
      onclick={openFilePicker}
      title="Add more documents to pipeline"
    >
      <span>+ Add</span>
    </button>
  </div>

  {#if isOpen}
    <div class="documents-popover">
      <div class="popover-header">
        <div class="popover-title">
          <span>Input Documents</span>
          <span class="count-tag">{appState.documents.length}</span>
        </div>
        <button class="close-popover-btn" onclick={() => (isOpen = false)}>×</button>
      </div>

      <div class="doc-list-scroll">
        {#each appState.documents as doc, idx}
          <div class="doc-item">
            <span class="item-icon">📄</span>
            <div class="item-info">
              <span class="item-name" title={doc.name}>{doc.name}</span>
              <span class="item-size">{formatFileSize(doc.size)}</span>
            </div>
            <button
              class="item-remove-btn"
              onclick={() => appState.removeDocument(idx)}
              title="Remove document"
            >
              ×
            </button>
          </div>
        {/each}
      </div>

      <div class="popover-footer">
        <button class="footer-btn add-btn" onclick={openFilePicker}>
          <span>+ Add More Files</span>
        </button>
        <button class="footer-btn clear-btn" onclick={clearAll}>
          <span>Clear All</span>
        </button>
      </div>
    </div>
  {/if}

  <input
    bind:this={fileInputRef}
    type="file"
    multiple
    accept=".pdf,.docx,.doc,.xlsx,.xls,.pptx,.ppt,.json,.html,.htm,.md,.markdown,.txt,application/pdf"
    onchange={handleFileInputChange}
    style="display: none;"
  />
</div>

<style>
  .pipeline-documents-badge-container {
    position: absolute;
    top: 16px;
    left: 16px;
    z-index: 100;
  }

  .badge-pill {
    display: inline-flex;
    align-items: center;
    background: var(--bg-surface-elevated, #1e293b);
    border: 1px solid var(--border-color, #334155);
    border-radius: 20px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
    overflow: hidden;
    backdrop-filter: blur(8px);
  }

  .badge-summary-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    background: none;
    border: none;
    color: var(--text-primary, #f8fafc);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
  }

  .badge-summary-btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .doc-icon {
    font-size: 1rem;
    line-height: 1;
  }

  .doc-title {
    max-width: 240px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .caret {
    font-size: 0.75rem;
    color: var(--text-secondary, #94a3b8);
    transition: transform 0.2s ease;
  }

  .caret.open {
    transform: rotate(180deg);
  }

  .add-quick-btn {
    display: flex;
    align-items: center;
    padding: 6px 12px;
    background: rgba(59, 130, 246, 0.15);
    border: none;
    border-left: 1px solid var(--border-color, #334155);
    color: var(--accent-primary, #60a5fa);
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .add-quick-btn:hover {
    background: var(--accent-primary, #3b82f6);
    color: #ffffff;
  }

  /* Popover */
  .documents-popover {
    position: absolute;
    top: 42px;
    left: 0;
    width: 320px;
    background: var(--bg-surface, #0f172a);
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-md, 8px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: fadeIn 0.15s ease;
  }

  .popover-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    border-bottom: 1px solid var(--border-color, #334155);
    background: var(--bg-surface-elevated, #1e293b);
  }

  .popover-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary, #f8fafc);
  }

  .count-tag {
    background: var(--accent-primary, #3b82f6);
    color: #ffffff;
    font-size: 0.7rem;
    padding: 1px 6px;
    border-radius: 10px;
    font-weight: 700;
  }

  .close-popover-btn {
    background: none;
    border: none;
    color: var(--text-secondary, #94a3b8);
    font-size: 1.2rem;
    line-height: 1;
    cursor: pointer;
    padding: 0 4px;
  }

  .close-popover-btn:hover {
    color: var(--text-primary, #ffffff);
  }

  .doc-list-scroll {
    max-height: 220px;
    overflow-y: auto;
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .doc-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    background: var(--bg-secondary, #1e293b);
    border: 1px solid transparent;
    border-radius: var(--border-radius-sm, 6px);
    transition: border-color 0.15s;
  }

  .doc-item:hover {
    border-color: var(--border-color, #334155);
  }

  .item-icon {
    font-size: 1.1rem;
    flex-shrink: 0;
  }

  .item-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .item-name {
    font-size: 0.8rem;
    color: var(--text-primary, #f8fafc);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-size {
    font-size: 0.7rem;
    color: var(--text-secondary, #94a3b8);
  }

  .item-remove-btn {
    background: none;
    border: none;
    color: var(--text-secondary, #94a3b8);
    font-size: 1.1rem;
    line-height: 1;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .item-remove-btn:hover {
    color: var(--error-color, #ef4444);
    background: rgba(239, 68, 68, 0.1);
  }

  .popover-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border-top: 1px solid var(--border-color, #334155);
    background: var(--bg-surface-elevated, #1e293b);
  }

  .footer-btn {
    padding: 6px 10px;
    font-size: 0.75rem;
    font-weight: 600;
    border-radius: var(--border-radius-sm, 6px);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .footer-btn.add-btn {
    background: var(--accent-primary, #3b82f6);
    border: none;
    color: #ffffff;
  }

  .footer-btn.add-btn:hover {
    background: #2563eb;
  }

  .footer-btn.clear-btn {
    background: transparent;
    border: 1px solid var(--border-color, #334155);
    color: var(--text-secondary, #94a3b8);
  }

  .footer-btn.clear-btn:hover {
    color: var(--error-color, #ef4444);
    border-color: var(--error-color, #ef4444);
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
