<script lang="ts">
  import { tick } from 'svelte';

  let { pdfDoc, pageNum = $bindable(1) } = $props();

  let numPages = $derived(pdfDoc ? pdfDoc.numPages : 0);
  let isRendering = false;
  let canvases: Record<number, HTMLCanvasElement> = {};
  let draggedPage = $state<number | null>(null);
  let dragOverPage = $state<number | null>(null);

  async function renderThumbnails() {
    if (!pdfDoc || numPages === 0 || isRendering) return;
    isRendering = true;
    await tick();

    try {
      for (let i = 1; i <= numPages; i++) {
        const canvas = canvases[i];
        if (!canvas) continue;

        const page = await pdfDoc.getPage(i);
        const viewport = page.getViewport({ scale: 0.2 });

        const ctx = canvas.getContext('2d');
        if (!ctx) continue;

        const outputScale = window.devicePixelRatio || 1;
        canvas.width = Math.floor(viewport.width * outputScale);
        canvas.height = Math.floor(viewport.height * outputScale);
        canvas.style.width = Math.floor(viewport.width) + "px";
        canvas.style.height = Math.floor(viewport.height) + "px";

        const transform = outputScale !== 1
          ? [outputScale, 0, 0, outputScale, 0, 0]
          : null;

        const renderContext = {
          canvasContext: ctx,
          transform: transform ? transform : undefined,
          viewport,
          canvas,
        };

        await page.render(renderContext as any).promise;
      }
    } catch (err) {
      console.error("Error rendering thumbnails:", err);
    } finally {
      isRendering = false;
    }
  }

  $effect(() => {
    if (pdfDoc) {
      renderThumbnails();
    }
  });

  function selectPage(page: number) {
    pageNum = page;
  }

  function handleDragStart(e: DragEvent, page: number) {
    draggedPage = page;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', String(page));
    }
  }

  function handleDragOver(e: DragEvent, page: number) {
    e.preventDefault();
    if (draggedPage !== null && draggedPage !== page) {
      dragOverPage = page;
    }
  }

  function handleDrop(e: DragEvent, targetPage: number) {
    e.preventDefault();
    if (draggedPage !== null && draggedPage !== targetPage) {
      window.dispatchEvent(new CustomEvent('paperpilot:reorder-page', {
        detail: { fromPage: draggedPage, toPage: targetPage }
      }));
    }
    draggedPage = null;
    dragOverPage = null;
  }

  function handleDragEnd() {
    draggedPage = null;
    dragOverPage = null;
  }

  function rotatePage(e: MouseEvent, page: number) {
    e.stopPropagation();
    window.dispatchEvent(new CustomEvent('paperpilot:rotate-single-page', {
      detail: { page }
    }));
  }

  function deletePage(e: MouseEvent, page: number) {
    e.stopPropagation();
    window.dispatchEvent(new CustomEvent('paperpilot:delete-single-page', {
      detail: { page }
    }));
  }
</script>

<div class="thumbnails-container" aria-label="Page Thumbnails">
  {#if numPages > 0}
    {#each Array(numPages) as _, index}
      {@const p = index + 1}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="thumbnail-wrapper"
        class:selected={pageNum === p}
        class:drag-over={dragOverPage === p}
        class:is-dragging={draggedPage === p}
        id="thumbnail-page-{p}"
        draggable="true"
        ondragstart={(e) => handleDragStart(e, p)}
        ondragover={(e) => handleDragOver(e, p)}
        ondrop={(e) => handleDrop(e, p)}
        ondragend={handleDragEnd}
        onclick={() => selectPage(p)}
        onkeydown={(e) => e.key === 'Enter' && selectPage(p)}
        tabindex="0"
        role="button"
        aria-label="Go to page {p}"
      >
        <div class="canvas-box">
          <canvas bind:this={canvases[p]}></canvas>

          <!-- Hover Action Overlay -->
          <div class="hover-actions">
            <button
              class="action-btn"
              id="thumb-rotate-{p}"
              title="Rotate Page {p}"
              aria-label="Rotate Page {p}"
              onclick={(e) => rotatePage(e, p)}
            >
              🔄
            </button>
            <button
              class="action-btn delete"
              id="thumb-delete-{p}"
              title="Delete Page {p}"
              aria-label="Delete Page {p}"
              onclick={(e) => deletePage(e, p)}
            >
              🗑️
            </button>
          </div>
        </div>
        <span class="page-num">{p}</span>
      </div>
    {/each}
  {/if}
</div>

<style>
  .thumbnails-container {
    width: 140px;
    height: 100%;
    overflow-y: auto;
    background-color: var(--bg-secondary, #13131a);
    border-right: 1px solid var(--border-color, #2a2a35);
    padding: 12px 8px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    user-select: none;
    flex-shrink: 0;
  }

  .thumbnail-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: grab;
    border-radius: var(--border-radius-sm, 4px);
    padding: 4px;
    transition: all var(--transition-fast, 0.15s ease);
    position: relative;
  }

  .thumbnail-wrapper:active {
    cursor: grabbing;
  }

  .thumbnail-wrapper.is-dragging {
    opacity: 0.4;
  }

  .thumbnail-wrapper.drag-over {
    border-top: 2px solid var(--accent-primary, #5e6ad2);
  }

  .thumbnail-wrapper:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
  }

  .thumbnail-wrapper.selected {
    background-color: rgba(94, 106, 210, 0.15);
  }

  .thumbnail-wrapper.selected .canvas-box {
    box-shadow: 0 0 0 2px var(--accent-primary, #5e6ad2);
  }

  .canvas-box {
    position: relative;
    border-radius: 2px;
    overflow: hidden;
    background-color: #ffffff;
    box-shadow: 0 1px 4px rgba(0,0,0,0.25);
    display: flex;
    justify-content: center;
  }

  .hover-actions {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    opacity: 0;
    transition: opacity var(--transition-fast, 0.15s ease);
  }

  .thumbnail-wrapper:hover .hover-actions {
    opacity: 1;
  }

  .action-btn {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.85);
    border: none;
    font-size: 11px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: transform var(--transition-fast, 0.15s ease), background 0.15s ease;
  }

  .action-btn:hover {
    transform: scale(1.15);
    background: #ffffff;
  }

  .action-btn.delete:hover {
    background: #fee2e2;
  }

  .page-num {
    margin-top: 4px;
    font-size: 11px;
    color: var(--text-secondary, #9ca3af);
  }

  .thumbnail-wrapper.selected .page-num {
    color: var(--text-primary, #ffffff);
    font-weight: 600;
  }
</style>