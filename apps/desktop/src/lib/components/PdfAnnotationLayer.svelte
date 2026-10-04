<script lang="ts">
  import type { Annotation } from '$lib/api/pdf';

  let {
    scale = 1.0,
    pageNum = 1,
    annotations = $bindable([] as Annotation[]),
    activeTool = 'none'
  } = $props();

  let layerRef: HTMLDivElement;
  let isDrawing = $state(false);
  let currentPath: {x: number, y: number}[] = $state([]);
  let currentSelection: {x: number, y: number, w: number, h: number} | null = $state(null);
  let startX = $state(0);
  let startY = $state(0);

  // Generate a random ID
  function generateId() {
    return Math.random().toString(36).substring(2, 9);
  }

  function getMouseCoordinates(e: MouseEvent | PointerEvent) {
    if (!layerRef) return { x: 0, y: 0 };
    const rect = layerRef.getBoundingClientRect();
    // The layer itself is scaled by CSS.
    // getBoundingClientRect returns the scaled bounds, but we want coordinates in the layer's unscaled coordinate space
    // to map to the PDF canvas correctly (since canvas handles scale internally).
    const x = (e.clientX - rect.left) / scale;
    const y = (e.clientY - rect.top) / scale;
    return { x, y };
  }

  function handlePointerDown(e: PointerEvent) {
    if (activeTool === 'none') return;

    // Notes are placed immediately on click, disregard button check because agent fires weird button events
    if (activeTool === 'note') {
      const { x, y } = getMouseCoordinates(e);
      annotations = [...annotations, {
        id: generateId(),
        type: 'note',
        page: pageNum,
        x,
        y,
        color: '#fef08a',
        content: 'New Note'
      }];
      return;
    }

    // Only handle main button clicks
    if (e.button !== 0) return;

    layerRef.setPointerCapture(e.pointerId);
    isDrawing = true;

    const { x, y } = getMouseCoordinates(e);
    startX = x;
    startY = y;

    if (activeTool === 'pen') {
      currentPath = [{ x, y }];
    } else if (['highlight', 'underline', 'strikethrough'].includes(activeTool)) {
      currentSelection = { x, y, w: 0, h: 0 };
    }
  }

  function handlePointerMove(e: PointerEvent) {
    if (!isDrawing) return;

    const { x, y } = getMouseCoordinates(e);

    if (activeTool === 'pen') {
      currentPath = [...currentPath, { x, y }];
    } else if (['highlight', 'underline', 'strikethrough'].includes(activeTool)) {
      currentSelection = {
        x: Math.min(startX, x),
        y: Math.min(startY, y),
        w: Math.abs(x - startX),
        h: Math.abs(y - startY)
      };
    }
  }

  function handlePointerUp(e: PointerEvent) {
    if (!isDrawing) return;
    isDrawing = false;
    layerRef.releasePointerCapture(e.pointerId);

    if (activeTool === 'pen' && currentPath.length > 1) {
      annotations = [...annotations, {
        id: generateId(),
        type: 'pen',
        page: pageNum,
        x: Math.min(...currentPath.map(p => p.x)), // approximate bounds
        y: Math.min(...currentPath.map(p => p.y)),
        color: '#dc2626', // Default red pen
        path: [...currentPath]
      }];

      currentPath = [];
    } else if (['highlight', 'underline', 'strikethrough'].includes(activeTool) && currentSelection && currentSelection.w > 5) {
      let color = '#fef08a'; // yellow highlight
      if (activeTool === 'underline') color = '#2563eb'; // blue underline
      if (activeTool === 'strikethrough') color = '#dc2626'; // red strike

      annotations = [...annotations, {
        id: generateId(),
        type: activeTool as 'highlight' | 'underline' | 'strikethrough',
        page: pageNum,
        x: currentSelection.x,
        y: currentSelection.y,
        w: currentSelection.w,
        h: currentSelection.h,
        color
      }];

      currentSelection = null;
    }
  }
</script>

<!-- The layer uses pointer events to track input, and it matches the scale of the canvas directly -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  bind:this={layerRef}
  class="annotation-layer"
  class:interactive={activeTool !== 'none'}
  onpointerdown={handlePointerDown}
  onpointermove={handlePointerMove}
  onpointerup={handlePointerUp}
>
  <!-- Render existing annotations for this page -->
  <!-- We apply the scale internally to the annotations positions to match the canvas scale visually -->
  {#each annotations.filter(a => a.page === pageNum) as annotation (annotation.id)}
    {#if annotation.type === 'highlight'}
      <div
        class="annot-highlight"
        style="left: {annotation.x * scale}px; top: {annotation.y * scale}px; width: {(annotation.w || 0) * scale}px; height: {(annotation.h || 0) * scale}px; background-color: {annotation.color};"
      ></div>
    {:else if annotation.type === 'underline'}
      <div
        class="annot-underline"
        style="left: {annotation.x * scale}px; top: {(annotation.y + (annotation.h || 0)) * scale}px; width: {(annotation.w || 0) * scale}px; background-color: {annotation.color};"
      ></div>
    {:else if annotation.type === 'strikethrough'}
      <div
        class="annot-strike"
        style="left: {annotation.x * scale}px; top: {(annotation.y + (annotation.h || 0)/2) * scale}px; width: {(annotation.w || 0) * scale}px; background-color: {annotation.color};"
      ></div>
    {:else if annotation.type === 'note'}
      <div
        class="annot-note"
        style="left: {annotation.x * scale}px; top: {annotation.y * scale}px;"
        title={annotation.content}
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill={annotation.color} stroke="#374151" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
        </svg>
      </div>
    {:else if annotation.type === 'pen' && annotation.path}
      <svg class="annot-svg" style="left: 0; top: 0;">
        <polyline
          points={annotation.path.map(p => `${p.x * scale},${p.y * scale}`).join(' ')}
          fill="none"
          stroke={annotation.color}
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    {/if}
  {/each}

  <!-- Render current drawing/selection -->
  {#if isDrawing}
    {#if activeTool === 'pen' && currentPath.length > 0}
      <svg class="annot-svg drawing" style="left: 0; top: 0;">
        <polyline
          points={currentPath.map(p => `${p.x * scale},${p.y * scale}`).join(' ')}
          fill="none"
          stroke="#dc2626"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    {:else if ['highlight', 'underline', 'strikethrough'].includes(activeTool) && currentSelection}
      {#if activeTool === 'highlight'}
        <div
          class="annot-highlight drawing"
          style="left: {currentSelection.x * scale}px; top: {currentSelection.y * scale}px; width: {currentSelection.w * scale}px; height: {currentSelection.h * scale}px; background-color: #fef08a;"
        ></div>
      {:else if activeTool === 'underline'}
        <div
          class="annot-underline drawing"
          style="left: {currentSelection.x * scale}px; top: {(currentSelection.y + currentSelection.h) * scale}px; width: {currentSelection.w * scale}px; background-color: #2563eb;"
        ></div>
      {:else if activeTool === 'strikethrough'}
        <div
          class="annot-strike drawing"
          style="left: {currentSelection.x * scale}px; top: {(currentSelection.y + currentSelection.h/2) * scale}px; width: {currentSelection.w * scale}px; background-color: #dc2626;"
        ></div>
      {/if}
    {/if}
  {/if}
</div>

<style>
  .annotation-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none; /* Let clicks pass through when not annotating */
    z-index: 5;
  }

  .annotation-layer.interactive {
    pointer-events: auto;
    cursor: crosshair;
  }

  .annot-highlight {
    position: absolute;
    mix-blend-mode: multiply;
    opacity: 0.5;
    pointer-events: none;
  }

  .annot-underline, .annot-strike {
    position: absolute;
    height: 2px;
    pointer-events: none;
  }

  .annot-note {
    position: absolute;
    transform: translate(-12px, -24px); /* Center pointer of icon on click */
    cursor: pointer;
    pointer-events: auto;
    filter: drop-shadow(0 2px 4px rgba(0,0,0,0.2));
  }

  .annot-note:hover {
    transform: translate(-12px, -24px) scale(1.1);
    z-index: 10;
  }

  .annot-svg {
    position: absolute;
    width: 100%;
    height: 100%;
    pointer-events: none;
    overflow: visible;
  }
</style>
