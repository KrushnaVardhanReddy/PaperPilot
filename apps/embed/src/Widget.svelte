<script lang="ts">
  interface Props {
    tools?: string;
    hideBadge?: boolean;
    brandColor?: string;
    theme?: string;
    logoUrl?: string;
  }

  let { tools = 'all', hideBadge = false, brandColor = '#3B82F6', theme = 'dark', logoUrl }: Props = $props();

  const allTools = [
    { id: 'merge', name: 'Merge PDF' },
    { id: 'compress', name: 'Compress PDF' },
    { id: 'watermark', name: 'Watermark' },
    { id: 'split', name: 'Split PDF' }
  ];

  let availableTools = $derived(
    tools === 'all'
      ? allTools
      : allTools.filter(t => tools.split(',').map(s => s.trim()).includes(t.id))
  );

  function dispatchEvent(eventName: string, detail: any = {}) {
    const event = new CustomEvent(`paperpilot:${eventName}`, {
      detail,
      bubbles: true,
      composed: true
    });
    // @ts-ignore
    document.activeElement?.dispatchEvent(event) || window.dispatchEvent(event);
  }

  let droppedFiles = $state<File[]>([]);
  let isDragging = $state(false);
  let fileInputRef: HTMLInputElement | null = null;

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = false;

    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const files = Array.from(e.dataTransfer.files);
      const pdfs = files.filter(f => f.type === 'application/pdf' || f.name.toLowerCase().endsWith('.pdf'));
      droppedFiles = pdfs.length > 0 ? pdfs : files;
      dispatchEvent('files-selected', { count: droppedFiles.length, files: droppedFiles.map(f => f.name) });
    }
  }

  function handleFileInput(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      droppedFiles = Array.from(target.files);
      dispatchEvent('files-selected', { count: droppedFiles.length, files: droppedFiles.map(f => f.name) });
    }
  }

  function triggerFileDialog() {
    fileInputRef?.click();
  }

  function handleProcessStart(toolId: string) {
    dispatchEvent('process-start', { tool: toolId, files: droppedFiles.map(f => f.name) });
    // Simulate process
    setTimeout(() => {
      dispatchEvent('process-complete', { tool: toolId, outputSizeBytes: 1024 });
      alert(`[PaperPilot Embed] Successfully executed ${toolId} on ${droppedFiles.length || 1} file(s)!`);
    }, 600);
  }
</script>

<div class="pp-widget" data-theme={theme}>
  {#if logoUrl}
    <div class="pp-header">
      <img src={logoUrl} alt="Logo" class="pp-logo" />
    </div>
  {/if}

  <input
    type="file"
    accept="application/pdf"
    multiple
    style="display: none;"
    bind:this={fileInputRef}
    onchange={handleFileInput}
  />

  <div
    class="pp-dropzone {isDragging ? 'dragging' : ''}"
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    onclick={triggerFileDialog}
    role="button"
    tabindex="0"
    onkeydown={(e) => { if (e.key === 'Enter') triggerFileDialog(); }}
  >
    {#if droppedFiles.length > 0}
      <p class="file-success">
        📄 <strong>{droppedFiles.length} PDF{droppedFiles.length > 1 ? 's' : ''} loaded</strong>: {droppedFiles.map(f => f.name).join(', ')}
      </p>
      <span class="sub-hint">Click or drop more files to replace</span>
    {:else}
      <p>📁 Drag and drop PDFs here, or <span class="browse-link">browse</span></p>
      <span class="sub-hint">Files processed 100% locally in browser</span>
    {/if}
  </div>

  <div class="pp-tools">
    {#each availableTools as tool (tool.id)}
      <button class="pp-tool-card" onclick={() => handleProcessStart(tool.id)} data-tool-id={tool.id}>
        {tool.name}
      </button>
    {/each}
  </div>

  {#if !hideBadge}
    <div class="pp-footer">
      <a href="https://usepaperpilot.com" target="_blank" rel="noopener" class="pp-badge">
        ⚡ Powered by PaperPilot
      </a>
    </div>
  {/if}
</div>

<style>
  .pp-widget {
    background-color: var(--pp-background);
    color: var(--pp-text);
    font-family: var(--pp-font);
    border-radius: var(--pp-radius);
    padding: 20px;
    box-sizing: border-box;
    width: 100%;
    border: 1px solid var(--pp-border);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .pp-header {
    display: flex;
    justify-content: center;
    margin-bottom: 8px;
  }

  .pp-logo {
    max-height: 40px;
    max-width: 100%;
  }

  .pp-dropzone {
    border: 2px dashed var(--pp-border);
    border-radius: var(--pp-radius);
    padding: 32px 20px;
    text-align: center;
    background-color: var(--pp-surface);
    color: var(--pp-text-muted);
    transition: all 0.2s ease;
    cursor: pointer;
    user-select: none;
  }

  .pp-dropzone:hover,
  .pp-dropzone.dragging {
    border-color: var(--pp-brand-color);
    background-color: rgba(94, 106, 210, 0.08);
    color: var(--pp-text);
  }

  .pp-dropzone p {
    margin: 0 0 6px 0;
    font-size: 15px;
  }

  .pp-dropzone .browse-link {
    color: var(--pp-brand-color);
    text-decoration: underline;
    font-weight: 600;
  }

  .pp-dropzone .sub-hint {
    font-size: 12px;
    opacity: 0.75;
    display: block;
  }

  .pp-dropzone .file-success {
    color: #10B981;
    font-size: 15px;
    word-break: break-all;
  }

  .pp-tools {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 12px;
  }

  .pp-tool-card {
    background-color: var(--pp-surface);
    color: var(--pp-text);
    border: 1px solid var(--pp-border);
    border-radius: 8px;
    padding: 16px 12px;
    text-align: center;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
    font-size: 14px;
    font-family: inherit;
  }

  .pp-tool-card:hover {
    border-color: var(--pp-brand-color);
    background-color: var(--pp-brand-color);
    color: white;
  }

  .pp-footer {
    display: flex;
    justify-content: center;
    margin-top: 8px;
  }

  .pp-badge {
    display: inline-block;
    padding: 6px 12px;
    background-color: var(--pp-surface);
    color: var(--pp-text-muted);
    text-decoration: none;
    font-size: 12px;
    border-radius: 16px;
    border: 1px solid var(--pp-border);
    transition: all 0.2s;
  }

  .pp-badge:hover {
    color: var(--pp-text);
    border-color: var(--pp-text-muted);
  }
</style>
