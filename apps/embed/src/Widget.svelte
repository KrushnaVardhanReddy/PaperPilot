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
    { id: 'merge', name: 'Merge PDFs', description: 'Combine multiple PDFs into one' },
    { id: 'split', name: 'Split PDF', description: 'Split by page range' },
    { id: 'rotate', name: 'Rotate Pages', description: 'Rotate pages clockwise' },
    { id: 'watermark', name: 'Watermark', description: 'Add custom watermark stamp' },
    { id: 'extract', name: 'Extract Pages', description: 'Extract specific pages or range' },
    { id: 'delete', name: 'Delete Pages', description: 'Remove specific pages or range' },
    { id: 'reorder', name: 'Reverse Order', description: 'Reverse page sequence' },
    { id: 'compress', name: 'Optimize/Save', description: 'Compress and optimize PDF streams' }
  ];

  let availableTools = $derived(
    tools === 'all'
      ? allTools
      : allTools.filter(t => tools.split(',').map(s => s.trim().toLowerCase()).includes(t.id))
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

  // Active tool and configurable options
  let activeToolId = $state<string | null>(null);
  let pageInput = $state('1'); // for extract, delete, split
  let rotateAngle = $state('90'); // 90, 180, 270
  let rotatePages = $state('all'); // 'all', or specific page like '1'
  let watermarkText = $state('CONFIDENTIAL');

  function selectTool(toolId: string) {
    if (activeToolId === toolId) {
      activeToolId = null;
    } else {
      activeToolId = toolId;
      if (toolId === 'extract' || toolId === 'delete') {
        pageInput = '1';
      } else if (toolId === 'split') {
        pageInput = '1-2';
      }
    }
  }

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
  let processedFiles = $state<Array<{ name: string; url: string; size: number }>>([]);

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
      processedFiles = [];
      dispatchEvent('files-selected', { count: droppedFiles.length, files: droppedFiles.map(f => f.name) });
    }
  }

  function handleFileInput(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const incoming = Array.from(target.files);
      droppedFiles = [...droppedFiles, ...incoming];
      processedFiles = [];
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
      const { PDFDocument, degrees, rgb, StandardFonts } = await import('pdf-lib');
      const outputs: Array<{ name: string; url: string; size: number }> = [];

      function parsePageNumbers(input: string, totalPages: number): number[] {
        const result = new Set<number>();
        const parts = input.split(',').map(s => s.trim()).filter(Boolean);
        for (const part of parts) {
          if (part.includes('-')) {
            const [startStr, endStr] = part.split('-');
            const start = parseInt(startStr, 10);
            const end = parseInt(endStr, 10);
            if (!isNaN(start) && !isNaN(end)) {
              for (let p = Math.max(1, start); p <= Math.min(totalPages, end); p++) {
                result.add(p - 1); // 0-based index
              }
            }
          } else {
            const p = parseInt(part, 10);
            if (!isNaN(p) && p >= 1 && p <= totalPages) {
              result.add(p - 1);
            }
          }
        }
        return Array.from(result).sort((a, b) => a - b);
      }

      if (toolId === 'merge') {
        const mergedDoc = await PDFDocument.create();

        for (const file of droppedFiles) {
          const arrayBuffer = await file.arrayBuffer();
          const loadedDoc = await PDFDocument.load(arrayBuffer);
          const pageIndices = loadedDoc.getPageIndices();
          const copiedPages = await mergedDoc.copyPages(loadedDoc, pageIndices);
          copiedPages.forEach(page => mergedDoc.addPage(page));
        }

        const finalBytes = await mergedDoc.save();
        const finalBlob = new Blob([finalBytes], { type: 'application/pdf' });
        outputs.push({
          name: 'merged_document.pdf',
          url: URL.createObjectURL(finalBlob),
          size: finalBlob.size
        });
      } else {
        // Batch process each file individually
        for (const file of droppedFiles) {
          const arrayBuffer = await file.arrayBuffer();
          let finalBytes: Uint8Array;
          let outName: string;

          if (toolId === 'rotate') {
            const doc = await PDFDocument.load(arrayBuffer);
            const pages = doc.getPages();
            const angleNum = parseInt(rotateAngle, 10) || 90;

            let targetIndices: number[];
            if (rotatePages.trim().toLowerCase() === 'all' || !rotatePages.trim()) {
              targetIndices = pages.map((_, i) => i);
            } else {
              targetIndices = parsePageNumbers(rotatePages, pages.length);
            }

            for (const idx of targetIndices) {
              if (pages[idx]) {
                const currentRotation = pages[idx].getRotation().angle;
                pages[idx].setRotation(degrees((currentRotation + angleNum) % 360));
              }
            }
            finalBytes = await doc.save();
            outName = `rotated_${file.name}`;
          } else if (toolId === 'watermark') {
            const doc = await PDFDocument.load(arrayBuffer);
            const font = await doc.embedFont(StandardFonts.HelveticaBold);
            const pages = doc.getPages();
            const stamp = watermarkText.trim() || 'CONFIDENTIAL';
            for (const page of pages) {
              const { width, height } = page.getSize();
              page.drawText(stamp, {
                x: width / 6,
                y: height / 2,
                size: Math.min(width, height) / 14,
                font,
                color: rgb(0.85, 0.2, 0.2),
                opacity: 0.35,
                rotate: { type: 'degrees', angle: 45 } as any
              });
            }
            finalBytes = await doc.save();
            outName = `watermarked_${file.name}`;
          } else if (toolId === 'split' || toolId === 'extract') {
            const srcDoc = await PDFDocument.load(arrayBuffer);
            const newDoc = await PDFDocument.create();
            const total = srcDoc.getPageCount();
            const indices = parsePageNumbers(pageInput, total);
            const validIndices = indices.length > 0 ? indices : [0];

            const copiedPages = await newDoc.copyPages(srcDoc, validIndices);
            copiedPages.forEach(p => newDoc.addPage(p));
            finalBytes = await newDoc.save();
            outName = `${toolId}_p${validIndices.map(i => i + 1).join('_')}_${file.name}`;
          } else if (toolId === 'delete') {
            const doc = await PDFDocument.load(arrayBuffer);
            const total = doc.getPageCount();
            const indices = parsePageNumbers(pageInput, total);
            const sortedDesc = [...indices].sort((a, b) => b - a);
            for (const idx of sortedDesc) {
              if (doc.getPageCount() > 1 && idx < doc.getPageCount()) {
                doc.removePage(idx);
              }
            }
            finalBytes = await doc.save();
            outName = `deleted_pages_${file.name}`;
          } else if (toolId === 'reorder') {
            const srcDoc = await PDFDocument.load(arrayBuffer);
            const newDoc = await PDFDocument.create();
            const count = srcDoc.getPageCount();
            const reversedIndices = Array.from({ length: count }, (_, i) => count - 1 - i);
            const copied = await newDoc.copyPages(srcDoc, reversedIndices);
            copied.forEach(p => newDoc.addPage(p));
            finalBytes = await newDoc.save();
            outName = `reversed_${file.name}`;
          } else {
            // compress / optimize or default
            const doc = await PDFDocument.load(arrayBuffer);
            finalBytes = await doc.save({ useObjectStreams: true });
            outName = `optimized_${file.name}`;
          }

          const blob = new Blob([finalBytes], { type: 'application/pdf' });
          outputs.push({
            name: outName,
            url: URL.createObjectURL(blob),
            size: blob.size
          });
        }
      }

      processedFiles = outputs;
      isProcessing = false;
      dispatchEvent('process-complete', {
        tool: toolId,
        count: outputs.length,
        files: outputs.map(o => o.name)
      });

      // Automatically trigger downloads for generated files
      outputs.forEach((item, index) => {
        setTimeout(() => {
          const a = document.createElement('a');
          a.href = item.url;
          a.download = item.name;
          document.body.appendChild(a);
          a.click();
          document.body.removeChild(a);
        }, index * 250); // slight stagger so browser doesn't block multi-download
      });
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

  {#if processedFiles.length > 0}
    <div class="pp-download-container">
      {#each processedFiles as file}
        <div class="pp-download-card">
          <div class="download-info">
            <span class="download-icon">✅</span>
            <div>
              <strong>{file.name}</strong>
              <span class="download-meta">{(file.size / 1024).toFixed(1)} KB &bull; Processed Successfully</span>
            </div>
          </div>
          <a href={file.url} download={file.name} class="download-btn">
            📥 Download
          </a>
        </div>
      {/each}
    </div>
  {/if}

  <div class="pp-tools">
    {#each availableTools as tool (tool.id)}
      <button
        class="pp-tool-card {activeToolId === tool.id ? 'active' : ''} {isProcessing ? 'disabled' : ''}"
        onclick={() => selectTool(tool.id)}
        disabled={isProcessing}
        data-tool-id={tool.id}
      >
        <span class="tool-name">{tool.name}</span>
      </button>
    {/each}
  </div>

  {#if activeToolId}
    <div class="pp-config-panel">
      <div class="config-header">
        <span class="config-title">
          ⚙️ Configure {availableTools.find(t => t.id === activeToolId)?.name}
        </span>
        <span class="config-desc">
          {availableTools.find(t => t.id === activeToolId)?.description}
        </span>
      </div>

      <div class="config-body">
        {#if activeToolId === 'extract' || activeToolId === 'delete'}
          <div class="config-field">
            <label for="page-input">Page numbers or ranges (e.g. 1, 3-5):</label>
            <input
              id="page-input"
              type="text"
              class="pp-input"
              bind:value={pageInput}
              placeholder="e.g. 1 or 2, 4-6"
            />
          </div>
        {:else if activeToolId === 'split'}
          <div class="config-field">
            <label for="split-input">Pages to extract/split (e.g. 1-2 or 3, 5):</label>
            <input
              id="split-input"
              type="text"
              class="pp-input"
              bind:value={pageInput}
              placeholder="e.g. 1-2"
            />
          </div>
        {:else if activeToolId === 'rotate'}
          <div class="config-row">
            <div class="config-field">
              <label for="rotate-angle">Rotation Angle:</label>
              <select id="rotate-angle" class="pp-select" bind:value={rotateAngle}>
                <option value="90">90° Clockwise</option>
                <option value="180">180° Flip</option>
                <option value="270">270° (90° Counter-Clockwise)</option>
              </select>
            </div>
            <div class="config-field">
              <label for="rotate-pages">Pages (all or 1, 2-3):</label>
              <input
                id="rotate-pages"
                type="text"
                class="pp-input"
                bind:value={rotatePages}
                placeholder="all"
              />
            </div>
          </div>
        {:else if activeToolId === 'watermark'}
          <div class="config-field">
            <label for="watermark-text">Watermark Stamp Text:</label>
            <input
              id="watermark-text"
              type="text"
              class="pp-input"
              bind:value={watermarkText}
              placeholder="CONFIDENTIAL"
            />
          </div>
        {:else if activeToolId === 'merge'}
          <p class="config-info">
            📑 Merges all {droppedFiles.length} uploaded files in order. Drop or add more files above to change order.
          </p>
        {:else if activeToolId === 'reorder'}
          <p class="config-info">
            🔀 Reverses the order of all pages in the document.
          </p>
        {:else if activeToolId === 'compress'}
          <p class="config-info">
            🗜️ Optimizes xref tables and flattens redundant object streams.
          </p>
        {/if}

        <button
          class="pp-execute-btn"
          onclick={() => handleProcessStart(activeToolId!)}
          disabled={isProcessing}
        >
          {isProcessing ? '⏳ Processing In-Browser...' : `Run ${availableTools.find(t => t.id === activeToolId)?.name} →`}
        </button>
      </div>
    </div>
  {/if}

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

  .pp-download-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
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

  .pp-tool-card.active {
    border-color: var(--pp-brand-color);
    background-color: var(--pp-brand-color);
    color: white;
    box-shadow: 0 0 0 2px rgba(94, 106, 210, 0.4);
  }

  .pp-config-panel {
    background-color: var(--pp-surface);
    border: 1px solid var(--pp-border);
    border-radius: var(--pp-radius);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .config-header {
    display: flex;
    flex-direction: column;
    gap: 4px;
    border-bottom: 1px solid var(--pp-border);
    padding-bottom: 8px;
  }

  .config-title {
    font-weight: 600;
    font-size: 14px;
    color: var(--pp-text);
  }

  .config-desc {
    font-size: 12px;
    color: var(--pp-text-muted);
  }

  .config-body {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .config-row {
    display: flex;
    gap: 12px;
  }

  .config-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
  }

  .config-field label {
    font-size: 12px;
    font-weight: 500;
    color: var(--pp-text-muted);
  }

  .pp-input, .pp-select {
    background-color: var(--pp-background);
    border: 1px solid var(--pp-border);
    color: var(--pp-text);
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 13px;
    outline: none;
    font-family: inherit;
    transition: border-color 0.2s;
  }

  .pp-input:focus, .pp-select:focus {
    border-color: var(--pp-brand-color);
  }

  .config-info {
    font-size: 13px;
    color: var(--pp-text-muted);
    margin: 4px 0;
  }

  .pp-execute-btn {
    background-color: var(--pp-brand-color);
    color: white;
    border: none;
    border-radius: 6px;
    padding: 10px 16px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s;
    font-family: inherit;
    align-self: flex-start;
  }

  .pp-execute-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .pp-execute-btn:disabled {
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
