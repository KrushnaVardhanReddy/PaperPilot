<script lang="ts">
  import EmbedGenerator from './EmbedGenerator.svelte';
  import ApiExplorer from './ApiExplorer.svelte';
  import BenchmarkMatrix from './BenchmarkMatrix.svelte';

  type TabId = 'embed' | 'api' | 'benchmarks';

  let activeTab = $state<TabId>('embed');

  const tabs: { id: TabId; label: string }[] = [
    { id: 'embed', label: '</> Embed Widget' },
    { id: 'api', label: '⚡ REST & MCP API' },
    { id: 'benchmarks', label: '📊 Benchmarks' }
  ];
</script>

<section class="developer-tabs-section" aria-label="Developer tools and benchmarks">
  <div class="tabs-header">
    <div class="tabs-bar" role="tablist" aria-label="Developer sections">
      {#each tabs as tab}
        <button
          type="button"
          role="tab"
          aria-selected={activeTab === tab.id}
          class="tab-btn"
          class:active={activeTab === tab.id}
          onclick={() => (activeTab = tab.id)}
        >
          {tab.label}
        </button>
      {/each}
    </div>
  </div>

  <div class="tabs-panel" role="tabpanel">
    {#if activeTab === 'embed'}
      <EmbedGenerator />
    {:else if activeTab === 'api'}
      <ApiExplorer />
    {:else if activeTab === 'benchmarks'}
      <BenchmarkMatrix />
    {/if}
  </div>
</section>

<style>
  .developer-tabs-section {
    width: 100%;
    padding-top: 24px;
  }

  .tabs-header {
    display: flex;
    justify-content: center;
    padding: 0 24px;
    margin-bottom: 8px;
  }

  .tabs-bar {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 6px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 12px;
  }

  .tab-btn {
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary);
    font-family: inherit;
    font-size: 0.9rem;
    font-weight: 500;
    padding: 8px 16px;
    border-radius: 8px;
    cursor: pointer;
    transition: color 0.15s ease, background-color 0.15s ease, border-color 0.15s ease;
  }

  .tab-btn:hover {
    color: var(--text-primary);
    background-color: rgba(255, 255, 255, 0.04);
  }

  .tab-btn.active {
    color: var(--text-primary);
    background-color: rgba(var(--accent-primary-rgb, 99, 102, 241), 0.12);
    border-color: var(--accent-primary);
  }

  .tabs-panel {
    width: 100%;
  }
</style>
