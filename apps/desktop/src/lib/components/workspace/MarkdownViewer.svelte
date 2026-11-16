<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { toastState } from '$lib/state/toast.svelte';

  let { file }: { file: File } = $props();

  let markdownContent = $state('');
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let activeTab = $state<'preview' | 'split' | 'raw'>('preview');

  async function loadMarkdownContent() {
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

      markdownContent = text;
    } catch (err: any) {
      loadError = err?.message || 'Failed to read Markdown file.';
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    if (file) {
      loadMarkdownContent();
    }
  });

  function copyToClipboard() {
    if (!markdownContent) return;
    navigator.clipboard.writeText(markdownContent);
    toastState.info('Markdown copied to clipboard');
  }

  function formatBytes(bytes: number): string {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
  }

  /**
   * Lightweight and secure zero-dependency markdown renderer
   */
  function renderMarkdown(md: string): string {
    if (!md) return '';

    // 1. Escape basic HTML entities to prevent XSS
    let escaped = md
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');

    // 2. Fenced code blocks
    escaped = escaped.replace(/```([a-zA-Z0-9_-]*)\n([\s\S]*?)```/g, (_match, _lang, code) => {
      return `<pre class="md-pre"><code class="md-code-block">${code.trim()}</code></pre>`;
    });

    // 3. Inline code
    escaped = escaped.replace(/`([^`]+)`/g, '<code class="md-inline-code">$1</code>');

    // 4. Headings (h1 - h6)
    escaped = escaped
      .replace(/^###### (.*$)/gim, '<h6 class="md-h6">$1</h6>')
      .replace(/^##### (.*$)/gim, '<h5 class="md-h5">$1</h5>')
      .replace(/^#### (.*$)/gim, '<h4 class="md-h4">$1</h4>')
      .replace(/^### (.*$)/gim, '<h3 class="md-h3">$1</h3>')
      .replace(/^## (.*$)/gim, '<h2 class="md-h2">$1</h2>')
      .replace(/^# (.*$)/gim, '<h1 class="md-h1">$1</h1>');

    // 5. Blockquotes
    escaped = escaped.replace(/^\> (.*$)/gim, '<blockquote class="md-blockquote">$1</blockquote>');

    // 6. Horizontal Rules
    escaped = escaped.replace(/^(\*\*\*|---|___)$/gim, '<hr class="md-hr" />');

    // 7. Bold and Italic
    escaped = escaped
      .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
      .replace(/__([^_]+)__/g, '<strong>$1</strong>')
      .replace(/\*([^*]+)\*/g, '<em>$1</em>')
      .replace(/_([^_]+)_/g, '<em>$1</em>');

    // 8. Markdown Links
    escaped = escaped.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener noreferrer" class="md-link">$1</a>');

    // 9. Unordered list items
    escaped = escaped.replace(/^\s*[-*+]\s+(.*$)/gim, '<li class="md-li">$1</li>');

    // 10. Simple paragraphs (newlines separation)
    const lines = escaped.split(/\n\n+/);
    const html = lines
      .map(block => {
        const trimmed = block.trim();
        if (
          trimmed.startsWith('<h') ||
          trimmed.startsWith('<pre') ||
          trimmed.startsWith('<blockquote') ||
          trimmed.startsWith('<hr') ||
          trimmed.startsWith('<li')
        ) {
          return trimmed;
        }
        return `<p class="md-p">${trimmed.replace(/\n/g, '<br/>')}</p>`;
      })
      .join('\n');

    return html;
  }
</script>

<div class="md-viewer-container">
  <!-- Toolbar Header -->
  <div class="md-toolbar">
    <div class="toolbar-left">
      <span class="file-icon">📝</span>
      <span class="file-title">{file.name}</span>
      <span class="file-badge">{formatBytes(file.size)}</span>
      <span class="status-badge valid">Markdown Document</span>
    </div>

    <div class="toolbar-center">
      <div class="view-tabs">
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'preview'}
          onclick={() => activeTab = 'preview'}
        >
          Rich Preview
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
          class:active={activeTab === 'raw'}
          onclick={() => activeTab = 'raw'}
        >
          Raw Markdown
        </button>
      </div>
    </div>

    <div class="toolbar-right">
      <button class="action-btn" onclick={copyToClipboard} title="Copy Markdown to clipboard">
        📋 Copy
      </button>
    </div>
  </div>

  <!-- Content Stage -->
  <div class="md-stage">
    {#if isLoading}
      <div class="md-loader">
        <span class="spinner">⏳</span>
        <span>Loading Markdown document...</span>
      </div>
    {:else if loadError}
      <div class="md-error">
        <span class="error-icon">⚠️</span>
        <p class="error-msg">{loadError}</p>
      </div>
    {:else if activeTab === 'preview'}
      <div class="markdown-preview-pane">
        <article class="markdown-body">
          {@html renderMarkdown(markdownContent)}
        </article>
      </div>
    {:else if activeTab === 'raw'}
      <div class="markdown-raw-pane">
        <pre class="md-code"><code>{markdownContent}</code></pre>
      </div>
    {:else if activeTab === 'split'}
      <div class="markdown-split-container">
        <div class="split-raw">
          <pre class="md-code"><code>{markdownContent}</code></pre>
        </div>
        <div class="split-divider"></div>
        <div class="split-preview">
          <article class="markdown-body">
            {@html renderMarkdown(markdownContent)}
          </article>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .md-viewer-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background: #0f172a;
    color: #e2e8f0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  .md-toolbar {
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
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.3);
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

  .md-stage {
    flex: 1;
    overflow: auto;
    display: flex;
    flex-direction: column;
  }

  .markdown-preview-pane, .markdown-raw-pane {
    flex: 1;
    overflow: auto;
    padding: 24px 36px;
  }

  .markdown-split-container {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .split-raw, .split-preview {
    flex: 1;
    overflow: auto;
    padding: 20px;
  }

  .split-divider {
    width: 1px;
    background: #334155;
  }

  .markdown-body {
    max-width: 860px;
    margin: 0 auto;
    line-height: 1.7;
    font-size: 0.95rem;
    color: #e2e8f0;
  }

  :global(.md-h1) {
    font-size: 1.8rem;
    font-weight: 700;
    color: #f8fafc;
    border-bottom: 1px solid #334155;
    padding-bottom: 8px;
    margin-top: 24px;
    margin-bottom: 16px;
  }

  :global(.md-h2) {
    font-size: 1.4rem;
    font-weight: 600;
    color: #f1f5f9;
    border-bottom: 1px solid #334155;
    padding-bottom: 6px;
    margin-top: 20px;
    margin-bottom: 14px;
  }

  :global(.md-h3) {
    font-size: 1.15rem;
    font-weight: 600;
    color: #e2e8f0;
    margin-top: 16px;
    margin-bottom: 10px;
  }

  :global(.md-p) {
    margin-bottom: 14px;
    color: #cbd5e1;
  }

  :global(.md-pre) {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 14px 16px;
    overflow-x: auto;
    margin-bottom: 16px;
  }

  :global(.md-code-block) {
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
    font-size: 0.88rem;
    color: #38bdf8;
  }

  :global(.md-inline-code) {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 4px;
    padding: 2px 6px;
    font-family: monospace;
    font-size: 0.85rem;
    color: #38bdf8;
  }

  :global(.md-blockquote) {
    border-left: 4px solid #3b82f6;
    padding-left: 14px;
    margin-left: 0;
    margin-bottom: 14px;
    color: #94a3b8;
    font-style: italic;
  }

  :global(.md-hr) {
    border: none;
    border-top: 1px solid #334155;
    margin: 24px 0;
  }

  :global(.md-li) {
    margin-left: 20px;
    margin-bottom: 6px;
    color: #cbd5e1;
  }

  :global(.md-link) {
    color: #60a5fa;
    text-decoration: underline;
  }

  .md-code {
    margin: 0;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.88rem;
    line-height: 1.6;
    color: #94a3b8;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .md-loader, .md-error {
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

