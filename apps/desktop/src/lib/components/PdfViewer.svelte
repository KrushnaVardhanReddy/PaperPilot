<script lang="ts">
  import { onMount } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';

  // Vite worker import trick
  import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.mjs?url';
  import PdfAnnotationLayer from './PdfAnnotationLayer.svelte';
  import PdfSearchBar from './PdfSearchBar.svelte';
  import type { Annotation } from '$lib/api/pdf';

  pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;

  let {
    fileUrl = '',
    pdfDoc = $bindable(null),
    pageNum = $bindable(1),
    scale = $bindable(1.2),
    activeTool = 'none',
    annotations = $bindable([] as Annotation[])
  } = $props();

  let canvas: HTMLCanvasElement;
  let isRendering = false; // Not a $state to prevent infinite loops in effects

  async function loadDocument(url: string) {
    if (!url) return;
    try {
      const loadingTask = pdfjsLib.getDocument({ url });
      pdfDoc = await loadingTask.promise;
      pageNum = 1;
    } catch (err) {
      console.error("Error loading PDF:", err);
    }
  }

  async function renderPage(doc: pdfjsLib.PDFDocumentProxy | null, currentPage: number, currentScale: number) {
    if (!doc || !canvas || isRendering) return;
    isRendering = true;

    try {
      const page = await doc.getPage(currentPage);
      const viewport = page.getViewport({ scale: currentScale });

      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      // Handle high-DPI displays (retina screens)
      const outputScale = window.devicePixelRatio || 1;
      canvas.width = Math.floor(viewport.width * outputScale);
      canvas.height = Math.floor(viewport.height * outputScale);

      // The CSS width/height should match the unscaled viewport to map correctly with 1x zoom layer
      canvas.style.width = Math.floor(viewport.width) + "px";
      canvas.style.height = Math.floor(viewport.height) + "px";

      const transform = outputScale !== 1
        ? [outputScale, 0, 0, outputScale, 0, 0]
        : null;

      const renderContext = {
        canvasContext: ctx,
        transform: transform ? transform : undefined,
        viewport,
        canvas, // added required property
      };

      await page.render(renderContext as any).promise;
    } catch (err) {
      console.error("Render error:", err);
    } finally {
      isRendering = false;
    }
  }

  // Reactively load document when fileUrl changes
  $effect(() => {
    if (fileUrl) {
      loadDocument(fileUrl);
    }
  });

  // Reactively re-render when dependencies change
  $effect(() => {
    // Read states synchronously so Svelte tracks them
    renderPage(pdfDoc, pageNum, scale);
  });


  $effect(() => {
    const handleFit = () => {
      if (pdfDoc && canvas) {
         pdfDoc.getPage(pageNum).then((page: pdfjsLib.PDFPageProxy) => {
           const viewport = page.getViewport({ scale: 1.0 });
           const container = canvas.closest('.viewer-container');
           if (container) {
             const containerWidth = container.clientWidth - 64; // account for 2rem padding
             const newScale = containerWidth / viewport.width;
             scale = Math.min(Math.max(newScale, 0.2), 5.0);
           }
         });
      }
    };
    window.addEventListener('paperpilot:fit-viewport', handleFit);
    return () => {
      window.removeEventListener('paperpilot:fit-viewport', handleFit);
    };
  });

  import type { SearchMatchItem } from './PdfSearchBar.svelte';
  let searchHighlights: SearchMatchItem[] = $state([]);
  let viewportState: any = $state(null);

  $effect(() => {
      if (pdfDoc && pageNum && scale) {
          pdfDoc.getPage(pageNum).then((page: pdfjsLib.PDFPageProxy) => {
              viewportState = page.getViewport({ scale: scale });
          });
      }
  });

  function handleGlobalKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f' && !e.shiftKey) {
      e.preventDefault();
      window.dispatchEvent(new CustomEvent('paperpilot:search-open'));
    }
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="pdf-viewer-wrapper">
  <PdfSearchBar {pdfDoc} bind:pageNum bind:searchHighlights />
  <div class="viewer-container">
    <div class="page-container">
      <canvas bind:this={canvas} style="display: block;"></canvas>
      {#if viewportState}
        <div class="search-highlights-layer" style="width: {viewportState.width}px; height: {viewportState.height}px;">
           {#each searchHighlights as item}
               {@const pt = pdfjsLib.Util.normalizeRect([
                   item.x,
                   item.y,
                   item.x + item.width,
                   item.y + item.height
               ])}
               {@const vpPt1 = viewportState.convertToViewportPoint(pt[0], pt[1])}
               {@const vpPt2 = viewportState.convertToViewportPoint(pt[2], pt[3])}
               <div class="search-highlight" style="left: {vpPt1[0]}px; top: {vpPt2[1]}px; width: {vpPt2[0] - vpPt1[0]}px; height: {vpPt1[1] - vpPt2[1]}px;"></div>
           {/each}
        </div>
      {/if}
      <PdfAnnotationLayer {scale} {pageNum} bind:annotations {activeTool} />
    </div>
  </div>
</div>

<style>
  .pdf-viewer-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    position: relative;
  }

  .viewer-container {
    display: flex;
    justify-content: center;
    overflow: auto;
    background-color: var(--bg-secondary, #f3f4f6);
    padding: 2rem;
    flex: 1;
  }

  .page-container {
    position: relative;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    background-color: white;
    /* Ensure the container sizes tightly around the canvas */
    display: inline-block;
    height: fit-content;
  }

  canvas {
    max-width: 100%; /* Ensure canvas doesn't break layout unexpectedly */
    object-fit: contain;
  }

  .search-highlights-layer {
    position: absolute;
    top: 0;
    left: 0;
    pointer-events: none;
    z-index: 10;
  }

  .search-highlight {
    position: absolute;
    background-color: rgba(255, 255, 0, 0.4);
    border-radius: 2px;
  }
</style>
