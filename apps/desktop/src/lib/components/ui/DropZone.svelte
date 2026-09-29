<script lang="ts">
  let isDragging = $state(false);
  let { ondrop }: { ondrop: (files: File[]) => void } = $props();

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
      const filesArray = Array.from(e.dataTransfer.files);
      ondrop(filesArray);
    }
  }

  function handleFileInput(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const filesArray = Array.from(target.files);
      ondrop(filesArray);
      target.value = ''; // Reset input
    }
  }
</script>

<div
  class="drop-zone"
  class:dragging={isDragging}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondragover={handleDragOver}
  ondrop={handleDrop}
  role="button"
  tabindex="0"
>
  <div class="drop-content">
    <div class="icon">📄</div>
    <h3 class="title">Drop PDF files here</h3>
    <p class="subtitle">or click to browse</p>

    <label class="browse-btn">
      Browse Files
      <input
        type="file"
        multiple
        accept=".pdf"
        onchange={handleFileInput}
      />
    </label>
  </div>
</div>

<style>
  .drop-zone {
    width: 100%;
    min-height: 200px;
    border: 2px dashed var(--border-color);
    border-radius: var(--border-radius-lg);
    background-color: var(--bg-surface);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-normal);
    position: relative;
    overflow: hidden;
  }

  .drop-zone:hover {
    border-color: var(--text-muted);
    background-color: var(--bg-surface-hover);
  }

  .drop-zone.dragging {
    border-color: var(--accent-primary);
    background-color: rgba(94, 106, 210, 0.1);
  }

  .drop-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 32px;
  }

  .icon {
    font-size: 3rem;
    margin-bottom: 16px;
    opacity: 0.8;
    transition: transform var(--transition-normal);
  }

  .drop-zone.dragging .icon {
    transform: scale(1.1);
    opacity: 1;
  }

  .title {
    font-size: 1.25rem;
    color: var(--text-primary);
    margin-bottom: 8px;
    font-weight: 600;
  }

  .subtitle {
    font-size: 0.95rem;
    color: var(--text-secondary);
    margin-bottom: 24px;
  }

  .browse-btn {
    display: inline-block;
    padding: 10px 20px;
    background-color: var(--accent-primary);
    color: #ffffff;
    border-radius: var(--border-radius-md);
    font-weight: 500;
    cursor: pointer;
    transition: background-color var(--transition-fast);
  }

  .browse-btn:hover {
    background-color: var(--accent-hover);
  }

  .browse-btn:active {
    background-color: var(--accent-active);
  }

  input[type="file"] {
    display: none;
  }

  @media (max-width: 599px) {
    /* Make drop zone padding smaller on mobile */
    .drop-content {
      padding: 32px 16px !important;
    }
  }
</style>
