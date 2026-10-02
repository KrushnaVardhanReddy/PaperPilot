<script lang="ts">
  import { onMount } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';
  import { appState } from '$lib/state/app.svelte';

  let {
    onClose
  }: {
    onClose?: () => void;
  } = $props();

  // Selected document indices for comparison
  let docAIndex = $state<number>(0);
  let docBIndex = $state<number>(appState.documents.length > 1 ? 1 : 0);

  let pageNum = $state(1);
  let scale = $state(1.0);
  let viewMode = $state<'split' | 'overlay' | 'side-by-side'>('split');
  let splitPercent = $state(50); // 0 to 100%
  let isDraggingSplitter = $state(false);

  // PDF Document proxies
  let pdfDocA: any = $state(null);
  let pdfDocB: any = $state(null);
  let totalPages = $derived(
    Math.max(pdfDocA ? pdfDocA.numPages : 0, pdfDocB ? pdfDocB.numPages : 0) || 1
  );

  let canvasA: HTMLCanvasElement | undefined = $state();
  let canvasB: HTMLCanvasElement | undefined = $state();
  let canvasDiff: HTMLCanvasElement | undefined = $state();
  let containerRef: HTMLDivElement | undefined = $state();

  let changedPixelCount = $state(0);
  let isComparing = $state(false);

  async function loadDoc(file: File): Promise<any> {
    const arrayBuffer = await file.arrayBuffer();
    const loadingTask = pdfjsLib.getDocument({ data: arrayBuffer });
    return await loadingTask.promise;
  }

  // Load Doc A
  $effect(() => {
    if (appState.documents[docAIndex]) {
      loadDoc(appState.documents[docAIndex]).then(doc => {
        pdfDocA = doc;
      }).catch(err => console.error("Failed loading Doc A:", err));
    }
  });

  // Load Doc B
  $effect(() => {
    if (appState.documents[docBIndex]) {
      loadDoc(appState.documents[docBIndex]).then(doc => {
        pdfDocB = doc;
      }).catch(err => console.error("Failed loading Doc B:", err));
    }
  });

  // Render both pages and compute diff when page, scale, or docs change
  $effect(() => {
    if (pdfDocA || pdfDocB) {
      renderComparison(pageNum, scale);
    }
  });

  async function renderComparison(page: number, currentScale: number) {
    isComparing = true;
    changedPixelCount = 0;

    try {
      let width = 600;
      let height = 800;

      // Render Doc A
      if (pdfDocA && page <= pdfDocA.numPages) {
        const pA = await pdfDocA.getPage(page);
        const vpA = pA.getViewport({ scale: currentScale });
        width = Math.floor(vpA.width);
        height = Math.floor(vpA.height);

        if (canvasA) {
          canvasA.width = width;
          canvasA.height = height;
          const ctxA = canvasA.getContext('2d');
          if (ctxA) {
            ctxA.clearRect(0, 0, width, height);
            await pA.render({ canvasContext: ctxA, viewport: vpA }).promise;
          }
        }
      } else if (canvasA) {
        canvasA.width = width;
        canvasA.height = height;
        const ctxA = canvasA.getContext('2d');
        if (ctxA) {
          ctxA.fillStyle = '#f8fafc';
          ctxA.fillRect(0, 0, width, height);
        }
      }

      // Render Doc B
      if (pdfDocB && page <= pdfDocB.numPages) {
        const pB = await pdfDocB.getPage(page);
        const vpB = pB.getViewport({ scale: currentScale });
        width = Math.max(width, Math.floor(vpB.width));
        height = Math.max(height, Math.floor(vpB.height));

        if (canvasB) {
          canvasB.width = width;
          canvasB.height = height;
          const ctxB = canvasB.getContext('2d');
          if (ctxB) {
            ctxB.clearRect(0, 0, width, height);
            await pB.render({ canvasContext: ctxB, viewport: vpB }).promise;
          }
        }
      } else if (canvasB) {
        canvasB.width = width;
        canvasB.height = height;
        const ctxB = canvasB.getContext('2d');
        if (ctxB) {
          ctxB.fillStyle = '#f8fafc';
          ctxB.fillRect(0, 0, width, height);
        }
      }

      // Compute Difference Map on canvasDiff
      if (canvasA && canvasB && canvasDiff) {
        canvasDiff.width = width;
        canvasDiff.height = height;
        const ctxA = canvasA.getContext('2d');
        const ctxB = canvasB.getContext('2d');
        const ctxDiff = canvasDiff.getContext('2d');

        if (ctxA && ctxB && ctxDiff) {
          const imgDataA = ctxA.getImageData(0, 0, width, height);
          const imgDataB = ctxB.getImageData(0, 0, width, height);
          const diffData = ctxDiff.createImageData(width, height);

          const dataA = imgDataA.data;
          const dataB = imgDataB.data;
          const out = diffData.data;
          let diffCount = 0;

          for (let i = 0; i < dataA.length; i += 4) {
            const rA = dataA[i], gA = dataA[i + 1], bA = dataA[i + 2], aA = dataA[i + 3];
            const rB = dataB[i], gB = dataB[i + 1], bB = dataB[i + 2], aB = dataB[i + 3];

            // Color Euclidean Distance
            const dist = Math.abs(rA - rB) + Math.abs(gA - gB) + Math.abs(bA - bB) + Math.abs(aA - aB);

            if (dist > 30) {
              diffCount++;
              // Highlight removed/altered as red, added/new as green tint
              out[i] = 239;     // Red
              out[i + 1] = 68;  // Green
              out[i + 2] = 68;  // Blue
              out[i + 3] = 200; // Alpha
            } else {
              // Dim identical content for high contrast
              const gray = (rA * 0.299 + gA * 0.587 + bA * 0.114);
              out[i] = gray;
              out[i + 1] = gray;
              out[i + 2] = gray;
              out[i + 3] = 70; // Faded
            }
          }

          ctxDiff.putImageData(diffData, 0, 0);
          changedPixelCount = diffCount;
        }
      }
    } catch (err) {
      console.error("Comparison render error:", err);
    } finally {
      isComparing = false;
    }
  }

  function handleSplitterDrag(e: MouseEvent) {
    if (!isDraggingSplitter || !containerRef) return;
    const rect = containerRef.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percent = Math.min(Math.max((x / rect.width) * 100, 5), 95);
    splitPercent = percent;
  }

  function stopSplitterDrag() {
    isDraggingSplitter = false;
    window.removeEventListener('mousemove', handleSplitterDrag);
    window.removeEventListener('mouseup', stopSplitterDrag);
  }

  function startSplitterDrag(e: MouseEvent) {
    e.preventDefault();
    isDraggingSplitter = true;
    window.addEventListener('mousemove', handleSplitterDrag);
    window.addEventListener('mouseup', stopSplitterDrag);
  }
</script>

<div class="diff-viewer" id="visual-diff-container">
  <!-- Top Control Header -->
  <header class="diff-header">
    <div class="diff-title-section">
      <span class="diff-badge">⚡ Core Feature</span>
      <h2>Visual PDF Comparison</h2>
      {#if changedPixelCount > 0}
        <span class="diff-status changed">⚠️ {changedPixelCount.toLocaleString()} visual pixel differences</span>
      {:else}
        <span class="diff-status identical">✅ Pages visually identical</span>
      {/if}
    </div>

    <!-- Document Selectors -->
    <div class="diff-doc-selectors">
      <div class="selector-group">
        <label for="doc-a-select">Base (Doc A):</label>
        <select id="doc-a-select" bind:value={docAIndex}>
          {#each appState.documents as doc, idx}
            <option value={idx}>{doc.name}</option>
          {/each}
        </select>
      </div>

      <span class="vs-divider">vs</span>

      <div class="selector-group">
        <label for="doc-b-select">Compare (Doc B):</label>
        <select id="doc-b-select" bind:value={docBIndex}>
          {#each appState.documents as doc, idx}
            <option value={idx}>{doc.name}</option>
          {/each}
        </select>
      </div>
    </div>

    <!-- View Mode Switches -->
    <div class="diff-mode-switches">
      <button
        class="mode-btn"
        class:active={viewMode === 'split'}
        id="diff-mode-split"
        onclick={() => viewMode = 'split'}
        title="Interactive split-slider before/after"
      >
        ↔️ Split Slider
      </button>
      <button
        class="mode-btn"
        class:active={viewMode === 'overlay'}
        id="diff-mode-overlay"
        onclick={() => viewMode = 'overlay'}
        title="Highlight pixel discrepancies"
      >
        🔥 Difference Map
      </button>
      <button
        class="mode-btn"
        class:active={viewMode === 'side-by-side'}
        id="diff-mode-side"
        onclick={() => viewMode = 'side-by-side'}
        title="Side by side synchronized view"
      >
        📑 Side-by-Side
      </button>
    </div>

    <!-- Pagination & Scale Controls -->
    <div class="diff-nav-controls">
      <button
        class="nav-btn"
        id="diff-prev-page"
        disabled={pageNum <= 1}
        onclick={() => pageNum = Math.max(pageNum - 1, 1)}
      >
        ‹ Prev Page
      </button>
      <span class="page-indicator">Page {pageNum} of {totalPages}</span>
      <button
        class="nav-btn"
        id="diff-next-page"
        disabled={pageNum >= totalPages}
        onclick={() => pageNum = Math.min(pageNum + 1, totalPages)}
      >
        Next Page ›
      </button>

      {#if onClose}
        <button class="close-btn" id="diff-close-btn" onclick={onClose} title="Exit Comparison">×</button>
      {/if}
    </div>
  </header>

  <!-- Interactive Comparison Canvas Area -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="diff-canvas-area" bind:this={containerRef}>
    {#if viewMode === 'split'}
      <div class="split-comparison-container">
        <!-- Underneath: Doc B (Right side) -->
        <canvas bind:this={canvasB} class="canvas-base"></canvas>

        <!-- Top Layer: Doc A (Clipped to splitPercent) -->
        <div class="split-clip-wrapper" style={`width: ${splitPercent}%;`}>
          <canvas bind:this={canvasA} class="canvas-overlay"></canvas>
        </div>

        <!-- Draggable Divider Handle -->
        <div
          class="split-slider-handle"
          id="diff-split-handle"
          style={`left: ${splitPercent}%;`}
          onmousedown={startSplitterDrag}
        >
          <div class="handle-bar"></div>
          <div class="handle-knob">‹ ›</div>
        </div>

        <!-- Hidden canvas for diff calc -->
        <canvas bind:this={canvasDiff} style="display: none;"></canvas>
      </div>

    {:else if viewMode === 'overlay'}
      <div class="overlay-container">
        <canvas bind:this={canvasDiff} class="canvas-diff-view" id="diff-overlay-canvas"></canvas>
        <!-- Retain A and B in DOM for rendering computations -->
        <canvas bind:this={canvasA} style="display: none;"></canvas>
        <canvas bind:this={canvasB} style="display: none;"></canvas>
      </div>

    {:else if viewMode === 'side-by-side'}
      <div class="side-by-side-container">
        <div class="side-pane">
          <span class="pane-label">Doc A: {appState.documents[docAIndex]?.name ?? ''}</span>
          <canvas bind:this={canvasA} class="side-canvas"></canvas>
        </div>
        <div class="side-pane">
          <span class="pane-label">Doc B: {appState.documents[docBIndex]?.name ?? ''}</span>
          <canvas bind:this={canvasB} class="side-canvas"></canvas>
        </div>
        <canvas bind:this={canvasDiff} style="display: none;"></canvas>
      </div>
    {/if}
  </div>
</div>

<style>
  .diff-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: var(--surface-0, #0d0d0f);
    color: var(--text-primary, #ffffff);
    overflow: hidden;
  }

  .diff-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 10px 16px;
    background: var(--surface-1, #141416);
    border-bottom: 1px solid var(--border-color, #27272a);
    flex-wrap: wrap;
    z-index: 10;
  }

  .diff-title-section {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .diff-badge {
    font-size: 11px;
    font-weight: 600;
    padding: 2px 8px;
    background: rgba(94, 106, 210, 0.2);
    color: var(--accent-primary, #5e6ad2);
    border-radius: 9999px;
  }

  .diff-title-section h2 {
    font-size: 14px;
    font-weight: 600;
    margin: 0;
  }

  .diff-status {
    font-size: 12px;
    padding: 2px 8px;
    border-radius: 4px;
  }

  .diff-status.changed {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }

  .diff-status.identical {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .diff-doc-selectors {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .selector-group {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-secondary, #9ca3af);
  }

  .selector-group select {
    background: var(--surface-2, #1e1e24);
    color: var(--text-primary, #ffffff);
    border: 1px solid var(--border-color, #27272a);
    border-radius: 4px;
    padding: 4px 8px;
    font-size: 12px;
    max-width: 140px;
    outline: none;
  }

  .vs-divider {
    font-weight: 600;
    font-size: 11px;
    color: var(--text-muted, #71717a);
  }

  .diff-mode-switches {
    display: flex;
    background: var(--surface-2, #1e1e24);
    border-radius: 6px;
    padding: 2px;
    gap: 2px;
  }

  .mode-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary, #9ca3af);
    font-size: 12px;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .mode-btn.active {
    background: var(--accent-primary, #5e6ad2);
    color: #ffffff;
    font-weight: 500;
  }

  .diff-nav-controls {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .nav-btn {
    background: var(--surface-2, #1e1e24);
    border: 1px solid var(--border-color, #27272a);
    color: var(--text-primary, #ffffff);
    font-size: 12px;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
  }

  .nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .page-indicator {
    font-size: 12px;
    color: var(--text-secondary, #9ca3af);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary, #9ca3af);
    font-size: 18px;
    cursor: pointer;
    padding: 0 6px;
  }

  .close-btn:hover {
    color: #ffffff;
  }

  .diff-canvas-area {
    flex: 1;
    position: relative;
    overflow: auto;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 24px;
    background: var(--bg-secondary, #121214);
  }

  /* Split View */
  .split-comparison-container {
    position: relative;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    background: #ffffff;
    display: inline-block;
  }

  .canvas-base {
    display: block;
  }

  .split-clip-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    overflow: hidden;
    border-right: 2px solid var(--accent-primary, #5e6ad2);
  }

  .canvas-overlay {
    display: block;
  }

  .split-slider-handle {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 20px;
    margin-left: -10px;
    cursor: ew-resize;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    z-index: 20;
  }

  .handle-bar {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    background: var(--accent-primary, #5e6ad2);
  }

  .handle-knob {
    position: relative;
    width: 28px;
    height: 28px;
    background: var(--accent-primary, #5e6ad2);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
    user-select: none;
  }

  /* Overlay View */
  .overlay-container {
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    background: #ffffff;
  }

  .canvas-diff-view {
    display: block;
  }

  /* Side-by-side View */
  .side-by-side-container {
    display: flex;
    gap: 20px;
    align-items: flex-start;
  }

  .side-pane {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .pane-label {
    font-size: 12px;
    color: var(--text-secondary, #9ca3af);
    font-weight: 500;
  }

  .side-canvas {
    display: block;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    background: #ffffff;
  }
</style>
