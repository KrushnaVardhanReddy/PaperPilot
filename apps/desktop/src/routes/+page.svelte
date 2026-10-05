<script lang="ts">

  async function safeInvoke(cmd: string, args?: any): Promise<any> {
    try {
      if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__) {
        const { invoke } = await import('@tauri-apps/api/core');
        return await invoke(cmd, args);
      }
    } catch (e) {
      console.warn(`[Tauri Mock] ${cmd} called in browser environment:`, args);
    }
    // Return success mock for UI testing
    return { success: true };
  }

  import { appState } from '$lib/state/app.svelte';
  import DropZone from '$lib/components/ui/DropZone.svelte';
  import DocumentList from '$lib/components/ui/DocumentList.svelte';
  import PdfViewer from '$lib/components/PdfViewer.svelte';
  import PdfToolbar from '$lib/components/PdfToolbar.svelte';
  import PdfThumbnails from '$lib/components/PdfThumbnails.svelte';
  import SettingsPanel from '$lib/components/layout/SettingsPanel.svelte';
  import OperationsPanel from '$lib/components/layout/OperationsPanel.svelte';
  import PipelineView from '$lib/components/pipeline/PipelineView.svelte';
  import type { Annotation } from '$lib/api/pdf';
  import PdfFormLayer from '$lib/components/PdfFormLayer.svelte';
  import ViewerRightPanel from '$lib/components/layout/ViewerRightPanel.svelte';
  import StatusBar from '$lib/components/layout/StatusBar.svelte';
  import DocumentTabBar from '$lib/components/layout/DocumentTabBar.svelte';
  import ApiDocsView from '$lib/components/layout/ApiDocsView.svelte';
  import PdfVisualDiff from '$lib/components/PdfVisualDiff.svelte';
  import { listen } from '@tauri-apps/api/event';

  import { onMount } from 'svelte';
  onMount(() => {
    window.addEventListener('test-select-doc', (e: any) => {
       appState.selectDocument(e.detail);
    });

    // Tauri events from titlebar
    let unlistenMenuToggleDiff: (() => void) | undefined;
    listen('menu-toggle-diff', () => {
      appState.toggleDiffView();
    }).then(fn => unlistenMenuToggleDiff = fn).catch(() => {});

    // Key bindings
    window.addEventListener('paperpilot:rotate-single-page', handleRotatePage as unknown as EventListener);
    window.addEventListener('paperpilot:delete-single-page', handleDeletePage as unknown as EventListener);
    window.addEventListener('paperpilot:reorder-page', handleReorderPage as unknown as EventListener);
    window.addEventListener('paperpilot:duplicate-page', handleDuplicatePage as unknown as EventListener);
    window.addEventListener('paperpilot:extract-single-page', handleExtractPage as unknown as EventListener);

    return () => {
      if (unlistenMenuToggleDiff) unlistenMenuToggleDiff();
      window.removeEventListener('paperpilot:rotate-single-page', handleRotatePage as unknown as EventListener);
      window.removeEventListener('paperpilot:delete-single-page', handleDeletePage as unknown as EventListener);
      window.removeEventListener('paperpilot:reorder-page', handleReorderPage as unknown as EventListener);
      window.removeEventListener('paperpilot:duplicate-page', handleDuplicatePage as unknown as EventListener);
      window.removeEventListener('paperpilot:extract-single-page', handleExtractPage as unknown as EventListener);
    };
  });

  import { savePdfForm } from '$lib/api/pdf';
  import { toastState } from '$lib/state/toast.svelte';


  async function handleRotatePage(e: CustomEvent<{ page: number, angle?: number }>) {
    if (appState.selectedDocumentIndex === null) return;
    const file = appState.documents[appState.selectedDocumentIndex];
    if (!file) return;

    appState.setLoading(true);
    try {
      const path = appState.documentPaths[appState.selectedDocumentIndex ?? 0] || file.name;
      const angle = e.detail.angle ?? 90;
      await safeInvoke('invoke_mcp_tool', {
        toolName: 'pdf_rotate',
        arguments: {
          input: path,
          pages: String(e.detail.page),
          angle: angle,
          output: path
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page ${e.detail.page} rotated by ${angle}°.`);
    } catch (err) {
      toastState.error(`Failed to rotate page: ${err}`);
    } finally {
      appState.setLoading(false);
    }
  }

  async function handleDeletePage(e: CustomEvent<{ page: number }>) {
    if (appState.selectedDocumentIndex === null) return;
    const file = appState.documents[appState.selectedDocumentIndex];
    if (!file) return;

    appState.setLoading(true);
    try {
      const path = appState.documentPaths[appState.selectedDocumentIndex ?? 0] || file.name;
      await safeInvoke('invoke_mcp_tool', {
        toolName: 'pdf_delete_pages',
        arguments: {
          input: path,
          pages: String(e.detail.page),
          output: path
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page ${e.detail.page} deleted.`);
    } catch (err) {
      toastState.error(`Failed to delete page: ${err}`);
    } finally {
      appState.setLoading(false);
    }
  }

  async function handleReorderPage(e: CustomEvent<{ fromPage: number, toPage: number, position?: 'top' | 'bottom' }>) {
    if (appState.selectedDocumentIndex === null || !pdfDoc) return;
    const file = appState.documents[appState.selectedDocumentIndex];
    if (!file) return;

    appState.setLoading(true);
    try {
      const path = appState.documentPaths[appState.selectedDocumentIndex ?? 0] || file.name;
      const { fromPage, toPage, position } = e.detail;

      const pages = Array.from({ length: pdfDoc.numPages }, (_, i) => i + 1);
      const [movedPage] = pages.splice(fromPage - 1, 1);

      // Calculate new index
      let targetIndex = toPage - 1;
      if (fromPage < toPage) {
          targetIndex -= 1;
      }
      if (position === 'bottom') {
        targetIndex += 1;
      }
      pages.splice(targetIndex, 0, movedPage);

      await safeInvoke('invoke_mcp_tool', {
        toolName: 'pdf_reorder_pages',
        arguments: {
          input: path,
          order: pages.join(','),
          output: path
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page moved to position ${targetIndex + 1}.`);
    } catch (err) {
      toastState.error(`Failed to reorder pages: ${err}`);
    } finally {
      appState.setLoading(false);
    }
  }

  async function handleDuplicatePage(e: CustomEvent<{ page: number }>) {
    if (appState.selectedDocumentIndex === null || !pdfDoc) return;
    const file = appState.documents[appState.selectedDocumentIndex];
    if (!file) return;

    appState.setLoading(true);
    try {
      const path = appState.documentPaths[appState.selectedDocumentIndex ?? 0] || file.name;
      const { page } = e.detail;

      const pages = Array.from({ length: pdfDoc.numPages }, (_, i) => i + 1);
      pages.splice(page, 0, page);

      await safeInvoke('invoke_mcp_tool', {
        toolName: 'pdf_reorder_pages',
        arguments: {
          input: path,
          order: pages.join(','),
          output: path
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page ${page} duplicated.`);
    } catch (err) {
      toastState.error(`Failed to duplicate page: ${err}`);
    } finally {
      appState.setLoading(false);
    }
  }

  async function handleExtractPage(e: CustomEvent<{ page: number }>) {
    if (appState.selectedDocumentIndex === null) return;
    const file = appState.documents[appState.selectedDocumentIndex];
    if (!file) return;

    const path = appState.documentPaths[appState.selectedDocumentIndex ?? 0] || file.name;
    const docDir = path.includes('/') ? path.substring(0, path.lastIndexOf('/')) : '';
    const baseName = file.name.replace(/\.pdf$/i, '');
    const defaultPath = docDir ? `${docDir}/${baseName}_page_${e.detail.page}.pdf` : `${baseName}_page_${e.detail.page}.pdf`;

    let outputPath: string | null = null;
    try {
      const { save } = await import('@tauri-apps/plugin-dialog');
      outputPath = await save({
        title: 'Extract Page As...',
        defaultPath: defaultPath,
        filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
      });
    } catch (err) {
      console.warn('Tauri dialog unavailable, falling back:', err);
      outputPath = defaultPath;
    }

    if (!outputPath) return; // User cancelled

    appState.setLoading(true);
    try {
      await safeInvoke('invoke_mcp_tool', {
        toolName: 'pdf_extract_pages',
        arguments: {
          input: path,
          pages: String(e.detail.page),
          output: outputPath
        }
      });
      toastState.success(`Page ${e.detail.page} extracted to ${outputPath}`);
    } catch (err) {
      toastState.error(`Failed to extract page: ${err}`);
    } finally {
      appState.setLoading(false);
    }
  }

  async function refreshCurrentDocument() {
    if (appState.selectedDocumentIndex === null) return;
    const idx = appState.selectedDocumentIndex;
    const file = appState.documents[idx];
    if (!file) return;
    const path = appState.documentPaths[idx];
    if (!path) return;

    try {
      const bytes: number[] = await safeInvoke('read_file_bytes', { path });
      const blob = new Blob([new Uint8Array(bytes)], { type: file.type || 'application/pdf' });
      const newFile = new File([blob], file.name, { type: file.type || 'application/pdf' });
      (newFile as any)._localPath = path;
      appState.documents[idx] = newFile;
      appState.documentPaths[idx] = path;
      appState.documents = [...appState.documents];
    } catch (err) {
      console.error("Failed to refresh document:", err);
    }
  }

  async function saveForm() {
    if (appState.selectedDocumentIndex === null) return;
    const file = appState.documents[appState.selectedDocumentIndex];
    if (!file) return;
    // Convert booleans to strings
    const stringValues: Record<string, string> = {};
    for (const [k, v] of Object.entries(formValues)) {
       stringValues[k] = typeof v === 'boolean' ? (v ? 'Yes' : 'Off') : String(v);
    }
    try {
      const path = appState.documentPaths[appState.selectedDocumentIndex ?? 0] || file.name;
      await savePdfForm(path, path, stringValues);
      toastState.success('Form saved successfully');
    } catch (e) {
      toastState.error(`Failed to save form: ${e}`);
    }
  }

  function handleFilesDropped(files: File[]) {
    if (files.length === 0) return;
    const startIndex = appState.documents.length;
    appState.addDocuments(files);
    appState.selectDocument(startIndex);
  }

  let currentFileUrl = $state('');

  let pdfDoc: any = $state(null);
  let pageNum = $state(1);
  let scale = $state(1.2);
  let numPages = $derived(pdfDoc ? pdfDoc.numPages : 0);
  let formValues = $state<Record<string, any>>({});
  let hasFormFields = $state(false);

  let activeTool = $state('none');
  let annotations = $state<Annotation[]>([]);

  $effect(() => {
    let objectUrl = '';
    let isAborted = false;

    async function loadFile() {
      const activeIndex = appState.selectedDocumentIndex;
      if (activeIndex !== null) {
        const file = appState.documents[activeIndex];
        if (file) {
          const isLoaded = (file as any)._isLoaded !== false;
          if (!isLoaded) {
            const path = (file as any)._localPath;
            if (path) {
              try {
                const bytes: number[] = await safeInvoke('read_file_bytes', { path });
                if (isAborted) return; // Prevent overwriting state if user switched tabs

                const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
                const newFile = new File([blob], file.name, { type: 'application/pdf' });
                Object.defineProperty(newFile, 'size', { value: file.size, writable: false });
                (newFile as any)._localPath = path;
                (newFile as any)._isLoaded = true;

                appState.documents[activeIndex] = newFile;
                objectUrl = URL.createObjectURL(newFile);
                currentFileUrl = objectUrl;
              } catch (e) {
                if (isAborted) return;
                console.error("Failed to lazy load file from Tauri, attempting fallback to original file", e);

                if (file.size > 0) {
                  objectUrl = URL.createObjectURL(file);
                  currentFileUrl = objectUrl;
                } else {
                  currentFileUrl = '';
                }
              }
            } else {
               // Fallback when no path is available (e.g. Playwright testing via standard file input)
               if (file.size > 0) {
                 objectUrl = URL.createObjectURL(file);
                 currentFileUrl = objectUrl;
               } else {
                 currentFileUrl = '';
               }
            }
          } else {
            objectUrl = URL.createObjectURL(file);
            currentFileUrl = objectUrl;
          }
        } else {
          currentFileUrl = '';
        }
      } else {
        currentFileUrl = '';
      }
    }

    loadFile();

    return () => {
      isAborted = true;
      if (objectUrl) {
        URL.revokeObjectURL(objectUrl);
      }
    };
  });
</script>

<svelte:window onkeydown={(e) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'd') {
    e.preventDefault();
    appState.toggleDiffView();
  }
}} />

<div class="page-container">
  {#if appState.activeTab === 'home' || appState.activeTab === 'documents'}
    <div class="workspace-wrapper">
      <DocumentTabBar />

      <div class="workspace-body">
        {#if appState.showDiffView}
          <PdfVisualDiff onClose={() => appState.showDiffView = false} />
        {:else if appState.selectedDocumentIndex !== null}
          <div class="viewer-wrapper">
            <div class="viewer-content">
            <PdfThumbnails {pdfDoc} bind:pageNum />

            <div class="viewer-main">
              <div class="toolbars-container">
                <PdfToolbar bind:pageNum {numPages} bind:scale bind:activeTool />
              </div>
              {#if hasFormFields}
                <div style="padding: 10px; text-align: center; background: var(--bg-surface);"><button class="save-form-btn" onclick={saveForm} style="padding: 8px 16px; background: #4f46e5; color: white; border: none; border-radius: 4px; cursor: pointer;">Save Form</button></div>
              {/if}
              <div class="pdf-wrapper" style="position: relative; flex: 1; overflow: hidden;">
                <PdfViewer
                  fileUrl={currentFileUrl}
                  bind:pdfDoc
                  bind:pageNum
                  bind:scale
                  activeTool={activeTool}
                  bind:annotations
                />
                <PdfFormLayer {pdfDoc} {pageNum} {scale} bind:formValues bind:hasFormFields />
              </div>
            </div>

            <ViewerRightPanel
              bind:annotations
              {pdfDoc}
              onJumpToPage={(p: number) => pageNum = p}
            />
          </div>
            <StatusBar
              {pageNum}
              {numPages}
              {scale}
              fileName={appState.selectedDocumentIndex !== null
                ? (appState.documents[appState.selectedDocumentIndex]?.name ?? '')
                : ''}
              fileSize={appState.selectedDocumentIndex !== null
                ? (appState.documents[appState.selectedDocumentIndex]?.size ?? 0)
                : 0}
            />
          </div>
        {:else}
          <div class="documents-layout">
            <div class="documents-view">
              <header class="view-header">
                <h2>Documents</h2>
                <p>Add PDF files to process</p>
              </header>

              <DropZone ondrop={handleFilesDropped} />
              <DocumentList />
            </div>
            <OperationsPanel />
          </div>
        {/if}
      </div>
    </div>
  {:else if appState.activeTab === 'settings'}
    <div class="content-area full-width">
      <SettingsPanel />
    </div>
  {:else if appState.activeTab === 'pipeline'}
    <div class="content-area full-width">
      <PipelineView />
    </div>
  {:else if appState.activeTab === 'api'}
    <div class="content-area full-width">
      <ApiDocsView />
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

  .workspace-wrapper {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .workspace-body {
    display: flex;
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .documents-layout {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .documents-view {
    flex: 1;
    padding: 32px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    max-width: 900px;
    margin: 0 auto;
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

  .toolbars-container {
    display: flex;
    flex-direction: column;
  }
</style>
