<script lang="ts">
  import { onMount } from 'svelte';

  let {
    pdfDoc = null,
    pageNum = $bindable(1)
  }: {
    pdfDoc: any;
    pageNum: number;
  } = $props();

  let isOpen = $state(false);
  let searchQuery = $state('');
  let currentMatchIndex = $state(0);
  let matches = $state<{ page: number; text: string }[]>([]);
  let isSearching = $state(false);
  let inputEl: HTMLInputElement | null = $state(null);

  async function performSearch(query: string) {
    if (!pdfDoc || !query.trim()) {
      matches = [];
      currentMatchIndex = 0;
      return;
    }

    isSearching = true;
    const found: { page: number; text: string }[] = [];
    const q = query.toLowerCase();

    try {
      for (let i = 1; i <= pdfDoc.numPages; i++) {
        const page = await pdfDoc.getPage(i);
        const textContent = await page.getTextContent();
        const pageText = textContent.items
          .map((item: any) => item.str)
          .join(' ');

        if (pageText.toLowerCase().includes(q)) {
          found.push({ page: i, text: pageText });
        }
      }
      matches = found;
      currentMatchIndex = found.length > 0 ? 0 : 0;
      if (found.length > 0) {
        pageNum = found[0].page;
      }
    } catch (err) {
      console.warn('Search error:', err);
    } finally {
      isSearching = false;
    }
  }

  function nextMatch() {
    if (matches.length === 0) return;
    currentMatchIndex = (currentMatchIndex + 1) % matches.length;
    pageNum = matches[currentMatchIndex].page;
  }

  function prevMatch() {
    if (matches.length === 0) return;
    currentMatchIndex = (currentMatchIndex - 1 + matches.length) % matches.length;
    pageNum = matches[currentMatchIndex].page;
  }

  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      isOpen = true;
      setTimeout(() => inputEl?.focus(), 50);
      return;
    }

    if (!isOpen) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      isOpen = false;
    }
  }

  function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (e.shiftKey) prevMatch();
      else nextMatch();
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

{#if isOpen}
  <div class="search-bar" id="pdf-search-bar" role="search">
    <input
      bind:this={inputEl}
      id="pdf-search-input"
      class="search-input"
      placeholder="Find in document..."
      bind:value={searchQuery}
      oninput={() => performSearch(searchQuery)}
      onkeydown={handleInputKeydown}
    />

    <span class="match-count" id="pdf-search-count">
      {#if isSearching}
        ...
      {:else if matches.length > 0}
        {currentMatchIndex + 1} of {matches.length}
      {:else if searchQuery.trim()}
        0 of 0
      {:else}

      {/if}
    </span>

    <button
      class="search-btn"
      id="pdf-search-prev"
      onclick={prevMatch}
      disabled={matches.length <= 1}
      title="Previous match (Shift+Enter)"
      aria-label="Previous match"
    >
      ‹
    </button>

    <button
      class="search-btn"
      id="pdf-search-next"
      onclick={nextMatch}
      disabled={matches.length <= 1}
      title="Next match (Enter)"
      aria-label="Next match"
    >
      ›
    </button>

    <button
      class="close-btn"
      id="pdf-search-close"
      onclick={() => isOpen = false}
      title="Close (Esc)"
      aria-label="Close search"
    >
      ×
    </button>
  </div>
{/if}

<style>
  .search-bar {
    position: absolute;
    top: 16px;
    right: 24px;
    z-index: 40;
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-md, 6px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.35);
    padding: 6px 10px;
    backdrop-filter: blur(8px);
  }

  .search-input {
    width: 170px;
    background: transparent;
    border: none;
    color: var(--text-primary, #ffffff);
    font-size: 0.85rem;
    outline: none;
  }

  .match-count {
    font-size: 0.75rem;
    color: var(--text-muted, #9ca3af);
    min-width: 50px;
    text-align: right;
  }

  .search-btn, .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    background: transparent;
    border: none;
    border-radius: var(--border-radius-sm, 4px);
    color: var(--text-secondary, #9ca3af);
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .search-btn:hover:not(:disabled), .close-btn:hover {
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.1));
    color: var(--text-primary, #ffffff);
  }

  .search-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .close-btn {
    font-size: 1rem;
    margin-left: 2px;
  }
</style>
