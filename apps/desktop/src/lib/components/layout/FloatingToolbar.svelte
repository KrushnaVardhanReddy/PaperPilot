<script lang="ts">
  import { appState } from '$lib/state/app.svelte';

  let {
    zoomIn = () => {},
    zoomOut = () => {},
    toggleSidebar = () => {},
    pageNum = 1,
    numPages = 0,
    scale = 1.0,
  }: {
    zoomIn?: () => void;
    zoomOut?: () => void;
    toggleSidebar?: () => void;
    pageNum?: number;
    numPages?: number;
    scale?: number;
  } = $props();

  let zoomPercent = $derived(Math.round(scale * 100));
</script>

{#if !appState.isMobile}
  <div class="floating-toolbar">
    <div class="toolbar-group">
      <button class="toolbar-btn" aria-label="Toggle Sidebar" onclick={toggleSidebar}>
        ☰
      </button>
    </div>

    <div class="toolbar-divider"></div>

    <div class="toolbar-group">
      <button class="toolbar-btn" aria-label="Previous Page">↑</button>
      <span class="page-pill">
        {pageNum} / {numPages}
      </span>
      <button class="toolbar-btn" aria-label="Next Page">↓</button>
    </div>

    <div class="toolbar-divider"></div>

    <div class="toolbar-group">
      <button class="toolbar-btn" aria-label="Zoom Out" onclick={zoomOut}>−</button>
      <span class="zoom-label">{zoomPercent}%</span>
      <button class="toolbar-btn" aria-label="Zoom In" onclick={zoomIn}>+</button>
    </div>

    <div class="toolbar-divider"></div>

    <div class="toolbar-group">
      <button class="toolbar-btn" aria-label="Search">🔍</button>
      <button class="toolbar-btn" aria-label="Annotations">✎</button>
      <button class="toolbar-btn" aria-label="Share">📤</button>
    </div>
  </div>
{/if}

<style>
  .floating-toolbar {
    position: absolute;
    top: 24px;
    left: 50%;
    transform: translateX(-50%);
    background: var(--glass-bg, rgba(30, 30, 35, 0.75));
    backdrop-filter: var(--glass-blur, blur(12px));
    -webkit-backdrop-filter: var(--glass-blur, blur(12px));
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
    border-radius: 24px;
    display: flex;
    align-items: center;
    padding: 6px 12px;
    gap: 8px;
    z-index: 20;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  }

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .toolbar-divider {
    width: 1px;
    height: 16px;
    background: rgba(255, 255, 255, 0.1);
    margin: 0 4px;
  }

  .toolbar-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary, #9ca3af);
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
  }
  .toolbar-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-primary, #ffffff);
  }

  .page-pill, .zoom-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary, #ffffff);
    min-width: 48px;
    text-align: center;
  }
</style>
