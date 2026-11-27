<script lang="ts">
  import {
    ALL_101_TOOLS,
    TOOLS_CATEGORIES,
    selectToolAndScroll,
    type CatalogTool
  } from '../lib/toolsCatalog';

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen = $bindable(false), onClose }: Props = $props();

  let activeCategory = $state<'all' | 'organize' | 'edit' | 'convert' | 'optimize' | 'intelligence'>('all');
  let filterText = $state('');

  let filteredTools = $derived.by(() => {
    let list = ALL_101_TOOLS;
    if (activeCategory !== 'all') {
      list = list.filter((t: CatalogTool) => t.category === activeCategory);
    }
    const q = filterText.trim().toLowerCase();
    if (q) {
      list = list.filter((t: CatalogTool) =>
        t.name.toLowerCase().includes(q) ||
        t.id.toLowerCase().includes(q) ||
        t.description.toLowerCase().includes(q)
      );
    }
    return list;
  });

  function handleSelect(tool: CatalogTool) {
    selectToolAndScroll(tool.id, tool.wasmToolId);
    onClose();
  }

  function getCategoryCount(catId: string) {
    return ALL_101_TOOLS.filter((t: CatalogTool) => t.category === catId).length;
  }
</script>

{#if isOpen}
  <div class="megamenu-backdrop" onclick={onClose} role="presentation"></div>
  <div class="megamenu-container" role="dialog" aria-label="Tools Mega Menu">
    <div class="megamenu-header">
      <div class="header-left">
        <h3 class="megamenu-title">
          <span>Explore All 101 Pure-Rust Tools</span>
          <span class="count-pill">101 Tools</span>
        </h3>
        <p class="megamenu-sub">Zero Cloud Uploads • 100% Client-Side WebAssembly Execution</p>
      </div>

      <div class="header-search">
        <span class="search-icon">🔍</span>
        <input
          type="text"
          class="menu-search-input"
          placeholder="Filter tools by name or task..."
          bind:value={filterText}
        />
        {#if filterText}
          <button class="clear-btn" onclick={() => filterText = ''}>✕</button>
        {/if}
      </div>
    </div>

    <!-- Category Tabs Filter -->
    <div class="category-tabs" role="tablist">
      <button
        type="button"
        class="tab-btn"
        class:active={activeCategory === 'all'}
        onclick={() => activeCategory = 'all'}
      >
        <span class="tab-icon">🌟</span>
        <span class="tab-label">All Tools</span>
        <span class="tab-count">{ALL_101_TOOLS.length}</span>
      </button>

      {#each TOOLS_CATEGORIES as cat}
        <button
          type="button"
          class="tab-btn"
          class:active={activeCategory === cat.id}
          onclick={() => activeCategory = cat.id}
        >
          <span class="tab-icon">{cat.icon}</span>
          <span class="tab-label">{cat.label}</span>
          <span class="tab-count">{getCategoryCount(cat.id)}</span>
        </button>
      {/each}
    </div>

    <!-- Mega Menu Grid Body -->
    <div class="megamenu-body">
      {#if activeCategory === 'all' && !filterText}
        <!-- Multi-Column Category View -->
        <div class="categories-grid">
          {#each TOOLS_CATEGORIES as cat}
            {@const catTools = ALL_101_TOOLS.filter(t => t.category === cat.id)}
            <div class="category-column">
              <div class="column-header">
                <span class="col-icon">{cat.icon}</span>
                <h4 class="col-title">{cat.label}</h4>
                <span class="col-count">{catTools.length}</span>
              </div>
              <div class="tools-list">
                {#each catTools as tool}
                  <button
                    type="button"
                    class="tool-card"
                    onclick={() => handleSelect(tool)}
                  >
                    <span class="tool-icon">{tool.icon}</span>
                    <div class="tool-info">
                      <div class="tool-name-row">
                        <span class="tool-title">{tool.name}</span>
                        {#if tool.isPopular}
                          <span class="star-badge">Hot</span>
                        {/if}
                      </div>
                      <span class="tool-snip">{tool.description}</span>
                    </div>
                  </button>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <!-- Filtered Grid View -->
        <div class="tools-flat-grid">
          {#each filteredTools as tool}
            <button
              type="button"
              class="tool-card flat-card"
              onclick={() => handleSelect(tool)}
            >
              <div class="flat-icon-wrap">
                <span class="tool-icon">{tool.icon}</span>
              </div>
              <div class="tool-info">
                <div class="tool-name-row">
                  <span class="tool-title">{tool.name}</span>
                  {#if tool.isPopular}
                    <span class="star-badge">Hot</span>
                  {/if}
                  <span class="cat-pill">{tool.categoryLabel}</span>
                </div>
                <span class="tool-snip">{tool.description}</span>
              </div>
            </button>
          {/each}
          {#if filteredTools.length === 0}
            <div class="empty-state">
              <span>🔍 No tools found matching "{filterText}"</span>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Mega Menu Footer -->
    <div class="megamenu-footer">
      <div class="footer-trust">
        <span class="pulse-dot">🟢</span>
        <span>PaperPilot executes entirely in your local browser sandbox via WebAssembly.</span>
      </div>
      <div class="footer-actions">
        <span class="quick-tip">Tip: Press <kbd>Cmd+K</kbd> / <kbd>Ctrl+K</kbd> to open quick finder</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .megamenu-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    z-index: 1000;
  }

  .megamenu-container {
    position: absolute;
    top: 72px;
    left: 50%;
    transform: translateX(-50%);
    width: 95%;
    max-width: 1440px;
    background: var(--bg-secondary, #121316);
    border: 1px solid var(--border-color, #27272a);
    border-radius: 16px;
    box-shadow: 0 25px 60px -15px rgba(0, 0, 0, 0.8), 0 0 0 1px rgba(255, 255, 255, 0.05);
    z-index: 1001;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: menuSlideDown 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes menuSlideDown {
    from {
      opacity: 0;
      transform: translate(-50%, -10px);
    }
    to {
      opacity: 1;
      transform: translate(-50%, 0);
    }
  }

  .megamenu-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 28px;
    border-bottom: 1px solid var(--border-color, #27272a);
    background: var(--bg-primary, #09090b);
    gap: 24px;
    flex-wrap: wrap;
  }

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .megamenu-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 700;
    display: flex;
    align-items: center;
    gap: 12px;
    color: var(--text-primary, #f4f4f5);
  }

  .count-pill {
    background: rgba(94, 106, 210, 0.2);
    color: var(--accent-primary, #6366f1);
    font-size: 0.75rem;
    font-weight: 700;
    padding: 3px 10px;
    border-radius: 12px;
    border: 1px solid rgba(94, 106, 210, 0.3);
  }

  .megamenu-sub {
    margin: 0;
    font-size: 0.85rem;
    color: var(--text-secondary, #a1a1aa);
  }

  .header-search {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--bg-secondary, #18181b);
    border: 1px solid var(--border-color, #27272a);
    border-radius: 10px;
    padding: 8px 14px;
    min-width: 280px;
    position: relative;
  }

  .search-icon {
    font-size: 0.95rem;
    color: var(--text-secondary, #71717a);
  }

  .menu-search-input {
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary, #f4f4f5);
    font-size: 0.9rem;
    width: 100%;
  }

  .clear-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary, #71717a);
    cursor: pointer;
    font-size: 0.85rem;
  }

  .category-tabs {
    display: flex;
    gap: 8px;
    padding: 12px 28px;
    background: var(--bg-secondary, #18181b);
    border-bottom: 1px solid var(--border-color, #27272a);
    overflow-x: auto;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    border-radius: 8px;
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary, #a1a1aa);
    font-size: 0.88rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .tab-btn:hover {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-primary, #f4f4f5);
  }

  .tab-btn.active {
    background: rgba(94, 106, 210, 0.15);
    border-color: rgba(94, 106, 210, 0.4);
    color: #a5b4fc;
    font-weight: 600;
  }

  .tab-count {
    background: rgba(255, 255, 255, 0.08);
    font-size: 0.72rem;
    padding: 1px 6px;
    border-radius: 10px;
    font-weight: 600;
  }

  .megamenu-body {
    flex: 1;
    overflow-y: auto;
    padding: 24px 28px;
  }

  .categories-grid {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 20px;
  }

  @media (max-width: 1200px) {
    .categories-grid {
      grid-template-columns: repeat(3, 1fr);
    }
  }

  @media (max-width: 768px) {
    .categories-grid {
      grid-template-columns: 1fr;
    }
  }

  .category-column {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .column-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding-bottom: 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .col-icon {
    font-size: 1.1rem;
  }

  .col-title {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 700;
    color: var(--text-primary, #f4f4f5);
    flex: 1;
  }

  .col-count {
    font-size: 0.75rem;
    color: var(--text-secondary, #71717a);
    font-weight: 600;
  }

  .tools-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .tool-card {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 8px;
    background: transparent;
    border: 1px solid transparent;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s ease;
    color: inherit;
    width: 100%;
  }

  .tool-card:hover {
    background: rgba(255, 255, 255, 0.06);
    border-color: rgba(255, 255, 255, 0.1);
    transform: translateX(2px);
  }

  .tool-icon {
    font-size: 1.15rem;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .tool-info {
    flex: 1;
    min-width: 0;
  }

  .tool-name-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 2px;
  }

  .tool-title {
    font-size: 0.88rem;
    font-weight: 600;
    color: var(--text-primary, #f4f4f5);
    line-height: 1.2;
  }

  .star-badge {
    background: rgba(245, 158, 11, 0.2);
    color: #fbbf24;
    font-size: 0.65rem;
    font-weight: 700;
    padding: 1px 5px;
    border-radius: 4px;
    text-transform: uppercase;
  }

  .cat-pill {
    margin-left: auto;
    font-size: 0.7rem;
    color: var(--text-secondary, #a1a1aa);
    background: rgba(255, 255, 255, 0.06);
    padding: 1px 6px;
    border-radius: 4px;
  }

  .tool-snip {
    display: block;
    font-size: 0.75rem;
    color: var(--text-secondary, #a1a1aa);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.3;
  }

  .tools-flat-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  @media (max-width: 900px) {
    .tools-flat-grid {
      grid-template-columns: 1fr;
    }
  }

  .flat-card {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.06);
    padding: 12px;
  }

  .flat-card:hover {
    background: rgba(94, 106, 210, 0.12);
    border-color: rgba(94, 106, 210, 0.3);
  }

  .flat-icon-wrap {
    width: 32px;
    height: 32px;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .empty-state {
    grid-column: 1 / -1;
    padding: 48px;
    text-align: center;
    color: var(--text-secondary, #a1a1aa);
  }

  .megamenu-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 28px;
    background: var(--bg-primary, #09090b);
    border-top: 1px solid var(--border-color, #27272a);
    font-size: 0.82rem;
    color: var(--text-secondary, #a1a1aa);
    flex-wrap: wrap;
    gap: 12px;
  }

  .footer-trust {
    display: flex;
    align-items: center;
    gap: 8px;
    color: #10b981;
    font-weight: 500;
  }

  .pulse-dot {
    font-size: 0.75rem;
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0% { opacity: 0.5; }
    50% { opacity: 1; }
    100% { opacity: 0.5; }
  }

  .quick-tip kbd {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 0.75rem;
    color: var(--text-primary, #f4f4f5);
  }
</style>
