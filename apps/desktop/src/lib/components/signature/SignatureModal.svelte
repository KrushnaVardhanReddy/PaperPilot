<script lang="ts">
  import { onMount } from 'svelte';
  import SignaturePadCanvas from './SignaturePadCanvas.svelte';
  import SignatureTypePreview from './SignatureTypePreview.svelte';

  export interface AdoptedSignature {
    id: string;
    dataUrl: string;
    width: number;
    height: number;
    label: string;
    type: 'type' | 'draw' | 'upload' | 'saved';
    isInitials?: boolean;
    createdAt?: number;
  }

  let {
    isOpen = true,
    initialTab = 'type' as 'type' | 'draw' | 'upload',
    onAdopt = (sig: AdoptedSignature) => {},
    onClose = () => {}
  } = $props<{
    isOpen?: boolean;
    initialTab?: 'type' | 'draw' | 'upload';
    onAdopt?: (sig: AdoptedSignature) => void;
    onClose?: () => void;
  }>();

  let activeTab = $state<'type' | 'draw' | 'upload'>('type');
  let typePreviewRef: any = $state(null);
  let padCanvasRef: any = $state(null);

  // Type tab state
  let typedName = $state('John Doe');
  let typedInitials = $state('JD');
  let typedColor = $state('#000000');
  let typedFontId = $state('caveat');
  let isInitialsMode = $state(false);

  // Draw tab state
  let drawColor = $state('#000000');
  let isPadEmpty = $state(true);

  // Upload tab state
  let uploadedDataUrl = $state<string | null>(null);
  let uploadedWidth = $state(0);
  let uploadedHeight = $state(0);
  let uploadedFileName = $state('');
  let isDraggingFile = $state(false);
  let fileInputRef: HTMLInputElement | null = $state(null);

  // Saved signatures state
  let savedSignatures = $state<AdoptedSignature[]>([]);

  const SAVED_SIGS_KEY = 'paperpilot:saved_signatures';

  function loadSavedSignatures() {
    if (typeof localStorage === 'undefined') return;
    try {
      const raw = localStorage.getItem(SAVED_SIGS_KEY);
      if (raw) {
        savedSignatures = JSON.parse(raw);
      }
    } catch (e) {
      console.warn('Failed to load saved signatures from localStorage', e);
    }
  }

  function persistSavedSignature(sig: AdoptedSignature) {
    if (typeof localStorage === 'undefined') return;
    try {
      const filtered = savedSignatures.filter(s => s.id !== sig.id);
      const updated = [sig, ...filtered].slice(0, 8); // Keep up to 8
      localStorage.setItem(SAVED_SIGS_KEY, JSON.stringify(updated));
      savedSignatures = updated;
    } catch (e) {
      console.warn('Failed to save signature to localStorage', e);
    }
  }

  function deleteSavedSignature(id: string, e?: Event) {
    if (e) e.stopPropagation();
    savedSignatures = savedSignatures.filter(s => s.id !== id);
    if (typeof localStorage !== 'undefined') {
      try {
        localStorage.setItem(SAVED_SIGS_KEY, JSON.stringify(savedSignatures));
      } catch {}
    }
  }

  function handleFileSelected(file: File) {
    if (!file || !file.type.startsWith('image/')) return;
    uploadedFileName = file.name;

    const reader = new FileReader();
    reader.onload = (e) => {
      const src = e.target?.result as string;
      if (!src) return;

      const img = new Image();
      img.onload = () => {
        // Remove white background on offscreen canvas
        const canvas = document.createElement('canvas');
        canvas.width = img.width;
        canvas.height = img.height;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        ctx.drawImage(img, 0, 0);
        const imgData = ctx.getImageData(0, 0, img.width, img.height);
        const data = imgData.data;

        let minX = img.width, minY = img.height, maxX = 0, maxY = 0;
        let hasInk = false;

        for (let i = 0; i < data.length; i += 4) {
          const r = data[i];
          const g = data[i + 1];
          const b = data[i + 2];
          const a = data[i + 3];

          // Calculate luminance
          const luminance = 0.299 * r + 0.587 * g + 0.114 * b;

          if (luminance > 220 || a < 20) {
            // Paper background: make transparent
            data[i + 3] = 0;
          } else if (luminance > 160) {
            // Soft anti-aliased edge
            const alphaRatio = 1.0 - (luminance - 160) / 60;
            data[i + 3] = Math.round(a * alphaRatio);
          } else {
            // Ink pixel
            const pixelIndex = i / 4;
            const px = pixelIndex % img.width;
            const py = Math.floor(pixelIndex / img.width);
            hasInk = true;
            if (px < minX) minX = px;
            if (px > maxX) maxX = px;
            if (py < minY) minY = py;
            if (py > maxY) maxY = py;
          }
        }

        ctx.putImageData(imgData, 0, 0);

        if (hasInk) {
          // Crop to tight bounding box
          const pad = 12;
          const cropX = Math.max(0, minX - pad);
          const cropY = Math.max(0, minY - pad);
          const cropW = Math.min(img.width - cropX, (maxX - minX) + pad * 2);
          const cropH = Math.min(img.height - cropY, (maxY - minY) + pad * 2);

          const trimmedCanvas = document.createElement('canvas');
          trimmedCanvas.width = cropW;
          trimmedCanvas.height = cropH;
          const tCtx = trimmedCanvas.getContext('2d');
          if (tCtx) {
            tCtx.drawImage(canvas, cropX, cropY, cropW, cropH, 0, 0, cropW, cropH);
            uploadedDataUrl = trimmedCanvas.toDataURL('image/png');
            uploadedWidth = cropW;
            uploadedHeight = cropH;
            return;
          }
        }

        uploadedDataUrl = canvas.toDataURL('image/png');
        uploadedWidth = img.width;
        uploadedHeight = img.height;
      };
      img.src = src;
    };
    reader.readAsDataURL(file);
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDraggingFile = false;
    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      handleFileSelected(e.dataTransfer.files[0]);
    }
  }

  function handleAdoptAndPlace() {
    let result: { dataUrl: string; width: number; height: number; label: string; isInitials?: boolean } | null = null;

    if (activeTab === 'type') {
      if (typePreviewRef) {
        const data = typePreviewRef.getSignatureData();
        result = {
          dataUrl: data.dataUrl,
          width: data.width,
          height: data.height,
          label: isInitialsMode ? `Initials (${data.text})` : `Signature (${data.text})`,
          isInitials: isInitialsMode
        };
      }
    } else if (activeTab === 'draw') {
      if (padCanvasRef) {
        const data = padCanvasRef.getSignatureData();
        if (!data.isEmpty) {
          result = {
            dataUrl: data.dataUrl,
            width: data.width,
            height: data.height,
            label: 'Drawn Signature',
            isInitials: false
          };
        }
      }
    } else if (activeTab === 'upload') {
      if (uploadedDataUrl) {
        result = {
          dataUrl: uploadedDataUrl,
          width: uploadedWidth || 200,
          height: uploadedHeight || 80,
          label: uploadedFileName ? `Uploaded (${uploadedFileName})` : 'Uploaded Signature',
          isInitials: false
        };
      }
    }

    if (!result || !result.dataUrl) return;

    const signatureObj: AdoptedSignature = {
      id: 'sig_' + Math.random().toString(36).substring(2, 9),
      dataUrl: result.dataUrl,
      width: result.width,
      height: result.height,
      label: result.label,
      type: activeTab,
      isInitials: result.isInitials,
      createdAt: Date.now()
    };

    persistSavedSignature(signatureObj);
    onAdopt(signatureObj);
    onClose();
  }

  function handleSelectSaved(saved: AdoptedSignature) {
    onAdopt(saved);
    onClose();
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }

  onMount(() => {
    activeTab = initialTab;
    loadSavedSignatures();
  });
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={(e) => { if (e.target === e.currentTarget) onClose(); }}>
    <div
      class="signature-modal-card"
      role="dialog"
      aria-modal="true"
      aria-labelledby="sig-modal-title"
    >
      <!-- Modal Header -->
      <div class="modal-header">
        <div class="title-wrap">
          <span class="title-icon">✍</span>
          <h2 id="sig-modal-title">Create & Adopt Signature</h2>
        </div>
        <button
          type="button"
          class="close-btn"
          onclick={onClose}
          aria-label="Close signature modal"
        >
          ✕
        </button>
      </div>

      <!-- Navigation Tabs -->
      <div class="modal-tabs">
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'type'}
          onclick={() => activeTab = 'type'}
        >
          ⌨️ Type
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'draw'}
          onclick={() => activeTab = 'draw'}
        >
          ✏️ Draw
        </button>
        <button
          type="button"
          class="tab-btn"
          class:active={activeTab === 'upload'}
          onclick={() => activeTab = 'upload'}
        >
          📁 Upload
        </button>
      </div>

      <!-- Tab Content Area -->
      <div class="tab-body">
        {#if activeTab === 'type'}
          <SignatureTypePreview
            bind:this={typePreviewRef}
            bind:name={typedName}
            bind:initials={typedInitials}
            bind:selectedColor={typedColor}
            bind:selectedFontId={typedFontId}
            bind:isInitialsMode={isInitialsMode}
          />
        {:else if activeTab === 'draw'}
          <div class="draw-tab-wrapper">
            <div class="draw-color-row">
              <span class="color-label">Ink Color:</span>
              <div class="color-swatches">
                {#each [{ label: 'Black', value: '#000000' }, { label: 'Blue', value: '#1e40af' }, { label: 'Dark Slate', value: '#334155' }] as c}
                  <button
                    type="button"
                    class="color-swatch-btn"
                    class:selected={drawColor === c.value}
                    style="background-color: {c.value};"
                    title={c.label}
                    onclick={() => drawColor = c.value}
                  >
                    {#if drawColor === c.value}
                      <span class="swatch-check">✓</span>
                    {/if}
                  </button>
                {/each}
              </div>
            </div>
            <SignaturePadCanvas
              bind:this={padCanvasRef}
              color={drawColor}
              strokeWidth={3}
              onStrokeChange={(empty) => isPadEmpty = empty}
            />
          </div>
        {:else if activeTab === 'upload'}
          <div class="upload-tab-wrapper">
            {#if !uploadedDataUrl}
              <!-- Drag and drop zone -->
              <div
                class="upload-dropzone"
                class:dragging={isDraggingFile}
                ondragover={(e) => { e.preventDefault(); isDraggingFile = true; }}
                ondragleave={() => isDraggingFile = false}
                ondrop={handleDrop}
                onclick={() => fileInputRef?.click()}
              >
                <input
                  bind:this={fileInputRef}
                  type="file"
                  accept="image/png,image/jpeg,image/webp,image/svg+xml"
                  style="display: none;"
                  onchange={(e) => {
                    const f = (e.target as HTMLInputElement).files?.[0];
                    if (f) handleFileSelected(f);
                  }}
                />
                <div class="dropzone-icon">🖼️</div>
                <div class="dropzone-title">Drop your signature image here</div>
                <div class="dropzone-hint">Supports PNG, JPG, or WebP. White backgrounds are automatically removed.</div>
                <button type="button" class="btn-browse-file">Choose Image</button>
              </div>
            {:else}
              <!-- Image preview card -->
              <div class="uploaded-preview-card">
                <div class="preview-canvas-box">
                  <img src={uploadedDataUrl} alt="Uploaded signature preview" class="preview-img" />
                </div>
                <div class="preview-card-info">
                  <div class="info-details">
                    <span class="file-name">{uploadedFileName || 'Signature Image'}</span>
                    <span class="file-dims">{uploadedWidth} × {uploadedHeight} px (Background removed)</span>
                  </div>
                  <button
                    type="button"
                    class="btn-remove-upload"
                    onclick={() => { uploadedDataUrl = null; uploadedFileName = ''; }}
                  >
                    Change Image
                  </button>
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Saved Signatures Bar (if any exist) -->
      {#if savedSignatures.length > 0}
        <div class="saved-signatures-section">
          <div class="saved-header">
            <span class="saved-title">Previously Saved Signatures</span>
          </div>
          <div class="saved-chips-list">
            {#each savedSignatures as saved}
              <div
                class="saved-sig-card"
                onclick={() => handleSelectSaved(saved)}
                title="Click to place this saved signature"
              >
                <div class="saved-sig-img-wrap">
                  <img src={saved.dataUrl} alt={saved.label} class="saved-sig-img" />
                </div>
                <div class="saved-sig-meta">
                  <span class="saved-sig-label">{saved.label}</span>
                  <button
                    type="button"
                    class="saved-delete-btn"
                    title="Delete saved signature"
                    onclick={(e) => deleteSavedSignature(saved.id, e)}
                  >
                    ✕
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Modal Footer -->
      <div class="modal-footer">
        <button type="button" class="btn-cancel" onclick={onClose}>
          Cancel
        </button>
        <button
          type="button"
          class="btn-adopt"
          onclick={handleAdoptAndPlace}
          disabled={activeTab === 'draw' && isPadEmpty || activeTab === 'upload' && !uploadedDataUrl}
        >
          Adopt & Sign
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(15, 23, 42, 0.65);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1050;
    padding: 1.5rem;
  }

  .signature-modal-card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    width: 100%;
    max-width: 620px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: modalPop 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes modalPop {
    from {
      opacity: 0;
      transform: scale(0.96) translateY(8px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.15rem 1.5rem;
    border-bottom: 1px solid #f1f5f9;
  }

  .title-wrap {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .title-icon {
    font-size: 20px;
  }

  .modal-header h2 {
    font-size: 1.15rem;
    font-weight: 600;
    color: #0f172a;
    margin: 0;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    font-size: 16px;
    cursor: pointer;
    border-radius: 6px;
    padding: 4px 8px;
    transition: all 0.15s ease;
  }

  .close-btn:hover {
    background: #f1f5f9;
    color: #0f172a;
  }

  .modal-tabs {
    display: flex;
    background: #f8fafc;
    border-bottom: 1px solid #e2e8f0;
    padding: 0 1.5rem;
    gap: 0.5rem;
  }

  .tab-btn {
    padding: 0.75rem 1.25rem;
    font-size: 14px;
    font-weight: 500;
    color: #64748b;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    color: #0f172a;
  }

  .tab-btn.active {
    color: #2563eb;
    border-bottom-color: #2563eb;
    font-weight: 600;
  }

  .tab-body {
    padding: 1.25rem 1.5rem;
    min-height: 280px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .draw-tab-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
  }

  .draw-color-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .color-label {
    font-size: 13px;
    color: #64748b;
    font-weight: 500;
  }

  .color-swatches {
    display: flex;
    gap: 0.4rem;
  }

  .color-swatch-btn {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 2px solid #ffffff;
    box-shadow: 0 0 0 1px #cbd5e1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.15s ease;
    padding: 0;
  }

  .color-swatch-btn:hover {
    transform: scale(1.1);
  }

  .color-swatch-btn.selected {
    box-shadow: 0 0 0 2px #2563eb;
    transform: scale(1.05);
  }

  .swatch-check {
    color: #ffffff;
    font-size: 12px;
    font-weight: bold;
  }

  .upload-tab-wrapper {
    width: 100%;
  }

  .upload-dropzone {
    border: 2px dashed #cbd5e1;
    border-radius: 8px;
    padding: 2.5rem 1.5rem;
    text-align: center;
    cursor: pointer;
    background: #f8fafc;
    transition: all 0.2s ease;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .upload-dropzone:hover,
  .upload-dropzone.dragging {
    border-color: #2563eb;
    background: #eff6ff;
  }

  .dropzone-icon {
    font-size: 32px;
  }

  .dropzone-title {
    font-size: 15px;
    font-weight: 600;
    color: #1e293b;
  }

  .dropzone-hint {
    font-size: 12px;
    color: #64748b;
    max-width: 360px;
  }

  .btn-browse-file {
    margin-top: 0.5rem;
    padding: 0.4rem 1rem;
    font-size: 13px;
    font-weight: 500;
    background: #ffffff;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    color: #334155;
    cursor: pointer;
  }

  .uploaded-preview-card {
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    overflow: hidden;
  }

  .preview-canvas-box {
    background: #f8fafc;
    padding: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 160px;
    background-image: radial-gradient(#e2e8f0 1px, transparent 1px);
    background-size: 16px 16px;
  }

  .preview-img {
    max-width: 100%;
    max-height: 140px;
    object-fit: contain;
  }

  .preview-card-info {
    padding: 0.75rem 1rem;
    background: #ffffff;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-top: 1px solid #e2e8f0;
  }

  .info-details {
    display: flex;
    flex-direction: column;
  }

  .file-name {
    font-size: 13px;
    font-weight: 600;
    color: #0f172a;
  }

  .file-dims {
    font-size: 11px;
    color: #64748b;
  }

  .btn-remove-upload {
    font-size: 12px;
    color: #dc2626;
    background: transparent;
    border: 1px solid #fca5a5;
    padding: 0.25rem 0.6rem;
    border-radius: 4px;
    cursor: pointer;
  }

  .saved-signatures-section {
    border-top: 1px solid #e2e8f0;
    padding: 0.75rem 1.5rem;
    background: #f8fafc;
  }

  .saved-header {
    margin-bottom: 0.5rem;
  }

  .saved-title {
    font-size: 11px;
    font-weight: 600;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .saved-chips-list {
    display: flex;
    gap: 0.6rem;
    overflow-x: auto;
    padding-bottom: 4px;
  }

  .saved-sig-card {
    flex-shrink: 0;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    background: #ffffff;
    padding: 4px 6px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
    width: 120px;
  }

  .saved-sig-card:hover {
    border-color: #2563eb;
    box-shadow: 0 2px 4px rgba(37, 99, 235, 0.15);
  }

  .saved-sig-img-wrap {
    width: 100%;
    height: 38px;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .saved-sig-img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }

  .saved-sig-meta {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    font-size: 10px;
    color: #64748b;
  }

  .saved-sig-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 85px;
  }

  .saved-delete-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    padding: 0 2px;
    font-size: 10px;
  }

  .saved-delete-btn:hover {
    color: #dc2626;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    border-top: 1px solid #f1f5f9;
    background: #ffffff;
  }

  .btn-cancel {
    padding: 0.5rem 1rem;
    font-size: 14px;
    font-weight: 500;
    background: #f1f5f9;
    color: #475569;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-cancel:hover {
    background: #e2e8f0;
    color: #0f172a;
  }

  .btn-adopt {
    padding: 0.5rem 1.25rem;
    font-size: 14px;
    font-weight: 600;
    background: #2563eb;
    color: #ffffff;
    border: 1px solid #1d4ed8;
    border-radius: 6px;
    cursor: pointer;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
    transition: all 0.15s ease;
  }

  .btn-adopt:hover:not(:disabled) {
    background: #1d4ed8;
  }

  .btn-adopt:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
