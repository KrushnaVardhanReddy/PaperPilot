<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { toastState } from '$lib/state/toast.svelte';

  let { file }: { file: File } = $props();

  let jsonContent = $state('');
  let parsedJson = $state<any>(null);
  let isValidJson = $state(true);
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let activeTab = $state<'formatted' | 'tree' | 'raw'>('formatted');
  let searchQuery = $state('');

  async function loadJsonContent() {
    isLoading = true;
    loadError = null;
    try {
      let text = '';
      const localPath = (file as any)._localPath;
      if (localPath) {
        try {
          const bytes = await invoke<number[]>('read_file_bytes', { path: localPath });
          text = new TextDecoder().decode(new Uint8Array(bytes));
        } catch {
          text = await file.text();
        }
      } else {
        text = await file.text();
      }

      jsonContent = text;
      try {
        parsedJson = JSON.parse(text);
        isValidJson = true;
      } catch (err: any) {
        isValidJson = false;
        parsedJson = null;
      }
    } catch (err: any) {
      loadError = err?.message || 'Failed to read JSON file.';
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    if (file) {
      loadJsonContent();
    }
  });

  function copyToClipboard() {
    if (!jsonContent) return;
    navigator.clipboard.writeText(jsonContent);
    toastState.info('JSON copied to clipboard');
  }

  function formatBytes(bytes: number): string {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
  }
</script>

<div class="json-viewer-container">
  <!-- Toolbar Header -->
  <div class="json-toolbar">
    <div class="toolbar-left">
      <span class="file-icon">⚡</span>
      <span class="file-title">{file.name}</span>
      <span class="file-badge">{formatBytes(file.size)}</span>
      {#if isValidJson}
        <span class="status-badge valid">✓ Valid JSON</span>
      {:else}
        <span class="status-badge invalid">⚠ Syntax Error</span>
      {/if}
    </div>

    <div class="toolbar-center">
      <div class="view-tabs">
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'formatted'}
          onclick={() => activeTab = 'formatted'}
        >
          Formatted
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'tree'}
          onclick={() => activeTab = 'tree'}
        >
          Visual Hierarchy
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'raw'}
          onclick={() => activeTab = 'raw'}
        >
          Raw
        </button>
      </div>
    </div>

    <div class="toolbar-right">
      <button class="action-btn" onclick={copyToClipboard} title="Copy JSON to clipboard">
        📋 Copy
      </button>
    </div>
  </div>

  <!-- Content Stage -->
  <div class="json-stage">
    {#if isLoading}
      <div class="json-loader">
        <span class="spinner">⏳</span>
        <span>Loading JSON document...</span>
      </div>
    {:else if loadError}
      <div class="json-error">
        <span class="error-icon">⚠️</span>
        <p class="error-msg">{loadError}</p>
      </div>
    {:else if activeTab === 'formatted' || activeTab === 'raw'}
      <div class="code-editor-pane">
        <pre class="json-code"><code>{activeTab === 'formatted' && isValidJson ? JSON.stringify(parsedJson, null, 2) : jsonContent}</code></pre>
      </div>
    {:else if activeTab === 'tree' && isValidJson}
      <div class="tree-pane">
        {#if parsedJson && typeof parsedJson === 'object'}
          {#if parsedJson.pages && Array.isArray(parsedJson.pages)}
            <!-- Specialized PaperPilot JsonDocument layout card preview -->
            <div class="document-summary-card">
              <div class="summary-header">
                <h3>Document Name: {parsedJson.document_name || file.name}</h3>
                <span class="page-count-badge">📄 {parsedJson.pages.length} Pages</span>
              </div>

              <div class="pages-grid">
                {#each parsedJson.pages as page, idx}
                  <div class="json-page-card">
                    <div class="page-card-header">
                      <span>Page {page.page_number || idx + 1}</span>
                      <span class="char-count">{page.char_count || page.text?.length || 0} chars</span>
                    </div>
                    <div class="page-card-body">
                      <p class="page-text-preview">{page.text || 'No text content'}</p>
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {:else}
            <!-- Generic JSON key-value inspector -->
            <div class="generic-tree-card">
              <pre class="generic-tree-code">{JSON.stringify(parsedJson, null, 2)}</pre>
            </div>
          {/if}
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .json-viewer-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background: #0f172a;
    color: #e2e8f0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  .json-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 18px;
    background: #1e293b;
    border-bottom: 1px solid #334155;
    gap: 12px;
  }

  .toolbar-left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .file-icon {
    font-size: 1.2rem;
  }

  .file-title {
    font-weight: 600;
    font-size: 0.95rem;
    color: #f8fafc;
  }

  .file-badge {
    background: #334155;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.75rem;
    color: #94a3b8;
  }

  .status-badge {
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .status-badge.valid {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .status-badge.invalid {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .view-tabs {
    display: flex;
    background: #0f172a;
    border-radius: 6px;
    padding: 3px;
    gap: 4px;
  }

  .tab-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    padding: 5px 12px;
    font-size: 0.8rem;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    color: #f1f5f9;
  }

  .tab-btn.active {
    background: #3b82f6;
    color: #ffffff;
    font-weight: 500;
  }

  .action-btn {
    background: #334155;
    border: 1px solid #475569;
    color: #e2e8f0;
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 0.8rem;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .action-btn:hover {
    background: #475569;
  }

  .json-stage {
    flex: 1;
    overflow: auto;
    padding: 20px;
    display: flex;
    flex-direction: column;
  }

  .code-editor-pane {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 16px;
    overflow: auto;
    flex: 1;
  }

  .json-code {
    margin: 0;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
    font-size: 0.88rem;
    line-height: 1.6;
    color: #38bdf8;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .tree-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .document-summary-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 18px;
  }

  .summary-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #334155;
    padding-bottom: 12px;
    margin-bottom: 16px;
  }

  .summary-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: #f8fafc;
  }

  .page-count-badge {
    background: #334155;
    color: #93c5fd;
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .pages-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
  }

  .json-page-card {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 6px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .page-card-header {
    background: #1e293b;
    padding: 8px 12px;
    font-size: 0.8rem;
    font-weight: 600;
    color: #cbd5e1;
    display: flex;
    justify-content: space-between;
    border-bottom: 1px solid #334155;
  }

  .char-count {
    font-weight: 400;
    color: #64748b;
  }

  .page-card-body {
    padding: 12px;
    flex: 1;
  }

  .page-text-preview {
    margin: 0;
    font-size: 0.82rem;
    color: #94a3b8;
    line-height: 1.5;
    white-space: pre-wrap;
    max-height: 180px;
    overflow-y: auto;
  }

  .generic-tree-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 16px;
  }

  .generic-tree-code {
    margin: 0;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.85rem;
    color: #38bdf8;
    white-space: pre-wrap;
  }

  .json-loader, .json-error {
    margin: auto;
    text-align: center;
    color: #94a3b8;
  }

  .error-icon {
    font-size: 2rem;
  }

  .error-msg {
    color: #f87171;
    margin-top: 8px;
  }
</style>

