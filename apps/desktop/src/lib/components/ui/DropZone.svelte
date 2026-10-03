<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
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

  async function handleBrowseClick(e: MouseEvent) {
    // Intercept click; use Tauri native dialog so we get full absolute paths
    e.preventDefault();
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: true,
      filters: [{ name: 'PDF Documents', extensions: ['pdf'] }]
    });
    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    const filesArray: File[] = [];
    for (const path of paths) {
      const fileName = path.split('/').pop() || path.split('\\').pop() || 'document.pdf';
      const bytes: number[] = await invoke('read_file_bytes', { path });
      const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
      const file = new File([blob], fileName, { type: 'application/pdf' });
      (file as any)._localPath = path;
      filesArray.push(file);
    }
    if (filesArray.length > 0) ondrop(filesArray);
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

    <button type="button" class="browse-btn" onclick={handleBrowseClick}>
      Browse Files
    </button>
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
    display: block;
    opacity: 0;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    cursor: pointer;
  }

  .browse-btn {
    position: relative;
  }

  @media (max-width: 599px) {
    /* Make drop zone padding smaller on mobile */
    .drop-content {
      padding: 32px 16px !important;
    }
  }
</style>
