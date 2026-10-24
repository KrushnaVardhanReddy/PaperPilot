<script lang="ts">
  import { onMount } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';
  import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.mjs?url';
  import { appState } from '$lib/state/app.svelte';

  pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;

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
  let viewMode = $state<'split' | 'overlay' | 'side-by-side' | 'text-diff'>('split');
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
  let canvasSideA: HTMLCanvasElement | undefined = $state();
  let canvasSideB: HTMLCanvasElement | undefined = $state();
  let canvasDiff: HTMLCanvasElement | undefined = $state();
  let containerRef: HTMLDivElement | undefined = $state();
  let canvasWidth = $state(600);

  let changedPixelCount = $state(0);
  let isComparing = $state(false);

  // Git-style unified text diff state
  interface DiffLine {
    type: 'add' | 'del' | 'eq';
    lineA?: number;
    lineB?: number;
    text: string;
  }
  let textDiffLines = $state<DiffLine[]>([]);
  let textDiffStats = $state({ additions: 0, deletions: 0 });

  async function loadDoc(file: File): Promise<any> {
    const arrayBuffer = await file.arrayBuffer();
    const loadingTask = pdfjsLib.getDocument({ data: arrayBuffer });
    return await loadingTask.promise;
  }

  async function loadBothDocs(idxA: number, idxB: number) {
    const fileA = appState.documents[idxA];
    const fileB = appState.documents[idxB];

    const [docA, docB] = await Promise.all([
      fileA ? loadDoc(fileA).catch(e => { console.error("Doc A load fail", e); return null; }) : Promise.resolve(null),
      fileB ? loadDoc(fileB).catch(e => { console.error("Doc B load fail", e); return null; }) : Promise.resolve(null)
    ]);

    pdfDocA = docA;
    pdfDocB = docB;
    await renderComparison(pageNum, scale);
  }

  // Reload when user picks from dropdown
  async function handleDocAChange(newIdx: number | string) {
    const idx = Number(newIdx);
    docAIndex = idx;
    const file = appState.documents[idx];
    if (file) {
      pdfDocA = await loadDoc(file);
      await renderComparison(pageNum, scale);
    }
  }

  async function handleDocBChange(newIdx: number | string) {
    const idx = Number(newIdx);
    docBIndex = idx;
    const file = appState.documents[idx];
    if (file) {
      pdfDocB = await loadDoc(file);
      await renderComparison(pageNum, scale);
    }
  }

  // Initial mount load
  onMount(() => {
    if (appState.documents.length > 0) {
      loadBothDocs(docAIndex, docBIndex);
    }
  });

  // Render both pages and compute diff when page, scale, viewMode, or docs change
  $effect(() => {
    // Explicit dependencies for Svelte 5 rune tracking
    const _docA = pdfDocA;
    const _docB = pdfDocB;
    const _p = pageNum;
    const _s = scale;
    const _m = viewMode;

    if (_docA || _docB) {
      renderComparison(_p, _s);
    }
  });

  async function renderComparison(page: number, currentScale: number) {
    isComparing = true;
    changedPixelCount = 0;

    try {
      let width = 600;
      let height = 800;

      // Determine page dimensions from whichever doc has it
      if (pdfDocA && page <= pdfDocA.numPages) {
        const pA = await pdfDocA.getPage(page);
        const vpA = pA.getViewport({ scale: currentScale });
        width = Math.floor(vpA.width);
        height = Math.floor(vpA.height);
      }
      if (pdfDocB && page <= pdfDocB.numPages) {
        const pB = await pdfDocB.getPage(page);
        const vpB = pB.getViewport({ scale: currentScale });
        width = Math.max(width, Math.floor(vpB.width));
        height = Math.max(height, Math.floor(vpB.height));
      }

      canvasWidth = width;

      // Render Doc A
      if (pdfDocA && page <= pdfDocA.numPages) {
        const pA = await pdfDocA.getPage(page);
        const vpA = pA.getViewport({ scale: currentScale });

        for (const c of [canvasA, canvasSideA]) {
          if (c) {
            c.width = width;
            c.height = height;
            const ctx = c.getContext('2d');
            if (ctx) {
              ctx.clearRect(0, 0, width, height);
              await pA.render({ canvasContext: ctx, viewport: vpA }).promise;
            }
          }
        }
      } else {
        for (const c of [canvasA, canvasSideA]) {
          if (c) {
            c.width = width;
            c.height = height;
            const ctx = c.getContext('2d');
            if (ctx) {
              ctx.fillStyle = '#f8fafc';
              ctx.fillRect(0, 0, width, height);
            }
          }
        }
      }

      // Render Doc B
      if (pdfDocB && page <= pdfDocB.numPages) {
        const pB = await pdfDocB.getPage(page);
        const vpB = pB.getViewport({ scale: currentScale });

        for (const c of [canvasB, canvasSideB]) {
          if (c) {
            c.width = width;
            c.height = height;
            const ctx = c.getContext('2d');
            if (ctx) {
              ctx.clearRect(0, 0, width, height);
              await pB.render({ canvasContext: ctx, viewport: vpB }).promise;
            }
          }
        }
      } else {
        for (const c of [canvasB, canvasSideB]) {
          if (c) {
            c.width = width;
            c.height = height;
            const ctx = c.getContext('2d');
            if (ctx) {
              ctx.fillStyle = '#f8fafc';
              ctx.fillRect(0, 0, width, height);
            }
          }
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

              // Lum calculation to detect dark text strokes vs white background
              const lumA = 0.299 * rA + 0.587 * gA + 0.114 * bA;
              const lumB = 0.299 * rB + 0.587 * gB + 0.114 * bB;

              if (lumA < 200 && lumB >= 200) {
                // Pixel existed in Doc A but removed in Doc B -> Red (Deletions)
                out[i] = 239;     // R
                out[i + 1] = 68;  // G
                out[i + 2] = 68;  // B
                out[i + 3] = 255; // A
              } else if (lumB < 200 && lumA >= 200) {
                // Pixel added in Doc B that wasn't in Doc A -> Emerald Green (Additions)
                out[i] = 16;      // R
                out[i + 1] = 185; // G
                out[i + 2] = 129; // B
                out[i + 3] = 255; // A
              } else {
                // Shifted or modified pixel color/position -> Amber
                out[i] = 245;     // R
                out[i + 1] = 158; // G
                out[i + 2] = 11;  // B
                out[i + 3] = 240; // A
              }
            } else {
              // Unchanged content: keep it legible in soft high-contrast grayscale
              const gray = Math.round(rA * 0.299 + gA * 0.587 + bA * 0.114);
              out[i] = gray;
              out[i + 1] = gray;
              out[i + 2] = gray;
              // Dim background white to crisp clean white, but keep text dark enough to read
              out[i + 3] = (gray > 240) ? 255 : 120;
            }
          }

          ctxDiff.putImageData(diffData, 0, 0);
          changedPixelCount = diffCount;
        }
      }

      // Compute Git-style Line Diff
      await computeTextDiff(page);
    } catch (err) {
      console.error("Comparison render error:", err);
    } finally {
      isComparing = false;
    }
  }

  // Extract lines of text from page
  async function getPageLines(doc: any, page: number): Promise<string[]> {
    if (!doc || page > doc.numPages) return [];
    try {
      const p = await doc.getPage(page);
      const textContent = await p.getTextContent();
      // Group text items by vertical position Y
      const lineMap = new Map<number, string[]>();
      for (const item of textContent.items) {
        if ('str' in item && item.str.trim()) {
          const y = Math.round(item.transform[5] / 4) * 4;
          if (!lineMap.has(y)) lineMap.set(y, []);
          lineMap.get(y)!.push(item.str);
        }
      }
      // Sort top to bottom (PDF Y is bottom-up)
      const sortedYs = Array.from(lineMap.keys()).sort((a, b) => b - a);
      return sortedYs.map(y => lineMap.get(y)!.join(' ').trim()).filter(Boolean);
    } catch {
      return [];
    }
  }

  async function computeTextDiff(page: number) {
    const [linesA, linesB] = await Promise.all([
      getPageLines(pdfDocA, page),
      getPageLines(pdfDocB, page)
    ]);

    // Simple LCS diff for clean Git-style view
    const diffs: DiffLine[] = [];
    let i = 0, j = 0;
    let addCount = 0, delCount = 0;

    while (i < linesA.length || j < linesB.length) {
      if (i < linesA.length && j < linesB.length && linesA[i] === linesB[j]) {
        diffs.push({ type: 'eq', lineA: i + 1, lineB: j + 1, text: linesA[i] });
        i++; j++;
      } else if (j < linesB.length && (!linesA.includes(linesB[j]) || linesB.indexOf(linesA[i]) > j)) {
        diffs.push({ type: 'add', lineB: j + 1, text: linesB[j] });
        addCount++;
        j++;
      } else if (i < linesA.length) {
        diffs.push({ type: 'del', lineA: i + 1, text: linesA[i] });
        delCount++;
        i++;
      } else {
        diffs.push({ type: 'add', lineB: j + 1, text: linesB[j] });
        addCount++;
        j++;
      }
    }

    textDiffLines = diffs;
    textDiffStats = { additions: addCount, deletions: delCount };
  }

  function handlePointerMove(e: PointerEvent) {
    if (!isDraggingSplitter || !containerRef) return;
    const rect = containerRef.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percent = Math.min(Math.max((x / rect.width) * 100, 2), 98);
    splitPercent = percent;
  }

  function handlePointerUp(e: PointerEvent) {
    isDraggingSplitter = false;
    try {
      (e.target as HTMLElement)?.releasePointerCapture(e.pointerId);
    } catch {}
    window.removeEventListener('pointermove', handlePointerMove);
    window.removeEventListener('pointerup', handlePointerUp);
  }

  function startSplitterDrag(e: PointerEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDraggingSplitter = true;
    try {
      (e.currentTarget as HTMLElement)?.setPointerCapture(e.pointerId);
    } catch {}
    window.addEventListener('pointermove', handlePointerMove);
    window.addEventListener('pointerup', handlePointerUp);
  }
</script>

<div class="diff-viewer" id="visual-diff-container">
  <!-- Top Control Header (Compact Single-Line Toolbar) -->
  <header class="diff-header">
    <!-- Left: Title & Quick Status -->
    <div class="diff-left-section">
      <div class="diff-brand">
        <span class="diff-badge">⚡ Compare</span>
      </div>

      <!-- Compact Document Selectors -->
      <div class="diff-doc-selectors">
        <select
          id="doc-a-select"
          class="compact-select"
          bind:value={docAIndex}
          onchange={(e) => handleDocAChange((e.target as HTMLSelectElement).value)}
          title="Base Document (Doc A)"
        >
          {#each appState.documents as doc, idx}
            <option value={idx}>{doc.name}</option>
          {/each}
        </select>

        <span class="vs-divider">vs</span>

        <select
          id="doc-b-select"
          class="compact-select"
          bind:value={docBIndex}
          onchange={(e) => handleDocBChange((e.target as HTMLSelectElement).value)}
          title="Comparison Document (Doc B)"
        >
          {#each appState.documents as doc, idx}
            <option value={idx}>{doc.name}</option>
          {/each}
        </select>
      </div>

      {#if changedPixelCount > 0}
        <span class="diff-status changed" title={`${changedPixelCount.toLocaleString()} visual pixels differ`}>
          ⚠️ {changedPixelCount.toLocaleString()} diffs
        </span>
      {:else}
        <span class="diff-status identical">✅ Identical</span>
      {/if}

      {#if viewMode === 'overlay'}
        <div class="diff-legend">
          <span class="legend-item"><span class="legend-dot red"></span> Removed</span>
          <span class="legend-item"><span class="legend-dot green"></span> Added</span>
        </div>
      {/if}
    </div>

    <!-- Center: View Mode Switches -->
    <div class="diff-mode-switches">
      <button
        class="mode-btn"
        class:active={viewMode === 'split'}
        id="diff-mode-split"
        onclick={() => viewMode = 'split'}
        title="Interactive split-slider before/after"
      >
        ↔️ Split
      </button>
      <button
        class="mode-btn"
        class:active={viewMode === 'overlay'}
        id="diff-mode-overlay"
        onclick={() => viewMode = 'overlay'}
        title="Highlight pixel discrepancies"
      >
        🔥 Overlay
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
      <button
        class="mode-btn"
        class:active={viewMode === 'text-diff'}
        id="diff-mode-git"
        onclick={() => viewMode = 'text-diff'}
        title="Git-style line additions & deletions"
      >
        🌿 Git Diff
      </button>
    </div>

    <!-- Right: Page Controls & Close -->
    <div class="diff-nav-controls">
      <button
        class="nav-btn"
        id="diff-prev-page"
        disabled={pageNum <= 1}
        onclick={() => pageNum = Math.max(pageNum - 1, 1)}
      >
        ‹
      </button>
      <span class="page-indicator">{pageNum}/{totalPages}</span>
      <button
        class="nav-btn"
        id="diff-next-page"
        disabled={pageNum >= totalPages}
        onclick={() => pageNum = Math.min(pageNum + 1, totalPages)}
      >
        ›
      </button>

      {#if onClose}
        <button class="close-btn" id="diff-close-btn" onclick={onClose} title="Exit Comparison">✕</button>
      {/if}
    </div>
  </header>

  <!-- Interactive Comparison Canvas Area -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="diff-canvas-area">
    <div
      class="split-comparison-container"
      bind:this={containerRef}
      class:hidden-mode={viewMode !== 'split'}
    >
      <!-- Underneath: Doc B (Right side) -->
      <canvas bind:this={canvasB} class="canvas-base"></canvas>

      <!-- Top Layer: Doc A (Clipped to splitPercent) -->
      <div class="split-clip-wrapper" style={`width: ${splitPercent}%;`}>
        <canvas bind:this={canvasA} class="canvas-overlay" style={`width: ${canvasWidth}px; max-width: none;`}></canvas>
      </div>

      <!-- Draggable Divider Handle -->
      <div
        class="split-slider-handle"
        id="diff-split-handle"
        style={`left: ${splitPercent}%;`}
        onpointerdown={startSplitterDrag}
      >
        <div class="handle-bar"></div>
        <div class="handle-knob">‹ ›</div>
      </div>
    </div>

    <!-- Difference Map (Overlay) View -->
    <div class="overlay-container" class:hidden-mode={viewMode !== 'overlay'}>
      <canvas bind:this={canvasDiff} class="canvas-diff-view" id="diff-overlay-canvas"></canvas>
    </div>

    <!-- Side-by-Side View -->
    <div class="side-by-side-container" class:hidden-mode={viewMode !== 'side-by-side'}>
      <div class="side-pane">
        <span class="pane-label">Doc A: {appState.documents[docAIndex]?.name ?? ''}</span>
        <canvas bind:this={canvasSideA} class="side-canvas"></canvas>
      </div>
      <div class="side-pane">
        <span class="pane-label">Doc B: {appState.documents[docBIndex]?.name ?? ''}</span>
        <canvas bind:this={canvasSideB} class="side-canvas"></canvas>
      </div>
    </div>

    <!-- Git-Style Line Diff View -->
    <div class="git-diff-container" class:hidden-mode={viewMode !== 'text-diff'}>
      <div class="git-diff-card">
        <div class="git-diff-header">
          <div class="git-diff-summary">
            <span class="git-diff-stat-add">+{textDiffStats.additions} additions</span>
            <span class="git-diff-stat-del">-{textDiffStats.deletions} deletions</span>
          </div>
          <span class="git-diff-file-label">
            {appState.documents[docAIndex]?.name ?? 'Doc A'} → {appState.documents[docBIndex]?.name ?? 'Doc B'}
          </span>
        </div>
        <div class="git-diff-body">
          {#if textDiffLines.length === 0}
            <div class="git-diff-empty">No text changes detected on page {pageNum}</div>
          {:else}
            <table class="git-diff-table">
              <tbody>
                {#each textDiffLines as item}
                  <tr class="git-diff-row {item.type}">
                    <td class="git-num line-a">{item.lineA ?? ''}</td>
                    <td class="git-num line-b">{item.lineB ?? ''}</td>
                    <td class="git-marker">
                      {#if item.type === 'add'}
                        +
                      {:else if item.type === 'del'}
                        -
                      {:else}
                        {' '}
                      {/if}
                    </td>
                    <td class="git-code">{item.text}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      </div>
    </div>
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
    gap: 12px;
    padding: 6px 14px;
    background: var(--surface-1, #141416);
    border-bottom: 1px solid var(--border-color, #27272a);
    flex-wrap: nowrap;
    z-index: 10;
    min-height: 44px;
  }

  .diff-left-section {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-shrink: 0;
  }

  .diff-brand {
    display: flex;
    align-items: center;
  }

  .diff-badge {
    font-size: 11px;
    font-weight: 700;
    padding: 2px 7px;
    background: rgba(94, 106, 210, 0.2);
    color: var(--accent-primary, #818cf8);
    border-radius: 4px;
  }

  .diff-status {
    font-size: 11px;
    padding: 2px 6px;
    border-radius: 4px;
    white-space: nowrap;
  }

  .diff-status.changed {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }

  .diff-status.identical {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .diff-legend {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 2px 6px;
    background: var(--surface-2, #1e1e24);
    border-radius: 4px;
    border: 1px solid var(--border-color, #27272a);
    font-size: 10px;
    color: var(--text-secondary, #9ca3af);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .legend-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .legend-dot.red {
    background: #ef4444;
  }

  .legend-dot.green {
    background: #10b981;
  }

  .diff-doc-selectors {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .compact-select {
    background-color: #18181b !important;
    color: #f4f4f5 !important;
    border: 1px solid #3f3f46 !important;
    border-radius: 4px;
    padding: 4px 22px 4px 8px;
    font-size: 12px;
    font-weight: 500;
    min-width: 140px;
    max-width: 190px;
    outline: none;
    cursor: pointer;
    text-overflow: ellipsis;
    white-space: nowrap;
    -webkit-appearance: none;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23a1a1aa' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 6px center;
  }

  .compact-select option {
    background-color: #18181b !important;
    color: #f4f4f5 !important;
    padding: 6px;
  }

  .vs-divider {
    font-weight: 600;
    font-size: 10px;
    color: var(--text-muted, #71717a);
    text-transform: uppercase;
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
    width: 32px;
    margin-left: -16px;
    cursor: ew-resize;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    z-index: 30;
    touch-action: none;
    user-select: none;
  }

  .handle-bar {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    background: var(--accent-primary, #5e6ad2);
    pointer-events: none;
  }

  .handle-knob {
    position: relative;
    width: 32px;
    height: 32px;
    background: var(--accent-primary, #5e6ad2);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    font-weight: 700;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
    user-select: none;
    pointer-events: auto;
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

  /* Git-Style Diff */
  .git-diff-container {
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
  }

  .git-diff-card {
    background: var(--surface-1, #141416);
    border: 1px solid var(--border-color, #27272a);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  }

  .git-diff-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 16px;
    background: var(--surface-2, #1e1e24);
    border-bottom: 1px solid var(--border-color, #27272a);
    font-size: 13px;
  }

  .git-diff-summary {
    display: flex;
    gap: 12px;
    font-weight: 600;
    font-size: 12px;
  }

  .git-diff-stat-add {
    color: #10b981;
  }

  .git-diff-stat-del {
    color: #ef4444;
  }

  .git-diff-file-label {
    color: var(--text-secondary, #9ca3af);
    font-family: monospace;
    font-size: 12px;
  }

  .git-diff-body {
    max-height: 70vh;
    overflow-y: auto;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
  }

  .git-diff-empty {
    padding: 32px;
    text-align: center;
    color: var(--text-secondary, #9ca3af);
  }

  .git-diff-table {
    width: 100%;
    border-collapse: collapse;
  }

  .git-diff-row.add {
    background: rgba(16, 185, 129, 0.15);
    color: #34d399;
  }

  .git-diff-row.del {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
  }

  .git-diff-row.eq {
    color: #cbd5e1;
  }

  .git-num {
    width: 40px;
    padding: 3px 8px;
    text-align: right;
    color: #52525b;
    user-select: none;
    border-right: 1px solid rgba(255, 255, 255, 0.05);
    font-size: 11px;
  }

  .git-marker {
    width: 20px;
    text-align: center;
    user-select: none;
    font-weight: bold;
    padding: 3px 0;
  }

  .git-diff-row.add .git-marker {
    color: #10b981;
  }

  .git-diff-row.del .git-marker {
    color: #ef4444;
  }

  .git-code {
    padding: 3px 12px;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .hidden-mode {
    display: none !important;
  }
</style>
