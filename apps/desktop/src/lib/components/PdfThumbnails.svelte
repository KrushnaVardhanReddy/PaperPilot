<script lang="ts">
  import { tick } from 'svelte';

  let { pdfDoc, pageNum = $bindable(1) } = $props();

  let numPages = $derived(pdfDoc ? pdfDoc.numPages : 0);
  let isRendering = false;
  let canvases: Record<number, HTMLCanvasElement> = {};
  let thumbnailWrappers: Record<number, HTMLElement> = {};
  let thumbnailsContainer = $state<HTMLElement | null>(null);
  let draggedPage = $state<number | null>(null);
  let dragOverPage = $state<number | null>(null);
  let dragInsertPosition = $state<'top' | 'bottom' | null>(null);

  // Confirmation dialog state
  let confirmDialog = $state<{
    visible: boolean;
    action: 'delete' | 'rotate' | null;
    page: number | null;
    angle?: number;
  }>({ visible: false, action: null, page: null });

  let contextMenu = $state({
    visible: false,
    x: 0,
    y: 0,
    page: null as number | null
  });

  function closeContextMenu() {
    contextMenu.visible = false;
  }

  $effect(() => {
    function handleGlobalClick() {
      if (contextMenu.visible) {
        closeContextMenu();
      }
    }
    function handleGlobalKeydown(e: KeyboardEvent) {
      if (e.key === 'Escape' && contextMenu.visible) {
        closeContextMenu();
      }
    }
    window.addEventListener('click', handleGlobalClick);
    window.addEventListener('contextmenu', handleGlobalClick);
    window.addEventListener('keydown', handleGlobalKeydown);
    return () => {
      window.removeEventListener('click', handleGlobalClick);
      window.removeEventListener('contextmenu', handleGlobalClick);
      window.removeEventListener('keydown', handleGlobalKeydown);
    };
  });

  async function renderThumbnails() {
    if (!pdfDoc || numPages === 0 || isRendering) return;
    isRendering = true;
    await tick();

    try {
      for (let i = 1; i <= numPages; i++) {
        const canvas = canvases[i];
        if (!canvas) continue;

        const page = await pdfDoc.getPage(i);
        const viewport = page.getViewport({ scale: 0.2 });

        const ctx = canvas.getContext('2d');
        if (!ctx) continue;

        const outputScale = window.devicePixelRatio || 1;
        canvas.width = Math.floor(viewport.width * outputScale);
        canvas.height = Math.floor(viewport.height * outputScale);
        canvas.style.width = Math.floor(viewport.width) + "px";
        canvas.style.height = Math.floor(viewport.height) + "px";

        const transform = outputScale !== 1
          ? [outputScale, 0, 0, outputScale, 0, 0]
          : null;

        const renderContext = {
          canvasContext: ctx,
          transform: transform ? transform : undefined,
          viewport,
          canvas,
        };

        await page.render(renderContext as any).promise;
      }
    } catch (err) {
      console.error("Error rendering thumbnails:", err);
    } finally {
      isRendering = false;
    }
  }

  $effect(() => {
    if (pdfDoc) {
      renderThumbnails();
    }
  });

  // Keep thumbnail in view when pageNum changes from outside
  $effect(() => {
    if (pageNum && thumbnailWrappers[pageNum]) {
      thumbnailWrappers[pageNum].scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    }
  });

  // Track scrolling in thumbnail container to navigate pageNum
  let scrollTimeout: any = null;
  function handleContainerScroll() {
    if (!thumbnailsContainer || numPages === 0) return;
    clearTimeout(scrollTimeout);
    scrollTimeout = setTimeout(() => {
      const containerRect = thumbnailsContainer?.getBoundingClientRect();
      if (!containerRect) return;
      const centerY = containerRect.top + containerRect.height / 2;

      let closestPage = pageNum;
      let minDistance = Infinity;

      for (let p = 1; p <= numPages; p++) {
        const el = thumbnailWrappers[p];
        if (!el) continue;
        const rect = el.getBoundingClientRect();
        const itemCenter = rect.top + rect.height / 2;
        const dist = Math.abs(itemCenter - centerY);
        if (dist < minDistance) {
          minDistance = dist;
          closestPage = p;
        }
      }

      if (closestPage !== pageNum) {
        pageNum = closestPage;
      }
    }, 120);
  }

  function selectPage(page: number) {
    pageNum = page;
  }

  function handleDragStart(e: DragEvent, page: number) {
    draggedPage = page;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', String(page));
    }
  }

  function handleDragOver(e: DragEvent, page: number) {
    e.preventDefault();
    if (draggedPage !== null && draggedPage !== page) {
      dragOverPage = page;
      const target = e.currentTarget as HTMLElement;
      const rect = target.getBoundingClientRect();
      const relativeY = e.clientY - rect.top;
      if (relativeY < rect.height / 2) {
        dragInsertPosition = 'top';
      } else {
        dragInsertPosition = 'bottom';
      }
    }
  }

  function handleDrop(e: DragEvent, targetPage: number) {
    e.preventDefault();
    if (draggedPage !== null && draggedPage !== targetPage && dragInsertPosition !== null) {
      window.dispatchEvent(new CustomEvent('paperpilot:reorder-page', {
        detail: { fromPage: draggedPage, toPage: targetPage, position: dragInsertPosition }
      }));
    }
    draggedPage = null;
    dragOverPage = null;
    dragInsertPosition = null;
  }

  function handleDragEnd() {
    draggedPage = null;
    dragOverPage = null;
    dragInsertPosition = null;
  }

  function rotatePage(e: MouseEvent, page: number) {
    e.stopPropagation();
    window.dispatchEvent(new CustomEvent('paperpilot:rotate-single-page', {
      detail: { page }
    }));
  }

  function deletePage(e: MouseEvent, page: number) {
    e.stopPropagation();
    confirmDialog = {
      visible: true,
      action: 'delete',
      page
    };
  }

  function handleContextMenu(e: MouseEvent, page: number) {
    e.preventDefault();
    contextMenu = {
      visible: true,
      x: e.clientX,
      y: e.clientY,
      page
    };
  }

  function handleContextAction(action: string, page: number | null) {
    if (page === null) return;

    switch (action) {
      case 'rotate_90':
        window.dispatchEvent(new CustomEvent('paperpilot:rotate-single-page', { detail: { page, angle: 90 } }));
        break;
      case 'rotate_180':
        window.dispatchEvent(new CustomEvent('paperpilot:rotate-single-page', { detail: { page, angle: 180 } }));
        break;
      case 'rotate_270':
        window.dispatchEvent(new CustomEvent('paperpilot:rotate-single-page', { detail: { page, angle: 270 } }));
        break;
      case 'duplicate':
        window.dispatchEvent(new CustomEvent('paperpilot:duplicate-page', { detail: { page } }));
        break;
      case 'extract':
        window.dispatchEvent(new CustomEvent('paperpilot:extract-single-page', { detail: { page } }));
        break;
      case 'delete':
        confirmDialog = {
          visible: true,
          action: 'delete',
          page
        };
        break;
    }
    closeContextMenu();
  }

  function handleConfirmAction() {
    if (confirmDialog.action === 'delete' && confirmDialog.page !== null) {
      window.dispatchEvent(new CustomEvent('paperpilot:delete-single-page', {
        detail: { page: confirmDialog.page }
      }));
    }
    confirmDialog = { visible: false, action: null, page: null };
  }

  function handleCancelConfirm() {
    confirmDialog = { visible: false, action: null, page: null };
  }
</script>

<div class="thumbnails-container" aria-label="Page Thumbnails">
  {#if numPages > 0}
    {#each Array(numPages) as _, index}
      {@const p = index + 1}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="thumbnail-wrapper"
        class:selected={pageNum === p}
        class:drag-over-top={dragOverPage === p && dragInsertPosition === 'top'}
        class:drag-over-bottom={dragOverPage === p && dragInsertPosition === 'bottom'}
        class:is-dragging={draggedPage === p}
        id="thumbnail-page-{p}"
        draggable="true"
        ondragstart={(e) => handleDragStart(e, p)}
        ondragover={(e) => handleDragOver(e, p)}
        ondrop={(e) => handleDrop(e, p)}
        ondragend={handleDragEnd}
        onclick={() => selectPage(p)}
        onkeydown={(e) => e.key === 'Enter' && selectPage(p)}
        tabindex="0"
        role="button"
        oncontextmenu={(e) => handleContextMenu(e, p)}
        aria-label="Go to page {p}"
      >
        <div class="drag-grip">⋮⋮</div>
        <div class="canvas-box">
          <canvas bind:this={canvases[p]}></canvas>

          <!-- Hover Action Overlay -->
          <div class="hover-actions">
            <button
              class="action-btn"
              id="thumb-rotate-{p}"
              title="Rotate Page {p}"
              aria-label="Rotate Page {p}"
              onclick={(e) => rotatePage(e, p)}
            >
              🔄
            </button>
            <button
              class="action-btn delete"
              id="thumb-delete-{p}"
              title="Delete Page {p}"
              aria-label="Delete Page {p}"
              onclick={(e) => deletePage(e, p)}
            >
              🗑️
            </button>
          </div>
        </div>
        <span class="page-num">{p}</span>
      </div>
    {/each}
  {/if}

  {#if contextMenu.visible}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="context-menu"
      style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
      onclick={(e) => e.stopPropagation()}
    >
      <button class="menu-item" onclick={() => handleContextAction('rotate_90', contextMenu.page)}>
        <span>🔄</span> Rotate Clockwise (90°)
      </button>
      <button class="menu-item" onclick={() => handleContextAction('rotate_270', contextMenu.page)}>
        <span>🔄</span> Rotate Counter-Clockwise (270°)
      </button>
      <button class="menu-item" onclick={() => handleContextAction('rotate_180', contextMenu.page)}>
        <span>🔄</span> Rotate 180°
      </button>
      <div class="menu-divider"></div>
      <button class="menu-item" onclick={() => handleContextAction('duplicate', contextMenu.page)}>
        <span>📑</span> Duplicate Page
      </button>
      <button class="menu-item" onclick={() => handleContextAction('extract', contextMenu.page)}>
        <span>✂️</span> Extract Page to New File
      </button>
      <div class="menu-divider"></div>
      <button class="menu-item danger" onclick={() => handleContextAction('delete', contextMenu.page)}>
        <span>🗑️</span> Delete Page
      </button>
    </div>
  {/if}

  {#if confirmDialog.visible}
    <!-- Confirmation Dialog Overlay -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="confirm-overlay" onclick={handleCancelConfirm}>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="confirm-dialog" onclick={(e) => e.stopPropagation()}>
        <div class="confirm-title">
          <span>⚠️</span> Delete Page {confirmDialog.page}?
        </div>
        <p class="confirm-desc">
          Are you sure you want to permanently delete page {confirmDialog.page}? This cannot be undone.
        </p>
        <div class="confirm-actions">
          <button class="confirm-btn cancel" onclick={handleCancelConfirm}>Cancel</button>
          <button class="confirm-btn danger" onclick={handleConfirmAction}>Delete</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .thumbnails-container {
    width: 140px;
    height: 100%;
    overflow-y: auto;
    background-color: var(--bg-secondary, #13131a);
    border-right: 1px solid var(--border-color, #2a2a35);
    padding: 12px 8px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    user-select: none;
    flex-shrink: 0;
  }

  .thumbnail-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: grab;
    border-radius: var(--border-radius-sm, 4px);
    padding: 4px;
    transition: all var(--transition-fast, 0.15s ease);
    position: relative;
  }

  .thumbnail-wrapper:active {
    cursor: grabbing;
  }

  .thumbnail-wrapper.is-dragging {
    opacity: 0.4;
  }

  .thumbnail-wrapper.drag-over-top {
    border-top: 2px solid var(--accent-primary, #5e6ad2);
  }

  .thumbnail-wrapper.drag-over-bottom {
    border-bottom: 2px solid var(--accent-primary, #5e6ad2);
  }

  .drag-grip {
    color: var(--text-tertiary, #6b7280);
    font-size: 14px;
    line-height: 1;
    margin-bottom: 4px;
    cursor: grab;
    opacity: 0.5;
    letter-spacing: -2px;
  }

  .drag-grip:active {
    cursor: grabbing;
  }

  .thumbnail-wrapper:hover .drag-grip {
    opacity: 1;
  }

  .thumbnail-wrapper:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
  }

  .thumbnail-wrapper.selected {
    background-color: rgba(94, 106, 210, 0.15);
  }

  .thumbnail-wrapper.selected .canvas-box {
    box-shadow: 0 0 0 2px var(--accent-primary, #5e6ad2);
  }

  .canvas-box {
    position: relative;
    border-radius: 2px;
    overflow: hidden;
    background-color: #ffffff;
    box-shadow: 0 1px 4px rgba(0,0,0,0.25);
    display: flex;
    justify-content: center;
  }

  .hover-actions {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    opacity: 0;
    transition: opacity var(--transition-fast, 0.15s ease);
  }

  .thumbnail-wrapper:hover .hover-actions {
    opacity: 1;
  }

  .action-btn {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.85);
    border: none;
    font-size: 11px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: transform var(--transition-fast, 0.15s ease), background 0.15s ease;
  }

  .action-btn:hover {
    transform: scale(1.15);
    background: #ffffff;
  }

  .action-btn.delete:hover {
    background: #fee2e2;
  }

  .page-num {
    margin-top: 4px;
    font-size: 11px;
    color: var(--text-secondary, #9ca3af);
  }

  .thumbnail-wrapper.selected .page-num {
    color: var(--text-primary, #ffffff);
    font-weight: 600;
  }

  .context-menu {
    position: fixed;
    z-index: 1000;
    background-color: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-sm, 4px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    padding: 4px 0;
    min-width: 220px;
    display: flex;
    flex-direction: column;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: transparent;
    border: none;
    color: var(--text-primary, #e2e8f0);
    font-size: 12px;
    text-align: left;
    cursor: pointer;
    transition: background-color var(--transition-fast, 0.15s ease);
    width: 100%;
  }

  .menu-item:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
  }

  .menu-item.danger {
    color: #f87171;
  }

  .menu-item.danger:hover {
    background-color: rgba(248, 113, 113, 0.1);
  }

  .menu-divider {
    height: 1px;
    background-color: var(--border-color, #2a2a35);
    margin: 4px 0;
  }

  /* Confirmation Dialog */
  .confirm-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 16px;
  }

  .confirm-dialog {
    background: var(--bg-surface-elevated, var(--bg-surface, #1e293b));
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-md, 8px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.4);
    padding: 20px;
    max-width: 320px;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .confirm-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .confirm-desc {
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.4;
    margin: 0;
  }

  .confirm-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 4px;
  }

  .confirm-btn {
    padding: 6px 14px;
    border-radius: var(--border-radius-sm, 6px);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    border: 1px solid var(--border-color);
    transition: all 0.15s ease;
  }

  .confirm-btn.cancel {
    background: var(--bg-surface);
    color: var(--text-secondary);
  }

  .confirm-btn.cancel:hover {
    background: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  .confirm-btn.danger {
    background: #ef4444;
    border-color: #ef4444;
    color: #ffffff;
  }

  .confirm-btn.danger:hover {
    background: #dc2626;
    border-color: #dc2626;
  }
</style>