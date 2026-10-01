<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { jobsState } from '$lib/state/jobs.svelte';
  import { invoke } from '@tauri-apps/api/core';

  interface ToolDefinition {
    id: string;
    title: string;
    description: string;
    icon: string;
    category: 'quick' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert' | 'ai';
    tags: string[];
  }

  const allTools: ToolDefinition[] = [
    // Quick / Popular
    { id: 'compress', title: 'Compress PDF', description: 'Reduce document file size with image optimization', icon: '🗜️', category: 'quick', tags: ['shrink', 'size', 'optimize'] },
    { id: 'merge', title: 'Merge PDFs', description: 'Combine multiple PDF files into one continuous document', icon: '📑', category: 'quick', tags: ['combine', 'join', 'append'] },
    { id: 'rotate', title: 'Rotate Pages', description: 'Rotate pages by 90, 180, or 270 degrees', icon: '🔄', category: 'quick', tags: ['orientation', 'turn'] },
    { id: 'compare', title: 'Compare PDFs', description: 'Diff two documents and highlight changes', icon: '🔍', category: 'quick', tags: ['diff', 'compare', 'difference'] },
    { id: 'ocr', title: 'OCR Text Recognition', description: 'Extract text from scanned pages using local OCR', icon: '👁️', category: 'quick', tags: ['scan', 'text', 'extract'] },
    { id: 'pdf_to_docx', title: 'PDF → Word', description: 'Convert document into editable DOCX format', icon: '📝', category: 'quick', tags: ['word', 'docx', 'convert', 'export'] },

    // Page Operations
    { id: 'merge', title: 'Merge Documents', description: 'Combine open tabs into a single PDF', icon: '📑', category: 'pages', tags: ['combine', 'join', 'concat'] },
    { id: 'split', title: 'Split PDF', description: 'Divide document into separate pages or ranges', icon: '✂️', category: 'pages', tags: ['divide', 'cut', 'separate'] },
    { id: 'compare', title: 'Compare Documents', description: 'Side-by-side comparison between two open tabs', icon: '🔍', category: 'pages', tags: ['diff', 'compare'] },
    { id: 'extract_pages', title: 'Extract Pages', description: 'Save specific pages into a new PDF document', icon: '📤', category: 'pages', tags: ['select', 'isolate'] },
    { id: 'rotate', title: 'Rotate Document', description: 'Orient landscape or portrait pages', icon: '🔄', category: 'pages', tags: ['orientation', 'turn', 'flip'] },

    // Edit & Markup
    { id: 'watermark', title: 'Add Watermark', description: 'Stamp text or branding across all pages', icon: '💧', category: 'edit', tags: ['stamp', 'text', 'brand', 'copyright'] },
    { id: 'flatten', title: 'Flatten Annotations', description: 'Bake annotations & forms permanently into page graphics', icon: '📄', category: 'edit', tags: ['bake', 'lock', 'rasterize'] },

    // Optimize & Repair
    { id: 'compress', title: 'Compress PDF', description: 'Lossy & lossless file size reduction', icon: '🗜️', category: 'optimize', tags: ['shrink', 'size', 'dpi'] },
    { id: 'repair', title: 'Repair PDF', description: 'Recover and rebuild corrupted or unreadable documents', icon: '🔧', category: 'optimize', tags: ['corrupt', 'fix', 'recover'] },

    // Security & Privacy
    { id: 'encrypt', title: 'Encrypt with Password', description: 'Protect document with AES-256 password encryption', icon: '🔒', category: 'security', tags: ['protect', 'password', 'lock'] },
    { id: 'decrypt', title: 'Remove Password', description: 'Remove password restrictions from unlocked PDF', icon: '🔓', category: 'security', tags: ['unlock', 'open'] },
    { id: 'metadata', title: 'Edit / Sanitize Metadata', description: 'Inspect and clean author, title, and creation timestamps', icon: '📋', category: 'security', tags: ['sanitize', 'privacy', 'info'] },

    // Convert & Export
    { id: 'pdf_to_docx', title: 'PDF to Word (DOCX)', description: 'Export document as editable Microsoft Word file', icon: '📝', category: 'convert', tags: ['word', 'docx'] },
    { id: 'pdf_to_xlsx', title: 'PDF to Excel (XLSX)', description: 'Extract tabular data into spreadsheets', icon: '📊', category: 'convert', tags: ['excel', 'sheet', 'table'] },
    { id: 'pdf_to_markdown', title: 'PDF to Markdown', description: 'Convert clean layout into LLM-ready markdown text', icon: '📑', category: 'convert', tags: ['llm', 'markdown', 'md'] },
    { id: 'extract_text', title: 'Extract Plain Text', description: 'Extract all raw textual content to a .txt file', icon: '📄', category: 'convert', tags: ['text', 'dump', 'raw'] },

    // AI & Intelligence
    { id: 'ocr', title: 'OCR Recognition', description: 'Run optical character recognition on scanned pages', icon: '👁️', category: 'ai', tags: ['ocr', 'scan', 'tesseract'] },
    { id: 'extract_images', title: 'Extract Embedded Images', description: 'Save all images found inside the document', icon: '🖼️', category: 'ai', tags: ['images', 'photos', 'export'] }
  ];

  const categories = [
    { id: 'quick', title: 'Quick Actions', icon: '⚡' },
    { id: 'pages', title: 'Page Management', icon: '📑' },
    { id: 'edit', title: 'Edit & Markup', icon: '✍️' },
    { id: 'optimize', title: 'Optimize & Repair', icon: '🗜️' },
    { id: 'security', title: 'Security & Privacy', icon: '🔒' },
    { id: 'convert', title: 'Convert & Export', icon: '🔄' },
    { id: 'ai', title: 'AI & Intelligence', icon: '🧠' }
  ];

  let searchQuery = $state('');
  let activeTool = $state<ToolDefinition | null>(null);

  // Tool parameter states
  let splitPoints = $state('');
  let compressQuality = $state(80);
  let rotateAngle = $state('90');
  let watermarkText = $state('');
  let password = $state('');
  let metadataTitle = $state('');

  // Filter tools based on search query
  let filteredTools = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) return allTools;
    return allTools.filter(t =>
      t.title.toLowerCase().includes(q) ||
      t.description.toLowerCase().includes(q) ||
      t.tags.some(tag => tag.toLowerCase().includes(q))
    );
  });

  function selectTool(tool: ToolDefinition) {
    activeTool = tool;
  }

  function backToDirectory() {
    activeTool = null;
  }

  async function handleRunOperation() {
    if (!activeTool) return;
    if (appState.documents.length === 0) {
      toastState.error('No documents available. Add or drop a PDF first.');
      return;
    }

    if (activeTool.id === 'merge' && appState.documents.length < 2) {
      toastState.error('Need at least 2 documents in tabs to merge.');
      return;
    }

    const docIndex = appState.selectedDocumentIndex ?? 0;
    const docName = appState.documents[docIndex]?.name || appState.documents[0].name;
    const toolName = `pdf_${activeTool.id}`;
    let args: Record<string, any> = {};

    switch (activeTool.id) {
      case 'merge':
        args = {
          inputs: appState.documents.map(d => d.name),
          output: `${docName.replace(/.pdf$/i, '')}_merged.pdf`
        };
        break;
      case 'split':
        args = {
          input: docName,
          output_dir: `${docName.replace(/.pdf$/i, '')}_split`
        };
        break;
      case 'compress':
        args = {
          input: docName,
          output: `${docName.replace(/.pdf$/i, '')}_compressed.pdf`
        };
        break;
      case 'rotate':
        args = {
          input: docName,
          pages: 'all',
          angle: parseInt(rotateAngle, 10),
          output: `${docName.replace(/.pdf$/i, '')}_rotated.pdf`
        };
        break;
      case 'watermark':
        args = {
          input: docName,
          text: watermarkText || 'CONFIDENTIAL',
          output: `${docName.replace(/.pdf$/i, '')}_watermarked.pdf`
        };
        break;
      case 'encrypt':
        args = {
          input: docName,
          password: password || 'paperpilot',
          output: `${docName.replace(/.pdf$/i, '')}_encrypted.pdf`
        };
        break;
      case 'decrypt':
        args = {
          input: docName,
          password: password || '',
          output: `${docName.replace(/.pdf$/i, '')}_decrypted.pdf`
        };
        break;
      case 'metadata':
        args = {
          input: docName,
          title: metadataTitle || docName,
          output: `${docName.replace(/.pdf$/i, '')}_metadata.pdf`
        };
        break;
      case 'extract_pages':
        args = {
          input: docName,
          pages: '1',
          output: `${docName.replace(/.pdf$/i, '')}_extracted.pdf`
        };
        break;
      case 'extract_text':
      case 'ocr':
        args = {
          input: docName,
          output: `${docName.replace(/.pdf$/i, '')}_text.txt`
        };
        break;
      default:
        args = {
          input: docName,
          output: `${docName.replace(/.pdf$/i, '')}_output.pdf`
        };
        break;
    }

    appState.setLoading(true);
    try {
      const result = await invoke('invoke_mcp_tool', { toolName, arguments: args });
      const resObj = result as any;
      if (resObj && resObj.success) {
        toastState.success(resObj.message || `${activeTool.title} completed successfully`);
        jobsState.addJob(toolName, 'success', resObj.message);
      } else {
        toastState.error(resObj?.message || `${activeTool.title} failed`);
        jobsState.addJob(toolName, 'error', resObj?.message);
      }
    } catch (error) {
      const errMsg = typeof error === 'string' ? error : (error as Error).message || 'Unknown error occurred';
      toastState.error(errMsg);
      jobsState.addJob(toolName, 'error', errMsg);
    } finally {
      appState.setLoading(false);
    }
  }

  function moveUp(index: number) {
    if (index > 0) {
      appState.reorderDocuments(index, index - 1);
    }
  }

  function moveDown(index: number) {
    if (index < appState.documents.length - 1) {
      appState.reorderDocuments(index, index + 1);
    }
  }
</script>

<div class="operations-panel" id="operations-panel">
  {#if activeTool}
    <!-- INSPECTOR MODE FOR ACTIVE TOOL -->
    <div class="inspector-header">
      <button class="back-btn" onclick={backToDirectory} id="btn-back-tools" title="Back to tool list">
        ‹ Back
      </button>
      <div class="inspector-title-wrap">
        <span class="tool-icon">{activeTool.icon}</span>
        <h3 class="inspector-title">{activeTool.title}</h3>
      </div>
    </div>

    <div class="inspector-content">
      <p class="tool-desc">{activeTool.description}</p>

      {#if activeTool.id === 'merge'}
        <div class="operation-config">
          <p class="section-desc">Reorder open documents for merging:</p>
          {#if appState.documents.length === 0}
            <div class="empty-list">No documents added.</div>
          {:else}
            <div class="reorder-list">
              {#each appState.documents as doc, i}
                <div class="reorder-item">
                  <span class="item-name" title={doc.name}>{doc.name}</span>
                  <div class="reorder-controls">
                    <button class="icon-btn" disabled={i === 0} onclick={() => moveUp(i)} title="Move Up">⬆️</button>
                    <button class="icon-btn" disabled={i === appState.documents.length - 1} onclick={() => moveDown(i)} title="Move Down">⬇️</button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {:else if activeTool.id === 'split'}
        <div class="operation-config">
          <label for="splitPoints" class="section-desc">Split Page Ranges (comma separated):</label>
          <input id="splitPoints" type="text" class="form-input" bind:value={splitPoints} placeholder="e.g. 1-3, 4-8" />
        </div>
      {:else if activeTool.id === 'compress'}
        <div class="operation-config">
          <label for="compressQuality" class="section-desc">Image Quality: {compressQuality}%</label>
          <input id="compressQuality" type="range" min="10" max="100" bind:value={compressQuality} class="range-input" />
          <div class="range-labels">
            <span>Smaller file</span>
            <span>Higher quality</span>
          </div>
        </div>
      {:else if activeTool.id === 'rotate'}
        <div class="operation-config">
          <label for="rotateAngle" class="section-desc">Rotation Angle:</label>
          <select id="rotateAngle" bind:value={rotateAngle} class="form-input">
            <option value="90">90° Clockwise</option>
            <option value="180">180° Flip</option>
            <option value="270">270° Counter-Clockwise</option>
          </select>
        </div>
      {:else if activeTool.id === 'watermark'}
        <div class="operation-config">
          <label for="watermarkText" class="section-desc">Watermark Text:</label>
          <input id="watermarkText" type="text" class="form-input" bind:value={watermarkText} placeholder="e.g. CONFIDENTIAL" />
        </div>
      {:else if activeTool.id === 'encrypt' || activeTool.id === 'decrypt'}
        <div class="operation-config">
          <label for="passwordField" class="section-desc">Password:</label>
          <input id="passwordField" type="password" class="form-input" bind:value={password} placeholder="Enter document password" />
        </div>
      {:else}
        <div class="operation-config">
          <p class="section-desc">Applies directly to the active document.</p>
        </div>
      {/if}

      <div class="action-area">
        <button
          class="run-btn"
          id="btn-run-operation"
          disabled={appState.documents.length === 0}
          onclick={handleRunOperation}
        >
          Run {activeTool.title}
        </button>
      </div>
    </div>
  {:else}
    <!-- DIRECTORY MODE (STIRLING PDF STYLE) -->
    <div class="panel-header">
      <div class="search-box">
        <span class="search-icon">🔍</span>
        <input
          id="tool-search-input"
          class="tool-search-input"
          type="text"
          placeholder="Search tools..."
          bind:value={searchQuery}
        />
        {#if searchQuery}
          <button class="clear-search-btn" onclick={() => searchQuery = ''}>×</button>
        {/if}
      </div>
    </div>

    <div class="panel-content">
      {#if searchQuery.trim()}
        <!-- Search Results View -->
        <div class="search-results-list">
          <span class="category-header">Search Results ({filteredTools.length})</span>
          {#if filteredTools.length === 0}
            <div class="empty-state">No matching tools found</div>
          {:else}
            {#each filteredTools as tool (tool.id + tool.category)}
              <button class="tool-card" onclick={() => selectTool(tool)} id="tool-{tool.id}">
                <span class="tool-card-icon">{tool.icon}</span>
                <div class="tool-card-info">
                  <span class="tool-card-title">{tool.title}</span>
                  <span class="tool-card-desc">{tool.description}</span>
                </div>
              </button>
            {/each}
          {/if}
        </div>
      {:else}
        <!-- Categorized Sections View -->
        {#each categories as cat}
          {@const toolsInCat = allTools.filter(t => t.category === cat.id)}
          {#if toolsInCat.length > 0}
            <div class="category-section">
              <span class="category-header">
                <span class="cat-icon">{cat.icon}</span>
                {cat.title}
              </span>
              <div class="category-grid">
                {#each toolsInCat as tool (tool.id + tool.category)}
                  <button class="tool-card" onclick={() => selectTool(tool)} id="tool-{tool.id}">
                    <span class="tool-card-icon">{tool.icon}</span>
                    <div class="tool-card-info">
                      <span class="tool-card-title">{tool.title}</span>
                      <span class="tool-card-desc">{tool.description}</span>
                    </div>
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .operations-panel {
    width: 320px;
    min-width: 320px;
    background-color: var(--bg-secondary, #141416);
    border-left: 1px solid var(--border-color, #2a2a35);
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-shrink: 0;
    overflow: hidden;
  }

  .panel-header {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #2a2a35);
    flex-shrink: 0;
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
    background-color: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-sm, 4px);
    padding: 0 10px;
  }

  .search-icon {
    font-size: 13px;
    opacity: 0.6;
    margin-right: 6px;
  }

  .tool-search-input {
    width: 100%;
    height: 32px;
    background: transparent;
    border: none;
    color: var(--text-primary, #ffffff);
    font-size: 13px;
    outline: none;
  }

  .clear-search-btn {
    background: transparent;
    border: none;
    color: var(--text-muted, #9ca3af);
    font-size: 16px;
    cursor: pointer;
    padding: 0 4px;
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    padding: 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .category-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .category-header {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted, #9ca3af);
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 2px;
  }

  .cat-icon {
    font-size: 12px;
  }

  .category-grid, .search-results-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .tool-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--border-radius-sm, 4px);
    cursor: pointer;
    text-align: left;
    transition: all var(--transition-fast, 0.15s ease);
    width: 100%;
  }

  .tool-card:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    border-color: var(--border-color, #2a2a35);
  }

  .tool-card-icon {
    font-size: 18px;
    flex-shrink: 0;
  }

  .tool-card-info {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .tool-card-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary, #ffffff);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tool-card-desc {
    font-size: 11px;
    color: var(--text-muted, #9ca3af);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* INSPECTOR VIEW */
  .inspector-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #2a2a35);
    background-color: var(--bg-secondary, #141416);
    flex-shrink: 0;
  }

  .back-btn {
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-sm, 4px);
    color: var(--text-secondary, #9ca3af);
    font-size: 12px;
    padding: 4px 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .back-btn:hover {
    color: var(--text-primary, #ffffff);
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.1));
  }

  .inspector-title-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
    overflow: hidden;
  }

  .inspector-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary, #ffffff);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .inspector-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .tool-desc {
    font-size: 12px;
    color: var(--text-secondary, #9ca3af);
    line-height: 1.4;
  }

  .section-desc {
    font-size: 12px;
    color: var(--text-secondary, #9ca3af);
    margin-bottom: 6px;
    display: block;
  }

  .form-input {
    width: 100%;
    padding: 8px 12px;
    background-color: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-sm, 4px);
    color: var(--text-primary, #ffffff);
    font-size: 13px;
    outline: none;
  }

  .range-input {
    width: 100%;
    accent-color: var(--accent-primary, #5e6ad2);
  }

  .range-labels {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    color: var(--text-muted, #9ca3af);
    margin-top: 2px;
  }

  .reorder-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    max-height: 180px;
    overflow-y: auto;
  }

  .reorder-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 10px;
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 4px;
    font-size: 12px;
  }

  .item-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 180px;
  }

  .reorder-controls {
    display: flex;
    gap: 4px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    font-size: 10px;
    cursor: pointer;
    padding: 2px 4px;
  }

  .icon-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .action-area {
    margin-top: auto;
    padding-top: 16px;
  }

  .run-btn {
    width: 100%;
    padding: 10px 16px;
    background-color: var(--accent-primary, #5e6ad2);
    color: #ffffff;
    border: none;
    border-radius: var(--border-radius-sm, 4px);
    font-weight: 500;
    font-size: 13px;
    cursor: pointer;
    transition: background-color var(--transition-fast, 0.15s ease);
  }

  .run-btn:hover:not(:disabled) {
    background-color: var(--accent-hover, #6e79d6);
  }

  .run-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .empty-state {
    padding: 20px 0;
    text-align: center;
    color: var(--text-muted, #9ca3af);
    font-size: 12px;
  }
</style>
