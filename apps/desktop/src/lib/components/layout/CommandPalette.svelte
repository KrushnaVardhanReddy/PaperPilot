<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { onMount } from 'svelte';

  let isOpen = $state(false);
  let searchQuery = $state('');
  let selectedIndex = $state(0);
  let inputEl: HTMLInputElement | null = $state(null);

  interface CommandItem {
    id: string;
    title: string;
    subtitle: string;
    category: 'Operations' | 'Documents' | 'View';
    icon: string;
    action: () => void;
  }

  const staticCommands: CommandItem[] = [
    {
      id: 'cmd-open',
      title: 'Open File',
      subtitle: 'Open a PDF document',
      category: 'View',
      icon: '📂',
      action: async () => {
        close();
        window.dispatchEvent(new CustomEvent('paperpilot:open-file'));
      }
    },
    {
      id: 'cmd-merge',
      title: 'Merge PDFs',
      subtitle: 'Combine multiple PDF files into one',
      category: 'Operations',
      icon: '📑',
      action: () => {
        close();
        appState.selectDocument(null);
      }
    },
    {
      id: 'cmd-compress',
      title: 'Compress PDF',
      subtitle: 'Reduce PDF file size',
      category: 'Operations',
      icon: '🗜️',
      action: () => {
        close();
        appState.selectDocument(null);
      }
    },
    {
      id: 'cmd-rotate',
      title: 'Rotate Pages',
      subtitle: 'Rotate PDF pages by 90, 180, or 270 degrees',
      category: 'Operations',
      icon: '🔄',
      action: () => {
        close();
        appState.selectDocument(null);
      }
    },
    {
      id: 'cmd-watermark',
      title: 'Add Watermark',
      subtitle: 'Stamp text watermark onto PDF pages',
      category: 'Operations',
      icon: '💧',
      action: () => {
        close();
        appState.selectDocument(null);
      }
    },
    {
      id: 'cmd-fit-width',
      title: 'Fit Width (100%)',
      subtitle: 'Reset PDF zoom to 100%',
      category: 'View',
      icon: '🔍',
      action: () => {
        close();
        window.dispatchEvent(new CustomEvent('paperpilot:fit-width'));
      }
    },
    {
      id: 'cmd-theme',
      title: 'Toggle Theme',
      subtitle: 'Switch between Dark and Light mode',
      category: 'View',
      icon: '🌗',
      action: () => {
        close();
        appState.setTheme(appState.theme === 'dark' ? 'light' : 'dark');
      }
    }
  ];

  let filteredCommands = $derived.by(() => {
    const docCommands: CommandItem[] = appState.documents.map((doc, idx) => ({
      id: `cmd-doc-${idx}`,
      title: doc.name,
      subtitle: `Jump to document (${idx + 1} of ${appState.documents.length})`,
      category: 'Documents',
      icon: '📄',
      action: () => {
        close();
        appState.selectDocument(idx);
      }
    }));

    const all = [...docCommands, ...staticCommands];
    if (!searchQuery.trim()) return all;

    const q = searchQuery.toLowerCase();
    return all.filter(item =>
      item.title.toLowerCase().includes(q) ||
      item.subtitle.toLowerCase().includes(q) ||
      item.category.toLowerCase().includes(q)
    );
  });

  function open() {
    isOpen = true;
    searchQuery = '';
    selectedIndex = 0;
    setTimeout(() => inputEl?.focus(), 50);
  }

  function close() {
    isOpen = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      if (isOpen) close();
      else open();
      return;
    }

    if (!isOpen) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      close();
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = (selectedIndex + 1) % (filteredCommands.length || 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = (selectedIndex - 1 + (filteredCommands.length || 1)) % (filteredCommands.length || 1);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filteredCommands[selectedIndex]) {
        filteredCommands[selectedIndex].action();
      }
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="palette-backdrop" onclick={close} role="presentation">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="palette-modal"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-label="Command Palette"
      id="command-palette-modal"
      tabindex="-1"
    >
      <div class="search-header">
        <span class="search-icon">🔍</span>
        <input
          bind:this={inputEl}
          id="command-palette-input"
          class="palette-input"
          placeholder="Type a command or search documents (Ctrl+K)..."
          bind:value={searchQuery}
        />
        <kbd class="esc-badge">Esc</kbd>
      </div>

      <div class="results-list" role="listbox">
        {#if filteredCommands.length === 0}
          <div class="empty-state">No matching commands</div>
        {:else}
          {#each filteredCommands as cmd, i (cmd.id)}
            <div
              class="result-item"
              class:selected={i === selectedIndex}
              role="option"
              aria-selected={i === selectedIndex}
              id="cmd-item-{i}"
              onclick={() => cmd.action()}
              onmouseenter={() => selectedIndex = i}
              tabindex="-1"
            >
              <span class="item-icon">{cmd.icon}</span>
              <div class="item-text">
                <span class="item-title">{cmd.title}</span>
                <span class="item-sub">{cmd.subtitle}</span>
              </div>
              <span class="item-category">{cmd.category}</span>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .palette-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    backdrop-filter: blur(4px);
    z-index: 1000;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-top: 15vh;
  }

  .palette-modal {
    width: 100%;
    max-width: 600px;
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-md, 8px);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .search-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #2a2a35);
    gap: 12px;
  }

  .search-icon {
    font-size: 1rem;
    opacity: 0.7;
  }

  .palette-input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary, #ffffff);
    font-size: 1rem;
    outline: none;
  }

  .esc-badge {
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.1));
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 0.75rem;
    color: var(--text-muted, #9ca3af);
  }

  .results-list {
    max-height: 340px;
    overflow-y: auto;
    padding: 8px;
  }

  .result-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    border-radius: var(--border-radius-sm, 4px);
    cursor: pointer;
    transition: background 0.1s ease;
  }

  .result-item.selected {
    background: var(--accent-primary, #5e6ad2);
    color: #ffffff;
  }

  .result-item.selected .item-sub,
  .result-item.selected .item-category {
    color: rgba(255, 255, 255, 0.8);
  }

  .item-icon {
    font-size: 1.1rem;
  }

  .item-text {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .item-title {
    font-size: 0.9rem;
    font-weight: 500;
  }

  .item-sub {
    font-size: 0.75rem;
    color: var(--text-muted, #9ca3af);
  }

  .item-category {
    font-size: 0.75rem;
    padding: 2px 6px;
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    border-radius: 4px;
    color: var(--text-secondary, #9ca3af);
  }

  .empty-state {
    padding: 24px;
    text-align: center;
    color: var(--text-muted, #9ca3af);
    font-size: 0.9rem;
  }
</style>
