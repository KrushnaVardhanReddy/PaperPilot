<script lang="ts">
  import PipelineCanvas from './PipelineCanvas.svelte';
  import PipelineRunner from './PipelineRunner.svelte';
  import StepConfigPanel from './StepConfigPanel.svelte';
  import ConditionInspector from './inspector/ConditionInspector.svelte';
  import FlowSelector from './FlowSelector.svelte';
  import { appState } from '$lib/state/app.svelte';
  import { pipelineState, OPERATION_REGISTRY, type OperationId } from '$lib/state/pipeline.svelte';
  import DropZone from '$lib/components/ui/DropZone.svelte';
  import PipelineDocumentsBadge from './PipelineDocumentsBadge.svelte';
  import { toastState } from '$lib/state/toast.svelte';

  let isCanvasDragging = $state(false);

  function handleFilesDropped(files: File[]) {
    appState.addDocuments(files);
  }

  function handleCanvasDragEnter(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer?.types?.includes('Files')) {
      isCanvasDragging = true;
    }
  }

  function handleCanvasDragLeave(e: DragEvent) {
    e.preventDefault();
    const currentTarget = e.currentTarget as HTMLElement;
    if (!currentTarget.contains(e.relatedTarget as Node)) {
      isCanvasDragging = false;
    }
  }

  function handleCanvasDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleCanvasDrop(e: DragEvent) {
    e.preventDefault();
    isCanvasDragging = false;

    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const filesArray = Array.from(e.dataTransfer.files).map(f => {
        (f as any)._localPath = (f as any).path || f.name;
        (f as any)._isLoaded = f.size > 0;
        return f;
      });
      appState.addDocuments(filesArray);
      toastState.success(`Added ${filesArray.length} document${filesArray.length > 1 ? 's' : ''} to pipeline`);
    }
  }

  function addStep(opId: string) {
    pipelineState.addStep(opId as OperationId);
  }

  const categories = {
    'Transform': ['pdf_rotate', 'pdf_crop', 'pdf_split', 'pdf_extract_pages', 'pdf_merge', 'pdf_remove_blank'],
    'Optimize': ['pdf_compress', 'pdf_linearize', 'pdf_repair'],
    'Security': ['pdf_encrypt', 'pdf_decrypt', 'pdf_redact', 'pdf_sanitize', 'pdf_sign', 'document_pii_scan'],
    'Content': ['pdf_watermark', 'pdf_metadata', 'pdf_header_footer', 'pdf_bates', 'pdf_ocr', 'pdf_flatten', 'pdf_page_numbers']
  };
</script>

<div class="pipeline-view" id="pipeline-view">
  <div class="top-bar">
    <div class="header-content">
      <h2>Visual Pipeline Builder</h2>
      <p>Chain multiple PDF operations together.</p>
    </div>
    <FlowSelector />

    <div class="runner-container">
      <PipelineRunner />
    </div>
  </div>

  <div class="studio-layout">
    <!-- Left Sidebar: Node Palette -->
    <div class="node-palette">
      <h3>Tools Palette</h3>
      <div class="palette-scroll">
        {#each Object.entries(categories) as [catName, opIds]}
          <div class="palette-category">
            <h4>{catName}</h4>
            <div class="tools-grid">
              {#each opIds as opId}
                {#if OPERATION_REGISTRY[opId as OperationId]}
                  <button class="tool-tile" onclick={() => addStep(opId)}>
                    <span class="icon">{OPERATION_REGISTRY[opId as OperationId].icon}</span>
                    <span class="label">{OPERATION_REGISTRY[opId as OperationId].label}</span>
                  </button>
                {/if}
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Center Canvas -->
    <div
      class="canvas-area"
      ondragenter={handleCanvasDragEnter}
      ondragover={handleCanvasDragOver}
      ondragleave={handleCanvasDragLeave}
      ondrop={handleCanvasDrop}
      role="region"
      aria-label="Pipeline canvas"
    >
      {#if appState.documents.length === 0}
        <div class="dropzone-overlay">
          <div class="dropzone-container">
            <DropZone ondrop={handleFilesDropped} />
          </div>
        </div>
      {:else}
        <PipelineDocumentsBadge />
      {/if}

      {#if isCanvasDragging}
        <div class="canvas-drag-overlay">
          <div class="drag-hint-card">
            <span class="drag-icon">📥</span>
            <span>Drop PDF files to add to pipeline</span>
          </div>
        </div>
      {/if}

      <PipelineCanvas />
    </div>

    <!-- Right Drawer: Config -->
    {#if pipelineState.selectedStepId}
      <div class="inspector-drawer">
        <div class="drawer-header">
          <h3>Properties</h3>
          <button class="close-btn" onclick={() => pipelineState.selectStep(null)}>×</button>
        </div>
        <div class="drawer-content">
          <StepConfigPanel />
          <ConditionInspector />
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .pipeline-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary, #020617);
  }

  .top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    background: var(--bg-surface, #0f172a);
    border-bottom: 1px solid var(--border-color, #334155);
    z-index: 10;
  }

  .header-content h2 {
    font-size: 1.25rem;
    color: var(--text-primary, #f8fafc);
    margin: 0 0 4px 0;
  }

  .header-content p {
    color: var(--text-secondary, #94a3b8);
    font-size: 0.85rem;
    margin: 0;
  }

  .runner-container {
    display: flex;
    align-items: center;
  }

  /* Override internal spacing inside runner to fit topbar */
  :global(#pipeline-runner) {
    margin-top: 0 !important;
    flex-direction: row !important;
    gap: 16px !important;
  }
  :global(#pipeline-runner .runner-status) {
    padding: 6px 12px !important;
  }

  .studio-layout {
    display: flex;
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  /* Left Node Palette */
  .node-palette {
    width: 260px;
    background: var(--bg-surface, #0f172a);
    border-right: 1px solid var(--border-color, #334155);
    display: flex;
    flex-direction: column;
  }

  .node-palette h3 {
    padding: 16px;
    margin: 0;
    font-size: 1rem;
    color: var(--text-primary);
    border-bottom: 1px solid var(--border-color);
  }

  .palette-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
  }

  .palette-category {
    margin-bottom: 24px;
  }

  .palette-category h4 {
    font-size: 0.8rem;
    text-transform: uppercase;
    color: var(--text-muted, #64748b);
    margin: 0 0 12px 0;
    letter-spacing: 0.05em;
  }

  .tools-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .tool-tile {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 12px 8px;
    background: var(--bg-secondary, #1e293b);
    border: 1px solid var(--border-color, #334155);
    border-radius: var(--border-radius-sm, 6px);
    cursor: pointer;
    transition: all 0.15s;
    color: var(--text-primary);
    gap: 6px;
  }

  .tool-tile:hover {
    border-color: var(--accent-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.1);
  }

  .tool-tile .icon {
    font-size: 1.4rem;
  }

  .tool-tile .label {
    font-size: 0.7rem;
    text-align: center;
    line-height: 1.2;
  }

  /* Center Canvas */
  .canvas-area {
    flex: 1;
    position: relative;
    display: flex;
    flex-direction: column;
  }

  .dropzone-overlay {
    position: absolute;
    top: 24px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 10;
    width: 400px;
    pointer-events: auto;
  }

  .canvas-drag-overlay {
    position: absolute;
    inset: 0;
    z-index: 200;
    background: rgba(15, 23, 42, 0.75);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
    border: 2px dashed var(--accent-primary, #3b82f6);
  }

  .drag-hint-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    background: var(--bg-surface, #1e293b);
    padding: 24px 36px;
    border-radius: var(--border-radius-lg, 12px);
    border: 1px solid var(--accent-primary, #3b82f6);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
    color: var(--text-primary, #f8fafc);
    font-size: 1.1rem;
    font-weight: 600;
  }

  .drag-icon {
    font-size: 2.5rem;
  }

  .pipeline-input-badge {
    position: absolute;
    top: 16px;
    left: 16px;
    z-index: 10;
    background: var(--bg-surface-elevated, #1e293b);
    padding: 8px 12px;
    border-radius: 20px;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  }

  /* Right Inspector Drawer */
  .inspector-drawer {
    width: 320px;
    background: var(--bg-surface, #0f172a);
    border-left: 1px solid var(--border-color, #334155);
    display: flex;
    flex-direction: column;
    box-shadow: -4px 0 15px rgba(0,0,0,0.1);
  }

  .drawer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .drawer-header h3 {
    margin: 0;
    font-size: 1rem;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.4rem;
    color: var(--text-muted);
    cursor: pointer;
    line-height: 1;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .drawer-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
</style>
