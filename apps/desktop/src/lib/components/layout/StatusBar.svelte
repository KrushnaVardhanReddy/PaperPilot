<script lang="ts">
  let {
    pageNum = 1,
    numPages = 0,
    scale = 1.2,
    fileName = '',
    fileSize = 0
  }: {
    pageNum: number;
    numPages: number;
    scale: number;
    fileName: string;
    fileSize: number;
  } = $props();

  function formatSize(bytes: number): string {
    if (!bytes || bytes === 0) return '';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  let zoomPercent = $derived(Math.round(scale * 100));
  let sizeStr = $derived(formatSize(fileSize));
</script>

{#if numPages > 0}
<div class="status-bar" role="status" aria-label="Document status">
  {#if fileName}
    <span class="status-item status-filename" title={fileName}>{fileName}</span>
    <span class="status-sep" aria-hidden="true">·</span>
  {/if}

  <span class="status-item" id="status-page">
    Page {pageNum} of {numPages}
  </span>

  <span class="status-sep" aria-hidden="true">·</span>

  <span class="status-item" id="status-zoom">{zoomPercent}%</span>

  {#if sizeStr}
    <span class="status-sep" aria-hidden="true">·</span>
    <span class="status-item" id="status-size">{sizeStr}</span>
  {/if}
</div>
{/if}

<style>
  .status-bar {
    height: 24px;
    background-color: var(--bg-secondary);
    border-top: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    padding: 0 16px;
    gap: 8px;
    font-size: 11px;
    color: var(--text-muted);
    flex-shrink: 0;
    user-select: none;
  }

  .status-item {
    white-space: nowrap;
  }

  .status-filename {
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text-secondary);
  }

  .status-sep {
    opacity: 0.35;
    font-size: 10px;
  }
</style>