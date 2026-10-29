<script lang="ts">
  let { isScrolled = false, onOpenSheet } = $props<{ isScrolled?: boolean; onOpenSheet: () => void }>();
</script>

<div class="fab-container">
  <button class="action-pill" class:collapsed={isScrolled} onclick={onOpenSheet}>
    {#if isScrolled}
      <span class="icon pulse">✨</span>
    {:else}
      <span class="icon">✨</span>
      <span class="text">AI Suggestion: Redact PII (Contract)</span>
      <span class="action">Apply</span>
    {/if}
  </button>
</div>

<style>
  .fab-container { position: absolute; bottom: 80px; left: 50%; transform: translateX(-50%); z-index: 60; }
  .action-pill { display: flex; align-items: center; gap: 8px; background: var(--glass-bg); backdrop-filter: var(--glass-blur); -webkit-backdrop-filter: var(--glass-blur); border: 1px solid rgba(255,255,255,0.15); border-radius: 40px; padding: 12px 20px; color: var(--text-primary); box-shadow: 0 4px 20px rgba(0,0,0,0.5); transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); cursor: pointer; overflow: hidden; white-space: nowrap; min-height: 48px; }
  .action-pill.collapsed { padding: 12px; width: 48px; border-radius: 24px; justify-content: center; }
  .text { font-size: 0.85rem; font-weight: 500; }
  .action { background: var(--accent-primary); color: white; padding: 4px 10px; border-radius: 12px; font-size: 0.75rem; font-weight: 600; margin-left: 8px; }
  .pulse { animation: glow 2s infinite; }
  @keyframes glow { 0% { filter: drop-shadow(0 0 2px var(--accent-primary)); } 50% { filter: drop-shadow(0 0 8px var(--accent-primary)); } 100% { filter: drop-shadow(0 0 2px var(--accent-primary)); } }
</style>
