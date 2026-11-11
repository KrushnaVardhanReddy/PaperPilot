<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { requireInvoke as safeInvoke } from '$lib/utils/tauri';
  import { formatUserFriendlyError } from '$lib/utils/errorFormatter';

  let { file, fileUrl }: { file: File; fileUrl: string } = $props();

  let zoom = $state(1);
  let rotation = $state(0);
  let isConverting = $state(false);
  let naturalWidth = $state(0);
  let naturalHeight = $state(0);

  function handleImageLoad(e: Event) {
    const img = e.currentTarget as HTMLImageElement;
    naturalWidth = img.naturalWidth;
    naturalHeight = img.naturalHeight;
  }

  function zoomIn() {
    zoom = Math.min(zoom + 0.25, 4);
  }

  function zoomOut() {
    zoom = Math.max(zoom - 0.25, 0.25);
  }

  function resetZoom() {
    zoom = 1;
    rotation = 0;
  }

  function rotateClockwise() {
    rotation = (rotation + 90) % 360;
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
  }

  async function convertToPdf() {
    const localPath = (file as any)._localPath;
    if (!localPath) {
      toastState.error('Conversion requires local file path', 'Please ensure the image file is saved locally.');
      return;
    }

    isConverting = true;
    appState.setLoading(true);

    try {
      const suggestedOutput = localPath.replace(/\.[^/.]+$/, '') + '.pdf';
      let outputPath = suggestedOutput;

      // Ask user where to save if Tauri dialog is available
      try {
        const { save } = await import('@tauri-apps/plugin-dialog');
        const userPath = await save({
          title: 'Save PDF Document',
          defaultPath: suggestedOutput,
          filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
        });
        if (!userPath) {
          isConverting = false;
          appState.setLoading(false);
          return;
        }
        outputPath = userPath;
      } catch {
        // Fallback to default outputPath
      }

      await safeInvoke('invoke_mcp_tool', {
        toolName: 'pdf_images_to_pdf',
        arguments: {
          images: [localPath],
          output: outputPath,
          _zero_trace: appState.zeroTrace
        }
      });

      toastState.success(`Converted to PDF successfully: ${outputPath.split('/').pop()}`);

      // Auto-load converted PDF if possible
      try {
        const meta = await safeInvoke<{ name: string; size_bytes: number; path: string }>('get_file_metadata', { path: outputPath });
        if (meta && meta.path) {
          const blob = new Blob([], { type: 'application/pdf' });
          const newPdf = new File([blob], meta.name, { type: 'application/pdf' });
          Object.defineProperty(newPdf, 'size', { value: meta.size_bytes, writable: false });
          (newPdf as any)._localPath = meta.path;
          (newPdf as any)._isLoaded = false;
          appState.addDocuments([newPdf]);
          appState.selectDocument(appState.documents.length - 1);
        }
      } catch (loadErr) {
        console.warn('Could not auto-open generated PDF in tab:', loadErr);
      }
    } catch (err) {
      const formatted = formatUserFriendlyError(err, 'Convert Image to PDF');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
    } finally {
      isConverting = false;
      appState.setLoading(false);
    }
  }
</script>

<div class="image-viewer-container" role="region" aria-label="Image Preview">
  <!-- Top Control Bar -->
  <header class="image-toolbar">
    <div class="meta-section">
      <span class="file-name" title={file.name}>{file.name}</span>
      <div class="badges">
        <span class="badge size-badge">{formatBytes(file.size)}</span>
        {#if naturalWidth > 0 && naturalHeight > 0}
          <span class="badge dim-badge">{naturalWidth} × {naturalHeight} px</span>
        {/if}
      </div>
    </div>

    <div class="controls-section">
      <div class="zoom-controls">
        <button type="button" class="ctrl-btn" onclick={zoomOut} title="Zoom Out" aria-label="Zoom Out" disabled={zoom <= 0.25}>
          −
        </button>
        <span class="zoom-level">{Math.round(zoom * 100)}%</span>
        <button type="button" class="ctrl-btn" onclick={zoomIn} title="Zoom In" aria-label="Zoom In" disabled={zoom >= 4}>
          +
        </button>
        <button type="button" class="ctrl-btn" onclick={resetZoom} title="Reset Zoom & Rotation">
          Reset
        </button>
      </div>

      <div class="divider"></div>

      <button type="button" class="ctrl-btn" onclick={rotateClockwise} title="Rotate 90° Clockwise">
        ↻ Rotate
      </button>

      <div class="divider"></div>

      <button
        type="button"
        class="convert-pdf-btn"
        onclick={convertToPdf}
        disabled={isConverting}
        title="Convert this image into a standard PDF document"
      >
        {#if isConverting}
          ⏳ Converting...
        {:else}
          📄 Convert to PDF
        {/if}
      </button>
    </div>
  </header>

  <!-- Viewport Area -->
  <main class="image-stage">
    {#if fileUrl}
      <div
        class="image-wrapper"
        style="transform: scale({zoom}) rotate({rotation}deg);"
      >
        <img
          src={fileUrl}
          alt={file.name}
          onload={handleImageLoad}
          class="preview-img"
        />
      </div>
    {:else}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading image preview...</p>
      </div>
    {/if}
  </main>
</div>

<style>
  .image-viewer-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    background-color: var(--bg-surface, #1e1e2e);
    color: var(--text-color, #cdd6f4);
    overflow: hidden;
  }

  .image-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.6rem 1.25rem;
    background-color: var(--bg-subtle, #181825);
    border-bottom: 1px solid var(--border-color, #313244);
    gap: 1rem;
    flex-wrap: wrap;
    z-index: 10;
  }

  .meta-section {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 0;
  }

  .file-name {
    font-weight: 600;
    font-size: 0.95rem;
    max-width: 280px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .badges {
    display: flex;
    gap: 0.4rem;
  }

  .badge {
    font-size: 0.75rem;
    padding: 0.15rem 0.5rem;
    border-radius: 4px;
    background-color: var(--bg-surface-elevated, #313244);
    color: var(--text-muted, #a6adc8);
    font-variant-numeric: tabular-nums;
  }

  .controls-section {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .zoom-controls {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    background-color: var(--bg-surface-elevated, #313244);
    border-radius: 6px;
    padding: 0.15rem 0.35rem;
  }

  .zoom-level {
    font-size: 0.8rem;
    font-weight: 500;
    min-width: 3.2rem;
    text-align: center;
    font-variant-numeric: tabular-nums;
  }

  .ctrl-btn {
    background: transparent;
    border: none;
    color: var(--text-color, #cdd6f4);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .ctrl-btn:hover:not(:disabled) {
    background-color: var(--bg-surface, #45475a);
  }

  .ctrl-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .divider {
    width: 1px;
    height: 1.25rem;
    background-color: var(--border-color, #45475a);
    margin: 0 0.25rem;
  }

  .convert-pdf-btn {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    background-color: var(--primary, #89b4fa);
    color: var(--primary-contrast, #11111b);
    border: none;
    border-radius: 6px;
    padding: 0.35rem 0.85rem;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: filter 0.15s ease;
  }

  .convert-pdf-btn:hover:not(:disabled) {
    filter: brightness(1.1);
  }

  .convert-pdf-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .image-stage {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: auto;
    position: relative;
    padding: 1.5rem;
    background: repeating-conic-gradient(var(--bg-subtle, #181825) 0% 25%, var(--bg-surface, #1e1e2e) 0% 50%) 50% / 24px 24px;
  }

  .image-wrapper {
    display: inline-block;
    transition: transform 0.2s cubic-bezier(0.2, 0, 0, 1);
    transform-origin: center center;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
    border-radius: 4px;
    line-height: 0;
  }

  .preview-img {
    max-width: 90vw;
    max-height: 75vh;
    object-fit: contain;
    border-radius: 4px;
    display: block;
    user-select: none;
    pointer-events: auto;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    color: var(--text-muted, #a6adc8);
  }

  .spinner {
    width: 2rem;
    height: 2rem;
    border: 3px solid rgba(255, 255, 255, 0.1);
    border-top-color: var(--primary, #89b4fa);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
