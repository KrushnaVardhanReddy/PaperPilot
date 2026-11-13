<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { toastState } from '$lib/state/toast.svelte';

  let { file }: { file: File } = $props();

  let htmlContent = $state('');
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let activeTab = $state<'rendered' | 'split' | 'source'>('rendered');
  let iframeSrcDoc = $state('');

  async function loadHtmlContent() {
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

      htmlContent = text;
      iframeSrcDoc = text;
    } catch (err: any) {
      loadError = err?.message || 'Failed to read HTML file.';
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    if (file) {
      loadHtmlContent();
    }
  });

  function copyToClipboard() {
    if (!htmlContent) return;
    navigator.clipboard.writeText(htmlContent);
    toastState.info('HTML source copied to clipboard');
  }

  function formatBytes(bytes: number): string {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
  }
</script>

<div class="html-viewer-container">
  <!-- Toolbar Header -->
  <div class="html-toolbar">
    <div class="toolbar-left">
      <span class="file-icon">🌐</span>
      <span class="file-title">{file.name}</span>
      <span class="file-badge">{formatBytes(file.size)}</span>
      <span class="status-badge valid">HTML Document</span>
    </div>

    <div class="toolbar-center">
      <div class="view-tabs">
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'rendered'}
          onclick={() => activeTab = 'rendered'}
        >
          Rendered View
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'split'}
          onclick={() => activeTab = 'split'}
        >
          Split View
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'source'}
          onclick={() => activeTab = 'source'}
        >
          HTML Source
        </button>
      </div>
    </div>

    <div class="toolbar-right">
      <button class="action-btn" onclick={copyToClipboard} title="Copy HTML source to clipboard">
        📋 Copy
      </button>
    </div>
  </div>

  <!-- Content Stage -->
  <div class="html-stage">
    {#if isLoading}
      <div class="html-loader">
        <span class="spinner">⏳</span>
        <span>Loading HTML document...</span>
      </div>
    {:else if loadError}
      <div class="html-error">
        <span class="error-icon">⚠️</span>
        <p class="error-msg">{loadError}</p>
      </div>
    {:else if activeTab === 'rendered'}
      <div class="rendered-pane">
        <iframe
          class="html-sandbox-iframe"
          title="HTML Rendered Preview"
          sandbox="allow-same-origin"
          srcdoc={iframeSrcDoc}
        ></iframe>
      </div>
    {:else if activeTab === 'source'}
      <div class="html-source-pane">
        <pre class="html-code"><code>{htmlContent}</code></pre>
      </div>
    {:else if activeTab === 'split'}
      <div class="html-split-container">
        <div class="split-source">
          <pre class="html-code"><code>{htmlContent}</code></pre>
        </div>
        <div class="split-divider"></div>
        <div class="split-rendered">
          <iframe
            class="html-sandbox-iframe"
            title="HTML Rendered Preview (Split)"
            sandbox="allow-same-origin"
            srcdoc={iframeSrcDoc}
          ></iframe>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .html-viewer-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background: #0f172a;
    color: #e2e8f0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  .html-toolbar {
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
    background: rgba(16, 185, 129, 0.15);
    color: #34d399;
    border: 1px solid rgba(16, 185, 129, 0.3);
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

  .html-stage {
    flex: 1;
    overflow: auto;
    display: flex;
    flex-direction: column;
  }

  .rendered-pane, .html-source-pane {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .html-source-pane {
    overflow: auto;
    padding: 24px;
    background: #1e293b;
    margin: 16px;
    border-radius: 8px;
    border: 1px solid #334155;
  }

  .html-sandbox-iframe {
    width: 100%;
    height: 100%;
    border: none;
    background: #ffffff;
  }

  .html-split-container {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .split-source {
    flex: 1;
    overflow: auto;
    padding: 20px;
    background: #1e293b;
  }

  .split-rendered {
    flex: 1;
    overflow: hidden;
    background: #ffffff;
  }

  .split-divider {
    width: 2px;
    background: #334155;
  }

  .html-code {
    margin: 0;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
    font-size: 0.88rem;
    line-height: 1.6;
    color: #38bdf8;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .html-loader, .html-error {
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

