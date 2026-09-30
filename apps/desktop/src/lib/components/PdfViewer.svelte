<script lang="ts">
  import { onMount } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';

  // Vite worker import trick
  import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.mjs?url';

  import PdfToolbar from './PdfToolbar.svelte';

  pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;

  let { fileUrl = '' } = $props();

  let canvas: HTMLCanvasElement;
  let pdfDoc: pdfjsLib.PDFDocumentProxy | null = $state(null);
  let pageNum = $state(1);
  let scale = $state(1.2);
  let numPages = $state(0);
  let isRendering = false; // Not a $state to prevent infinite loops in effects

  async function loadDocument(url: string) {
    if (!url) return;
    try {
      const loadingTask = pdfjsLib.getDocument({ url });
      pdfDoc = await loadingTask.promise;
      numPages = pdfDoc.numPages;
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
  <PdfToolbar bind:pageNum bind:scale {numPages} />

  <div class="viewer-container">
    <canvas bind:this={canvas}></canvas>
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

  canvas {
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    background-color: white;
    max-width: 100%; /* Ensure canvas doesn't break layout unexpectedly */
    object-fit: contain;
  }
</style>
