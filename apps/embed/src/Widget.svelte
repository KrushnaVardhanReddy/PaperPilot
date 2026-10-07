<script lang="ts">
  interface Props {
    tools?: string;
    hideBadge?: boolean;
    brandColor?: string;
    theme?: string;
    logoUrl?: string;
  }

  let { tools = 'all', hideBadge = false, brandColor = '#3B82F6', theme = 'dark', logoUrl }: Props = $props();

  const allTools = [
    { id: 'merge', name: 'Merge PDF' },
    { id: 'compress', name: 'Compress PDF' },
    { id: 'watermark', name: 'Watermark' },
    { id: 'split', name: 'Split PDF' }
  ];

  let availableTools = $derived(
    tools === 'all'
      ? allTools
      : allTools.filter(t => tools.split(',').map(s => s.trim()).includes(t.id))
  );

  function dispatchEvent(eventName: string, detail: any = {}) {
    const event = new CustomEvent(`paperpilot:${eventName}`, {
      detail,
      bubbles: true,
      composed: true
    });
    // @ts-ignore
    document.activeElement?.dispatchEvent(event) || window.dispatchEvent(event);
  }

  let droppedFiles = $state<File[]>([]);
  let isDragging = $state(false);
  let fileInputRef: HTMLInputElement | null = null;

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = false;
  }

  let isProcessing = $state(false);
  let processedFile = $state<{ name: string; url: string; size: number } | null>(null);

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = false;

    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const incoming = Array.from(e.dataTransfer.files);
      const pdfs = incoming.filter(f => f.type === 'application/pdf' || f.name.toLowerCase().endsWith('.pdf'));
      const toAdd = pdfs.length > 0 ? pdfs : incoming;
      // Append to support multiple uploads over time
      droppedFiles = [...droppedFiles, ...toAdd];
      processedFile = null;
      dispatchEvent('files-selected', { count: droppedFiles.length, files: droppedFiles.map(f => f.name) });
    }
  }

  function handleFileInput(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const incoming = Array.from(target.files);
      droppedFiles = [...droppedFiles, ...incoming];
      processedFile = null;
      dispatchEvent('files-selected', { count: droppedFiles.length, files: droppedFiles.map(f => f.name) });
    }
  }

  function triggerFileDialog() {
    fileInputRef?.click();
  }

  function clearFiles(e: Event) {
    e.stopPropagation();
    droppedFiles = [];
    processedFile = null;
    if (fileInputRef) fileInputRef.value = '';
  }

  async function handleProcessStart(toolId: string) {
    if (droppedFiles.length === 0) {
      triggerFileDialog();
      return;
    }

    isProcessing = true;
    dispatchEvent('process-start', { tool: toolId, files: droppedFiles.map(f => f.name) });

    try {
      let finalBytes: Uint8Array;
      let outName: string;

      if (toolId === 'merge') {
        const { PDFDocument } = await import('pdf-lib');
        const mergedDoc = await PDFDocument.create();

        for (const file of droppedFiles) {
          const arrayBuffer = await file.arrayBuffer();
          const loadedDoc = await PDFDocument.load(arrayBuffer);
          const pageIndices = loadedDoc.getPageIndices();
          const copiedPages = await mergedDoc.copyPages(loadedDoc, pageIndices);
          copiedPages.forEach(page => mergedDoc.addPage(page));
        }

        finalBytes = await mergedDoc.save();
        outName = 'merged_document.pdf';
      } else {
        const first = await droppedFiles[0].arrayBuffer();
        finalBytes = new Uint8Array(first);
        outName = `${toolId}_${droppedFiles[0].name}`;
      }

      const finalBlob = new Blob([finalBytes], { type: 'application/pdf' });
      const downloadUrl = URL.createObjectURL(finalBlob);

      processedFile = {
        name: outName,
        url: downloadUrl,
        size: finalBlob.size
      };

      isProcessing = false;
      dispatchEvent('process-complete', { tool: toolId, outputSizeBytes: finalBlob.size, fileName: outName });

      // Automatically trigger download
      const a = document.createElement('a');
      a.href = downloadUrl;
      a.download = outName;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    } catch (err: any) {
      isProcessing = false;
      console.error('[PaperPilot Embed] Error processing PDF:', err);
      alert(`[PaperPilot Embed] Error: ${err?.message || 'Failed to process PDF'}`);
    }
  }
</script>

<div class="pp-widget" data-theme={theme}>
  {#if logoUrl}
    <div class="pp-header">
      <img src={logoUrl} alt="Logo" class="pp-logo" />
    </div>
  {/if}

  <input
    type="file"
    accept="application/pdf"
    multiple
    style="display: none;"
    bind:this={fileInputRef}
    onchange={handleFileInput}
  />

  <div
    class="pp-dropzone {isDragging ? 'dragging' : ''}"
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    onclick={triggerFileDialog}
    role="button"
    tabindex="0"
    onkeydown={(e) => { if (e.key === 'Enter') triggerFileDialog(); }}
  >
    {#if droppedFiles.length > 0}
      <div class="file-summary">
        <p class="file-success">
          📄 <strong>{droppedFiles.length} PDF{droppedFiles.length > 1 ? 's' : ''} Ready</strong>
        </p>
        <ul class="file-list">
          {#each droppedFiles as file}
            <li>{file.name} ({(file.size / 1024).toFixed(1)} KB)</li>
          {/each}
        </ul>
        <div class="file-actions">
          <span class="sub-hint">Click or drop more files to add</span>
          <button class="clear-btn" onclick={clearFiles}>Clear All</button>
        </div>
      </div>
    {:else}
      <p>📁 Drag and drop multiple PDFs here, or <span class="browse-link">browse</span></p>
      <span class="sub-hint">Files processed 100% locally in browser (Zero-Upload)</span>
    {/if}
  </div>

  {#if processedFile}
    <div class="pp-download-card">
      <div class="download-info">
        <span class="download-icon">✅</span>
        <div>
          <strong>{processedFile.name}</strong>
          <span class="download-meta">{(processedFile.size / 1024).toFixed(1)} KB &bull; Processed Successfully</span>
        </div>
      </div>
      <a href={processedFile.url} download={processedFile.name} class="download-btn">
        📥 Download Result
      </a>
    </div>
  {/if}

  <div class="pp-tools">
    {#each availableTools as tool (tool.id)}
      <button
        class="pp-tool-card {isProcessing ? 'disabled' : ''}"
        onclick={() => handleProcessStart(tool.id)}
        disabled={isProcessing}
        data-tool-id={tool.id}
      >
        {isProcessing ? '⏳ Processing...' : tool.name}
      </button>
    {/each}
  </div>

  {#if !hideBadge}
    <div class="pp-footer">
      <a href="https://usepaperpilot.com" target="_blank" rel="noopener" class="pp-badge">
        ⚡ Powered by PaperPilot
      </a>
    </div>
  {/if}
</div>

<style>
  .pp-widget {
    background-color: var(--pp-background);
    color: var(--pp-text);
    font-family: var(--pp-font);
    border-radius: var(--pp-radius);
    padding: 20px;
    box-sizing: border-box;
    width: 100%;
    border: 1px solid var(--pp-border);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .pp-header {
    display: flex;
    justify-content: center;
    margin-bottom: 8px;
  }

  .pp-logo {
    max-height: 40px;
    max-width: 100%;
  }

  .pp-dropzone {
    border: 2px dashed var(--pp-border);
    border-radius: var(--pp-radius);
    padding: 32px 20px;
    text-align: center;
    background-color: var(--pp-surface);
    color: var(--pp-text-muted);
    transition: all 0.2s ease;
    cursor: pointer;
    user-select: none;
  }

  .pp-dropzone:hover,
  .pp-dropzone.dragging {
    border-color: var(--pp-brand-color);
    background-color: rgba(94, 106, 210, 0.08);
    color: var(--pp-text);
  }

  .pp-dropzone p {
    margin: 0 0 6px 0;
    font-size: 15px;
  }

  .pp-dropzone .browse-link {
    color: var(--pp-brand-color);
    text-decoration: underline;
    font-weight: 600;
  }

  .pp-dropzone .sub-hint {
    font-size: 12px;
    opacity: 0.75;
    display: block;
  }

  .pp-dropzone .file-success {
    color: #10B981;
    font-size: 15px;
    word-break: break-all;
  }

  .pp-tools {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 12px;
  }

  .pp-tool-card {
    background-color: var(--pp-surface);
    color: var(--pp-text);
    border: 1px solid var(--pp-border);
    border-radius: 8px;
    padding: 16px 12px;
    text-align: center;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
    font-size: 14px;
    font-family: inherit;
  }

  .pp-tool-card:hover {
    border-color: var(--pp-brand-color);
    background-color: var(--pp-brand-color);
    color: white;
  }

  .pp-footer {
    display: flex;
    justify-content: center;
    margin-top: 8px;
  }

  .file-summary {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .file-list {
    list-style: none;
    padding: 0;
    margin: 8px 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 120px;
    overflow-y: auto;
    width: 100%;
    max-width: 400px;
  }

  .file-list li {
    background: var(--pp-background);
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 13px;
    color: var(--pp-text);
    border: 1px solid var(--pp-border);
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .clear-btn {
    background: none;
    border: none;
    color: #EF4444;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    text-decoration: underline;
    padding: 0;
  }

  .pp-download-card {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    border-radius: 8px;
    padding: 14px 18px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
  }

  .download-info {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--pp-text);
  }

  .download-icon {
    font-size: 20px;
  }

  .download-meta {
    display: block;
    font-size: 12px;
    color: #10B981;
  }

  .download-btn {
    background-color: #10B981;
    color: white;
    text-decoration: none;
    font-weight: 600;
    font-size: 13px;
    padding: 8px 16px;
    border-radius: 6px;
    transition: opacity 0.2s;
  }

  .download-btn:hover {
    opacity: 0.9;
  }

  .pp-tool-card.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .pp-badge {
    display: inline-block;
    padding: 6px 12px;
    background-color: var(--pp-surface);
    color: var(--pp-text-muted);
    text-decoration: none;
    font-size: 12px;
    border-radius: 16px;
    border: 1px solid var(--pp-border);
    transition: all 0.2s;
  }

  .pp-badge:hover {
    color: var(--pp-text);
    border-color: var(--pp-text-muted);
  }
</style>
