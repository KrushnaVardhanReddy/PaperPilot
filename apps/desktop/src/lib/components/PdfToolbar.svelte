<script lang="ts">
  import { onMount } from 'svelte';

  let {
    pageNum = $bindable(1),
    numPages = 0,
    scale = $bindable(1.2),
  } = $props();

  function prevPage() {
    if (pageNum > 1) {
      pageNum -= 1;
    }
  }

  function nextPage() {
    if (pageNum < numPages) {
      pageNum += 1;
    }
  }

  function zoomIn() {
    scale += 0.2;
  }

  function zoomOut() {
    if (scale > 0.4) {
      scale -= 0.2;
    }
  }

  onMount(() => {
    function onZoomIn() { scale = Math.min(scale + 0.2, 5.0); }
    function onZoomOut() { scale = Math.max(scale - 0.2, 0.2); }
    function onFitWidth() { scale = 1.0; }

    window.addEventListener('paperpilot:zoom-in', onZoomIn);
    window.addEventListener('paperpilot:zoom-out', onZoomOut);
    window.addEventListener('paperpilot:fit-width', onFitWidth);

    return () => {
      window.removeEventListener('paperpilot:zoom-in', onZoomIn);
      window.removeEventListener('paperpilot:zoom-out', onZoomOut);
      window.removeEventListener('paperpilot:fit-width', onFitWidth);
    };
  });
</script>

<div class="toolbar">
  <div class="toolbar-group">
    <button
      class="toolbar-btn"
      onclick={prevPage}
      disabled={pageNum <= 1}
      title="Previous Page"
    >
      Prev
    </button>
    <span class="page-info">
      Page {pageNum} of {numPages || 1}
    </span>
    <button
      class="toolbar-btn"
      onclick={nextPage}
      disabled={pageNum >= numPages}
      title="Next Page"
    >
      Next
    </button>
  </div>

  <div class="toolbar-group">
    <button class="toolbar-btn" onclick={zoomOut} title="Zoom Out" disabled={scale <= 0.4}>
      Zoom Out
    </button>
    <span class="zoom-info">
      {Math.round(scale * 100)}%
    </span>
    <button class="toolbar-btn" onclick={zoomIn} title="Zoom In">
      Zoom In
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 2rem;
    padding: 0.75rem 1rem;
    background-color: var(--bg-surface, #ffffff);
    border-bottom: 1px solid var(--border-color, #e5e7eb);
    box-shadow: 0 1px 3px rgba(0,0,0,0.05);
    z-index: 10;
  }

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .toolbar-btn {
    padding: 0.4rem 0.8rem;
    background-color: var(--bg-primary, #f9fafb);
    border: 1px solid var(--border-color, #d1d5db);
    border-radius: 4px;
    font-size: 0.9rem;
    color: var(--text-primary, #374151);
    cursor: pointer;
    transition: all 0.2s;
  }

  .toolbar-btn:hover:not(:disabled) {
    background-color: var(--bg-surface-hover, #f3f4f6);
    border-color: var(--accent-primary, #3b82f6);
    color: var(--accent-primary, #3b82f6);
  }

  .toolbar-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .page-info, .zoom-info {
    font-size: 0.9rem;
    color: var(--text-secondary, #4b5563);
    min-width: 80px;
    text-align: center;
  }
</style>
