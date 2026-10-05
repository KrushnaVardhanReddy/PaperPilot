<script lang="ts">
  import { onMount } from 'svelte';
  import PdfAnnotationPanel from '$lib/components/PdfAnnotationPanel.svelte';
  import PdfInfoPanel from '$lib/components/PdfInfoPanel.svelte';
  import PdfChatPanel from '$lib/components/PdfChatPanel.svelte';
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
  let activeTab = $state<'annotations' | 'info' | 'chat'>('annotations');
  let panelWidth = $state(300);
  let isDragging = $state(false);

  onMount(() => {
    const storedCollapsed = localStorage.getItem('viewer-right-panel-collapsed');
    if (storedCollapsed === 'true') isCollapsed = true;

    const storedTab = localStorage.getItem('viewer-right-panel-tab');
    if (storedTab === 'info' || storedTab === 'annotations') {
      activeTab = storedTab;
    }

    const storedWidth = localStorage.getItem('viewer-right-panel-width');
    if (storedWidth) {
      const parsed = parseInt(storedWidth, 10);
      if (!isNaN(parsed) && parsed >= 220 && parsed <= 800) {
        panelWidth = parsed;
      }
    }
  });

  $effect(() => {
    localStorage.setItem('viewer-right-panel-collapsed', String(isCollapsed));
  });

  $effect(() => {
    localStorage.setItem('viewer-right-panel-tab', activeTab);

    // Auto-comfort width on chat tab
    if (activeTab === 'chat' && !isCollapsed && panelWidth < 420) {
      panelWidth = 420;
    }
  });

  $effect(() => {
    localStorage.setItem('viewer-right-panel-width', String(panelWidth));
  });

  function startResize(e: MouseEvent) {
    if (isCollapsed) return;
    isDragging = true;
    const startX = e.clientX;
    const startWidth = panelWidth;

    function onMouseMove(moveEvent: MouseEvent) {
      const delta = startX - moveEvent.clientX;
      const newWidth = Math.min(Math.max(startWidth + delta, 240), 800);
      panelWidth = newWidth;
    }

    function onMouseUp() {
      isDragging = false;
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
    }

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  }

  function handleResizeDblClick() {
    panelWidth = panelWidth > 350 ? 300 : 500;
  }
</script>

<div
  class="right-panel"
  class:collapsed={isCollapsed}
  class:dragging={isDragging}
  style={!isCollapsed ? `width: ${panelWidth}px; min-width: ${panelWidth}px;` : ''}
>
  <!-- Draggable left resize handle -->
  {#if !isCollapsed}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="resize-handle"
      id="right-panel-resize-handle"
      title="Drag to resize panel, double-click to toggle width"
      onmousedown={startResize}
      ondblclick={handleResizeDblClick}
    ></div>
  {/if}

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
      <button
        class="tab-btn"
        class:active={activeTab === 'chat'}
        id="right-panel-tab-chat"
        role="tab"
        aria-selected={activeTab === 'chat'}
        onclick={() => activeTab = 'chat'}
      >
        💬 AI Chat
      </button>
    </div>

    <div class="panel-content" role="tabpanel">
      {#if activeTab === 'annotations'}
        <PdfAnnotationPanel
          bind:annotations
          {onJumpToPage}
        />
      {:else if activeTab === 'chat'}
        <PdfChatPanel />
      {:else}
        <PdfInfoPanel {pdfDoc} />
      {/if}
    </div>
  {/if}
</div>

<style>
  .right-panel {
    position: relative;
    overflow: hidden;
    border-left: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    transition: width 0.2s ease, min-width 0.2s ease;
  }

  .right-panel.dragging {
    transition: none; /* smooth real-time drag without transition lag */
    user-select: none;
  }

  .right-panel.collapsed {
    width: 40px !important;
    min-width: 40px !important;
  }

  .resize-handle {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 6px;
    cursor: col-resize;
    z-index: 20;
    background: transparent;
    transition: background 0.15s ease;
  }

  .resize-handle:hover,
  .right-panel.dragging .resize-handle {
    background: var(--accent-primary);
  }

  .collapse-toggle {
    position: absolute;
    left: 6px;
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
    padding-left: 26px; /* leave room for resize handle and toggle */
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
