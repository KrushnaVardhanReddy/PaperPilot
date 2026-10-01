<script lang="ts">
  let { activeTool = $bindable('none') } = $props();

  const tools = [
    { id: 'none', label: 'Pointer', icon: 'M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z' },
    { id: 'highlight', label: 'Highlight', icon: 'M12 19l7-7 3 3-7 7-3-3z M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z' },
    { id: 'underline', label: 'Underline', icon: 'M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3 M4 21h16' },
    { id: 'strikethrough', label: 'Strike', icon: 'M5 12h14 M12 6a4 4 0 0 0-4 4 M16 16a4 4 0 0 1-4 4' },
    { id: 'pen', label: 'Pen', icon: 'M12 19l7-7 3 3-7 7-3-3z M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z' }, // reusing icon for simplicity
    { id: 'note', label: 'Note', icon: 'M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z' }
  ];
</script>

<div class="annotation-toolbar">
  {#each tools as tool}
    <button
      class="tool-btn"
      class:active={activeTool === tool.id}
      onclick={() => activeTool = tool.id}
      title={tool.label}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d={tool.icon}></path>
      </svg>
      <span class="tool-label">{tool.label}</span>
    </button>
  {/each}
</div>

<style>
  .annotation-toolbar {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background-color: var(--bg-surface, #ffffff);
    border-bottom: 1px solid var(--border-color, #e5e7eb);
    z-index: 10;
  }

  .tool-btn {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.4rem 0.6rem;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    font-size: 0.85rem;
    color: var(--text-secondary, #4b5563);
    cursor: pointer;
    transition: all 0.2s;
  }

  .tool-btn:hover {
    background-color: var(--bg-surface-hover, #f3f4f6);
    color: var(--text-primary, #111827);
  }

  .tool-btn.active {
    background-color: rgba(94, 106, 210, 0.1);
    border-color: var(--accent-primary, #5E6AD2);
    color: var(--accent-primary, #5E6AD2);
  }

  .tool-label {
    display: none;
  }

  @media (min-width: 1024px) {
    .tool-label {
      display: inline;
    }
  }
</style>
