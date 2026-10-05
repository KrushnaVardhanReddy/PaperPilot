<script lang="ts">
  let isDragging = $state(false);
  let { ondrop, multiple = true }: { ondrop: (files: File[]) => void; multiple?: boolean } = $props();

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
  role="button"
  tabindex="0"
>
  <div class="drop-content">
    <div class="icon">📄</div>
    <h3 class="title">Drop PDF file{multiple ? 's' : ''} here</h3>
    <p class="subtitle">or click to browse</p>

    <label class="browse-btn">
      Browse Files
      <input type="file" {multiple} accept=".pdf,application/pdf" onchange={handleFileInputChange} />
    </label>
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

  .browse-btn {
    display: inline-block;
    padding: 10px 20px;
    background-color: var(--accent-primary, #4F46E5);
    color: #ffffff;
    border-radius: var(--border-radius-md, 8px);
    font-weight: 500;
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
