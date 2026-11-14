<script lang="ts">
  import { onMount } from 'svelte';

  export interface BookmarkItem {
    id: string;
    title: string;
    bold?: boolean;
    italic?: boolean;
    color?: Uint8ClampedArray;
    dest: string | any[] | null;
    url?: string | null;
    items?: BookmarkItem[];
  }

  let {
    pdfDoc,
    pageNum = $bindable(1)
  } = $props<{
    pdfDoc: any;
    pageNum?: number;
  }>();

  let rawOutline = $state<BookmarkItem[]>([]);
  let isLoading = $state(false);
  let searchQuery = $state('');
  let expandedMap = $state<Record<string, boolean>>({});
  let activeItemId = $state<string | null>(null);

  // Generate unique IDs for items recursively so we can track expansion state reliably
  function assignIds(items: any[], prefix = 'bm'): BookmarkItem[] {
    return items.map((item, index) => {
      const id = `${prefix}-${index}`;
      return {
        ...item,
        id,
        items: item.items && item.items.length > 0 ? assignIds(item.items, id) : []
      };
    });
  }

  // Load outline when pdfDoc changes
  $effect(() => {
    const doc = pdfDoc;
    if (!doc) {
      rawOutline = [];
      expandedMap = {};
      return;
    }

    let isCancelled = false;
    isLoading = true;

    doc.getOutline().then((outline: any) => {
      if (isCancelled) return;
      if (Array.isArray(outline) && outline.length > 0) {
        const assigned = assignIds(outline);
        rawOutline = assigned;
        // Default top-level items to expanded
        const initialMap: Record<string, boolean> = {};
        for (const item of assigned) {
          initialMap[item.id] = true;
        }
        expandedMap = initialMap;
      } else {
        rawOutline = [];
        expandedMap = {};
      }
      isLoading = false;
    }).catch((err: any) => {
      if (isCancelled) return;
      console.warn('Failed to load PDF outline:', err);
      rawOutline = [];
      expandedMap = {};
      isLoading = false;
    });

    return () => {
      isCancelled = true;
    };
  });

  // Filter items recursively based on search query
  function filterOutline(items: BookmarkItem[], query: string): BookmarkItem[] {
    if (!query.trim()) return items;
    const lowerQuery = query.toLowerCase();

    const filtered: BookmarkItem[] = [];
    for (const item of items) {
      const matchesSelf = (item.title || '').toLowerCase().includes(lowerQuery);
      const childMatches = item.items && item.items.length > 0 ? filterOutline(item.items, query) : [];

      if (matchesSelf || childMatches.length > 0) {
        filtered.push({
          ...item,
          items: childMatches
        });
      }
    }
    return filtered;
  }

  let filteredOutline = $derived(filterOutline(rawOutline, searchQuery));

  function toggleExpand(id: string, e?: MouseEvent) {
    if (e) e.stopPropagation();
    expandedMap[id] = !expandedMap[id];
  }

  function setAllExpanded(items: BookmarkItem[], expand: boolean) {
    const nextMap = { ...expandedMap };
    function traverse(list: BookmarkItem[]) {
      for (const item of list) {
        nextMap[item.id] = expand;
        if (item.items && item.items.length > 0) {
          traverse(item.items);
        }
      }
    }
    traverse(items);
    expandedMap = nextMap;
  }

  function expandAll() {
    setAllExpanded(rawOutline, true);
  }

  function collapseAll() {
    setAllExpanded(rawOutline, false);
  }

  async function handleItemClick(item: BookmarkItem) {
    activeItemId = item.id;
    if (!pdfDoc) return;

    try {
      let dest = item.dest;
      // If destination is a string identifier, fetch the explicit destination array
      if (typeof dest === 'string') {
        dest = await pdfDoc.getDestination(dest);
      }

      if (Array.isArray(dest) && dest.length > 0) {
        const destRef = dest[0];
        let targetIndex: number;
        if (typeof destRef === 'object' && destRef !== null) {
          targetIndex = await pdfDoc.getPageIndex(destRef);
        } else if (typeof destRef === 'number') {
          targetIndex = destRef;
        } else {
          return;
        }

        if (typeof targetIndex === 'number' && targetIndex >= 0) {
          pageNum = targetIndex + 1;
        }
      } else if (item.url) {
        // If it's an external web link, open it safely in default browser
        window.open(item.url, '_blank', 'noopener,noreferrer');
      }
    } catch (err) {
      console.warn('Failed to navigate to bookmark destination:', err);
    }
  }

  function formatBookmarkColor(color?: Uint8ClampedArray): string | undefined {
    if (!color || color.length < 3) return undefined;
    return `rgb(${color[0]}, ${color[1]}, ${color[2]})`;
  }
</script>

<div class="bookmarks-panel">
  <!-- Search & Action Toolbar -->
  <div class="panel-toolbar">
    <div class="search-input-wrapper">
      <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <input
        type="text"
        placeholder="Search bookmarks..."
        bind:value={searchQuery}
        class="search-input"
      />
      {#if searchQuery}
        <button
          class="clear-search-btn"
          onclick={() => (searchQuery = '')}
          title="Clear search"
          aria-label="Clear search"
        >
          ×
        </button>
      {/if}
    </div>

    <div class="toolbar-actions">
      <button
        class="action-btn"
        onclick={expandAll}
        title="Expand All"
        aria-label="Expand All"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="7 13 12 18 17 13"></polyline>
          <polyline points="7 6 12 11 17 6"></polyline>
        </svg>
        <span>Expand</span>
      </button>
      <button
        class="action-btn"
        onclick={collapseAll}
        title="Collapse All"
        aria-label="Collapse All"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="17 11 12 6 7 11"></polyline>
          <polyline points="17 18 12 13 7 18"></polyline>
        </svg>
        <span>Collapse</span>
      </button>
    </div>
  </div>

  <!-- Bookmarks Content Tree -->
  <div class="panel-content">
    {#if isLoading}
      <div class="panel-status">
        <div class="status-spinner"></div>
        <span>Loading bookmarks...</span>
      </div>
    {:else if filteredOutline.length === 0}
      <div class="panel-empty">
        <div class="empty-icon">🔖</div>
        <p class="empty-text">
          {#if searchQuery}
            No bookmarks matching "{searchQuery}"
          {:else}
            No bookmarks/outline found in this document
          {/if}
        </p>
      </div>
    {:else}
      <div class="tree-container" role="tree" aria-label="Document Bookmarks">
        {#snippet renderTree(items: BookmarkItem[], depth: number)}
          {#each items as item (item.id)}
            {@const hasChildren = item.items && item.items.length > 0}
            {@const isExpanded = searchQuery.trim().length > 0 || !!expandedMap[item.id]}
            {@const isActive = activeItemId === item.id}
            {@const textColor = formatBookmarkColor(item.color)}

            <div
              class="tree-node"
              role="treeitem"
              aria-expanded={hasChildren ? isExpanded : undefined}
              aria-selected={isActive}
            >
              <div
                class="node-row"
                class:active={isActive}
                style="padding-left: {depth * 14 + 6}px;"
                onclick={() => handleItemClick(item)}
                onkeydown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    handleItemClick(item);
                  }
                }}
                tabindex="0"
                role="button"
              >
                <!-- Expand / Collapse chevron -->
                <span class="chevron-wrapper">
                  {#if hasChildren}
                    <button
                      class="chevron-btn"
                      onclick={(e) => toggleExpand(item.id, e)}
                      title={isExpanded ? 'Collapse' : 'Expand'}
                      aria-label={isExpanded ? 'Collapse' : 'Expand'}
                    >
                      <svg
                        class="chevron-icon"
                        class:expanded={isExpanded}
                        xmlns="http://www.w3.org/2000/svg"
                        width="11"
                        height="11"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                      >
                        <polyline points="9 18 15 12 9 6"></polyline>
                      </svg>
                    </button>
                  {:else}
                    <span class="chevron-placeholder"></span>
                  {/if}
                </span>

                <!-- Bookmark Icon -->
                <svg
                  class="bookmark-icon"
                  xmlns="http://www.w3.org/2000/svg"
                  width="13"
                  height="13"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"></path>
                </svg>

                <!-- Bookmark Title -->
                <span
                  class="node-title"
                  class:bold={item.bold}
                  class:italic={item.italic}
                  style={textColor ? `color: ${textColor};` : ''}
                  title={item.title}
                >
                  {item.title || '(Untitled Bookmark)'}
                </span>
              </div>

              <!-- Nested Children -->
              {#if hasChildren && isExpanded && item.items}
                <div class="nested-tree" role="group">
                  {@render renderTree(item.items, depth + 1)}
                </div>
              {/if}
            </div>
          {/each}
        {/snippet}

        {@render renderTree(filteredOutline, 0)}
      </div>
    {/if}
  </div>
</div>

<style>
  .bookmarks-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    overflow: hidden;
    background-color: var(--bg-surface, #1e293b);
    color: var(--text-primary, #f8fafc);
  }

  .panel-toolbar {
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    border-bottom: 1px solid var(--border-color, #334155);
    background-color: var(--bg-surface, #1e293b);
    flex-shrink: 0;
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .search-icon {
    position: absolute;
    left: 9px;
    color: var(--text-muted, #94a3b8);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: 6px 26px 6px 28px;
    background: var(--bg-primary, #0f172a);
    border: 1px solid var(--border-color, #334155);
    border-radius: 6px;
    color: var(--text-primary, #f8fafc);
    font-size: 0.8rem;
    outline: none;
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
  }

  .search-input:focus {
    border-color: var(--accent-primary, #3b82f6);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }

  .search-input::placeholder {
    color: var(--text-muted, #64748b);
  }

  .clear-search-btn {
    position: absolute;
    right: 6px;
    background: transparent;
    border: none;
    color: var(--text-muted, #94a3b8);
    font-size: 1rem;
    cursor: pointer;
    padding: 0 4px;
    line-height: 1;
    border-radius: 4px;
  }

  .clear-search-btn:hover {
    color: var(--text-primary, #ffffff);
  }

  .toolbar-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .action-btn {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    background: var(--bg-primary, #0f172a);
    border: 1px solid var(--border-color, #334155);
    border-radius: 4px;
    color: var(--text-secondary, #cbd5e1);
    font-size: 0.73rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn:hover {
    background: var(--bg-surface-elevated, #334155);
    color: var(--text-primary, #ffffff);
    border-color: var(--text-muted, #64748b);
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 6px 0;
  }

  .panel-status {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 32px 16px;
    font-size: 0.85rem;
    color: var(--text-secondary, #94a3b8);
  }

  .status-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid var(--border-color, #334155);
    border-top-color: var(--accent-primary, #3b82f6);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .panel-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 40px 16px;
    color: var(--text-muted, #64748b);
  }

  .empty-icon {
    font-size: 1.8rem;
    margin-bottom: 8px;
    opacity: 0.6;
  }

  .empty-text {
    font-size: 0.82rem;
    line-height: 1.4;
    margin: 0;
  }

  .tree-container {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  .tree-node {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  .node-row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding-top: 4px;
    padding-bottom: 4px;
    padding-right: 10px;
    min-height: 28px;
    cursor: pointer;
    border-radius: 4px;
    margin: 1px 4px;
    user-select: none;
    transition: background-color 0.1s ease, color 0.1s ease;
    color: var(--text-secondary, #cbd5e1);
  }

  .node-row:hover {
    background-color: rgba(255, 255, 255, 0.05);
    color: var(--text-primary, #ffffff);
  }

  .node-row:focus-visible {
    outline: 2px solid var(--accent-primary, #3b82f6);
    outline-offset: -1px;
  }

  .node-row.active {
    background-color: rgba(59, 130, 246, 0.18);
    color: var(--accent-primary, #60a5fa);
    font-weight: 500;
  }

  .chevron-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }

  .chevron-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-muted, #94a3b8);
    cursor: pointer;
    padding: 0;
    width: 14px;
    height: 14px;
    border-radius: 3px;
    transition: transform 0.15s ease, color 0.15s ease;
  }

  .chevron-btn:hover {
    color: var(--text-primary, #ffffff);
    background-color: rgba(255, 255, 255, 0.1);
  }

  .chevron-icon {
    transition: transform 0.15s ease;
  }

  .chevron-icon.expanded {
    transform: rotate(90deg);
  }

  .chevron-placeholder {
    width: 14px;
    height: 14px;
  }

  .bookmark-icon {
    color: var(--text-muted, #94a3b8);
    flex-shrink: 0;
  }

  .node-row.active .bookmark-icon {
    color: var(--accent-primary, #60a5fa);
  }

  .node-title {
    font-size: 0.8rem;
    line-height: 1.3;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .node-title.bold {
    font-weight: 600;
  }

  .node-title.italic {
    font-style: italic;
  }

  .nested-tree {
    display: flex;
    flex-direction: column;
    width: 100%;
  }
</style>
