<script lang="ts">
  import { onMount } from 'svelte';
  import PdfAnnotationPanel from '$lib/components/PdfAnnotationPanel.svelte';
  import PdfInfoPanel from '$lib/components/PdfInfoPanel.svelte';
  import type { Annotation } from '$lib/api/pdf';

  let {
    annotations = $bindable<Annotation[]>([]),
    pdfDoc = null,
    onJumpToPage
  }: {
    annotations: Annotation[];
    pdfDoc: any;
    onJumpToPage: (page: number) => void;
  } = $props();

  let isCollapsed = $state(false);
  let activeTab = $state<'annotations' | 'info'>('annotations');

  onMount(() => {
    const storedCollapsed = localStorage.getItem('viewer-right-panel-collapsed');
    if (storedCollapsed === 'true') isCollapsed = true;

    const storedTab = localStorage.getItem('viewer-right-panel-tab');
    if (storedTab === 'info' || storedTab === 'annotations') {
      activeTab = storedTab;
    }
  });

  $effect(() => {
    localStorage.setItem('viewer-right-panel-collapsed', String(isCollapsed));
  });

  $effect(() => {
    localStorage.setItem('viewer-right-panel-tab', activeTab);
  });
</script>

<div class="right-panel" class:collapsed={isCollapsed}>
  <button
    class="collapse-toggle"
    id="right-panel-toggle"
    onclick={() => isCollapsed = !isCollapsed}
    title={isCollapsed ? 'Expand panel' : 'Collapse panel'}
    aria-label={isCollapsed ? 'Expand panel' : 'Collapse panel'}
  >
    {isCollapsed ? '›' : '‹'}
  </button>

  {#if !isCollapsed}
    <div class="panel-tabs" role="tablist" aria-label="Panel sections">
      <button
        class="tab-btn"
        class:active={activeTab === 'annotations'}
        id="right-panel-tab-annotations"
        role="tab"
        aria-selected={activeTab === 'annotations'}
        onclick={() => activeTab = 'annotations'}
      >
        Annotations ({annotations.length})
      </button>
      <button
        class="tab-btn"
        class:active={activeTab === 'info'}
        id="right-panel-tab-info"
        role="tab"
        aria-selected={activeTab === 'info'}
        onclick={() => activeTab = 'info'}
      >
        Info
      </button>
    </div>

    <div class="panel-content" role="tabpanel">
      {#if activeTab === 'annotations'}
        <PdfAnnotationPanel
          bind:annotations
          {onJumpToPage}
        />
      {:else}
        <PdfInfoPanel {pdfDoc} />
      {/if}
    </div>
  {/if}
</div>

<style>
  .right-panel {
    position: relative;
    width: 280px;
    min-width: 280px;
    transition: width 0.25s ease, min-width 0.25s ease;
    overflow: hidden;
    border-left: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .right-panel.collapsed {
    width: 40px;
    min-width: 40px;
  }

  .collapse-toggle {
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 0 4px 4px 0;
    color: var(--text-muted);
    width: 16px;
    height: 40px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    z-index: 10;
    transition: all var(--transition-fast);
    padding: 0;
  }

  .collapse-toggle:hover {
    color: var(--text-primary);
    background: var(--bg-surface-hover);
  }

  .panel-tabs {
    display: flex;
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
    padding-left: 20px; /* leave room for the collapse toggle */
  }

  .tab-btn {
    flex: 1;
    padding: 8px 12px;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    transition: all var(--transition-fast);
    font-family: inherit;
    white-space: nowrap;
  }

  .tab-btn:hover {
    color: var(--text-primary);
    background: var(--bg-surface);
  }

  .tab-btn.active {
    color: var(--text-primary);
    border-bottom-color: var(--accent-primary);
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
</style>