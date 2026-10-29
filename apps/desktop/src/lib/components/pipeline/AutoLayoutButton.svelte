<script lang="ts">
  import { useSvelteFlow } from '@xyflow/svelte';
  import { pipelineState } from '$lib/state/pipeline.svelte';

  const { fitView } = useSvelteFlow();

  function handleAutoLayout() {
    pipelineState.autoLayout();
    setTimeout(() => {
      fitView({ padding: 0.25, duration: 400 });
    }, 50);
  }
</script>

<button
  class="auto-layout-btn"
  onclick={handleAutoLayout}
  title="Auto-format and organize canvas layout"
>
  <span class="icon">🪄</span>
  <span class="text">Auto Format</span>
</button>

<style>
  .auto-layout-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: var(--bg-surface, #1e293b);
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-sm, 6px);
    color: var(--text-primary, #f8fafc);
    font-size: 0.8rem;
    font-weight: 500;
    cursor: pointer;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    transition: all 0.15s ease;
    user-select: none;
  }

  .auto-layout-btn:hover {
    background: var(--bg-secondary, #334155);
    border-color: var(--accent-primary, #3b82f6);
    color: var(--accent-primary, #60a5fa);
    transform: translateY(-1px);
  }

  .icon {
    font-size: 0.95rem;
    line-height: 1;
  }

  .text {
    line-height: 1;
  }
</style>
