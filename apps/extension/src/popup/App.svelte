<script lang="ts">
  import { onMount } from 'svelte';
  import {
    checkGatewayHealth,
    chunkDocumentViaGateway,
    type GatewayHealth,
    type ChunkItem,
  } from '../background/gatewayClient';
  import {
    splitPdf,
    mergePdfs,
    rotatePdf,
    watermarkPdf,
    compressPdf,
    extractText,
    extractMetadata,
    type PdfMetadata,
    getWasmStatus,
  } from '../wasm/wasmBridge';

  // State runes
  let gatewayConnected = $state<boolean>(false);
  let gatewayInfo = $state<GatewayHealth | null>(null);
  let wasmEngineStatus = $state<string>('uninitialized');
  let activeTab = $state<'actions' | 'tools' | 'preview'>('actions');
  let activeTool = $state<'split' | 'merge' | 'rotate' | 'watermark' | 'compress' | 'extract'>('split');

  // File drop & processing state
  let currentFile = $state<File | null>(null);
  let fileBytes = $state<Uint8Array | null>(null);
  let metadata = $state<PdfMetadata | null>(null);
  let secondaryFile = $state<File | null>(null);
  let secondaryBytes = $state<Uint8Array | null>(null);
  let isDragOver = $state<boolean>(false);
  let isProcessing = $state<boolean>(false);
  let statusMessage = $state<string>('');
  let statusType = $state<'info' | 'success' | 'error'>('info');

  // Tool settings
  let splitRange = $state<string>('1');
  let rotateAngle = $state<number>(90);
  let watermarkText = $state<string>('CONFIDENTIAL');
  let watermarkOpacity = $state<number>(0.3);
  let compressQuality = $state<string>('medium');

  // Results state
  let capturedMarkdown = $state<string>('');
  let capturedTitle = $state<string>('');
  let chunks = $state<ChunkItem[]>([]);
  let downloadUrl = $state<string | null>(null);
  let downloadName = $state<string | null>(null);

  onMount(async () => {
    await refreshConnection();
    wasmEngineStatus = getWasmStatus();

    // Check if background worker cached recent captures
    if (typeof chrome !== 'undefined' && chrome.storage?.local) {
      chrome.storage.local.get(['lastExtractedArticle', 'lastSemanticChunks'], (result) => {
        if (result.lastExtractedArticle) {
          capturedTitle = result.lastExtractedArticle.title;
          capturedMarkdown = result.lastExtractedArticle.markdown;
        }
        if (result.lastSemanticChunks) {
          chunks = result.lastSemanticChunks.chunks || [];
        }
      });
    }
  });

  async function refreshConnection() {
    isProcessing = true;
    try {
      const health = await checkGatewayHealth();
      gatewayConnected = health.ok;
      gatewayInfo = health;
      wasmEngineStatus = getWasmStatus();
    } catch {
      gatewayConnected = false;
    } finally {
      isProcessing = false;
    }
  }

  function setStatus(msg: string, type: 'info' | 'success' | 'error' = 'info') {
    statusMessage = msg;
    statusType = type;
  }

  // Handle PDF file selection / drop
  async function handleFile(file: File) {
    if (!file.name.toLowerCase().endsWith('.pdf') && file.type !== 'application/pdf') {
      setStatus('Please select a valid PDF document.', 'error');
      return;
    }

    try {
      isProcessing = true;
      setStatus(`Loading ${file.name}...`);
      currentFile = file;
      const buffer = await file.arrayBuffer();
      fileBytes = new Uint8Array(buffer);
      metadata = await extractMetadata(fileBytes);
      setStatus(`Loaded "${file.name}" (${metadata.pageCount} pages)`, 'success');
    } catch (err: any) {
      setStatus(`Failed to read PDF: ${err.message || err}`, 'error');
    } finally {
      isProcessing = false;
    }
  }

  async function handleSecondaryFile(file: File) {
    try {
      secondaryFile = file;
      const buf = await file.arrayBuffer();
      secondaryBytes = new Uint8Array(buf);
      setStatus(`Added 2nd PDF "${file.name}" for merging`, 'info');
    } catch (err: any) {
      setStatus(`Failed to read 2nd PDF: ${err.message}`, 'error');
    }
  }

  function onDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    if (e.dataTransfer && e.dataTransfer.files.length > 0) {
      handleFile(e.dataTransfer.files[0]);
    }
  }

  function onFileInput(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      handleFile(input.files[0]);
    }
  }

  // Action: Extract Active Tab to Markdown
  async function extractActiveTab() {
    isProcessing = true;
    setStatus('Extracting clean article from active tab...');
    try {
      if (typeof chrome === 'undefined' || !chrome.tabs) {
        throw new Error('Chrome extension tabs API not available');
      }

      const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
      if (!tabs[0]?.id) {
        throw new Error('No active tab found');
      }

      const response = await chrome.tabs.sendMessage(tabs[0].id, { action: 'EXTRACT_ARTICLE' });
      if (response && response.success) {
        capturedTitle = response.data.title;
        capturedMarkdown = response.data.markdown;
        activeTab = 'preview';
        setStatus(`Extracted article "${capturedTitle}" (${response.data.wordCount} words)`, 'success');
      } else {
        throw new Error(response?.error || 'No readable article detected on page');
      }
    } catch (err: any) {
      setStatus(`Extraction failed: ${err.message}`, 'error');
    } finally {
      isProcessing = false;
    }
  }

  // Action: Chunk for AI Context
  async function chunkActiveContent() {
    isProcessing = true;
    setStatus('Analyzing document & generating semantic chunks for AI...');
    try {
      let contentToChunk = capturedMarkdown;

      // If no extracted markdown yet, extract from tab first
      if (!contentToChunk) {
        if (typeof chrome !== 'undefined' && chrome.tabs) {
          const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
          if (tabs[0]?.id) {
            const resp = await chrome.tabs.sendMessage(tabs[0].id, { action: 'EXTRACT_ARTICLE' });
            if (resp && resp.success) {
              contentToChunk = resp.data.markdown;
              capturedTitle = resp.data.title;
              capturedMarkdown = contentToChunk;
            }
          }
        }
      }

      if (!contentToChunk && fileBytes) {
        contentToChunk = await extractText(fileBytes);
        capturedTitle = currentFile?.name || 'Local Document';
      }

      if (!contentToChunk) {
        throw new Error('No document content or active page text available to chunk');
      }

      if (gatewayConnected) {
        const result = await chunkDocumentViaGateway({ text: contentToChunk }, { max_chunk_size: 900 });
        chunks = result.chunks;
      } else {
        // Standalone WASM / semantic paragraph mode
        const paragraphs = contentToChunk.split(/\n\n+/).filter((p) => p.trim().length > 30);
        chunks = paragraphs.map((text, i) => ({
          index: i + 1,
          text: text.trim(),
          tokenCount: Math.ceil(text.length / 4),
        }));
      }

      activeTab = 'preview';
      setStatus(`Generated ${chunks.length} AI semantic chunks`, 'success');
    } catch (err: any) {
      setStatus(`Chunking failed: ${err.message}`, 'error');
    } finally {
      isProcessing = false;
    }
  }

  // PDF Tool: Run current tool
  async function executeCurrentPdfTool() {
    if (!fileBytes || !currentFile) {
      setStatus('Please load a PDF file first.', 'error');
      return;
    }

    isProcessing = true;
    setStatus(`Executing ${activeTool}...`);
    try {
      let resultBytes: Uint8Array | null = null;
      let outName = `paperpilot-${activeTool}-${currentFile.name}`;

      switch (activeTool) {
        case 'split': {
          const parts = await splitPdf(fileBytes, splitRange);
          if (parts.length > 0) {
            resultBytes = parts[0];
            outName = `part1-${currentFile.name}`;
          }
          break;
        }
        case 'merge': {
          if (!secondaryBytes) {
            throw new Error('Please select a 2nd PDF to merge');
          }
          resultBytes = await mergePdfs([fileBytes, secondaryBytes]);
          outName = `merged-${currentFile.name}`;
          break;
        }
        case 'rotate': {
          resultBytes = await rotatePdf(fileBytes, rotateAngle);
          outName = `rotated-${rotateAngle}deg-${currentFile.name}`;
          break;
        }
        case 'watermark': {
          resultBytes = await watermarkPdf(fileBytes, watermarkText, 45, watermarkOpacity);
          outName = `watermarked-${currentFile.name}`;
          break;
        }
        case 'compress': {
          resultBytes = await compressPdf(fileBytes, compressQuality);
          outName = `compressed-${currentFile.name}`;
          break;
        }
        case 'extract': {
          const text = await extractText(fileBytes);
          capturedTitle = currentFile.name;
          capturedMarkdown = text;
          activeTab = 'preview';
          setStatus(`Extracted text from "${currentFile.name}"`, 'success');
          isProcessing = false;
          return;
        }
      }

      if (resultBytes) {
        const blob = new Blob([resultBytes], { type: 'application/pdf' });
        downloadUrl = URL.createObjectURL(blob);
        downloadName = outName;
        setStatus(`Successfully generated ${outName}!`, 'success');
      }
    } catch (err: any) {
      setStatus(`Operation failed: ${err.message}`, 'error');
    } finally {
      isProcessing = false;
    }
  }

  function copyToClipboard(text: string, label = 'Copied!') {
    navigator.clipboard.writeText(text);
    setStatus(label, 'success');
  }

  function downloadText(text: string, filename: string) {
    const blob = new Blob([text], { type: 'text/markdown;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="app">
  <!-- Brand Header -->
  <header class="header">
    <div class="header-left">
      <div class="logo-mark">
        <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" stroke="#38bdf8" />
          <polyline points="14 2 14 8 20 8" stroke="#38bdf8" />
          <path d="M12 18v-6" stroke="#10b981" />
          <path d="m9 15 3-3 3 3" stroke="#10b981" />
        </svg>
      </div>
      <div class="titles">
        <div class="title-row">
          <span class="app-name">PaperPilot</span>
          <span class="app-sub">Companion</span>
        </div>
        <span class="version-label">v1.0 • Universal Doc AI</span>
      </div>
    </div>

    <!-- Connection Status Badge -->
    <div class="header-right">
      {#if gatewayConnected}
        <button
          class="badge badge-desktop"
          title="PaperPilot Desktop Gateway active on port 7823. Click to recheck."
          onclick={refreshConnection}
        >
          <span class="dot pulse"></span>
          <span class="badge-text">⚡ Desktop Connected (7823)</span>
        </button>
      {:else}
        <button
          class="badge badge-wasm"
          title="Gateway offline. Running in zero-install client-side WebAssembly mode. Click to recheck."
          onclick={refreshConnection}
        >
          <span class="dot"></span>
          <span class="badge-text">🌐 Standalone WASM Mode</span>
        </button>
      {/if}
    </div>
  </header>

  <!-- Navigation Tabs -->
  <nav class="nav-tabs">
    <button
      class="tab-btn"
      class:active={activeTab === 'actions'}
      onclick={() => (activeTab = 'actions')}
    >
      ⚡ Quick Actions
    </button>
    <button
      class="tab-btn"
      class:active={activeTab === 'tools'}
      onclick={() => (activeTab = 'tools')}
    >
      🛠️ PDF Tools
    </button>
    <button
      class="tab-btn"
      class:active={activeTab === 'preview'}
      onclick={() => (activeTab = 'preview')}
    >
      📑 Content ({chunks.length > 0 ? `${chunks.length} chunks` : capturedMarkdown ? 'Markdown' : '0'})
    </button>
  </nav>

  <!-- Main Content Area -->
  <main class="main">
    <!-- TAB 1: Quick Actions -->
    {#if activeTab === 'actions'}
      <section class="section">
        <div class="actions-grid">
          <button
            class="action-card primary-action"
            onclick={extractActiveTab}
            disabled={isProcessing}
          >
            <div class="action-icon">📄</div>
            <div class="action-info">
              <span class="action-title">Extract Active Tab to Markdown</span>
              <span class="action-desc">Strips clutter, ads, navigation and formats into clean Markdown</span>
            </div>
          </button>

          <button
            class="action-card ai-action"
            onclick={chunkActiveContent}
            disabled={isProcessing}
          >
            <div class="action-icon">🧠</div>
            <div class="action-info">
              <span class="action-title">Chunk for AI Context</span>
              <span class="action-desc">
                {gatewayConnected ? 'Neural semantic chunking via Desktop Gateway' : 'Client-side WASM chunking with token estimates'}
              </span>
            </div>
          </button>

          <button
            class="action-card tools-action"
            onclick={() => (activeTab = 'tools')}
          >
            <div class="action-icon">🔧</div>
            <div class="action-info">
              <span class="action-title">Quick PDF Tools</span>
              <span class="action-desc">Split, Merge, Watermark, Compress, Rotate directly in-browser</span>
            </div>
          </button>
        </div>

        <!-- Drop Zone in Quick Actions -->
        <div
          class="dropzone"
          class:drag-over={isDragOver}
          ondragover={(e) => { e.preventDefault(); isDragOver = true; }}
          ondragleave={() => (isDragOver = false)}
          ondrop={onDrop}
          role="region"
          aria-label="PDF drop zone"
        >
          <input
            type="file"
            id="pdf-upload"
            accept=".pdf,application/pdf"
            class="file-input"
            onchange={onFileInput}
          />
          <label for="pdf-upload" class="dropzone-label">
            <svg viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="2" class="upload-icon">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="17 8 12 3 7 8" />
              <line x1="12" y1="3" x2="12" y2="15" />
            </svg>
            <span class="dropzone-primary">
              {currentFile ? currentFile.name : 'Drop local PDF here or click to browse'}
            </span>
            <span class="dropzone-secondary">
              {metadata ? `${metadata.pageCount} pages • ${metadata.version} • ${(currentFile!.size / 1024).toFixed(1)} KB` : '100% private in-browser WASM processing'}
            </span>
          </label>
        </div>
      </section>
    {/if}

    <!-- TAB 2: PDF Tools -->
    {#if activeTab === 'tools'}
      <section class="section">
        <!-- Sub-tool selector bar -->
        <div class="tool-pills">
          <button class="pill" class:active={activeTool === 'split'} onclick={() => (activeTool = 'split')}>Split</button>
          <button class="pill" class:active={activeTool === 'merge'} onclick={() => (activeTool = 'merge')}>Merge</button>
          <button class="pill" class:active={activeTool === 'rotate'} onclick={() => (activeTool = 'rotate')}>Rotate</button>
          <button class="pill" class:active={activeTool === 'watermark'} onclick={() => (activeTool = 'watermark')}>Watermark</button>
          <button class="pill" class:active={activeTool === 'compress'} onclick={() => (activeTool = 'compress')}>Compress</button>
          <button class="pill" class:active={activeTool === 'extract'} onclick={() => (activeTool = 'extract')}>Text</button>
        </div>

        <!-- File preview header -->
        <div class="file-summary-bar">
          {#if currentFile}
            <div class="file-badge">
              <span class="file-icon">📄</span>
              <span class="file-name" title={currentFile.name}>{currentFile.name}</span>
              <span class="file-pages">{metadata?.pageCount ?? '?'} pgs</span>
            </div>
          {:else}
            <div class="no-file-prompt">
              <input type="file" id="tool-upload" accept=".pdf" class="file-input" onchange={onFileInput} />
              <label for="tool-upload" class="load-btn">Select PDF to Process</label>
            </div>
          {/if}
        </div>

        <!-- Tool Specific Form -->
        <div class="tool-form-card">
          {#if activeTool === 'split'}
            <div class="form-group">
              <label for="split-range">Page Ranges (e.g. "1", "1-2, 3-5"):</label>
              <input id="split-range" type="text" bind:value={splitRange} class="text-input" placeholder="1-2, 3-4" />
            </div>
          {:else if activeTool === 'merge'}
            <div class="form-group">
              <label for="sec-file">Second PDF Document to Merge:</label>
              <input
                id="sec-file"
                type="file"
                accept=".pdf"
                class="file-input"
                onchange={(e) => {
                  const f = (e.target as HTMLInputElement).files?.[0];
                  if (f) handleSecondaryFile(f);
                }}
              />
              <span class="sub-label">{secondaryFile ? `Attached: ${secondaryFile.name}` : 'Select 2nd PDF to append'}</span>
            </div>
          {:else if activeTool === 'rotate'}
            <div class="form-group">
              <label for="rotate-angle">Rotation Angle:</label>
              <div class="radio-row">
                <button class="choice-btn" class:selected={rotateAngle === 90} onclick={() => (rotateAngle = 90)}>90° CW</button>
                <button class="choice-btn" class:selected={rotateAngle === 180} onclick={() => (rotateAngle = 180)}>180°</button>
                <button class="choice-btn" class:selected={rotateAngle === 270} onclick={() => (rotateAngle = 270)}>270° CW</button>
              </div>
            </div>
          {:else if activeTool === 'watermark'}
            <div class="form-group">
              <label for="wm-text">Watermark Text:</label>
              <input id="wm-text" type="text" bind:value={watermarkText} class="text-input" placeholder="CONFIDENTIAL" />
              <label for="wm-op" class="mt-2">Opacity: {watermarkOpacity}</label>
              <input id="wm-op" type="range" min="0.1" max="0.9" step="0.1" bind:value={watermarkOpacity} class="range-input" />
            </div>
          {:else if activeTool === 'compress'}
            <div class="form-group">
              <label for="comp-q">Compression Profile:</label>
              <select id="comp-q" bind:value={compressQuality} class="select-input">
                <option value="low">Low (Fast stream clean)</option>
                <option value="medium">Medium (Deflate + clean crossrefs)</option>
                <option value="high">High (Maximum compaction)</option>
              </select>
            </div>
          {:else if activeTool === 'extract'}
            <div class="info-note">
              Extracts all pure embedded text content from the document with layout preservation.
            </div>
          {/if}

          <button
            class="btn-execute"
            onclick={executeCurrentPdfTool}
            disabled={!fileBytes || isProcessing}
          >
            {isProcessing ? 'Processing...' : `Run ${activeTool.toUpperCase()}`}
          </button>
        </div>

        <!-- Download ready card -->
        {#if downloadUrl && downloadName}
          <div class="download-card">
            <span class="dl-title">🎉 Result Ready: {downloadName}</span>
            <a href={downloadUrl} download={downloadName} class="dl-button">
              📥 Download PDF
            </a>
          </div>
        {/if}
      </section>
    {/if}

    <!-- TAB 3: Extracted Preview & Chunks -->
    {#if activeTab === 'preview'}
      <section class="section">
        {#if chunks.length > 0}
          <div class="preview-header">
            <div class="ph-title">
              <h3>AI Semantic Chunks</h3>
              <span class="chunk-badge">{chunks.length} chunks</span>
            </div>
            <button
              class="btn-small"
              onclick={() => copyToClipboard(chunks.map((c) => `[Chunk ${c.index}]\n${c.text}`).join('\n\n---\n\n'), 'Copied all chunks!')}
            >
              📋 Copy All
            </button>
          </div>

          <div class="chunks-container">
            {#each chunks as chunk, i}
              <div class="chunk-card">
                <div class="chunk-meta">
                  <span class="chunk-num">#{chunk.index || i + 1}</span>
                  {#if chunk.tokenCount}
                    <span class="chunk-tokens">~{chunk.tokenCount} tokens</span>
                  {/if}
                  <button
                    class="btn-micro"
                    onclick={() => copyToClipboard(chunk.text, `Copied chunk #${chunk.index || i + 1}`)}
                  >
                    Copy
                  </button>
                </div>
                <p class="chunk-text">{chunk.text}</p>
              </div>
            {/each}
          </div>
        {:else if capturedMarkdown}
          <div class="preview-header">
            <div class="ph-title">
              <h3>{capturedTitle || 'Extracted Document'}</h3>
            </div>
            <div class="ph-actions">
              <button
                class="btn-small"
                onclick={() => copyToClipboard(capturedMarkdown, 'Markdown copied!')}
              >
                📋 Copy
              </button>
              <button
                class="btn-small"
                onclick={() => downloadText(capturedMarkdown, `${capturedTitle || 'document'}.md`)}
              >
                📥 Save .md
              </button>
            </div>
          </div>

          <div class="markdown-viewer">
            <pre class="md-pre">{capturedMarkdown}</pre>
          </div>
        {:else}
          <div class="empty-state">
            <p>No content captured yet.</p>
            <p class="sub">Click "Extract Active Tab" or "Quick PDF Tools" to begin.</p>
          </div>
        {/if}
      </section>
    {/if}
  </main>

  <!-- Status / Toast Bar -->
  {#if statusMessage}
    <footer class="status-bar" class:success={statusType === 'success'} class:error={statusType === 'error'}>
      <span class="status-text">{statusMessage}</span>
    </footer>
  {/if}
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 560px;
    background-color: #0b0f17;
    color: #e2e8f0;
    font-size: 13px;
    box-sizing: border-box;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: #0f172a;
    border-bottom: 1px solid #1e293b;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .logo-mark {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    background: linear-gradient(135deg, #1e293b, #0f172a);
    border: 1px solid #334155;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .title-row {
    display: flex;
    align-items: baseline;
    gap: 6px;
  }

  .app-name {
    font-weight: 700;
    font-size: 15px;
    color: #f8fafc;
    letter-spacing: -0.3px;
  }

  .app-sub {
    font-size: 11px;
    color: #38bdf8;
    text-transform: uppercase;
    font-weight: 600;
    letter-spacing: 0.5px;
  }

  .version-label {
    font-size: 10px;
    color: #64748b;
  }

  .badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 9999px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.2s ease;
  }

  .badge-desktop {
    background: rgba(16, 185, 129, 0.15);
    color: #34d399;
    border-color: rgba(16, 185, 129, 0.3);
  }

  .badge-wasm {
    background: rgba(56, 189, 248, 0.12);
    color: #38bdf8;
    border-color: rgba(56, 189, 248, 0.25);
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
  }

  .dot.pulse {
    box-shadow: 0 0 8px currentColor;
  }

  .nav-tabs {
    display: flex;
    border-bottom: 1px solid #1e293b;
    background: #0f172a;
    padding: 0 12px;
  }

  .tab-btn {
    flex: 1;
    padding: 10px 8px;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: #94a3b8;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tab-btn:hover {
    color: #f1f5f9;
  }

  .tab-btn.active {
    color: #38bdf8;
    border-bottom-color: #38bdf8;
    font-weight: 600;
  }

  .main {
    flex: 1;
    padding: 14px 16px;
    overflow-y: auto;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .actions-grid {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .action-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 10px;
    background: #1e293b;
    border: 1px solid #334155;
    color: #f1f5f9;
    cursor: pointer;
    text-align: left;
    transition: transform 0.15s ease, background 0.15s ease, border-color 0.15s ease;
  }

  .action-card:hover {
    transform: translateY(-1px);
    background: #253349;
    border-color: #475569;
  }

  .action-card:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .primary-action:hover {
    border-color: #38bdf8;
  }

  .ai-action:hover {
    border-color: #a855f7;
  }

  .action-icon {
    font-size: 20px;
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    background: #0f172a;
    border: 1px solid #334155;
  }

  .action-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .action-title {
    font-size: 13px;
    font-weight: 600;
    color: #f8fafc;
  }

  .action-desc {
    font-size: 11px;
    color: #94a3b8;
    line-height: 1.3;
  }

  .dropzone {
    margin-top: 4px;
    border: 2px dashed #334155;
    border-radius: 10px;
    padding: 18px 12px;
    text-align: center;
    background: rgba(15, 23, 42, 0.6);
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .dropzone:hover,
  .dropzone.drag-over {
    border-color: #38bdf8;
    background: rgba(56, 189, 248, 0.05);
  }

  .file-input {
    display: none;
  }

  .dropzone-label {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    cursor: pointer;
  }

  .upload-icon {
    color: #64748b;
  }

  .dropzone:hover .upload-icon {
    color: #38bdf8;
  }

  .dropzone-primary {
    font-size: 12px;
    font-weight: 600;
    color: #cbd5e1;
  }

  .dropzone-secondary {
    font-size: 11px;
    color: #64748b;
  }

  .tool-pills {
    display: flex;
    gap: 6px;
    overflow-x: auto;
    padding-bottom: 2px;
  }

  .pill {
    padding: 6px 12px;
    border-radius: 6px;
    background: #1e293b;
    border: 1px solid #334155;
    color: #94a3b8;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
  }

  .pill.active {
    background: #0284c7;
    color: #ffffff;
    border-color: #38bdf8;
  }

  .file-summary-bar {
    background: #0f172a;
    padding: 8px 12px;
    border-radius: 8px;
    border: 1px solid #1e293b;
  }

  .file-badge {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .file-name {
    font-weight: 500;
    color: #f1f5f9;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 250px;
  }

  .file-pages {
    margin-left: auto;
    font-size: 11px;
    color: #38bdf8;
    background: rgba(56, 189, 248, 0.1);
    padding: 2px 6px;
    border-radius: 4px;
  }

  .load-btn {
    display: inline-block;
    padding: 6px 12px;
    background: #334155;
    color: #f1f5f9;
    border-radius: 6px;
    font-size: 11px;
    cursor: pointer;
  }

  .tool-form-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 10px;
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .text-input,
  .select-input {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 6px;
    padding: 8px 10px;
    color: #f8fafc;
    font-size: 12px;
  }

  .text-input:focus,
  .select-input:focus {
    border-color: #38bdf8;
    outline: none;
  }

  .radio-row {
    display: flex;
    gap: 8px;
  }

  .choice-btn {
    flex: 1;
    padding: 6px;
    background: #0f172a;
    border: 1px solid #334155;
    color: #cbd5e1;
    border-radius: 6px;
    cursor: pointer;
    font-size: 11px;
  }

  .choice-btn.selected {
    background: #0284c7;
    color: #fff;
    border-color: #38bdf8;
  }

  .range-input {
    accent-color: #38bdf8;
  }

  .btn-execute {
    padding: 10px;
    border-radius: 6px;
    background: linear-gradient(135deg, #0284c7, #2563eb);
    color: #ffffff;
    font-weight: 600;
    border: none;
    cursor: pointer;
    transition: opacity 0.2s ease;
  }

  .btn-execute:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn-execute:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .download-card {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    border-radius: 8px;
    padding: 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .dl-title {
    font-size: 12px;
    color: #34d399;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 240px;
  }

  .dl-button {
    background: #10b981;
    color: #0b0f17;
    font-weight: 600;
    padding: 6px 12px;
    border-radius: 6px;
    text-decoration: none;
    font-size: 11px;
  }

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 8px;
    border-bottom: 1px solid #1e293b;
  }

  .ph-title {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .ph-title h3 {
    margin: 0;
    font-size: 13px;
    color: #f8fafc;
  }

  .chunk-badge {
    background: #334155;
    color: #38bdf8;
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: 600;
  }

  .ph-actions {
    display: flex;
    gap: 6px;
  }

  .btn-small {
    padding: 4px 10px;
    background: #1e293b;
    border: 1px solid #334155;
    color: #f1f5f9;
    border-radius: 4px;
    font-size: 11px;
    cursor: pointer;
  }

  .chunks-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 380px;
    overflow-y: auto;
  }

  .chunk-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 10px;
  }

  .chunk-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 6px;
  }

  .chunk-num {
    font-weight: 700;
    color: #38bdf8;
    font-size: 11px;
  }

  .chunk-tokens {
    font-size: 10px;
    color: #94a3b8;
  }

  .btn-micro {
    margin-left: auto;
    background: #0f172a;
    border: 1px solid #334155;
    color: #cbd5e1;
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 4px;
    cursor: pointer;
  }

  .chunk-text {
    margin: 0;
    font-size: 12px;
    line-height: 1.4;
    color: #e2e8f0;
    word-break: break-word;
  }

  .markdown-viewer {
    max-height: 380px;
    overflow-y: auto;
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 8px;
    padding: 12px;
  }

  .md-pre {
    margin: 0;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 11px;
    white-space: pre-wrap;
    word-break: break-word;
    color: #cbd5e1;
  }

  .empty-state {
    padding: 40px 16px;
    text-align: center;
    color: #64748b;
  }

  .empty-state p {
    margin: 4px 0;
  }

  .empty-state .sub {
    font-size: 11px;
  }

  .status-bar {
    padding: 6px 12px;
    background: #1e293b;
    border-top: 1px solid #334155;
    font-size: 11px;
    color: #cbd5e1;
  }

  .status-bar.success {
    background: rgba(16, 185, 129, 0.15);
    color: #34d399;
  }

  .status-bar.error {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
  }
</style>
