<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { onMount, onDestroy } from 'svelte';

  import ImageCropModal from './ImageCropModal.svelte';

  let { onClose }: { onClose: () => void } = $props();

  let videoElement = $state<HTMLVideoElement | null>(null);
  let canvasElement = $state<HTMLCanvasElement | null>(null);
  let mediaStream = $state<MediaStream | null>(null);

  let capturedImages = $state<string[]>([]); // Data URLs
  let isOcrEnabled = $state(true);
  let isConverting = $state(false);

  // Crop modal state
  let croppingImageIndex = $state<number | null>(null);
  let croppingImageSrc = $state<string | null>(null);

  async function startCamera() {
    try {
      mediaStream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: "environment" }
      });
      if (videoElement) {
        videoElement.srcObject = mediaStream;
        await videoElement.play();
      }
    } catch (err) {
      console.error("Camera error:", err);
      toastState.error("Could not access camera");
    }
  }

  function stopCamera() {
    if (mediaStream) {
      mediaStream.getTracks().forEach(track => track.stop());
      mediaStream = null;
    }
  }

  onMount(() => {
    startCamera();
  });

  onDestroy(() => {
    stopCamera();
  });

  function takePhoto() {
    if (!videoElement || !canvasElement) return;

    const context = canvasElement.getContext('2d');
    if (!context) return;

    canvasElement.width = videoElement.videoWidth;
    canvasElement.height = videoElement.videoHeight;
    context.drawImage(videoElement, 0, 0, canvasElement.width, canvasElement.height);

    const dataUrl = canvasElement.toDataURL('image/jpeg', 0.9);
    // Immediately open the crop tool for this newly taken photo (Kaagaz workflow)
    croppingImageIndex = capturedImages.length;
    croppingImageSrc = dataUrl;
  }

  function handleCropConfirm(croppedDataUrl: string) {
    if (croppingImageIndex !== null) {
      if (croppingImageIndex >= capturedImages.length) {
        capturedImages = [...capturedImages, croppedDataUrl];
      } else {
        capturedImages[croppingImageIndex] = croppedDataUrl;
      }
    }
    croppingImageIndex = null;
    croppingImageSrc = null;
  }

  function handleCropCancel() {
    // If cancelling a new photo that wasn't saved yet, just discard or keep raw
    croppingImageIndex = null;
    croppingImageSrc = null;
  }

  function editPhoto(index: number) {
    croppingImageIndex = index;
    croppingImageSrc = capturedImages[index];
  }

  function deletePhoto(index: number) {
    capturedImages = capturedImages.filter((_, i) => i !== index);
  }

  async function convertToPdf() {
    if (capturedImages.length === 0) return;

    isConverting = true;
    try {
      const { WasmPdfClientImpl } = await import('$lib/wasm/index');
      const client = new WasmPdfClientImpl();

      const payloadFiles = capturedImages.map(d => {
        const base64Data = d.split(',')[1];
        const binaryStr = atob(base64Data);
        const len = binaryStr.length;
        const bytes = new Uint8Array(len);
        for (let i = 0; i < len; i++) {
          bytes[i] = binaryStr.charCodeAt(i);
        }
        return bytes;
      });

      const res = await (client as any).runOperation('images_to_pdf', { files: payloadFiles });

      if (res && res.length > 0) {
        let finalRes = res;

        // Handle OCR if enabled
        if (isOcrEnabled) {
             // Currently not wiring up OCR to avoid complexity if it doesn't exist yet for WASM OCR.
        }

        const blob = new Blob([finalRes], { type: 'application/pdf' });
        const fileName = "scanned_" + Date.now() + ".pdf";
        const file = new File([blob], fileName, { type: 'application/pdf' });
        (file as any)._localPath = fileName;
        (file as any)._isLoaded = true;

        appState.addDocuments([file]);
        toastState.success(`Created PDF from ${capturedImages.length} images`);
        onClose();
      } else {
        throw new Error('Empty result from PDF generation');
      }
    } catch (err: any) {
      console.error(err);
      toastState.error('Failed to convert to PDF: ' + (err.message || String(err)));
    } finally {
      isConverting = false;
    }
  }
</script>

<div class="scanner-modal">
  <div class="scanner-header">
    <button class="close-btn" onclick={onClose}>×</button>
    <h2>Document Scanner</h2>
    <div class="ocr-toggle">
      <label>
        <input type="checkbox" bind:checked={isOcrEnabled} />
        Make Searchable (OCR)
      </label>
    </div>
  </div>

  <div class="viewfinder-container">
    <!-- svelte-ignore a11y_media_has_caption -->
    <video bind:this={videoElement} autoplay playsinline></video>
    <canvas bind:this={canvasElement} style="display: none;"></canvas>
  </div>

  <div class="controls-container">
    {#if capturedImages.length > 0}
      <div class="carousel">
        {#each capturedImages as img, i}
          <div class="carousel-item">
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <img src={img} alt="Scanned page {i+1}" onclick={() => editPhoto(i)} title="Tap to crop/adjust" />
            <div class="carousel-actions">
              <button class="crop-action-btn" onclick={() => editPhoto(i)} title="Crop">✂️</button>
              <span>p.{i+1}</span>
              <button onclick={() => deletePhoto(i)} title="Delete">🗑</button>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <div class="action-bar">
      <button class="shutter-btn" onclick={takePhoto} aria-label="Take photo"></button>
      {#if capturedImages.length > 0}
        <button class="convert-btn" onclick={convertToPdf} disabled={isConverting}>
          {isConverting ? 'Processing...' : `Convert to PDF (${capturedImages.length})`}
        </button>
      {/if}
    </div>
  </div>

  {#if croppingImageSrc}
    <ImageCropModal
      imageSrc={croppingImageSrc}
      onConfirm={handleCropConfirm}
      onCancel={handleCropCancel}
    />
  {/if}
</div>

<style>
  .scanner-modal {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: #000;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    color: white;
  }
  .scanner-header {
    display: flex;
    align-items: center;
    padding: 16px;
    background: rgba(0,0,0,0.5);
    position: absolute;
    top: 0; left: 0; right: 0;
    z-index: 10;
  }
  .scanner-header h2 {
    margin: 0;
    font-size: 1.2rem;
    flex: 1;
    text-align: center;
  }
  .close-btn {
    background: none; border: none; color: white; font-size: 2rem; cursor: pointer;
  }
  .ocr-toggle {
    font-size: 0.8rem;
  }
  .viewfinder-container {
    flex: 1;
    background: #111;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
  }
  video {
    min-width: 100%;
    min-height: 100%;
    object-fit: cover;
  }
  .controls-container {
    position: absolute;
    bottom: 0; left: 0; right: 0;
    padding: 16px;
    background: rgba(0,0,0,0.8);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .carousel {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    padding-bottom: 8px;
  }
  .carousel-item {
    width: 60px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .carousel-item img {
    width: 100%;
    aspect-ratio: 1/1.414;
    object-fit: cover;
    border: 1px solid #444;
    border-radius: 4px;
  }
  .carousel-actions {
    display: flex;
    justify-content: space-between;
    width: 100%;
    font-size: 0.7rem;
    margin-top: 4px;
  }
  .carousel-actions button {
    background: none; border: none; color: #ff4444; cursor: pointer;
  }
  .action-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .shutter-btn {
    width: 64px; height: 64px;
    border-radius: 50%;
    border: 4px solid white;
    background: rgba(255,255,255,0.3);
    cursor: pointer;
    margin: 0 auto;
  }
  .shutter-btn:active {
    background: rgba(255,255,255,0.8);
  }
  .convert-btn {
    padding: 12px 24px;
    background: #5e6ad2;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: bold;
    cursor: pointer;
  }
  .convert-btn:disabled {
    opacity: 0.5;
  }
</style>
