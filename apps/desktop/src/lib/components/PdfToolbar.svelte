<script lang="ts">
  import { onMount } from 'svelte';

  let {
    pageNum = $bindable(1),
    numPages = 0,
    scale = $bindable(1.2),
  } = $props();

  let inputPage = $state(String(pageNum));
  let isEditingPage = $state(false);

  // Keep local page input in sync with external page changes (thumbnails, jumpToPage)
  $effect(() => {
    if (!isEditingPage) {
      inputPage = String(pageNum);
    }
  });

  function applyPageJump() {
    isEditingPage = false;
    const parsed = parseInt(inputPage, 10);
    if (!isNaN(parsed) && parsed >= 1 && parsed <= (numPages || 1)) {
      pageNum = parsed;
    } else {
      inputPage = String(pageNum);
    }
  }

  function handlePageKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      applyPageJump();
      (e.target as HTMLInputElement).blur();
    } else if (e.key === 'Escape') {
      inputPage = String(pageNum);
      isEditingPage = false;
      (e.target as HTMLInputElement).blur();
    }
  }

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
    scale = Math.min(Math.round((scale + 0.2) * 10) / 10, 5.0);
  }

  function zoomOut() {
    scale = Math.max(Math.round((scale - 0.2) * 10) / 10, 0.2);
  }

  function setZoomPreset(val: number) {
    scale = val;
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
  <!-- Page Navigation Group with Direct Page Input -->
  <div class="toolbar-group">
    <button
      class="toolbar-btn"
      id="toolbar-prev-page"
      onclick={prevPage}
      disabled={pageNum <= 1}
      title="Previous Page (Left Arrow)"
    >
      ‹
    </button>

    <div class="page-jump-container">
      <input
        type="text"
        id="toolbar-page-input"
        class="page-input"
        bind:value={inputPage}
        onfocus={() => isEditingPage = true}
        onblur={applyPageJump}
        onkeydown={handlePageKeydown}
        title="Type page number and press Enter"
        aria-label="Current page number"
      />
      <span class="page-total">/ {numPages || 1}</span>
    </div>

    <button
      class="toolbar-btn"
      id="toolbar-next-page"
      onclick={nextPage}
      disabled={pageNum >= numPages}
      title="Next Page (Right Arrow)"
    >
      ›
    </button>
  </div>

  <!-- Zoom Controls with Presets Dropdown & Buttons -->
  <div class="toolbar-group">
    <button
      class="toolbar-btn"
      id="toolbar-zoom-out"
      onclick={zoomOut}
      title="Zoom Out (Ctrl -)"
      disabled={scale <= 0.2}
    >
      −
    </button>

    <div class="zoom-dropdown-wrapper">
      <select
        id="toolbar-zoom-select"
        class="zoom-select"
        value={Math.round(scale * 100)}
        onchange={(e) => setZoomPreset(parseInt((e.target as HTMLSelectElement).value, 10) / 100)}
        title="Zoom Preset"
        aria-label="Zoom level"
      >
        <option value="50">50%</option>
        <option value="75">75%</option>
        <option value="100">100%</option>
        <option value="125">125%</option>
        <option value="150">150%</option>
        <option value="175">175%</option>
        <option value="200">200%</option>
        <option value="300">300%</option>
        {#if ![50, 75, 100, 125, 150, 175, 200, 300].includes(Math.round(scale * 100))}
          <option value={Math.round(scale * 100)}>{Math.round(scale * 100)}%</option>
        {/if}
      </select>
    </div>

    <button
      class="toolbar-btn"
      id="toolbar-zoom-in"
      onclick={zoomIn}
      title="Zoom In (Ctrl +)"
      disabled={scale >= 5.0}
    >
      +
    </button>

    <button
      class="toolbar-btn-text"
      id="toolbar-fit-width"
      onclick={() => scale = 1.0}
      title="Reset to 100% (Fit Width)"
    >
      Fit
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1.5rem;
    padding: 0.5rem 1rem;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-color);
    box-shadow: var(--shadow-sm);
    z-index: 10;
  }

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }

  .toolbar-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm, 4px);
    font-size: 1rem;
    font-weight: 500;
    color: var(--text-primary);
    cursor: pointer;
    transition: all var(--transition-fast, 0.15s ease);
    padding: 0;
  }

  .toolbar-btn:hover:not(:disabled) {
    background-color: var(--bg-surface-hover);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .toolbar-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .toolbar-btn-text {
    height: 28px;
    padding: 0 8px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm, 4px);
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast, 0.15s ease);
  }

  .toolbar-btn-text:hover {
    color: var(--accent-primary);
    border-color: var(--accent-primary);
    background-color: var(--bg-surface-hover);
  }

  .page-jump-container {
    display: flex;
    align-items: center;
    gap: 4px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm, 4px);
    padding: 0 6px;
    height: 28px;
  }

  .page-input {
    width: 32px;
    height: 22px;
    background: transparent;
    border: none;
    text-align: center;
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-primary);
    outline: none;
    padding: 0;
  }

  .page-input:focus {
    background: var(--bg-surface-hover);
    border-radius: 2px;
  }

  .page-total {
    font-size: 0.85rem;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .zoom-dropdown-wrapper {
    position: relative;
  }

  .zoom-select {
    height: 28px;
    padding: 0 6px;
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm, 4px);
    font-size: 0.85rem;
    color: var(--text-primary);
    cursor: pointer;
    outline: none;
    font-weight: 500;
  }

  .zoom-select:hover {
    border-color: var(--accent-primary);
  }
</style>
