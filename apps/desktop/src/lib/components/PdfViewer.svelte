<script lang="ts">
  import { onMount } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';

  // Vite worker import trick
  import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.mjs?url';
  import PdfAnnotationLayer from './PdfAnnotationLayer.svelte';
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
</script>

<div class="pdf-viewer-wrapper">
  <div class="viewer-container">
    <div class="page-container">
      <canvas bind:this={canvas} style="display: block;"></canvas>
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
</style>
