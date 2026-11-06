<script lang="ts">
  export interface PlacedStamp {
    id: string;
    page: number;
    x: number; // PDF points from left
    y: number; // PDF points from bottom (for pdf_stamp lopdf)
    topY: number; // PDF points from top (for DOM layout)
    width: number; // PDF points
    height: number; // PDF points
    dataUrl: string;
  }

  export interface ArmedSignature {
    dataUrl: string;
    width: number;
    height: number;
    label?: string;
    isInitials?: boolean;
  }

  let {
    scale = 1.0,
    pageNum = 1,
    pageDimensions = { width: 595.28, height: 841.89 },
    armedSignature = $bindable<ArmedSignature | null>(null),
    placedStamps = $bindable<PlacedStamp[]>([]),
    onStampPlaced = (stamp: PlacedStamp) => {},
    onStampsChanged = (stamps: PlacedStamp[]) => {}
  } = $props<{
    scale?: number;
    pageNum?: number;
    pageDimensions?: { width: number; height: number };
    armedSignature?: ArmedSignature | null;
    placedStamps?: PlacedStamp[];
    onStampPlaced?: (stamp: PlacedStamp) => void;
    onStampsChanged?: (stamps: PlacedStamp[]) => void;
  }>();

  let layerRef: HTMLDivElement | null = $state(null);
  let selectedStampId = $state<string | null>(null);

  // Ghost preview tracking
  let mouseX = $state(0);
  let mouseY = $state(0);
  let isMouseOverLayer = $state(false);

  // Dragging placed stamp state
  let isDraggingStamp = $state(false);
  let dragStampId = $state<string | null>(null);
  let dragStartX = $state(0);
  let dragStartY = $state(0);
  let initialStampX = $state(0);
  let initialStampTopY = $state(0);

  // Resizing placed stamp state
  let isResizing = $state(false);
  let resizeHandle = $state<string | null>(null);
  let resizeStampId = $state<string | null>(null);
  let resizeStartX = $state(0);
  let resizeStartY = $state(0);
  let initialResizeX = $state(0);
  let initialResizeTopY = $state(0);
  let initialResizeW = $state(0);
  let initialResizeH = $state(0);

  function getMousePdfCoords(e: MouseEvent | PointerEvent): { x: number; topY: number } {
    if (!layerRef) return { x: 0, topY: 0 };
    const rect = layerRef.getBoundingClientRect();
    const x = (e.clientX - rect.left) / scale;
    const topY = (e.clientY - rect.top) / scale;
    return { x, topY };
  }

  function handleLayerPointerMove(e: PointerEvent) {
    if (!layerRef) return;
    const rect = layerRef.getBoundingClientRect();
    mouseX = e.clientX - rect.left;
    mouseY = e.clientY - rect.top;
    isMouseOverLayer = true;

    // Handle Dragging
    if (isDraggingStamp && dragStampId) {
      const coords = getMousePdfCoords(e);
      const dx = coords.x - dragStartX;
      const dy = coords.topY - dragStartY;

      placedStamps = placedStamps.map((s: PlacedStamp) => {
        if (s.id === dragStampId) {
          const newX = Math.max(0, Math.min(pageDimensions.width - s.width, initialStampX + dx));
          const newTopY = Math.max(0, Math.min(pageDimensions.height - s.height, initialStampTopY + dy));
          const newPdfY = pageDimensions.height - newTopY - s.height;
          return {
            ...s,
            x: Math.round(newX * 10) / 10,
            topY: Math.round(newTopY * 10) / 10,
            y: Math.round(newPdfY * 10) / 10
          };
        }
        return s;
      });
      onStampsChanged(placedStamps);
      return;
    }

    // Handle Resizing
    if (isResizing && resizeStampId && resizeHandle) {
      const currentHandle = resizeHandle;
      const coords = getMousePdfCoords(e);
      const dx = coords.x - resizeStartX;
      const dy = coords.topY - resizeStartY;

      placedStamps = placedStamps.map((s: PlacedStamp) => {
        if (s.id === resizeStampId) {
          let newX = initialResizeX;
          let newTopY = initialResizeTopY;
          let newW = initialResizeW;
          let newH = initialResizeH;
          const minSize = 25;

          if (currentHandle.includes('e')) {
            newW = Math.max(minSize, initialResizeW + dx);
          }
          if (currentHandle.includes('s')) {
            newH = Math.max(minSize, initialResizeH + dy);
          }
          if (currentHandle.includes('w')) {
            const rawW = initialResizeW - dx;
            if (rawW > minSize) {
              newW = rawW;
              newX = initialResizeX + dx;
            }
          }
          if (currentHandle.includes('n')) {
            const rawH = initialResizeH - dy;
            if (rawH > minSize) {
              newH = rawH;
              newTopY = initialResizeTopY + dy;
            }
          }

          const newPdfY = pageDimensions.height - newTopY - newH;
          return {
            ...s,
            x: Math.round(newX * 10) / 10,
            topY: Math.round(newTopY * 10) / 10,
            y: Math.round(newPdfY * 10) / 10,
            width: Math.round(newW * 10) / 10,
            height: Math.round(newH * 10) / 10
          };
        }
        return s;
      });
      onStampsChanged(placedStamps);
      return;
    }
  }

  function handleLayerPointerLeave() {
    isMouseOverLayer = false;
  }

  function handleLayerClick(e: MouseEvent) {
    // If we're armed with a signature, place it on click
    if (armedSignature) {
      const coords = getMousePdfCoords(e);

      // Default stamp dimensions in PDF points
      let width = armedSignature.width || 140;
      let height = armedSignature.height || 50;

      // Ensure reasonable initial size on the PDF (max 180pt width)
      if (width > 180) {
        const aspect = height / width;
        width = 180;
        height = Math.round(180 * aspect);
      }
      if (width < 60) width = 80;
      if (height < 25) height = 35;

      const topY = Math.max(0, Math.min(pageDimensions.height - height, coords.topY - height / 2));
      const x = Math.max(0, Math.min(pageDimensions.width - width, coords.x - width / 2));
      const pdfY = pageDimensions.height - topY - height;

      const newStamp: PlacedStamp = {
        id: 'stamp_' + Math.random().toString(36).substring(2, 9),
        page: pageNum,
        x: Math.round(x * 10) / 10,
        y: Math.round(pdfY * 10) / 10,
        topY: Math.round(topY * 10) / 10,
        width: Math.round(width * 10) / 10,
        height: Math.round(height * 10) / 10,
        dataUrl: armedSignature.dataUrl
      };

      placedStamps = [...placedStamps, newStamp];
      selectedStampId = newStamp.id;
      armedSignature = null;
      onStampPlaced(newStamp);
      onStampsChanged(placedStamps);
      return;
    }

    // Deselect if clicking on empty background
    if ((e.target as HTMLElement).classList.contains('signature-stamp-layer')) {
      selectedStampId = null;
    }
  }

  function handleStampPointerDown(stamp: PlacedStamp, e: PointerEvent) {
    e.stopPropagation();
    selectedStampId = stamp.id;
    isDraggingStamp = true;
    dragStampId = stamp.id;

    const coords = getMousePdfCoords(e);
    dragStartX = coords.x;
    dragStartY = coords.topY;
    initialStampX = stamp.x;
    initialStampTopY = stamp.topY;

    if (layerRef) {
      layerRef.setPointerCapture(e.pointerId);
    }
  }

  function handleResizePointerDown(stamp: PlacedStamp, handle: string, e: PointerEvent) {
    e.stopPropagation();
    isResizing = true;
    resizeHandle = handle;
    resizeStampId = stamp.id;

    const coords = getMousePdfCoords(e);
    resizeStartX = coords.x;
    resizeStartY = coords.topY;
    initialResizeX = stamp.x;
    initialResizeTopY = stamp.topY;
    initialResizeW = stamp.width;
    initialResizeH = stamp.height;

    if (layerRef) {
      layerRef.setPointerCapture(e.pointerId);
    }
  }

  function handleGlobalPointerUp(e: PointerEvent) {
    if (isDraggingStamp) {
      isDraggingStamp = false;
      dragStampId = null;
    }
    if (isResizing) {
      isResizing = false;
      resizeHandle = null;
      resizeStampId = null;
    }
    if (layerRef && e.pointerId) {
      try {
        layerRef.releasePointerCapture(e.pointerId);
      } catch {}
    }
  }

  function deleteStamp(id: string, e?: Event) {
    if (e) e.stopPropagation();
    placedStamps = placedStamps.filter((s: PlacedStamp) => s.id !== id);
    if (selectedStampId === id) selectedStampId = null;
    onStampsChanged(placedStamps);
  }

  function handleKeydown(e: KeyboardEvent) {
    if ((e.key === 'Delete' || e.key === 'Backspace') && selectedStampId) {
      const activeEl = document.activeElement;
      if (activeEl && (activeEl.tagName === 'INPUT' || activeEl.tagName === 'TEXTAREA')) return;
      deleteStamp(selectedStampId);
    } else if (e.key === 'Escape') {
      if (armedSignature) {
        armedSignature = null;
      } else {
        selectedStampId = null;
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  bind:this={layerRef}
  class="signature-stamp-layer"
  class:placing={!!armedSignature}
  onpointermove={handleLayerPointerMove}
  onpointerleave={handleLayerPointerLeave}
  onpointerup={handleGlobalPointerUp}
  onclick={handleLayerClick}
>
  <!-- Ghost preview following cursor during armed placement mode -->
  {#if armedSignature && isMouseOverLayer}
    {@const previewW = (armedSignature.width > 180 ? 180 : Math.max(80, armedSignature.width)) * scale}
    {@const previewH = (armedSignature.width > 180 ? (180 * (armedSignature.height / armedSignature.width)) : Math.max(35, armedSignature.height)) * scale}
    <div
      class="ghost-stamp-preview"
      style="left: {mouseX - previewW / 2}px; top: {mouseY - previewH / 2}px; width: {previewW}px; height: {previewH}px;"
    >
      <img src={armedSignature.dataUrl} alt="Signature Ghost Preview" class="ghost-img" />
      <span class="ghost-hint">Click page to stamp</span>
    </div>
  {/if}

  <!-- Placed Stamps for the Current Page -->
  {#each placedStamps.filter((s: PlacedStamp) => s.page === pageNum) as stamp (stamp.id)}
    {@const isSelected = selectedStampId === stamp.id}
    <div
      class="placed-stamp-box"
      class:selected={isSelected}
      style="left: {stamp.x * scale}px; top: {stamp.topY * scale}px; width: {stamp.width * scale}px; height: {stamp.height * scale}px;"
      onpointerdown={(e) => handleStampPointerDown(stamp, e)}
    >
      <img src={stamp.dataUrl} alt="Placed signature stamp" class="stamp-image" draggable="false" />

      {#if isSelected}
        <!-- Drag Handle Indicator -->
        <div class="stamp-drag-badge" title="Drag to reposition">
          ⋮⋮ Move
        </div>

        <!-- Delete Button -->
        <button
          type="button"
          class="stamp-delete-btn"
          title="Delete signature (or press Backspace)"
          onclick={(e) => deleteStamp(stamp.id, e)}
          onpointerdown={(e) => e.stopPropagation()}
        >
          ✕
        </button>

        <!-- 4 Corner Resize Handles -->
        <div
          class="resize-handle handle-nw"
          title="Resize top-left"
          onpointerdown={(e) => handleResizePointerDown(stamp, 'nw', e)}
        ></div>
        <div
          class="resize-handle handle-ne"
          title="Resize top-right"
          onpointerdown={(e) => handleResizePointerDown(stamp, 'ne', e)}
        ></div>
        <div
          class="resize-handle handle-se"
          title="Resize bottom-right"
          onpointerdown={(e) => handleResizePointerDown(stamp, 'se', e)}
        ></div>
        <div
          class="resize-handle handle-sw"
          title="Resize bottom-left"
          onpointerdown={(e) => handleResizePointerDown(stamp, 'sw', e)}
        ></div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .signature-stamp-layer {
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 25;
    overflow: hidden;
  }

  .signature-stamp-layer.placing {
    pointer-events: auto;
    cursor: crosshair;
  }

  .ghost-stamp-preview {
    position: absolute;
    pointer-events: none;
    opacity: 0.65;
    border: 1.5px dashed #2563eb;
    background: rgba(37, 99, 235, 0.08);
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    transform: translate(0, 0);
    z-index: 40;
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.2);
  }

  .ghost-img {
    max-width: 90%;
    max-height: 80%;
    object-fit: contain;
  }

  .ghost-hint {
    position: absolute;
    bottom: -22px;
    font-size: 11px;
    font-weight: 600;
    color: #ffffff;
    background: #1e40af;
    padding: 2px 8px;
    border-radius: 4px;
    white-space: nowrap;
  }

  .placed-stamp-box {
    position: absolute;
    pointer-events: auto;
    cursor: move;
    border: 1.5px solid transparent;
    user-select: none;
    touch-action: none;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.1s ease;
  }

  .placed-stamp-box:hover {
    border-color: #93c5fd;
    background: rgba(37, 99, 235, 0.03);
  }

  .placed-stamp-box.selected {
    border: 2px solid #2563eb;
    background: rgba(37, 99, 235, 0.05);
    box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.8), 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .stamp-image {
    width: 100%;
    height: 100%;
    object-fit: contain;
    pointer-events: none;
  }

  .stamp-drag-badge {
    position: absolute;
    top: -24px;
    left: 0;
    background: #2563eb;
    color: #ffffff;
    font-size: 10px;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: 4px;
    pointer-events: none;
    letter-spacing: 0.02em;
  }

  .stamp-delete-btn {
    position: absolute;
    top: -12px;
    right: -12px;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: #ef4444;
    color: #ffffff;
    border: 2px solid #ffffff;
    font-size: 12px;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    transition: transform 0.15s ease, background 0.15s ease;
    padding: 0;
    line-height: 1;
    z-index: 30;
  }

  .stamp-delete-btn:hover {
    background: #dc2626;
    transform: scale(1.15);
  }

  .resize-handle {
    position: absolute;
    width: 10px;
    height: 10px;
    background: #ffffff;
    border: 2px solid #2563eb;
    border-radius: 2px;
    z-index: 28;
  }

  .handle-nw {
    top: -5px;
    left: -5px;
    cursor: nwse-resize;
  }

  .handle-ne {
    top: -5px;
    right: -5px;
    cursor: nesw-resize;
  }

  .handle-se {
    bottom: -5px;
    right: -5px;
    cursor: nwse-resize;
  }

  .handle-sw {
    bottom: -5px;
    left: -5px;
    cursor: nesw-resize;
  }
</style>
