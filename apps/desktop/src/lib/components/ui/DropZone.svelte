<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  let isDragging = $state(false);
  let { ondrop, fileContext = 'none' }: {
    ondrop: (files: File[]) => void;
    fileContext?: 'pdf' | 'office' | 'mixed' | 'none';
  } = $props();

  let dropIcon = $derived(() => {
    switch (fileContext) {
      case 'pdf':    return '📄';
      case 'office': return '📁';
      case 'mixed':  return '🗂️';
      default:       return '📂';
    }
  });

  let dropTitle = $derived(() =>
    fileContext === 'none' ? 'Open or Drag & Drop Documents' : 'Add more files'
  );

  function getMimeType(name: string): string {
    const ext = name.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
      pdf: 'application/pdf',
      docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
      doc: 'application/msword',
      xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      xls: 'application/vnd.ms-excel',
      pptx: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      ppt: 'application/vnd.ms-powerpoint',
      json: 'application/json',
      html: 'text/html',
      htm: 'text/html',
      md: 'text/markdown',
      markdown: 'text/markdown',
      txt: 'text/plain',
      csv: 'text/csv',
    };
    return map[ext] ?? 'application/octet-stream';
  }

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const filesArray = Array.from(e.dataTransfer.files);
      ondrop(filesArray);
    }
  }

  let fileInputRef = $state<HTMLInputElement | null>(null);

  async function handleBrowseClick(e: MouseEvent) {
    if (appState.isMobile) {
      // On mobile / Android, trigger the standard file picker which supports Content URIs
      fileInputRef?.click();
      return;
    }

    // On desktop: use Tauri native dialog so we get full absolute paths
    e.preventDefault();
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: true,
      filters: [
        { name: 'All Supported Formats', extensions: ['pdf','docx','doc','xlsx','xls','pptx','ppt','json','html','htm','md','markdown','txt'] },
        { name: 'PDF Files', extensions: ['pdf'] },
        { name: 'Word Documents', extensions: ['docx','doc'] },
        { name: 'Excel Spreadsheets', extensions: ['xlsx','xls','csv'] },
        { name: 'PowerPoint Presentations', extensions: ['pptx','ppt'] },
        { name: 'Web & Markdown (HTML, MD)', extensions: ['html','htm','md','markdown'] },
        { name: 'Data Files (JSON, TXT, CSV)', extensions: ['json','txt','csv'] },
        { name: 'All Files', extensions: ['*'] }
      ]
    });
    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    const filesArray: File[] = [];
    for (const path of paths) {
      const meta = await invoke<{ name: string; size_bytes: number; path: string }>('get_file_metadata', { path });
      const mimeType = getMimeType(meta.name);
      const blob = new Blob([], { type: mimeType });
      const file = new File([blob], meta.name, { type: mimeType });
      Object.defineProperty(file, 'size', { value: meta.size_bytes, writable: false });
      (file as any)._localPath = meta.path;
      (file as any)._isLoaded = false;
      filesArray.push(file);
    }
    if (filesArray.length > 0) ondrop(filesArray);
  }

  import { appState } from '$lib/state/app.svelte';

  async function handleFileInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      let filesArray = Array.from(target.files);

      // On mobile, Android uses content:// URIs and we can't rely on POSIX paths.
      // So we force reading the file bytes into memory if it's mobile.
      if (appState.isMobile) {
        for (let i = 0; i < filesArray.length; i++) {
           const f = filesArray[i];
           const buf = await f.arrayBuffer();
           const newBlob = new Blob([new Uint8Array(buf)], { type: f.type });
           const newFile = new File([newBlob], f.name, { type: f.type });
           (newFile as any)._localPath = f.name; // Can't use path
           (newFile as any)._isLoaded = true;
           filesArray[i] = newFile;
        }
      } else {
        filesArray = filesArray.map(f => {
          (f as any)._localPath = (f as any).path || f.name;
          (f as any)._isLoaded = f.size > 0;
          return f;
        });
      }

      ondrop(filesArray);
    }
    target.value = '';
  }

  let showUrlInput = $state(false);
  let importUrl = $state('');
  let isFetchingUrl = $state(false);

  import { toastState } from '$lib/state/toast.svelte';

  async function handleUrlImport(e: Event) {
    e.preventDefault();
    if (!importUrl) return;

    try {
      isFetchingUrl = true;
      const targetUrl = importUrl.trim();
      let bytes: Uint8Array;
      let filename = 'remote_document.pdf';

      // Use Tauri Rust backend to download bypassing browser CORS
      if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__) {
        const res: { name: string; bytes: number[] } = await invoke('download_remote_file', { url: targetUrl });
        filename = res.name || filename;
        bytes = new Uint8Array(res.bytes);
      } else {
        let options: RequestInit = {};
        if (appState.zeroTrace) {
          options.headers = { 'X-Zero-Trace': 'true' };
        }
        const res = await fetch(targetUrl, options);
        if (!res.ok) {
          throw new Error(`HTTP Error: ${res.status}`);
        }
        const buf = await res.arrayBuffer();
        bytes = new Uint8Array(buf);
        const parsedUrl = new URL(targetUrl);
        filename = parsedUrl.pathname.split('/').pop() || filename;
      }

      // Check %PDF magic bytes
      if (bytes.length < 4 || bytes[0] !== 0x25 || bytes[1] !== 0x50 || bytes[2] !== 0x44 || bytes[3] !== 0x46) {
        throw new Error('URL does not return a valid PDF document');
      }

      const blob = new Blob([bytes.buffer as ArrayBuffer], { type: 'application/pdf' });
      const file = new File([blob], filename, { type: 'application/pdf' });
      (file as any)._localPath = filename;
      (file as any)._isLoaded = true;

      ondrop([file]);
      showUrlInput = false;
      importUrl = '';
      toastState.add('success', `Imported ${filename}`);
    } catch (err: any) {
      toastState.add('error', `Failed to import remote PDF: ${err.message}`);
    } finally {
      isFetchingUrl = false;
    }
  }
</script>

<div
  class="drop-zone"
  class:dragging={isDragging}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondragover={handleDragOver}
  ondrop={handleDrop}
  role="button"
  tabindex="0"
>
  <div class="drop-content">
    <div class="icon" aria-hidden="true">{dropIcon()}</div>
    <h3 class="title">{dropTitle()}</h3>

    <div class="privacy-badge" title="Zero cloud dependencies — 100% private local execution">
      <span class="privacy-badge-icon">🔒</span>
      <span class="privacy-badge-text">100% Offline &amp; Private: Documents never leave your computer</span>
    </div>

    <p class="subtitle">Fast, privacy-first document transformation. All processing occurs locally on your machine with zero cloud uploads.</p>

    <div class="format-pills" aria-label="Supported file formats">
      <span class="format-pill">.pdf</span>
      <span class="format-pill">.docx</span>
      <span class="format-pill">.xlsx</span>
      <span class="format-pill">.md</span>
      <span class="format-pill">.html</span>
      <span class="format-pill">.json</span>
      <span class="format-pill">.csv</span>
    </div>

    <div class="actions">
      <button
        type="button"
        class="browse-btn"
        onclick={handleBrowseClick}
        title="Select documents from your computer (.pdf, .docx, .xlsx, .md, .html, .json, .csv)"
        aria-label="Browse files on computer"
      >
        📁 Browse Files
      </button>
      {#if !showUrlInput}
        <button
          type="button"
          class="url-btn"
          onclick={() => showUrlInput = true}
          title="Fetch a document securely from a direct web URL directly into local memory"
          aria-label="Import document from URL"
        >
          🔗 Import from URL
        </button>
      {/if}
    </div>

    {#if showUrlInput}
      <form class="url-input-form" onsubmit={handleUrlImport}>
        <input
          type="url"
          bind:value={importUrl}
          placeholder="https://example.com/document.pdf"
          required
          disabled={isFetchingUrl}
        />
        <button type="submit" disabled={isFetchingUrl}>
          {#if isFetchingUrl}
            Fetching...
          {:else}
            Import
          {/if}
        </button>
        <button type="button" class="cancel-btn" onclick={() => showUrlInput = false} disabled={isFetchingUrl}>
          Cancel
        </button>
      </form>
    {/if}

    <!-- Hidden file input for mobile / Playwright test compatibility -->
    <input
      bind:this={fileInputRef}
      type="file"
      multiple
      accept=".pdf,.docx,.doc,.xlsx,.xls,.pptx,.ppt,.json,.html,.htm,.md,.markdown,.txt,application/pdf"
      onchange={handleFileInputChange}
      style="display: none;"
    />
  </div>
</div>

<style>
  .drop-zone {
    width: 100%;
    min-height: 200px;
    border: 2px dashed var(--border-color);
    border-radius: var(--border-radius-lg);
    background-color: var(--bg-surface);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-normal);
    position: relative;
    overflow: hidden;
  }

  .drop-zone:hover {
    border-color: var(--text-muted);
    background-color: var(--bg-surface-hover);
  }

  .drop-zone.dragging {
    border-color: var(--accent-primary);
    background-color: rgba(94, 106, 210, 0.1);
  }

  .drop-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 32px;
  }

  .icon {
    font-size: 3rem;
    margin-bottom: 16px;
    opacity: 0.8;
    transition: transform var(--transition-normal);
  }

  .drop-zone.dragging .icon {
    transform: scale(1.1);
    opacity: 1;
  }

  .title {
    font-size: 1.35rem;
    color: var(--text-primary);
    margin-bottom: 8px;
    font-weight: 700;
    letter-spacing: -0.01em;
  }

  .privacy-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    background: rgba(16, 185, 129, 0.12);
    border: 1px solid rgba(16, 185, 129, 0.35);
    border-radius: 9999px;
    color: #10b981;
    font-size: 0.78rem;
    font-weight: 500;
    margin-bottom: 12px;
  }

  .privacy-badge-icon {
    font-size: 0.85rem;
  }

  .subtitle {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 16px;
    max-width: 480px;
    line-height: 1.5;
  }

  .format-pills {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 8px;
    margin-bottom: 20px;
    max-width: 440px;
  }

  .format-pill {
    padding: 3px 10px;
    font-size: 0.75rem;
    font-weight: 600;
    font-family: var(--font-mono, monospace);
    border-radius: 6px;
    background: var(--bg-surface-elevated, #27272a);
    color: var(--text-secondary, #a1a1aa);
    border: 1px solid var(--border-color, #3f3f46);
    text-transform: lowercase;
    transition: all 0.15s ease;
  }

  .format-pill:hover {
    border-color: var(--accent-primary, #6366f1);
    color: var(--text-primary, #ffffff);
  }

  .actions {
    display: flex;
    gap: 12px;
    margin-bottom: 16px;
  }

  .browse-btn, .url-btn {
    display: inline-block;
    padding: 10px 20px;
    background-color: var(--accent-primary);
    color: #ffffff;
    border-radius: var(--border-radius-md);
    font-weight: 500;
    cursor: pointer;
    transition: background-color var(--transition-fast);
  }

  .url-btn {
    background-color: var(--bg-surface-elevated);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .browse-btn:hover {
    background-color: var(--accent-hover);
  }

  .url-btn:hover {
    background-color: var(--bg-surface-hover);
  }

  .browse-btn:active {
    background-color: var(--accent-active);
  }

  .url-input-form {
    display: flex;
    gap: 8px;
    margin-top: 16px;
    align-items: center;
    width: 100%;
    max-width: 400px;
  }

  .url-input-form input {
    flex: 1;
    padding: 10px;
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    background-color: var(--bg-surface);
    color: var(--text-primary);
  }

  .url-input-form button {
    padding: 10px 16px;
    background-color: var(--accent-primary);
    color: white;
    border-radius: var(--border-radius-md);
    font-weight: 500;
  }

  .url-input-form button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .url-input-form .cancel-btn {
    background-color: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
  }

  .url-input-form .cancel-btn:hover:not(:disabled) {
    background-color: var(--bg-surface-hover);
  }

  input[type="file"] {
    display: block;
    opacity: 0;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    cursor: pointer;
  }

  .browse-btn {
    position: relative;
  }

  @media (max-width: 599px) {
    /* Make drop zone padding smaller on mobile */
    .drop-content {
      padding: 32px 16px !important;
    }
  }
</style>
