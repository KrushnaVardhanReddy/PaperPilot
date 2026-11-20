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
  let imgElement = $state<HTMLImageElement | null>(null);

  // Region selection state (in natural image pixel coordinates)
  let isSelecting = $state(false);
  let startX = $state(0);
  let startY = $state(0);
  let currentSelection = $state<{ x: number; y: number; width: number; height: number } | null>(null);

  let isRegionSelectTool = $derived(
    appState.activeTool?.id === 'img_redact' ||
    appState.activeTool?.id === 'img_crop' ||
    appState.activeTool?.id === 'img_crop_to_content'
  );

  // Sync external appState.activeImageRegion with currentSelection
  $effect(() => {
    if (!appState.activeImageRegion) {
      currentSelection = null;
    } else {
      currentSelection = appState.activeImageRegion;
    }
  });

  function handleImageLoad(e: Event) {
    const img = e.currentTarget as HTMLImageElement;
    imgElement = img;
    naturalWidth = img.naturalWidth;
    naturalHeight = img.naturalHeight;
  }

  function getNaturalCoords(e: PointerEvent): { x: number; y: number } | null {
    if (!imgElement || naturalWidth === 0 || naturalHeight === 0) return null;
    const rect = imgElement.getBoundingClientRect();
    if (rect.width === 0 || rect.height === 0) return null;

    // Relative to the image element
    const relX = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
    const relY = Math.max(0, Math.min(e.clientY - rect.top, rect.height));

    const scaleX = naturalWidth / rect.width;
    const scaleY = naturalHeight / rect.height;

    const natX = Math.round(relX * scaleX);
    const natY = Math.round(relY * scaleY);

    return {
      x: Math.max(0, Math.min(natX, naturalWidth)),
      y: Math.max(0, Math.min(natY, naturalHeight))
    };
  }

  function handlePointerDown(e: PointerEvent) {
    if (!isRegionSelectTool || e.button !== 0) return;
    const coords = getNaturalCoords(e);
    if (!coords) return;

    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    isSelecting = true;
    startX = coords.x;
    startY = coords.y;
    currentSelection = {
      x: coords.x,
      y: coords.y,
      width: 0,
      height: 0
    };
  }

  function handlePointerMove(e: PointerEvent) {
    if (!isSelecting) return;
    const coords = getNaturalCoords(e);
    if (!coords) return;

    const x = Math.min(startX, coords.x);
    const y = Math.min(startY, coords.y);
    const width = Math.abs(coords.x - startX);
    const height = Math.abs(coords.y - startY);

    currentSelection = { x, y, width, height };
  }

  function handlePointerUp(e: PointerEvent) {
    if (!isSelecting) return;
    isSelecting = false;
    try {
      (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
    } catch {
      // Ignore if pointer capture was lost
    }

    if (currentSelection && currentSelection.width > 3 && currentSelection.height > 3) {
      appState.setImageRegion({ ...currentSelection });
    } else {
      currentSelection = null;
      appState.clearImageRegion();
    }
  }

  function clearSelection() {
    currentSelection = null;
    appState.clearImageRegion();
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

      {#if isRegionSelectTool}
        <div class="region-tool-badge" class:has-selection={currentSelection !== null}>
          <span class="region-icon">⛶</span>
          {#if currentSelection}
            <span>Region: {currentSelection.x},{currentSelection.y} ({currentSelection.width}×{currentSelection.height}px)</span>
            <button
              type="button"
              class="clear-region-btn"
              onclick={clearSelection}
              title="Clear region selection"
              aria-label="Clear selection"
            >
              ✕
            </button>
          {:else}
            <span>Drag image to select region</span>
          {/if}
        </div>
        <div class="divider"></div>
      {/if}

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
        class:crosshair-cursor={isRegionSelectTool}
        style="transform: scale({zoom}) rotate({rotation}deg);"
      >
        <img
          src={fileUrl}
          alt={file.name}
          onload={handleImageLoad}
          class="preview-img"
          draggable="false"
        />

        {#if naturalWidth > 0 && naturalHeight > 0}
          <!-- Interactive Region Selection Overlay -->
          <div
            class="selection-overlay"
            class:active={isRegionSelectTool}
            onpointerdown={handlePointerDown}
            onpointermove={handlePointerMove}
            onpointerup={handlePointerUp}
            onpointercancel={handlePointerUp}
            role="presentation"
          >
            {#if currentSelection && (currentSelection.width > 0 || currentSelection.height > 0)}
              <div
                class="marquee-box"
                style="
                  left: {(currentSelection.x / naturalWidth) * 100}%;
                  top: {(currentSelection.y / naturalHeight) * 100}%;
                  width: {(currentSelection.width / naturalWidth) * 100}%;
                  height: {(currentSelection.height / naturalHeight) * 100}%;
                "
              >
                <!-- Resizing & corner handles -->
                <div class="marquee-handle top-left"></div>
                <div class="marquee-handle top-right"></div>
                <div class="marquee-handle bottom-left"></div>
                <div class="marquee-handle bottom-right"></div>
                <div class="marquee-coords">
                  {currentSelection.width} × {currentSelection.height} px
                </div>
              </div>
            {/if}
          </div>
        {/if}
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

  .region-tool-badge {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    background-color: rgba(137, 180, 250, 0.15);
    color: var(--primary, #89b4fa);
    border: 1px solid rgba(137, 180, 250, 0.3);
    padding: 0.25rem 0.6rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .region-tool-badge.has-selection {
    background-color: rgba(243, 139, 168, 0.15);
    color: #f38ba8;
    border-color: rgba(243, 139, 168, 0.4);
  }

  .region-icon {
    font-size: 0.9rem;
  }

  .clear-region-btn {
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    font-size: 0.75rem;
    padding: 0 0.2rem;
    border-radius: 3px;
    margin-left: 0.25rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    opacity: 0.7;
    transition: opacity 0.15s ease;
  }

  .clear-region-btn:hover {
    opacity: 1;
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
    position: relative;
    display: inline-block;
    transition: transform 0.2s cubic-bezier(0.2, 0, 0, 1);
    transform-origin: center center;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
    border-radius: 4px;
    line-height: 0;
  }

  .image-wrapper.crosshair-cursor {
    cursor: crosshair;
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

  .selection-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 5;
    user-select: none;
    touch-action: none;
  }

  .selection-overlay.active {
    pointer-events: auto;
    cursor: crosshair;
  }

  .marquee-box {
    position: absolute;
    border: 2px solid #f38ba8;
    background-color: rgba(243, 139, 168, 0.25);
    box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.5), inset 0 0 12px rgba(243, 139, 168, 0.2);
    pointer-events: none;
    box-sizing: border-box;
  }

  .marquee-handle {
    position: absolute;
    width: 8px;
    height: 8px;
    background-color: #f38ba8;
    border: 1px solid #11111b;
    border-radius: 2px;
  }

  .marquee-handle.top-left {
    top: -4px;
    left: -4px;
  }

  .marquee-handle.top-right {
    top: -4px;
    right: -4px;
  }

  .marquee-handle.bottom-left {
    bottom: -4px;
    left: -4px;
  }

  .marquee-handle.bottom-right {
    bottom: -4px;
    right: -4px;
  }

  .marquee-coords {
    position: absolute;
    bottom: -22px;
    left: 0;
    background: rgba(17, 17, 27, 0.85);
    color: #cdd6f4;
    border: 1px solid #45475a;
    font-size: 11px;
    padding: 1px 5px;
    border-radius: 3px;
    white-space: nowrap;
    line-height: normal;
    pointer-events: none;
    font-family: monospace;
    font-variant-numeric: tabular-nums;
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
