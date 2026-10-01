<script lang="ts">
  import type { Annotation } from '$lib/api/pdf';

  let {
    annotations = $bindable([] as Annotation[]),
    onJumpToPage = (page: number) => {}
  } = $props();

  function removeAnnotation(id: string) {
    annotations = annotations.filter(a => a.id !== id);
  }
</script>

<div class="annotation-panel">
  <div class="panel-header">
    <h3>Annotations ({annotations.length})</h3>
  </div>

  <div class="panel-content">
    {#if annotations.length === 0}
      <div class="empty-state">
        <p>No annotations yet.</p>
        <p class="subtitle">Use the toolbar to add highlights, notes, and drawings.</p>
      </div>
    {:else}
      <ul class="annotation-list">
        {#each annotations as annotation}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
          <li class="annotation-item" onclick={() => onJumpToPage(annotation.page)}>
            <div class="item-icon" style="color: {annotation.color}">
              {#if annotation.type === 'highlight'}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19l7-7 3 3-7 7-3-3z"></path><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"></path></svg>
              {:else if annotation.type === 'underline'}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3"></path><line x1="4" y1="21" x2="20" y2="21"></line></svg>
              {:else if annotation.type === 'strikethrough'}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"></path><path d="M12 6a4 4 0 0 0-4 4"></path><path d="M16 16a4 4 0 0 1-4 4"></path></svg>
              {:else if annotation.type === 'note'}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path></svg>
              {:else if annotation.type === 'pen'}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path></svg>
              {/if}
            </div>
            <div class="item-details">
              <span class="item-type">{annotation.type}</span>
              <span class="item-page">Page {annotation.page}</span>
            </div>
            <button class="remove-btn" onclick={(e) => { e.stopPropagation(); removeAnnotation(annotation.id); }} title="Remove Annotation">
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .annotation-panel {
    width: 260px;
    background-color: var(--bg-surface, #ffffff);
    border-left: 1px solid var(--border-color, #e5e7eb);
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .panel-header {
    padding: 1rem;
    border-bottom: 1px solid var(--border-color, #e5e7eb);
  }

  .panel-header h3 {
    margin: 0;
    font-size: 0.95rem;
    color: var(--text-primary, #111827);
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }

  .empty-state {
    text-align: center;
    color: var(--text-secondary, #6b7280);
    padding: 2rem 0;
  }

  .empty-state p {
    margin: 0;
    font-size: 0.9rem;
  }

  .empty-state .subtitle {
    margin-top: 0.5rem;
    font-size: 0.8rem;
    color: var(--text-muted, #9ca3af);
  }

  .annotation-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .annotation-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background-color: var(--bg-primary, #f9fafb);
    border: 1px solid var(--border-color, #e5e7eb);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    position: relative;
  }

  .annotation-item:hover {
    background-color: var(--bg-surface-hover, #f3f4f6);
    border-color: var(--accent-primary, #3b82f6);
  }

  .item-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    background-color: var(--bg-surface, #ffffff);
    border-radius: 4px;
    box-shadow: 0 1px 2px rgba(0,0,0,0.05);
  }

  .item-details {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .item-type {
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-primary, #111827);
    text-transform: capitalize;
  }

  .item-page {
    font-size: 0.75rem;
    color: var(--text-secondary, #6b7280);
  }

  .remove-btn {
    display: none;
    background: none;
    border: none;
    color: var(--text-muted, #9ca3af);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .annotation-item:hover .remove-btn {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .remove-btn:hover {
    background-color: rgba(220, 38, 38, 0.1);
    color: #dc2626;
  }
</style>
