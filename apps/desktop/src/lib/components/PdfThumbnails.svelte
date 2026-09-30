<script lang="ts">
  import { tick } from 'svelte';

  let { pdfDoc, pageNum = $bindable(1) } = $props();

  let numPages = $derived(pdfDoc ? pdfDoc.numPages : 0);
  let isRendering = false; // Not a $state to prevent infinite loops in effects

  // We need to keep track of canvas elements
  let canvases: Record<number, HTMLCanvasElement> = {};

  async function renderThumbnails() {
    if (!pdfDoc || numPages === 0 || isRendering) return;

    isRendering = true;

    // Wait for Svelte to create the DOM elements
    await tick();

    try {
      for (let i = 1; i <= numPages; i++) {
        const canvas = canvases[i];
        if (!canvas) continue;

        const page = await pdfDoc.getPage(i);
        // Render at a small scale for thumbnail
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
    // When pdfDoc changes, render all thumbnails
    if (pdfDoc) {
      renderThumbnails();
    }
  });

  function selectPage(i: number) {
    pageNum = i;
  }
</script>

<div class="thumbnail-strip">
  {#if numPages > 0}
    {#each Array.from({ length: numPages }, (_, i) => i + 1) as i}
      <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
      <div
        class="thumbnail-container {pageNum === i ? 'active' : ''}"
        onclick={() => selectPage(i)}
      >
        <div class="page-number">{i}</div>
        <canvas bind:this={canvases[i]}></canvas>
      </div>
    {/each}
  {/if}
</div>

<style>
  .thumbnail-strip {
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    width: 200px;
    background: #f9fafb;
    border-right: 1px solid #e5e7eb;
    padding: 1rem;
    gap: 1rem;
    align-items: center;
  }

  .thumbnail-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: pointer;
    padding: 8px;
    border-radius: 4px;
    border: 2px solid transparent;
    transition: all 0.2s;
  }

  .thumbnail-container:hover {
    background-color: #e5e7eb;
  }

  .thumbnail-container.active {
    background-color: #e5e7eb;
    border-color: var(--accent-primary, #3b82f6);
  }

  .page-number {
    font-size: 0.8rem;
    color: var(--text-secondary, #4b5563);
    margin-bottom: 4px;
  }

  canvas {
    background-color: white;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
    max-width: 150px; /* Ensure they fit in the strip */
  }
</style>
