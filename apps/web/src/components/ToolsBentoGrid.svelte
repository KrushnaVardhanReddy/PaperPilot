<script lang="ts">
  import {
    ALL_101_TOOLS,
    TOOLS_CATEGORIES,
    selectToolAndScroll,
    type CatalogTool
  } from '../lib/toolsCatalog';

  let { onOpenDesktopModal }: { onOpenDesktopModal: () => void } = $props();

  let selectedCategory = $state<string>('all');
  let wasmOnly = $state(false);
  let searchQuery = $state('');

  const filteredTools = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    return ALL_101_TOOLS.filter((tool) => {
      if (selectedCategory !== 'all' && tool.category !== selectedCategory) {
        return false;
      }
      if (wasmOnly && tool.runtime !== 'wasm') {
        return false;
      }
      if (q) {
        const hay = `${tool.name} ${tool.description} ${tool.categoryLabel}`.toLowerCase();
        if (!hay.includes(q)) return false;
      }
      return true;
    });
  });

  function handleToolClick(tool: CatalogTool) {
    if (tool.runtime === 'desktop') {
      onOpenDesktopModal();
    } else {
      selectToolAndScroll(tool.id, tool.wasmToolId);
    }
  }
</script>

<section class="bento-section" id="tools-bento-grid" aria-label="All PDF Tools">
  <div class="bento-container">
    <div class="bento-header">
      <div class="header-text">
        <h2 class="section-title">All 101+ PDF Tools</h2>
        <p class="section-subtitle">
          Pick any tool to launch instantly in your browser via local WebAssembly, or run heavy batch jobs on Desktop.
        </p>
      </div>
      <div class="search-wrap">
        <span class="search-icon" aria-hidden="true">🔍</span>
        <input
          type="search"
          class="search-input"
          placeholder="Search tools (e.g. merge, ocr, redact)..."
          bind:value={searchQuery}
          aria-label="Search PDF tools"
        />
      </div>
    </div>

    <div class="filter-bar">
      <div class="category-tabs" role="tablist" aria-label="Tool categories">
        <button
          type="button"
          role="tab"
          aria-selected={selectedCategory === 'all'}
          class="tab-pill"
          class:active={selectedCategory === 'all'}
          onclick={() => (selectedCategory = 'all')}
        >
          All
          <span class="tab-count">{ALL_101_TOOLS.length}</span>
        </button>

        {#each TOOLS_CATEGORIES as cat (cat.id)}
          <button
            type="button"
            role="tab"
            aria-selected={selectedCategory === cat.id}
            class="tab-pill"
            class:active={selectedCategory === cat.id}
            onclick={() => (selectedCategory = cat.id)}
          >
            <span class="tab-icon">{cat.icon}</span>
            {cat.label}
          </button>
        {/each}
      </div>

      <button
        type="button"
        class="wasm-toggle"
        class:active={wasmOnly}
        aria-pressed={wasmOnly}
        onclick={() => (wasmOnly = !wasmOnly)}
      >
        <span>⚡ WASM Only</span>
      </button>
    </div>

    {#if filteredTools.length === 0}
      <div class="empty-state">
        <p class="empty-title">No tools match "{searchQuery}"</p>
        <button
          type="button"
          class="reset-btn"
          onclick={() => {
            searchQuery = '';
            selectedCategory = 'all';
            wasmOnly = false;
          }}
        >
          Reset filters
        </button>
      </div>
    {:else}
      <div class="bento-grid">
        {#each filteredTools as tool (tool.id)}
          {@const isWasm = tool.runtime !== 'desktop'}
          <button
            type="button"
            class="bento-card"
            onclick={() => handleToolClick(tool)}
          >
            <div class="card-top">
              <span class="card-icon" aria-hidden="true">{tool.icon}</span>
              <span
                class="runtime-badge"
                class:wasm={isWasm}
                class:desktop={!isWasm}
              >
                {isWasm ? 'WASM' : 'Desktop'}
              </span>
            </div>
            <h3 class="card-title">{tool.name}</h3>
            <p class="card-desc">{tool.description}</p>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</section>

<style>
  .bento-section {
    padding: 56px 24px 64px;
    background-color: var(--bg-primary);
    border-top: 1px solid var(--border-color);
  }

  .bento-container {
    max-width: 1200px;
    margin: 0 auto;
  }

  .bento-header {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-end;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 24px;
  }

  .section-title {
    margin: 0 0 6px;
    font-size: 1.75rem;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }

  .section-subtitle {
    margin: 0;
    font-size: 0.95rem;
    color: var(--text-secondary);
    max-width: 600px;
  }

  .search-wrap {
    position: relative;
    flex: 1 1 280px;
    max-width: 340px;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 0.9rem;
    pointer-events: none;
    opacity: 0.7;
  }

  .search-input {
    width: 100%;
    box-sizing: border-box;
    padding: 10px 14px 10px 36px;
    border-radius: 10px;
    border: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 0.9rem;
    outline: none;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .search-input:focus {
    border-color: #4f46e5;
    box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.15);
  }

  .filter-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 28px;
  }

  .category-tabs {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .tab-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 7px 14px;
    border-radius: 9999px;
    border: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.18s ease;
  }

  .tab-pill:hover {
    color: var(--text-primary);
    border-color: var(--text-muted);
  }

  .tab-pill.active {
    background-color: var(--text-primary);
    color: var(--bg-primary);
    border-color: var(--text-primary);
  }

  .tab-count {
    font-size: 0.75rem;
    opacity: 0.75;
  }

  .wasm-toggle {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 7px 14px;
    border-radius: 9999px;
    border: 1px solid rgba(16, 185, 129, 0.35);
    background-color: rgba(16, 185, 129, 0.08);
    color: #10b981;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.18s ease;
  }

  .wasm-toggle:hover {
    background-color: rgba(16, 185, 129, 0.16);
  }

  .wasm-toggle.active {
    background-color: #10b981;
    color: #06281e;
    border-color: #10b981;
  }

  .bento-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(230px, 1fr));
    gap: 14px;
  }

  .bento-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    text-align: left;
    padding: 16px;
    border-radius: 14px;
    border: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    cursor: pointer;
    transition: transform 0.18s ease, border-color 0.18s ease, box-shadow 0.18s ease;
  }

  .bento-card:hover {
    transform: translateY(-2px);
    border-color: rgba(99, 102, 241, 0.45);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.12);
  }

  .card-top {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 10px;
  }

  .card-icon {
    font-size: 1.5rem;
    line-height: 1;
  }

  .runtime-badge {
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.03em;
    padding: 3px 8px;
    border-radius: 9999px;
    text-transform: uppercase;
  }

  .runtime-badge.wasm {
    background-color: rgba(16, 185, 129, 0.14);
    color: #10b981;
    border: 1px solid rgba(16, 185, 129, 0.3);
  }

  .runtime-badge.desktop {
    background-color: rgba(99, 102, 241, 0.14);
    color: #6366f1;
    border: 1px solid rgba(99, 102, 241, 0.3);
  }

  .card-title {
    margin: 0 0 6px;
    font-size: 0.98rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .card-desc {
    margin: 0;
    font-size: 0.82rem;
    line-height: 1.45;
    color: var(--text-secondary);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .empty-state {
    text-align: center;
    padding: 48px 20px;
    border: 1px dashed var(--border-color);
    border-radius: 14px;
    color: var(--text-secondary);
  }

  .empty-title {
    margin: 0 0 12px;
    font-size: 1rem;
  }

  .reset-btn {
    padding: 8px 16px;
    border-radius: 8px;
    border: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 0.85rem;
    cursor: pointer;
  }
</style>
