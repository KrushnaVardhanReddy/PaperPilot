<script lang="ts">
  let isDragging = $state(false);
  let isLoadingSample = $state(false);
  let { ondrop, multiple = true }: { ondrop: (files: File[]) => void; multiple?: boolean } = $props();

  async function loadSamplePdf(samplePath: string, fileName: string) {
    if (isLoadingSample) return;
    isLoadingSample = true;
    try {
      const response = await fetch(samplePath);
      if (!response.ok) {
        throw new Error(`Failed to load sample: ${response.statusText}`);
      }
      const blob = await response.blob();
      const sampleFile = new File([blob], fileName, { type: 'application/pdf' });
      ondrop([sampleFile]);
    } catch (err) {
      console.error('Failed to load sample PDF:', err);
      alert('Could not load sample PDF. Please browse or drop a local file.');
    } finally {
      isLoadingSample = false;
    }
  }

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      let filesArray = Array.from(e.dataTransfer.files).filter(f => f.type === 'application/pdf' || f.name.toLowerCase().endsWith('.pdf'));
      if (!multiple && filesArray.length > 1) {
        filesArray = [filesArray[0]];
      }
      if (filesArray.length > 0) {
          ondrop(filesArray);
      } else {
          alert('Please drop PDF files only.');
      }
    }
  }

  function handleFileInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      let filesArray = Array.from(target.files).filter(f => f.type === 'application/pdf' || f.name.toLowerCase().endsWith('.pdf'));
      if (!multiple && filesArray.length > 1) {
          filesArray = [filesArray[0]];
      }
      if (filesArray.length > 0) {
          ondrop(filesArray);
      }
    }
    target.value = '';
  }
</script>

<div
  class="drop-zone"
  class:dragging={isDragging}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondragover={handleDragOver}
  ondrop={handleDrop}
  role="region"
  aria-label="PDF drop zone"
>
  <div class="drop-content">
    <div class="icon">📄</div>
    <h3 class="title">Drop PDF file{multiple ? 's' : ''} here</h3>
    <p class="subtitle">or click to browse</p>

    <div class="actions-row">
      <label class="browse-btn">
        Browse Files
        <input type="file" {multiple} accept=".pdf,application/pdf" onchange={handleFileInputChange} />
      </label>

      <span class="or-divider">or</span>

      <button
        type="button"
        class="sample-btn"
        disabled={isLoadingSample}
        onclick={() => loadSamplePdf('/samples/sample_document.pdf', 'sample_document.pdf')}
      >
        <span class="sparkle">✨</span> {isLoadingSample ? 'Loading...' : 'Try with Sample PDF'}
      </button>

      <a
        href="/samples/sample_document.pdf"
        target="_blank"
        rel="noopener noreferrer"
        class="view-sample-link"
        title="View or preview sample PDF in a new tab"
      >
        👁️ View Sample
      </a>
    </div>

    <div class="privacy-pill">
      <span aria-hidden="true">🔒</span> 0 KB Uploaded • Executed in Local WASM Memory
    </div>
  </div>
</div>

<style>
  .drop-zone {
    width: 100%;
    min-height: 200px;
    border: 2px dashed var(--border-color, #E5E7EB);
    border-radius: var(--border-radius-lg, 12px);
    background-color: var(--bg-surface, #F0F2F5);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-normal, 250ms ease);
    position: relative;
    overflow: hidden;
  }

  .drop-zone:hover {
    border-color: var(--text-muted, #6B7280);
    background-color: var(--bg-surface-hover, #E4E7EB);
  }

  .drop-zone.dragging {
    border-color: var(--accent-primary, #4F46E5);
    background-color: rgba(79, 70, 229, 0.1);
  }

  .drop-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 32px;
  }

  .privacy-pill {
    margin-top: 16px;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 12px;
    border-radius: 9999px;
    font-size: 0.78rem;
    font-weight: 500;
    color: #10b981;
    background-color: rgba(16, 185, 129, 0.08);
    border: 1px solid rgba(16, 185, 129, 0.24);
  }

  .icon {
    font-size: 3rem;
    margin-bottom: 16px;
    opacity: 0.8;
    transition: transform var(--transition-normal, 250ms ease);
  }

  .drop-zone.dragging .icon {
    transform: scale(1.1);
    opacity: 1;
  }

  .title {
    font-size: 1.25rem;
    color: var(--text-primary, #111827);
    margin-bottom: 8px;
    font-weight: 600;
  }

  .subtitle {
    font-size: 0.95rem;
    color: var(--text-secondary, #4B5563);
    margin-bottom: 24px;
  }

  .actions-row {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .or-divider {
    font-size: 0.875rem;
    color: var(--text-muted, #9CA3AF);
    font-weight: 500;
  }

  .sample-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 9px 18px;
    background-color: var(--bg-surface, #FFFFFF);
    color: var(--text-primary, #1F2937);
    border: 1px solid var(--border-color, #D1D5DB);
    border-radius: var(--border-radius-md, 8px);
    font-weight: 500;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all var(--transition-fast, 150ms ease);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .sample-btn:hover:not(:disabled) {
    background-color: var(--bg-surface-hover, #F9FAFB);
    border-color: var(--accent-primary, #4F46E5);
    color: var(--accent-primary, #4F46E5);
    transform: translateY(-1px);
    box-shadow: 0 2px 4px rgba(79, 70, 229, 0.15);
  }

  .sample-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .sparkle {
    font-size: 1rem;
  }

  .view-sample-link {
    font-size: 0.85rem;
    color: var(--text-secondary, #4B5563);
    text-decoration: underline;
    text-underline-offset: 3px;
    cursor: pointer;
    padding: 4px 6px;
    border-radius: 4px;
    transition: color var(--transition-fast, 150ms ease);
  }

  .view-sample-link:hover {
    color: var(--accent-primary, #4F46E5);
  }

  .browse-btn {
    display: inline-block;
    padding: 10px 20px;
    background-color: var(--accent-primary, #4F46E5);
    color: #ffffff;
    border-radius: var(--border-radius-md, 8px);
    font-weight: 500;
    font-size: 0.95rem;
    cursor: pointer;
    transition: background-color var(--transition-fast, 150ms ease);
    position: relative;
  }

  .browse-btn:hover {
    background-color: var(--accent-hover, #4338CA);
  }

  .browse-btn:active {
    background-color: var(--accent-active, #3730A3);
  }

  input[type="file"] {
    display: block;
    opacity: 0;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    cursor: pointer;
  }

  @media (max-width: 599px) {
    .drop-content {
      padding: 32px 16px !important;
    }
  }
</style>
