<script lang="ts">
  import { onMount } from 'svelte';

  let {
    isOpen = $bindable(false),
    onSelectPrompt,
    onClose
  }: {
    isOpen: boolean;
    onSelectPrompt: (promptText: string) => void;
    onClose: () => void;
  } = $props();

  let searchQuery = $state('');
  let searchInput = $state<HTMLInputElement | null>(null);

  const categories = [
    {
      title: "⚡ Quick Actions",
      items: [
        "Rotate 90 degrees clockwise",
        "Rotate 180 degrees upside down",
        "Compress this PDF",
        "Squish this document"
      ]
    },
    {
      title: "📑 Pages & Organization",
      items: [
        "Split pages 1 to 5",
        "Extract pages 2, 4, and 6",
        "Delete pages 3 and 7",
        "Reorder pages 3, 1, 2",
        "Burst into single page files",
        "Remove all blank pages",
        "Crop margins 10,10,200,200"
      ]
    },
    {
      title: "🔄 Conversions & Extraction",
      items: [
        "Convert to Word (docx)",
        "Convert to Excel spreadsheet",
        "Convert to Markdown",
        "Extract all plain text",
        "Extract embedded images",
        "Convert to PDF/A archive"
      ]
    },
    {
      title: "🔒 Security & Integrity",
      items: [
        "Encrypt with password mysecret123",
        "Remove password from PDF",
        "Redact confidential text",
        "Add digital signature",
        "Sanitize author metadata",
        "Validate PDF structure",
        "Generate SHA-256 hash"
      ]
    },
    {
      title: "✍️ Edit, Stamps & Forms",
      items: [
        "Add watermark CONFIDENTIAL",
        "Add Bates numbering prefix LAW-",
        "Add header Confidential and footer Page 1",
        "Add page numbers at bottom",
        "Flatten all annotations",
        "Read form field values"
      ]
    }
  ];

  let filteredCategories = $derived(
    categories.map(category => ({
      ...category,
      items: category.items.filter(item =>
        item.toLowerCase().includes(searchQuery.toLowerCase()) ||
        category.title.toLowerCase().includes(searchQuery.toLowerCase())
      )
    })).filter(category => category.items.length > 0)
  );

  $effect(() => {
    if (isOpen && searchInput) {
      setTimeout(() => {
        searchInput?.focus();
      }, 50);
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) {
      onClose();
    }
  }

  function handleSelect(prompt: string) {
    onSelectPrompt(prompt);
    onClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="drawer-backdrop" onclick={onClose}>
    <div class="drawer" onclick={(e) => e.stopPropagation()}>
      <div class="drawer-header">
        <h3>💡 Command Examples</h3>
        <button class="close-btn" onclick={onClose} aria-label="Close cheat sheet">✕</button>
      </div>

      <div class="search-container">
        <input
          type="text"
          bind:this={searchInput}
          bind:value={searchQuery}
          placeholder="Search examples (e.g. split, word, encrypt, bates)..."
          class="search-input"
        />
      </div>

      <div class="drawer-content">
        {#each filteredCategories as category}
          <div class="category">
            <h4>{category.title}</h4>
            <div class="items">
              {#each category.items as item}
                <div class="item">
                  <span class="prompt-text">"{item}"</span>
                  <button class="try-btn" onclick={() => handleSelect(item)}>
                    Try Prompt ↗
                  </button>
                </div>
              {/each}
            </div>
          </div>
        {/each}
        {#if filteredCategories.length === 0}
          <div class="no-results">
            No examples found matching "{searchQuery}"
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .drawer-backdrop {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    z-index: 100;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
  }

  .drawer {
    background: var(--bg-primary);
    border-top: 1px solid var(--border-color);
    box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.1);
    height: 70%;
    max-height: 500px;
    display: flex;
    flex-direction: column;
    animation: slideUp 0.2s ease-out;
  }

  @keyframes slideUp {
    from {
      transform: translateY(100%);
    }
    to {
      transform: translateY(0);
    }
  }

  .drawer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .drawer-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .close-btn {
    background: transparent;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: var(--text-muted);
    transition: color 0.2s;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .search-container {
    padding: 1rem;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-secondary);
  }

  .search-input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-family: inherit;
    box-sizing: border-box;
  }

  .search-input:focus {
    outline: 2px solid var(--accent-primary);
    border-color: transparent;
  }

  .drawer-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .category h4 {
    margin: 0 0 0.75rem 0;
    font-size: 0.95rem;
    color: var(--accent-primary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .items {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    gap: 1rem;
    transition: border-color 0.2s, background 0.2s;
  }

  .item:hover {
    border-color: var(--accent-primary);
    background: var(--bg-surface-hover);
  }

  .prompt-text {
    color: var(--text-primary);
    font-size: 0.9rem;
    word-break: break-word;
  }

  .try-btn {
    background: transparent;
    border: 1px solid var(--accent-primary);
    color: var(--accent-primary);
    padding: 0.4rem 0.75rem;
    border-radius: 4px;
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.2s;
  }

  .try-btn:hover {
    background: var(--accent-primary);
    color: white;
  }

  .no-results {
    text-align: center;
    padding: 2rem;
    color: var(--text-muted);
    font-style: italic;
  }
</style>
