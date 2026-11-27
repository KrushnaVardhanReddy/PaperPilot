<script lang="ts">
  import { onMount } from 'svelte';
  import { ALL_101_TOOLS, selectToolAndScroll, type CatalogTool } from '../lib/toolsCatalog';

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen = $bindable(false), onClose }: Props = $props();

  let searchQuery = $state('');
  let selectedIndex = $state(0);
  let searchInputRef: HTMLInputElement | null = $state(null);

  let filteredTools = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) {
      return ALL_101_TOOLS.slice(0, 15);
    }
    return ALL_101_TOOLS.filter((tool: CatalogTool) => {
      return (
        tool.name.toLowerCase().includes(q) ||
        tool.id.toLowerCase().includes(q) ||
        tool.description.toLowerCase().includes(q) ||
        tool.categoryLabel.toLowerCase().includes(q)
      );
    });
  });

  $effect(() => {
    if (isOpen) {
      searchQuery = '';
      selectedIndex = 0;
      setTimeout(() => {
        searchInputRef?.focus();
      }, 50);
    }
  });

  function handleKeyDown(e: KeyboardEvent) {
    if (!isOpen) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      onClose();
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (filteredTools.length > 0) {
        selectedIndex = (selectedIndex + 1) % filteredTools.length;
      }
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (filteredTools.length > 0) {
        selectedIndex = (selectedIndex - 1 + filteredTools.length) % filteredTools.length;
      }
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const target = filteredTools[selectedIndex];
      if (target) {
        handleSelect(target);
      }
    }
  }

  function handleSelect(tool: CatalogTool) {
    selectToolAndScroll(tool.id, tool.wasmToolId);
    onClose();
  }

  onMount(() => {
    const globalKeyHandler = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        isOpen = !isOpen;
      }
    };
    window.addEventListener('keydown', globalKeyHandler);
    return () => {
      window.removeEventListener('keydown', globalKeyHandler);
    };
  });
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if isOpen}
  <div class="finder-backdrop" onclick={onClose} role="presentation">
    <!-- svelte-ignore a11y_interactive_supports_focus -->
    <div
      class="finder-modal"
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-label="Tool Quick Finder"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="finder-header">
        <span class="search-icon">🔍</span>
        <input
          bind:this={searchInputRef}
          type="text"
          class="finder-input"
          placeholder="Search all 101 tools... (e.g. Merge, Edit Text, OCR, Compress, Watermark)"
          bind:value={searchQuery}
        />
        <span class="esc-badge" onclick={onClose} role="button" tabindex="0">ESC</span>
      </div>

      <div class="finder-body">
        {#if filteredTools.length === 0}
          <div class="no-results">
            <span class="empty-icon">📂</span>
            <p>No tools found matching "<strong>{searchQuery}</strong>"</p>
            <span class="hint">Try searching for "merge", "split", "word", or "edit"</span>
          </div>
        {:else}
          <div class="results-list" role="listbox">
            {#each filteredTools as tool, idx}
              <button
                type="button"
                class="result-item"
                class:active={idx === selectedIndex}
                role="option"
                aria-selected={idx === selectedIndex}
                onclick={() => handleSelect(tool)}
                onmouseenter={() => (selectedIndex = idx)}
              >
                <div class="tool-icon-wrapper">
                  <span class="tool-icon">{tool.icon}</span>
                </div>
                <div class="tool-details">
                  <div class="tool-title-row">
                    <span class="tool-name">{tool.name}</span>
                    {#if tool.runtime === 'desktop'}
                      <span class="runtime-badge desktop">Desktop</span>
                    {:else}
                      <span class="runtime-badge wasm">WASM</span>
                    {/if}
                    {#if tool.isPopular}
                      <span class="pop-pill">Popular</span>
                    {/if}
                    <span class="category-tag">{tool.categoryLabel}</span>
                  </div>
                  <p class="tool-desc">{tool.description}</p>
                </div>
                <div class="tool-action-hint">
                  <span class="run-arrow">↵</span>
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <div class="finder-footer">
        <div class="footer-tips">
          <span><kbd>↑</kbd> <kbd>↓</kbd> to navigate</span>
          <span><kbd>↵</kbd> to select</span>
          <span><kbd>ESC</kbd> to close</span>
        </div>
        <div class="footer-brand">
          <span>⚡ PaperPilot WASM Engine</span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .finder-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(8px);
    z-index: 9999;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 10vh;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .finder-modal {
    width: 90%;
    max-width: 680px;
    background: var(--bg-secondary, #18181b);
    border: 1px solid var(--border-color, #27272a);
    border-radius: 16px;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    outline: none;
    max-height: 80vh;
  }

  .finder-header {
    display: flex;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color, #27272a);
    gap: 12px;
    background: var(--bg-primary, #09090b);
  }

  .search-icon {
    font-size: 1.25rem;
    color: var(--text-secondary, #a1a1aa);
  }

  .finder-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    font-size: 1.1rem;
    color: var(--text-primary, #f4f4f5);
    font-family: inherit;
  }

  .finder-input::placeholder {
    color: var(--text-secondary, #71717a);
    font-size: 0.95rem;
  }

  .esc-badge {
    padding: 4px 8px;
    font-size: 0.75rem;
    font-weight: 600;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 6px;
    color: var(--text-secondary, #a1a1aa);
    cursor: pointer;
  }

  .esc-badge:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .finder-body {
    overflow-y: auto;
    max-height: 480px;
    padding: 8px;
  }

  .results-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .result-item {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 10px 14px;
    border-radius: 10px;
    background: transparent;
    border: 1px solid transparent;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s ease;
    color: inherit;
    width: 100%;
  }

  .result-item.active,
  .result-item:hover {
    background: rgba(94, 106, 210, 0.15);
    border-color: rgba(94, 106, 210, 0.3);
  }

  .tool-icon-wrapper {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .tool-details {
    flex: 1;
    min-width: 0;
  }

  .tool-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 2px;
  }

  .tool-name {
    font-weight: 600;
    font-size: 0.95rem;
    color: var(--text-primary, #f4f4f5);
  }

  .pop-pill {
    font-size: 0.68rem;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 10px;
    background: rgba(236, 72, 153, 0.2);
    color: #f472b6;
    border: 1px solid rgba(236, 72, 153, 0.3);
    text-transform: uppercase;
  }

  .category-tag {
    font-size: 0.72rem;
    color: var(--text-secondary, #a1a1aa);
    background: rgba(255, 255, 255, 0.05);
    padding: 2px 6px;
    border-radius: 4px;
    margin-left: auto;
  }

  .tool-desc {
    margin: 0;
    font-size: 0.8rem;
    color: var(--text-secondary, #a1a1aa);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tool-action-hint {
    padding-left: 8px;
    color: var(--text-secondary, #71717a);
    font-size: 1.1rem;
    opacity: 0;
    transition: opacity 0.15s;
  }

  .result-item.active .tool-action-hint,
  .result-item:hover .tool-action-hint {
    opacity: 1;
    color: var(--accent-primary, #6366f1);
  }

  .no-results {
    padding: 48px 24px;
    text-align: center;
    color: var(--text-secondary, #a1a1aa);
  }

  .empty-icon {
    font-size: 2.5rem;
    display: block;
    margin-bottom: 12px;
  }

  .hint {
    display: block;
    margin-top: 8px;
    font-size: 0.85rem;
    color: var(--text-muted, #71717a);
  }

  .finder-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 20px;
    background: var(--bg-primary, #09090b);
    border-top: 1px solid var(--border-color, #27272a);
    font-size: 0.75rem;
    color: var(--text-secondary, #71717a);
  }

  .footer-tips {
    display: flex;
    gap: 12px;
  }

  .footer-tips kbd {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 4px;
    padding: 1px 5px;
    font-family: inherit;
    font-size: 0.7rem;
    color: var(--text-primary, #e4e4e7);
  }

  .footer-brand {
    font-weight: 500;
  }

  .runtime-badge {
    font-size: 0.65rem;
    font-weight: 700;
    padding: 1px 6px;
    border-radius: 4px;
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .runtime-badge.wasm {
    background: rgba(16, 185, 129, 0.12);
    color: #34d399;
    border: 1px solid rgba(16, 185, 129, 0.28);
  }

  .runtime-badge.desktop {
    background: rgba(99, 102, 241, 0.12);
    color: #818cf8;
    border: 1px solid rgba(99, 102, 241, 0.28);
  }
</style>
