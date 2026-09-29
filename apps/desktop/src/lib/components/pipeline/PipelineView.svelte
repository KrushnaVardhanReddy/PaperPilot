<script lang="ts">
  import PipelineCanvas from './PipelineCanvas.svelte';
  import PipelineRunner from './PipelineRunner.svelte';
  import StepConfigPanel from './StepConfigPanel.svelte';
  import { appState } from '$lib/state/app.svelte';
  import DropZone from '$lib/components/ui/DropZone.svelte';

  function handleFilesDropped(files: File[]) {
    appState.addDocuments(files);
  }
</script>

<div class="pipeline-view" id="pipeline-view">
  <header class="pipeline-header">
    <h2>Visual Pipeline Builder</h2>
    <p>Chain multiple PDF operations together. They execute in order, one output feeding the next.</p>
  </header>

  {#if appState.documents.length === 0}
    <div class="pipeline-dropzone">
      <DropZone ondrop={handleFilesDropped} />
    </div>
  {:else}
    <div class="pipeline-input">
      <span class="input-label">Input:</span>
      <span class="input-filename">📄 {appState.documents[0].name}</span>
    </div>
  {/if}

  <PipelineCanvas />

  <StepConfigPanel />

  <PipelineRunner />
</div>

<style>
  .pipeline-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 32px;
    gap: 24px;
    overflow-y: auto;
  }

  .pipeline-header h2 {
    font-size: 1.75rem;
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .pipeline-header p {
    color: var(--text-secondary);
    font-size: 0.95rem;
  }

  .pipeline-dropzone {
    max-width: 600px;
  }

  .pipeline-input {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .input-filename {
    font-weight: 600;
    color: var(--text-primary);
  }
</style>
