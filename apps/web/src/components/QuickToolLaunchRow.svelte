<script lang="ts">
  import { POPULAR_HERO_TOOLS, selectToolAndScroll } from '../lib/toolsCatalog';

  interface Props {
    onOpenAllTools?: () => void;
  }

  let { onOpenAllTools }: Props = $props();

  function handleQuickTool(tool: typeof POPULAR_HERO_TOOLS[0]) {
    selectToolAndScroll(tool.id, tool.wasmId);
  }
</script>

<div class="quick-launch-container">
  <div class="quick-launch-title">
    <span class="lightning-icon">⚡</span>
    <span>Popular Instant Tools:</span>
  </div>

  <div class="tools-pills-row">
    {#each POPULAR_HERO_TOOLS as tool}
      <button
        type="button"
        class="quick-pill-btn"
        onclick={() => handleQuickTool(tool)}
      >
        <span class="pill-icon">{tool.icon}</span>
        <span class="pill-label">{tool.label}</span>
      </button>
    {/each}

    {#if onOpenAllTools}
      <button
        type="button"
        class="all-tools-pill-btn"
        onclick={onOpenAllTools}
      >
        <span>All 101 Tools ▾</span>
      </button>
    {/if}
  </div>
</div>

<style>
  .quick-launch-container {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 14px;
    margin-top: 24px;
    margin-bottom: 24px;
    flex-wrap: wrap;
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(6px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .quick-launch-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.88rem;
    font-weight: 600;
    color: var(--text-secondary, #a1a1aa);
  }

  .lightning-icon {
    color: #f59e0b;
    font-size: 0.95rem;
  }

  .tools-pills-row {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .quick-pill-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color, #27272a);
    padding: 8px 16px;
    border-radius: 20px;
    color: var(--text-primary, #f4f4f5);
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
  }

  .quick-pill-btn:hover {
    background: rgba(94, 106, 210, 0.2);
    border-color: rgba(94, 106, 210, 0.5);
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(94, 106, 210, 0.25);
    color: #ffffff;
  }

  .pill-icon {
    font-size: 1rem;
  }

  .all-tools-pill-btn {
    display: flex;
    align-items: center;
    background: transparent;
    border: 1px dashed var(--border-color, #3f3f46);
    padding: 8px 14px;
    border-radius: 20px;
    color: var(--text-secondary, #a1a1aa);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .all-tools-pill-btn:hover {
    border-color: var(--accent-primary, #6366f1);
    color: var(--accent-primary, #6366f1);
    background: rgba(99, 102, 241, 0.08);
  }
</style>
