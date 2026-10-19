<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { getFileTypeInfo } from '$lib/utils/fileType';
  import { invoke } from '@tauri-apps/api/core';

  let { file, docIndex }: { file: File; docIndex: number } = $props();

  let fileInfo = $derived(getFileTypeInfo(file.name));
  let isConverting = $state(false);
  let conversionStatus = $state<string | null>(null);
  let conversionError = $state<string | null>(null);

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  async function executeTool(toolId: string) {
    const localPath = (file as any)._localPath || appState.documentPaths[docIndex];
    if (!localPath) {
      conversionError = 'File path not found. Please re-open the document from disk.';
      return;
    }

    isConverting = true;
    conversionStatus = `Processing ${toolId}...`;
    conversionError = null;

    try {
      if (toolId === 'excel_to_json' || toolId === 'excel_to_csv') {
        const ext = toolId === 'excel_to_json' ? 'json' : 'csv';
        const outputPath = localPath.replace(/\.[^/.]+$/, `.${ext}`);
        
        await invoke('run_operation', {
          args: [
            toolId,
            '--input', localPath,
            '--output', outputPath
          ]
        });

        conversionStatus = `Successfully generated ${outputPath.split('/').pop() || outputPath}`;
      } else if (toolId === 'office_to_markdown') {
        const outputPath = localPath.replace(/\.[^/.]+$/, '.md');
        await invoke('run_operation', {
          args: [
            'office_to_markdown',
            '--input', localPath,
            '--output', outputPath
          ]
        });
        conversionStatus = `Successfully converted to Markdown: ${outputPath.split('/').pop() || outputPath}`;
      } else if (toolId === 'pdf_convert_excel') {
        // Convert spreadsheet / CSV to PDF
        const outputPath = localPath.replace(/\.[^/.]+$/, '.pdf');
        await invoke('run_operation', {
          args: [
            'pdf_convert_excel',
            '--input', localPath,
            '--output', outputPath
          ]
        });
        conversionStatus = `Created PDF: ${outputPath.split('/').pop() || outputPath}`;
        
        // Load the generated PDF into workspace automatically!
        const bytes: number[] = await invoke('read_file_bytes', { path: outputPath });
        const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
        const newPdfFile = new File([blob], outputPath.split('/').pop() || 'converted.pdf', { type: 'application/pdf' });
        (newPdfFile as any)._localPath = outputPath;
        (newPdfFile as any)._isLoaded = true;
        appState.addDocuments([newPdfFile]);
        appState.selectDocument(appState.documents.length - 1);
      }
    } catch (e: any) {
      console.error(`Operation ${toolId} failed:`, e);
      conversionError = String(e?.message || e || 'Tool execution failed');
      conversionStatus = null;
    } finally {
      isConverting = false;
    }
  }
</script>

<div class="non-pdf-viewer">
  <div class="card-container">
    <div class="doc-badge-header">
      <span class="file-icon-large">{fileInfo.icon}</span>
      <div class="file-main-info">
        <h2 class="file-title" title={file.name}>{file.name}</h2>
        <div class="file-meta-row">
          <span class="meta-tag format">{fileInfo.label}</span>
          <span class="meta-tag size">{formatBytes(file.size)}</span>
          {#if (file as any)._localPath}
            <span class="meta-tag path" title={(file as any)._localPath}>
              {(file as any)._localPath.split('/').slice(-3).join('/')}
            </span>
          {/if}
        </div>
      </div>
    </div>

    <div class="info-alert">
      <div class="alert-icon">💡</div>
      <div class="alert-content">
        {#if fileInfo.category === 'spreadsheet'}
          <strong>Spreadsheet Loaded:</strong> This is a native spreadsheet file. PaperPilot gives you zero-install local tools to convert, extract, or turn this sheet directly into a searchable PDF document.
        {:else if fileInfo.category === 'document'}
          <strong>Office Document Loaded:</strong> Word documents can be converted to PDF, clean Markdown, or extracted as structured text using local rust engines.
        {:else}
          <strong>Binary File:</strong> Use the dedicated tools below or switch to another open tab.
        {/if}
      </div>
    </div>

    <div class="action-grid">
      {#if fileInfo.category === 'spreadsheet'}
        <div class="action-card">
          <div class="action-header">
            <span class="action-icon">📄</span>
            <div class="action-info">
              <h3>Convert to PDF</h3>
              <p>Turn this spreadsheet into a clean, printable PDF and preview it immediately in the workspace.</p>
            </div>
          </div>
          <button
            class="action-btn primary"
            disabled={isConverting}
            onclick={() => executeTool('pdf_convert_excel')}
          >
            {isConverting ? 'Processing...' : 'Generate & Open PDF'}
          </button>
        </div>

        <div class="action-card">
          <div class="action-header">
            <span class="action-icon">{'{ }'}</span>
            <div class="action-info">
              <h3>Export to JSON</h3>
              <p>Extract sheets, column headers, and rows into structured JSON for programmatic pipelines.</p>
            </div>
          </div>
          <button
            class="action-btn"
            disabled={isConverting}
            onclick={() => executeTool('excel_to_json')}
          >
            Export JSON
          </button>
        </div>

        <div class="action-card">
          <div class="action-header">
            <span class="action-icon">📑</span>
            <div class="action-info">
              <h3>Export to CSV</h3>
              <p>Export rows to standard RFC 4180 CSV for universal table compatibility.</p>
            </div>
          </div>
          <button
            class="action-btn"
            disabled={isConverting}
            onclick={() => executeTool('excel_to_csv')}
          >
            Export CSV
          </button>
        </div>

        <div class="action-card">
          <div class="action-header">
            <span class="action-icon">📝</span>
            <div class="action-info">
              <h3>Convert to Markdown</h3>
              <p>Generate clean Markdown tables suitable for documentation and LLM context windows.</p>
            </div>
          </div>
          <button
            class="action-btn"
            disabled={isConverting}
            onclick={() => executeTool('office_to_markdown')}
          >
            Convert to Markdown
          </button>
        </div>
      {:else}
        <div class="action-card">
          <div class="action-header">
            <span class="action-icon">📝</span>
            <div class="action-info">
              <h3>Convert to Markdown</h3>
              <p>Parse document contents into clean GitHub-flavored Markdown text.</p>
            </div>
          </div>
          <button
            class="action-btn primary"
            disabled={isConverting}
            onclick={() => executeTool('office_to_markdown')}
          >
            Convert to Markdown
          </button>
        </div>
      {/if}
    </div>

    {#if conversionStatus}
      <div class="status-banner success">
        <span class="status-icon">✓</span>
        <span>{conversionStatus}</span>
      </div>
    {/if}

    {#if conversionError}
      <div class="status-banner error">
        <span class="status-icon">⚠️</span>
        <span>{conversionError}</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .non-pdf-viewer {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding: 2.5rem 1.5rem;
    overflow-y: auto;
    background: var(--bg-canvas, #0f172a);
    color: var(--text-primary, #f8fafc);
  }

  .card-container {
    width: 100%;
    max-width: 820px;
    background: var(--bg-surface, #1e293b);
    border: 1px solid var(--border-color, #334155);
    border-radius: 14px;
    padding: 2rem;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.35);
    display: flex;
    flex-direction: column;
    gap: 1.75rem;
  }

  .doc-badge-header {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border-color, #334155);
  }

  .file-icon-large {
    font-size: 3.5rem;
    line-height: 1;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    padding: 0.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .file-main-info {
    flex: 1;
    min-width: 0;
  }

  .file-title {
    margin: 0 0 0.5rem 0;
    font-size: 1.4rem;
    font-weight: 600;
    color: #f1f5f9;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-meta-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }

  .meta-tag {
    font-size: 0.78rem;
    padding: 0.25rem 0.65rem;
    border-radius: 6px;
    background: #0f172a;
    border: 1px solid #334155;
    color: #94a3b8;
  }

  .meta-tag.format {
    background: rgba(16, 185, 129, 0.15);
    border-color: rgba(16, 185, 129, 0.3);
    color: #34d399;
    font-weight: 500;
  }

  .meta-tag.path {
    max-width: 280px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .info-alert {
    display: flex;
    gap: 0.85rem;
    align-items: flex-start;
    padding: 1rem 1.25rem;
    border-radius: 8px;
    background: rgba(59, 130, 246, 0.08);
    border: 1px solid rgba(59, 130, 246, 0.25);
    color: #cbd5e1;
    font-size: 0.9rem;
    line-height: 1.45;
  }

  .alert-icon {
    font-size: 1.25rem;
    line-height: 1;
  }

  .action-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1rem;
  }

  .action-card {
    background: rgba(15, 23, 42, 0.6);
    border: 1px solid var(--border-color, #334155);
    border-radius: 10px;
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 1rem;
    transition: transform 0.15s ease, border-color 0.15s ease;
  }

  .action-card:hover {
    border-color: #64748b;
    transform: translateY(-1px);
  }

  .action-header {
    display: flex;
    gap: 0.85rem;
    align-items: flex-start;
  }

  .action-icon {
    font-size: 1.5rem;
    line-height: 1;
    margin-top: 0.1rem;
  }

  .action-info h3 {
    margin: 0 0 0.3rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #f8fafc;
  }

  .action-info p {
    margin: 0;
    font-size: 0.82rem;
    color: #94a3b8;
    line-height: 1.4;
  }

  .action-btn {
    padding: 0.6rem 1rem;
    border-radius: 6px;
    border: 1px solid #475569;
    background: #1e293b;
    color: #e2e8f0;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn:hover:not(:disabled) {
    background: #334155;
    color: #ffffff;
    border-color: #64748b;
  }

  .action-btn.primary {
    background: #3b82f6;
    border-color: #2563eb;
    color: #ffffff;
  }

  .action-btn.primary:hover:not(:disabled) {
    background: #2563eb;
    border-color: #1d4ed8;
  }

  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .status-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    font-size: 0.88rem;
  }

  .status-banner.success {
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: #34d399;
  }

  .status-banner.error {
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
  }

  .status-icon {
    font-weight: bold;
  }
</style>
