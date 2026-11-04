<script lang="ts">
  import { onMount } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';

  // Vite worker import trick
  import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.mjs?url';
  import PdfAnnotationLayer from './PdfAnnotationLayer.svelte';
  import PdfSearchBar from './PdfSearchBar.svelte';
  import type { Annotation } from '$lib/api/pdf';

  if (typeof window !== 'undefined') {
    try {
      pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(pdfjsWorker, window.location.href).href;
    } catch {
      pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;
    }
  }

  let {
    fileUrl = '',
    file = null as File | Blob | null,
    pdfDoc = $bindable(null),
    pageNum = $bindable(1),
    scale = $bindable(1.2),
    activeTool = 'none',
    annotations = $bindable([] as Annotation[])
  } = $props();

  let canvas: HTMLCanvasElement;
  let isRendering = false; // Not a $state to prevent infinite loops in effects

  // Password prompt state
  let showPasswordModal = $state(false);
  let passwordInput = $state('');
  let passwordError = $state('');
  let passwordCallback: ((password: string) => void) | null = null;
  let passwordInputEl: HTMLInputElement | null = $state(null);

  function handlePasswordSubmit(e?: Event) {
    if (e) e.preventDefault();
    if (!passwordCallback) return;
    const pwd = passwordInput;
    passwordError = '';
    passwordCallback(pwd);
  }

  function handlePasswordCancel() {
    showPasswordModal = false;
    passwordInput = '';
    passwordError = '';
    if (passwordCallback) {
      // Pass an empty string or trigger cancellation
      passwordCallback('');
      passwordCallback = null;
    }
  }

  $effect(() => {
    if (showPasswordModal && passwordInputEl) {
      setTimeout(() => passwordInputEl?.focus(), 50);
    }
  });

  async function loadDocument(url: string, fileObj?: File | Blob | null) {
    if (!url && !fileObj) {
      pdfDoc = null;
      showPasswordModal = false;
      return;
    }
    try {
      let loadingTask;
      if (fileObj && typeof (fileObj as any).arrayBuffer === 'function') {
        const buffer = await fileObj.arrayBuffer();
        loadingTask = pdfjsLib.getDocument({ data: new Uint8Array(buffer) });
      } else if (url && url.startsWith('blob:')) {
        try {
          const res = await fetch(url);
          const buffer = await res.arrayBuffer();
          loadingTask = pdfjsLib.getDocument({ data: new Uint8Array(buffer) });
        } catch (fetchErr) {
          console.warn("fetch blob failed, falling back to url", fetchErr);
          loadingTask = pdfjsLib.getDocument({ url });
        }
      } else if (url) {
        loadingTask = pdfjsLib.getDocument({ url });
      }

      if (loadingTask) {
        loadingTask.onPassword = (updatePassword: (password: string) => void, reason: number) => {
          passwordCallback = updatePassword;
          showPasswordModal = true;
          if (reason === (pdfjsLib.PasswordResponses?.INCORRECT_PASSWORD ?? 2)) {
            passwordError = 'Incorrect password. Please try again.';
          } else {
            passwordError = '';
          }
        };

        pdfDoc = await loadingTask.promise;
        showPasswordModal = false;
        passwordInput = '';
        passwordError = '';
        passwordCallback = null;
        pageNum = 1;
      }
    } catch (err: any) {
      console.error("Error loading PDF:", err);
      if (err?.name === 'PasswordException') {
        showPasswordModal = true;
        passwordError = 'This document is protected with a password.';
      } else {
        pdfDoc = null;
        showPasswordModal = false;
      }
    }
  }

  async function renderPage(doc: pdfjsLib.PDFDocumentProxy | null, currentPage: number, currentScale: number) {
    if (!doc || !canvas || isRendering) return;
    isRendering = true;

    try {
      const page = await doc.getPage(currentPage);
      const viewport = page.getViewport({ scale: currentScale });

      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      // Handle high-DPI displays (retina screens)
      const outputScale = window.devicePixelRatio || 1;
      canvas.width = Math.floor(viewport.width * outputScale);
      canvas.height = Math.floor(viewport.height * outputScale);

      // The CSS width/height should match the unscaled viewport to map correctly with 1x zoom layer
      canvas.style.width = Math.floor(viewport.width) + "px";
      canvas.style.height = Math.floor(viewport.height) + "px";

      const transform = outputScale !== 1
        ? [outputScale, 0, 0, outputScale, 0, 0]
        : null;

      const renderContext = {
        canvasContext: ctx,
        transform: transform ? transform : undefined,
        viewport,
        canvas, // added required property
      };

      await page.render(renderContext as any).promise;
    } catch (err) {
      console.error("Render error:", err);
    } finally {
      isRendering = false;
    }
  }

  // Reactively load document when file or fileUrl changes
  $effect(() => {
    if (file || fileUrl) {
      loadDocument(fileUrl, file);
    } else {
      pdfDoc = null;
    }
  });

  // Reactively re-render when dependencies change
  $effect(() => {
    // Read states synchronously so Svelte tracks them
    renderPage(pdfDoc, pageNum, scale);
  });


  $effect(() => {
    const handleFit = () => {
      if (pdfDoc && canvas) {
         pdfDoc.getPage(pageNum).then((page: pdfjsLib.PDFPageProxy) => {
           const viewport = page.getViewport({ scale: 1.0 });
           const container = canvas.closest('.viewer-container');
           if (container) {
             const containerWidth = container.clientWidth - 64; // account for 2rem padding
             const newScale = containerWidth / viewport.width;
             scale = Math.min(Math.max(newScale, 0.2), 5.0);
           }
         });
      }
    };
    window.addEventListener('paperpilot:fit-viewport', handleFit);
    return () => {
      window.removeEventListener('paperpilot:fit-viewport', handleFit);
    };
  });

  import type { SearchMatchItem } from './PdfSearchBar.svelte';
  let searchHighlights: SearchMatchItem[] = $state([]);
  let viewportState: any = $state(null);

  let citationHighlight: { page: number; bbox?: number[] } | null = $state(null);
  let citationTimeoutId: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
      const handleJumpToCitation = (e: CustomEvent<{ page: number, bbox?: number[] }>) => {
          const { page, bbox } = e.detail;
          if (page) {
              pageNum = page;
              citationHighlight = { page, bbox };

              if (citationTimeoutId) {
                  clearTimeout(citationTimeoutId);
              }
              citationTimeoutId = setTimeout(() => {
                  citationHighlight = null;
              }, 2500);
          }
      };

      window.addEventListener('paperpilot:jump-to-citation', handleJumpToCitation as EventListener);
      return () => {
          window.removeEventListener('paperpilot:jump-to-citation', handleJumpToCitation as EventListener);
          if (citationTimeoutId) clearTimeout(citationTimeoutId);
      };
  });

  $effect(() => {
      if (pdfDoc && pageNum && scale) {
          pdfDoc.getPage(pageNum).then((page: pdfjsLib.PDFPageProxy) => {
              viewportState = page.getViewport({ scale: scale });
          });
      }
  });

  function handleGlobalKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f' && !e.shiftKey) {
      e.preventDefault();
      window.dispatchEvent(new CustomEvent('paperpilot:search-open'));
    }
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="pdf-viewer-wrapper">
  {#if showPasswordModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="password-modal-backdrop" onclick={(e) => { if (e.target === e.currentTarget) handlePasswordCancel(); }}>
      <div class="password-modal-card" role="dialog" aria-modal="true" aria-labelledby="pwd-title">
        <div class="password-modal-header">
          <div class="lock-icon-badge">🔒</div>
          <div>
            <h3 id="pwd-title">Password Required</h3>
            <p class="pwd-subtitle">This document is encrypted. Enter password to view.</p>
          </div>
        </div>
        <form onsubmit={handlePasswordSubmit}>
          <div class="input-group">
            <input
              bind:this={passwordInputEl}
              type="password"
              class="password-input"
              class:has-error={!!passwordError}
              bind:value={passwordInput}
              placeholder="Enter password..."
              autocomplete="current-password"
            />
            {#if passwordError}
              <p class="error-msg">{passwordError}</p>
            {/if}
          </div>
          <div class="password-modal-actions">
            <button type="button" class="btn-cancel" onclick={handlePasswordCancel}>Cancel</button>
            <button type="submit" class="btn-unlock" disabled={!passwordInput.trim()}>Unlock Document</button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  <PdfSearchBar {pdfDoc} bind:pageNum bind:searchHighlights />
  <div class="viewer-container">
    {#if citationHighlight && citationHighlight.page === pageNum && !citationHighlight.bbox}
        <div class="page-citation-banner">
            Viewing Cited Page {pageNum}
        </div>
    {/if}
    <div class="page-container">
      <canvas bind:this={canvas} style="display: block;"></canvas>
      {#if viewportState}
        {#if citationHighlight && citationHighlight.page === pageNum && citationHighlight.bbox}
            {@const pt = pdfjsLib.Util.normalizeRect(citationHighlight.bbox)}
            {@const vpPt1 = viewportState.convertToViewportPoint(pt[0], pt[1])}
            {@const vpPt2 = viewportState.convertToViewportPoint(pt[2], pt[3])}
            <div class="citation-pulse" style="left: {Math.min(vpPt1[0], vpPt2[0])}px; top: {Math.min(vpPt1[1], vpPt2[1])}px; width: {Math.abs(vpPt2[0] - vpPt1[0])}px; height: {Math.abs(vpPt1[1] - vpPt2[1])}px;"></div>
        {/if}
        <div class="search-highlights-layer" style="width: {viewportState.width}px; height: {viewportState.height}px;">
           {#each searchHighlights as item}
               {@const pt = pdfjsLib.Util.normalizeRect([
                   item.x,
                   item.y,
                   item.x + item.width,
                   item.y + item.height
               ])}
               {@const vpPt1 = viewportState.convertToViewportPoint(pt[0], pt[1])}
               {@const vpPt2 = viewportState.convertToViewportPoint(pt[2], pt[3])}
               <div class="search-highlight" style="left: {vpPt1[0]}px; top: {vpPt2[1]}px; width: {vpPt2[0] - vpPt1[0]}px; height: {vpPt1[1] - vpPt2[1]}px;"></div>
           {/each}
        </div>
      {/if}
      <PdfAnnotationLayer {scale} {pageNum} bind:annotations {activeTool} />
    </div>
  </div>
</div>

<style>
  .pdf-viewer-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    position: relative;
  }

  .viewer-container {
    display: flex;
    justify-content: center;
    align-items: flex-start;
    overflow: auto;
    background-color: var(--bg-secondary, #f3f4f6);
    padding: 2rem;
    flex: 1;
    min-height: 0;
    min-width: 0;
  }

  .page-container {
    position: relative;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    background-color: white;
    /* Ensure the container sizes tightly around the canvas */
    display: inline-block;
    height: fit-content;
    flex-shrink: 0;
    margin: auto;
  }

  canvas {
    display: block;
    max-width: none;
  }

  .search-highlights-layer {
    position: absolute;
    top: 0;
    left: 0;
    pointer-events: none;
    z-index: 10;
  }

  .search-highlight {
    position: absolute;
    background-color: rgba(255, 255, 0, 0.4);
    border-radius: 2px;
  }

  .page-citation-banner {
    position: absolute;
    top: 4rem;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(99, 102, 241, 0.9);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.9rem;
    font-weight: 500;
    z-index: 50;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
    animation: fadeInOut 2.5s ease-in-out forwards;
    pointer-events: none;
  }

  .citation-pulse {
    position: absolute;
    border: 2px solid #6366f1;
    background: rgba(99, 102, 241, 0.2);
    border-radius: 4px;
    z-index: 20;
    pointer-events: none;
    animation: pulse-ring 2.5s cubic-bezier(0.215, 0.61, 0.355, 1) forwards;
  }

  @keyframes pulse-ring {
    0% {
      transform: scale(0.95);
      box-shadow: 0 0 0 0 rgba(99, 102, 241, 0.7);
      opacity: 1;
    }
    70% {
      transform: scale(1.02);
      box-shadow: 0 0 0 10px rgba(99, 102, 241, 0);
      opacity: 0.8;
    }
    100% {
      transform: scale(1);
      box-shadow: 0 0 0 0 rgba(99, 102, 241, 0);
      opacity: 0;
    }
  }

  @keyframes fadeInOut {
    0% { opacity: 0; transform: translate(-50%, -10px); }
    15% { opacity: 1; transform: translate(-50%, 0); }
    85% { opacity: 1; transform: translate(-50%, 0); }
    100% { opacity: 0; transform: translate(-50%, -10px); }
  }

  /* Password Modal Styling */
  .password-modal-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1.5rem;
  }

  .password-modal-card {
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 12px;
    padding: 1.5rem;
    width: 100%;
    max-width: 400px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5), 0 10px 10px -5px rgba(0, 0, 0, 0.2);
    color: var(--text-primary, #ffffff);
  }

  .password-modal-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.25rem;
  }

  .lock-icon-badge {
    font-size: 1.75rem;
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 50%;
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .password-modal-header h3 {
    margin: 0;
    font-size: 1.15rem;
    font-weight: 600;
  }

  .pwd-subtitle {
    margin: 0.25rem 0 0;
    font-size: 0.85rem;
    color: var(--text-muted, #9ca3af);
  }

  .input-group {
    margin-bottom: 1.25rem;
  }

  .password-input {
    width: 100%;
    box-sizing: border-box;
    padding: 0.65rem 0.85rem;
    border-radius: 8px;
    border: 1px solid var(--border-color, #374151);
    background: var(--bg-input, #111827);
    color: var(--text-primary, #ffffff);
    font-size: 0.95rem;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .password-input:focus {
    border-color: var(--accent-primary, #6366f1);
    box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
  }

  .password-input.has-error {
    border-color: #ef4444;
  }

  .error-msg {
    margin: 0.4rem 0 0;
    color: #ef4444;
    font-size: 0.8rem;
  }

  .password-modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }

  .btn-cancel {
    padding: 0.55rem 1rem;
    border-radius: 6px;
    border: 1px solid var(--border-color, #374151);
    background: transparent;
    color: var(--text-muted, #9ca3af);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-cancel:hover {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-primary, #ffffff);
  }

  .btn-unlock {
    padding: 0.55rem 1.25rem;
    border-radius: 6px;
    border: none;
    background: var(--accent-primary, #6366f1);
    color: #ffffff;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .btn-unlock:hover:not(:disabled) {
    background: #4f46e5;
  }

  .btn-unlock:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
