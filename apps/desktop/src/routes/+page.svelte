<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import DropZone from '$lib/components/ui/DropZone.svelte';
  import DocumentList from '$lib/components/ui/DocumentList.svelte';
  import PdfViewer from '$lib/components/PdfViewer.svelte';
  import PdfToolbar from '$lib/components/PdfToolbar.svelte';
  import PdfThumbnails from '$lib/components/PdfThumbnails.svelte';
  import PdfInfoPanel from '$lib/components/PdfInfoPanel.svelte';
  import SettingsPanel from '$lib/components/layout/SettingsPanel.svelte';
  import OperationsPanel from '$lib/components/layout/OperationsPanel.svelte';
  import PipelineView from '$lib/components/pipeline/PipelineView.svelte';

  function handleFilesDropped(files: File[]) {
    appState.addDocuments(files);
  }

  let currentFileUrl = $state('');

  let pdfDoc: any = $state(null);
  let pageNum = $state(1);
  let scale = $state(1.2);
  let numPages = $derived(pdfDoc ? pdfDoc.numPages : 0);

  $effect(() => {
    if (appState.selectedDocumentIndex !== null) {
      const file = appState.documents[appState.selectedDocumentIndex];
      if (file) {
        currentFileUrl = URL.createObjectURL(file);
      } else {
        currentFileUrl = '';
      }
    } else {
      currentFileUrl = '';
    }

    return () => {
      if (currentFileUrl) {
        URL.revokeObjectURL(currentFileUrl);
      }
    };
  });
</script>

<div class="page-container">
  {#if appState.activeTab === 'home' || appState.activeTab === 'documents'}
    <div class="content-area">
      {#if appState.selectedDocumentIndex !== null}
        <div class="viewer-wrapper">
          <div class="viewer-header">
            <button class="back-btn" onclick={() => appState.selectDocument(null)}>
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="19" y1="12" x2="5" y2="12"></line>
                <polyline points="12 19 5 12 12 5"></polyline>
              </svg>
              Back to Documents
            </button>
            <h3 class="doc-title">{appState.documents[appState.selectedDocumentIndex]?.name}</h3>
          </div>
          <div class="viewer-content">
            <PdfThumbnails {pdfDoc} bind:pageNum />

            <div class="viewer-main">
              <PdfToolbar bind:pageNum {numPages} bind:scale />
              <PdfViewer fileUrl={currentFileUrl} bind:pdfDoc bind:pageNum bind:scale />
            </div>

            <PdfInfoPanel {pdfDoc} />
          </div>
        </div>
      {:else}
        <div class="documents-view">
          <header class="view-header">
            <h2>Documents</h2>
            <p>Add PDF files to process</p>
          </header>

          <DropZone ondrop={handleFilesDropped} />
          <DocumentList />
        </div>
      {/if}
    </div>

    <OperationsPanel />
  {:else if appState.activeTab === 'settings'}
    <div class="content-area full-width">
      <SettingsPanel />
    </div>
  {:else if appState.activeTab === 'pipeline'}
    <div class="content-area full-width">
      <PipelineView />
    </div>
  {/if}
</div>

<style>
  .page-container {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .content-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .content-area.full-width {
    width: 100%;
  }

  .documents-view {
    padding: 32px;
    max-width: 800px;
    margin: 0 auto;
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .view-header {
    margin-bottom: 24px;
  }

  .view-header h2 {
    font-size: 1.75rem;
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .view-header p {
    color: var(--text-secondary);
    font-size: 0.95rem;
  }

  .viewer-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
  }

  .viewer-header {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 24px;
    background-color: var(--bg-surface, #ffffff);
    border-bottom: 1px solid var(--border-color, #e5e7eb);
  }

  .back-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: none;
    border: none;
    color: var(--text-secondary, #4b5563);
    cursor: pointer;
    font-size: 0.95rem;
    padding: 6px 12px;
    border-radius: 4px;
    transition: all 0.2s;
  }

  .back-btn:hover {
    color: var(--text-primary, #111827);
    background-color: var(--bg-surface-hover, #f3f4f6);
  }

  .doc-title {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary, #111827);
    font-weight: 500;
  }

  .viewer-content {
    flex: 1;
    overflow: hidden;
    position: relative;
    display: flex;
    flex-direction: row;
  }

  .viewer-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
</style>
