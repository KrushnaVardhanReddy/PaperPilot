<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let { onClose }: { onClose: () => void } = $props();

  let videoElement = $state<HTMLVideoElement | null>(null);
  let canvasElement = $state<HTMLCanvasElement | null>(null);
  let mediaStream = $state<MediaStream | null>(null);

  let capturedImages = $state<string[]>([]); // Data URLs
  let isOcrEnabled = $state(true);
  let isConverting = $state(false);

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

    const dataUrl = canvasElement.toDataURL('image/jpeg', 0.8);
    capturedImages = [...capturedImages, dataUrl];
  }

  function deletePhoto(index: number) {
    capturedImages = capturedImages.filter((_, i) => i !== index);
  }

  async function convertToPdf() {
    if (capturedImages.length === 0) return;

    isConverting = true;
    try {
      // Create a multi-part JSON containing the base64 images to pass to process_mobile_buffer.
      // Wait, process_mobile_buffer expects input_bytes of the main file. We can pass a dummy file and pass the images in params, OR since it is "images_to_pdf", the mobile bridge might not have this specifically yet!
      // Let's implement it in mobile bridge if it's missing, but here we invoke it.

      const payload = {
          images: capturedImages.map(d => d.split(',')[1]), // base64 parts
          ocr: isOcrEnabled
      };

      // We will pass the first image as the main buffer, or an empty buffer if images_to_pdf takes an array of images.
      const dummyBytes = new Uint8Array(0);

      const res: any = await invoke('process_mobile_buffer', {
        fileName: "scanned.pdf",
        inputBytes: Array.from(dummyBytes), // Need Array of numbers for rust
        operation: "images_to_pdf",
        params: payload
      });

      if (res.success) {
        // res.dataBase64
        const binaryStr = atob(res.dataBase64);
        const len = binaryStr.length;
        const bytes = new Uint8Array(len);
        for (let i = 0; i < len; i++) {
            bytes[i] = binaryStr.charCodeAt(i);
        }
        const blob = new Blob([bytes], { type: 'application/pdf' });
        const file = new File([blob], res.fileName, { type: 'application/pdf' });
        (file as any)._localPath = res.fileName;
        (file as any)._isLoaded = true;

        appState.addDocuments([file]);
        toastState.success(`Created PDF from ${capturedImages.length} images`);
        onClose();
      } else {
        toastState.error("Failed to convert", res.message || "Unknown error");
      }
    } catch (err: any) {
       // fallback toast
       if (err && typeof err === 'string' && err.includes('model')) {
          toastState.error("OCR Model missing. Please download the offline pack.");
       } else {
          toastState.error("Conversion failed", String(err));
       }
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
            <img src={img} alt="Scanned page {i+1}" />
            <div class="carousel-actions">
              <span>Page {i+1}</span>
              <button onclick={() => deletePhoto(i)}>🗑</button>
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
