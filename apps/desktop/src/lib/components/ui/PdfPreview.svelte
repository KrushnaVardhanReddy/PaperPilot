<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';

  // Touch gesture state
  let initialPinchDistance = $state<number | null>(null);
  let currentScale = $state(1);
  let touchStartX = $state<number | null>(null);

  async function handleShare() {
     const doc = appState.documents[appState.selectedDocumentIndex!];
     if (!doc) return;

     if (navigator.share && appState.isMobile) {
        try {
           await navigator.share({
              title: doc.name,
              files: [doc]
           });
        } catch (e) {
           toastState.error("Share failed", String(e));
        }
     } else {
        toastState.error("Native share not supported here");
     }
  }

  // Pointer Handlers

  let activePointers = $state(new Map<number, PointerEvent>());

  function getDistance(p1: PointerEvent, p2: PointerEvent) {
    return Math.sqrt(Math.pow(p2.clientX - p1.clientX, 2) + Math.pow(p2.clientY - p1.clientY, 2));
  }

  function onPointerDown(e: PointerEvent) {
    if (!appState.isMobile) return;
    activePointers.set(e.pointerId, e);

    if (activePointers.size === 1) {
      touchStartX = e.clientX;
    } else if (activePointers.size === 2) {
      const pointers = Array.from(activePointers.values());
      initialPinchDistance = getDistance(pointers[0], pointers[1]);
    }
  }

  function onPointerMove(e: PointerEvent) {
    if (!appState.isMobile) return;

    if (activePointers.has(e.pointerId)) {
      activePointers.set(e.pointerId, e);
    }

    if (activePointers.size === 2 && initialPinchDistance) {
      const pointers = Array.from(activePointers.values());
      const currentDist = getDistance(pointers[0], pointers[1]);
      currentScale = Math.max(0.5, Math.min(3, currentScale * (currentDist / initialPinchDistance)));
      initialPinchDistance = currentDist; // Reset for smooth continuous zooming
    }
  }

  function onPointerUp(e: PointerEvent) {
    if (!appState.isMobile) return;

    activePointers.delete(e.pointerId);

    if (activePointers.size < 2) {
      initialPinchDistance = null;
    }

    if (touchStartX !== null && activePointers.size === 0) {
      const diffX = e.clientX - touchStartX;
      if (Math.abs(diffX) > 80 && currentScale <= 1.1) {
        if (diffX > 0) {
           toastState.success("Swiped Right"); // Trigger prevPage() logic if multi-page viewer
        } else {
           toastState.success("Swiped Left"); // Trigger nextPage() logic
        }
      }
      touchStartX = null;
    }
  }

  function onPointerCancel(e: PointerEvent) {
     activePointers.delete(e.pointerId);
     initialPinchDistance = null;
     touchStartX = null;
  }


  let pages = $state(Array.from({ length: 12 }, (_, i) => ({ id: i + 1, num: i + 1, rotation: 0 })));
  let selectedPages = $state(new Set<number>());

  function togglePageSelection(pageNum: number) {
    const newSelection = new Set(selectedPages);
    if (newSelection.has(pageNum)) {
      newSelection.delete(pageNum);
    } else {
      newSelection.add(pageNum);
    }
    selectedPages = newSelection;
  }

  function clearSelection() {
    selectedPages = new Set();
  }

  function selectAll() {
    selectedPages = new Set(pages.map(p => p.id));
  }

  function handleBack() {
    appState.selectDocument(null);
  }

  let draggedIndex = $state<number | null>(null);

  function handleDragStart(e: DragEvent, index: number) {
    draggedIndex = index;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = 'move';
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'move';
    }
  }

  function handleDrop(e: DragEvent, targetIndex: number) {
    e.preventDefault();
    if (draggedIndex === null || draggedIndex === targetIndex) return;

    const newPages = [...pages];
    const [draggedItem] = newPages.splice(draggedIndex, 1);
    newPages.splice(targetIndex, 0, draggedItem);
    pages = newPages;
    draggedIndex = null;
  }

  function handleRotate(e: MouseEvent, id: number) {
    e.stopPropagation();
    const index = pages.findIndex(p => p.id === id);
    if (index !== -1) {
      pages[index].rotation += 90;
    }
  }

  function handleDelete(e: MouseEvent, id: number) {
    e.stopPropagation();
    pages = pages.filter(p => p.id !== id);
    if (selectedPages.has(id)) {
      const newSelection = new Set(selectedPages);
      newSelection.delete(id);
      selectedPages = newSelection;
    }
  }
</script>

<div class="pdf-preview-container"
  onpointerdown={onPointerDown}
  onpointermove={onPointerMove}
  onpointerup={onPointerUp}
  style="transform: scale({currentScale}); transform-origin: top left; transition: transform 0.1s ease;"
>
  <div class="preview-header">
    <div class="header-left">
      <button class="back-btn" onclick={handleBack} title="Back to List">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="19" y1="12" x2="5" y2="12"></line>
          <polyline points="12 19 5 12 12 5"></polyline>
        </svg>
      </button>
      <div class="doc-info">
        <h3>{appState.documents[appState.selectedDocumentIndex!]?.name}</h3>
        <span class="page-count">{pages.length} pages</span>
      </div>
    </div>

    <div class="header-actions">
      {#if appState.isMobile}
         <button class="action-btn" onclick={handleShare}>📤 Export & Share</button>
      {/if}
      <button class="action-btn" onclick={selectAll}>Select All</button>
      <button class="action-btn" onclick={clearSelection} disabled={selectedPages.size === 0}>Clear Selection</button>
    </div>
  </div>

  <div class="preview-content">
    <div class="thumbnails-grid">
      {#each pages as page, i}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="thumbnail-wrapper {selectedPages.has(page.id) ? 'selected' : ''}"
          onclick={() => togglePageSelection(page.id)}
          draggable="true"
          ondragstart={(e) => handleDragStart(e, i)}
          ondragover={handleDragOver}
          ondrop={(e) => handleDrop(e, i)}
        >
          <div class="thumbnail" style="transform: rotate({page.rotation}deg)">
            <button class="rotate-btn" onclick={(e) => handleRotate(e, page.id)} title="Rotate Page">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M2.5 2v6h6M2.66 15.57a10 10 0 1 0 .57-8.38l-5.67-5.67"/>
              </svg>
            </button>
            <span class="mock-content">PDF Content</span>
          </div>
          <button class="delete-btn" onclick={(e) => handleDelete(e, page.id)} title="Delete Page">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="3 6 5 6 21 6"></polyline>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
            </svg>
          </button>
          <span class="page-number">Page {page.num}</span>
          {#if selectedPages.has(page.id)}
            <div class="selection-indicator">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"></polyline>
              </svg>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .pdf-preview-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: var(--bg-primary);
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .back-btn {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px;
    border-radius: var(--border-radius-sm);
    transition: all var(--transition-fast);
  }

  .back-btn:hover {
    color: var(--text-primary);
    background-color: var(--bg-surface-hover);
  }

  .doc-info {
    display: flex;
    flex-direction: column;
  }

  .doc-info h3 {
    font-size: 1.1rem;
    color: var(--text-primary);
    margin: 0;
    margin-bottom: 2px;
  }

  .page-count {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .header-actions {
    display: flex;
    gap: 12px;
  }

  .action-btn {
    background-color: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    padding: 6px 12px;
    border-radius: var(--border-radius-md);
    font-size: 0.85rem;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .action-btn:hover:not(:disabled) {
    background-color: var(--bg-surface-hover);
    border-color: var(--text-muted);
  }

  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .preview-content {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }

  .thumbnails-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 24px;
    padding-bottom: 24px;
  }

  .thumbnail-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    position: relative;
    transition: transform var(--transition-fast);
  }

  .thumbnail-wrapper:hover {
    transform: translateY(-2px);
  }

  .thumbnail {
    width: 100%;
    aspect-ratio: 1 / 1.414; /* A4 aspect ratio */
    background-color: #ffffff;
    border-radius: 4px;
    box-shadow: var(--shadow-md);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid transparent;
    transition: all var(--transition-fast);
    position: relative;
    overflow: hidden;
  }

  .mock-content {
    color: #e0e0e0;
    font-size: 1.2rem;
    font-weight: bold;
    transform: rotate(-45deg);
    user-select: none;
  }

  .thumbnail-wrapper:hover .thumbnail {
    box-shadow: var(--shadow-lg);
  }

  .thumbnail-wrapper.selected .thumbnail {
    border-color: var(--accent-primary);
  }

  .page-number {
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .thumbnail-wrapper.selected .page-number {
    color: var(--accent-primary);
  }

  .rotate-btn {
    position: absolute;
    top: 8px;
    left: 8px;
    width: 28px;
    height: 28px;
    background-color: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    opacity: 0;
    transition: all var(--transition-fast);
    z-index: 10;
  }

  .thumbnail-wrapper:hover .rotate-btn {
    opacity: 1;
  }

  .rotate-btn:hover {
    background-color: var(--bg-surface-hover);
    color: var(--accent-primary);
    border-color: var(--accent-primary);
  }

  .delete-btn {
    position: absolute;
    bottom: 24px;
    right: 8px;
    width: 28px;
    height: 28px;
    background-color: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    opacity: 0;
    transition: all var(--transition-fast);
    z-index: 10;
  }

  .thumbnail-wrapper:hover .delete-btn {
    opacity: 1;
  }

  .delete-btn:hover {
    background-color: #ffebee;
    color: #e53935;
    border-color: #ef9a9a;
  }

  .selection-indicator {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 24px;
    height: 24px;
    background-color: var(--accent-primary);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 4px rgba(0,0,0,0.2);
  }
</style>
