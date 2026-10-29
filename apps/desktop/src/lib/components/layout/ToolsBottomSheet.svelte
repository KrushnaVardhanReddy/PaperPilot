<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  let { isOpen = false } = $props<{ isOpen?: boolean }>();
  function invokeTool(toolId: string) { appState.openTool({ id: toolId }); }
</script>

<div class="bottom-sheet" class:open={isOpen}>
  <div class="sheet-handle"></div>
  <div class="sheet-content">
    <div class="carousel">
      <button class="chip" onclick={() => invokeTool('redact-pii')}>⚡ Redact PII</button>
      <button class="chip" onclick={() => invokeTool('sign')}>✍️ Sign</button>
      <button class="chip" onclick={() => invokeTool('ocr')}>📑 OCR</button>
      <button class="chip" onclick={() => invokeTool('compress')}>🗜️ Compress</button>
    </div>
    <div class="engine-badge">🛡 Local Engine ACTIVE · 0 Cloud Bytes</div>
  </div>
</div>

<style>
  .bottom-sheet { position: fixed; bottom: 0; left: 0; width: 100%; background: var(--surface-1); border-top-left-radius: 16px; border-top-right-radius: 16px; box-shadow: 0 -4px 24px rgba(0,0,0,0.5); transform: translateY(100%); transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1); z-index: 100; padding-bottom: var(--safe-area-bottom, 20px); }
  .bottom-sheet.open { transform: translateY(0); }
  .sheet-handle { width: 40px; height: 4px; background: var(--border-color); border-radius: 2px; margin: 12px auto; }
  .sheet-content { padding: 0 16px 16px 16px; display: flex; flex-direction: column; gap: 16px; }
  .carousel { display: flex; gap: 12px; overflow-x: auto; padding-bottom: 8px; scrollbar-width: none; }
  .carousel::-webkit-scrollbar { display: none; }
  .chip { flex: 0 0 auto; background: var(--surface-2); border: 1px solid var(--border-color); padding: 10px 16px; border-radius: 20px; font-size: 0.9rem; color: var(--text-primary); white-space: nowrap; min-height: 48px; cursor: pointer; }
  .engine-badge { background: var(--engine-green-bg); color: var(--watching-green); border: 1px solid rgba(34, 197, 94, 0.2); border-radius: 20px; padding: 12px; font-size: 0.8rem; font-weight: 600; text-align: center; margin-top: 16px; min-height: 48px; display: flex; align-items: center; justify-content: center; }
</style>
