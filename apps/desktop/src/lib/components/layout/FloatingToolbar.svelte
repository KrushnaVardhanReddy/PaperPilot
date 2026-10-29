<script lang="ts">
  import { appState } from '$lib/state/app.svelte';


  let { pageNum = 1, numPages = 1, scale = 1.0 } = $props();
  function dispatchAction(evt: string) { window.dispatchEvent(new CustomEvent(evt)); }
</script>

<div class="floating-toolbar">
  <div class="toolbar-group">
    <button class="tool-btn" title="Previous Page" onclick={() => dispatchAction('paperpilot:page-prev')}>↑</button>
    <span class="page-pill">{pageNum} / {numPages}</span>
    <button class="tool-btn" title="Next Page" onclick={() => dispatchAction('paperpilot:page-next')}>↓</button>
  </div>
  <div class="divider"></div>
  <div class="toolbar-group">
    <button class="tool-btn" title="Zoom Out" onclick={() => dispatchAction('paperpilot:zoom-out')}>−</button>
    <span class="zoom-level">{Math.round(scale * 100)}%</span>
    <button class="tool-btn" title="Zoom In" onclick={() => dispatchAction('paperpilot:zoom-in')}>+</button>
  </div>
  <div class="divider"></div>
  <div class="toolbar-group">
    <button class="tool-btn" title="Search">🔍</button>
    <button class="tool-btn" title="Share">📤</button>
  </div>
</div>

<style>
  .floating-toolbar { position: absolute; top: 24px; left: 50%; transform: translateX(-50%); display: flex; align-items: center; gap: 12px; background: var(--glass-bg); backdrop-filter: var(--glass-blur); -webkit-backdrop-filter: var(--glass-blur); border: 1px solid var(--glass-border); padding: 8px 16px; border-radius: 40px; box-shadow: 0 4px 24px rgba(0,0,0,0.4); z-index: 50; }
  .toolbar-group { display: flex; align-items: center; gap: 8px; }
  .tool-btn { width: 32px; height: 32px; border-radius: 50%; background: transparent; border: none; color: var(--text-primary); cursor: pointer; display: flex; align-items: center; justify-content: center; transition: background 0.2s; }
  .tool-btn:hover { background: rgba(255,255,255,0.1); }
  .page-pill, .zoom-level { font-size: 0.85rem; color: var(--text-primary); font-variant-numeric: tabular-nums; font-weight: 500; }
  .divider { width: 1px; height: 20px; background: var(--border-color); }
</style>
