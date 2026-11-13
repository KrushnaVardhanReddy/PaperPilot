<script lang="ts">
  import type { ToolDefinition } from '$lib/types/tools';
  import { allTools, allCategories } from '$lib/types/toolList';
  import { rankToolsByContext } from '$lib/services/toolRegistry';


  import { requireInvoke as safeInvoke } from '$lib/utils/tauri';

  import { appState } from '$lib/state/app.svelte';
  import DropZone from '$lib/components/ui/DropZone.svelte';
  import DocumentList from '$lib/components/ui/DocumentList.svelte';
  import PdfViewer from '$lib/components/PdfViewer.svelte';
  import PdfThumbnails from '$lib/components/PdfThumbnails.svelte';
  import SettingsPanel from '$lib/components/layout/SettingsPanel.svelte';
  import OperationsPanel from '$lib/components/layout/OperationsPanel.svelte';
  import GlobalCommandBar from '$lib/components/GlobalCommandBar.svelte';
  import PipelineView from '$lib/components/pipeline/PipelineView.svelte';
  import type { Annotation } from '$lib/api/pdf';
  import PdfFormLayer from '$lib/components/PdfFormLayer.svelte';
  import ApiDocsView from '$lib/components/layout/ApiDocsView.svelte';
  import PdfVisualDiff from '$lib/components/PdfVisualDiff.svelte';
  import SpreadsheetViewer from '$lib/components/workspace/SpreadsheetViewer.svelte';
  import ImageViewer from '$lib/components/workspace/ImageViewer.svelte';
  import JsonViewer from '$lib/components/workspace/JsonViewer.svelte';
  import MarkdownViewer from '$lib/components/workspace/MarkdownViewer.svelte';
  import HtmlViewer from '$lib/components/workspace/HtmlViewer.svelte';
  import DocxViewer from '$lib/components/workspace/DocxViewer.svelte';
  import DocumentTabBar from '$lib/components/layout/DocumentTabBar.svelte';
  import SignatureModal, { type AdoptedSignature } from '$lib/components/signature/SignatureModal.svelte';
  import type { PlacedStamp, ArmedSignature } from '$lib/components/signature/SignatureStampLayer.svelte';
  import { isPdfFile, isSpreadsheetFile, isImageFile, isJsonFile, isMarkdownFile, isHtmlFile, isDocxFile, getMimeType } from '$lib/utils/fileType';
  import { listen } from '@tauri-apps/api/event';

  let drawerMode = $state<'auto' | 'files' | 'pages'>('auto');

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

    // Key bindings & viewer actions
    const handleZoomIn = () => { scale = Math.min(Math.round((scale + 0.2) * 10) / 10, 5.0); };
    const handleZoomOut = () => { scale = Math.max(Math.round((scale - 0.2) * 10) / 10, 0.2); };

    window.addEventListener('paperpilot:zoom-in', handleZoomIn);
    window.addEventListener('paperpilot:zoom-out', handleZoomOut);
    window.addEventListener('paperpilot:rotate-single-page', handleRotatePage as unknown as EventListener);
    window.addEventListener('paperpilot:delete-single-page', handleDeletePage as unknown as EventListener);
    window.addEventListener('paperpilot:reorder-page', handleReorderPage as unknown as EventListener);
    window.addEventListener('paperpilot:duplicate-page', handleDuplicatePage as unknown as EventListener);
    window.addEventListener('paperpilot:extract-single-page', handleExtractPage as unknown as EventListener);

    return () => {
      if (unlistenMenuToggleDiff) unlistenMenuToggleDiff();
      window.removeEventListener('paperpilot:zoom-in', handleZoomIn);
      window.removeEventListener('paperpilot:zoom-out', handleZoomOut);
      window.removeEventListener('paperpilot:rotate-single-page', handleRotatePage as unknown as EventListener);
      window.removeEventListener('paperpilot:delete-single-page', handleDeletePage as unknown as EventListener);
      window.removeEventListener('paperpilot:reorder-page', handleReorderPage as unknown as EventListener);
      window.removeEventListener('paperpilot:duplicate-page', handleDuplicatePage as unknown as EventListener);
      window.removeEventListener('paperpilot:extract-single-page', handleExtractPage as unknown as EventListener);
    };
  });

  import { savePdfForm } from '$lib/api/pdf';
  import { toastState } from '$lib/state/toast.svelte';
  import { formatUserFriendlyError } from '$lib/utils/errorFormatter';


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
          output: path,
          _zero_trace: appState.zeroTrace
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page ${e.detail.page} rotated by ${angle}°.`);
    } catch (err) {
      const formatted = formatUserFriendlyError(err, 'Rotate Page');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
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
          output: path,
          _zero_trace: appState.zeroTrace
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page ${e.detail.page} deleted.`);
    } catch (err) {
      const formatted = formatUserFriendlyError(err, 'Delete Page');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
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
          output: path,
          _zero_trace: appState.zeroTrace
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page moved to position ${targetIndex + 1}.`);
    } catch (err) {
      const formatted = formatUserFriendlyError(err, 'Reorder Pages');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
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
          output: path,
          _zero_trace: appState.zeroTrace
        }
      });
      await refreshCurrentDocument();
      toastState.success(`Page ${page} duplicated.`);
    } catch (err) {
      const formatted = formatUserFriendlyError(err, 'Duplicate Page');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
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
          output: outputPath,
          _zero_trace: appState.zeroTrace
        }
      });
      toastState.success(`Page ${e.detail.page} extracted to ${outputPath}`);
    } catch (err) {
      const formatted = formatUserFriendlyError(err, 'Extract Page');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
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
      const oldInsp = appState.inspections.get(file);
      if (oldInsp) appState.inspections.set(newFile, oldInsp);
      appState.documents[idx] = newFile;
      appState.documentPaths[idx] = path;
      appState.documents = [...appState.documents];
    } catch (err) {
      console.error("Failed to refresh document:", err);
    }
  }

  async function handleSaveForm() {
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
      const formatted = formatUserFriendlyError(e, 'Save Form');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
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
  let isOperationsDrawerOpen = $derived(appState.isOperationsDrawerOpen);
  let numPages = $derived(pdfDoc ? pdfDoc.numPages : 0);
  let formValues = $state<Record<string, any>>({});
  let hasFormFields = $state(false);

  let activeTool = $state('none');
  let annotations = $state<Annotation[]>([]);

  // Spec 152: Interactive Signature & Stamp State
  let showSignatureModal = $state(false);
  let signatureModalInitialTab = $state<'type' | 'draw' | 'upload'>('type');
  let showSignatureMenu = $state(false);
  let armedSignature = $state<ArmedSignature | null>(null);
  let placedStamps = $state<PlacedStamp[]>([]);
  let isBakingSignature = $state(false);

  function getSavedSignatures(): AdoptedSignature[] {
    if (typeof localStorage === 'undefined') return [];
    try {
      const raw = localStorage.getItem('paperpilot:saved_signatures');
      return raw ? JSON.parse(raw) : [];
    } catch {
      return [];
    }
  }

  function handleSignPillClick() {
    const saved = getSavedSignatures();
    if (saved.length === 0 && !armedSignature) {
      signatureModalInitialTab = 'type';
      showSignatureModal = true;
      showSignatureMenu = false;
    } else {
      showSignatureMenu = !showSignatureMenu;
    }
  }

  function handleAdoptSignature(sig: AdoptedSignature) {
    armedSignature = {
      dataUrl: sig.dataUrl,
      width: sig.width,
      height: sig.height,
      label: sig.label,
      isInitials: sig.isInitials
    };
    showSignatureModal = false;
    showSignatureMenu = false;
    toastState.info(`Signature armed. Click anywhere on Page ${pageNum} to place it.`);
  }

  function handlePlaceSignatureClick() {
    showSignatureMenu = false;
    const saved = getSavedSignatures();
    const sig = saved.find(s => !s.isInitials) || saved[0];
    if (sig) {
      armedSignature = {
        dataUrl: sig.dataUrl,
        width: sig.width,
        height: sig.height,
        label: sig.label,
        isInitials: false
      };
      toastState.info(`Signature armed. Click on Page ${pageNum} to place it.`);
    } else {
      signatureModalInitialTab = 'type';
      showSignatureModal = true;
    }
  }

  function handlePlaceInitialsClick() {
    showSignatureMenu = false;
    const saved = getSavedSignatures();
    const initialsSig = saved.find(s => s.isInitials);
    if (initialsSig) {
      armedSignature = {
        dataUrl: initialsSig.dataUrl,
        width: initialsSig.width,
        height: initialsSig.height,
        label: initialsSig.label,
        isInitials: true
      };
      toastState.info(`Initials armed. Click on Page ${pageNum} to place them.`);
    } else {
      signatureModalInitialTab = 'type';
      showSignatureModal = true;
    }
  }

  function handleFreehandPenClick() {
    showSignatureMenu = false;
    armedSignature = null;
    activeTool = activeTool === 'pen' ? 'none' : 'pen';
  }

  function handleWindowClick(e: MouseEvent) {
    if (showSignatureMenu) {
      const target = e.target as HTMLElement;
      if (!target.closest('.signature-menu-popover') && !target.closest('#btn-sign-annotate')) {
        showSignatureMenu = false;
      }
    }
  }

  async function handleApplySignatureToDocument() {
    if (placedStamps.length === 0 || appState.selectedDocumentIndex === null) return;
    const docIdx = appState.selectedDocumentIndex;
    const file = appState.documents[docIdx];
    if (!file) return;
    const path = appState.documentPaths[docIdx] || (file as any)._localPath || file.name;

    isBakingSignature = true;
    appState.setLoading(true);

    try {
      for (const stamp of placedStamps) {
        await safeInvoke('invoke_mcp_tool', {
          toolName: 'pdf_stamp',
          arguments: {
            input: path,
            output: path,
            image_data: stamp.dataUrl,
            x: stamp.x,
            y: stamp.y,
            width: stamp.width,
            height: stamp.height,
            pages: String(stamp.page),
            opacity: 1.0
          }
        });
      }

      toastState.success(`Signature successfully applied to Page ${placedStamps[0]?.page || pageNum}!`);
      placedStamps = [];
      armedSignature = null;

      await refreshCurrentDocument();
    } catch (err: any) {
      console.error('Failed to bake signature:', err);
      const formatted = formatUserFriendlyError(err, 'Apply Signature');
      toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
    } finally {
      isBakingSignature = false;
      appState.setLoading(false);
    }
  }

  let activePackIds = $derived(
    appState.featurePacks.filter((p: any) => p.is_enabled).map((p: any) => p.id)
  );
  let ranked = $derived(
    rankToolsByContext(allTools, appState.documents, activePackIds)
  );

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

                const inferredMime = file.type || getMimeType(file.name);
                const blob = new Blob([new Uint8Array(bytes)], { type: inferredMime });
                const newFile = new File([blob], file.name, { type: inferredMime });
                Object.defineProperty(newFile, 'size', { value: file.size, writable: false });
                (newFile as any)._localPath = path;
                (newFile as any)._isLoaded = true;

                const oldFile = appState.documents[activeIndex];
                const oldInspection = appState.inspections.get(oldFile);
                if (oldInspection) {
                  appState.inspections.set(newFile, oldInspection);
                }
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

<svelte:window
  onclick={handleWindowClick}
  onkeydown={(e) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'd') {
    e.preventDefault();
    appState.toggleDiffView();
  }
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'b') {
    e.preventDefault();
    appState.toggleContextPanel();
  }
}} />

<div class="page-container three-column-layout">
  {#if appState.activeTab === 'home' || appState.activeTab === 'documents'}
    <div class="workspace-wrapper">
      <DocumentTabBar />
      <div class="workspace-body">
        <!-- COLUMN 2: Contextual Drawer -->
        {#if appState.isContextPanelOpen}
          <div class="context-drawer">
            <div class="context-drawer-header">
              <div class="drawer-mode-tabs">
                <button
                  class="drawer-tab-btn"
                  class:active={drawerMode === 'files' || (drawerMode === 'auto' && appState.selectedDocumentIndex === null)}
                  onclick={() => drawerMode = 'files'}
                  title="Document Library"
                >
                  📁 Files
                </button>
                <button
                  class="drawer-tab-btn"
                  class:active={drawerMode === 'pages' || (drawerMode === 'auto' && appState.selectedDocumentIndex !== null)}
                  disabled={appState.selectedDocumentIndex === null || !isPdfFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
                  onclick={() => drawerMode = 'pages'}
                  title="Page Thumbnails"
                >
                  📑 Pages
                </button>
              </div>
              <button class="collapse-btn" title="Collapse Drawer" onclick={() => appState.toggleContextPanel()}>
                ‹
              </button>
            </div>

            {#if drawerMode === 'files' || (drawerMode === 'auto' && appState.selectedDocumentIndex === null)}
              <div class="drawer-library">
                <DocumentList />
              </div>
            {:else if isPdfFile(appState.documents[appState.selectedDocumentIndex ?? 0]?.name ?? '')}
              <div class="drawer-thumbnails">
                <PdfThumbnails {pdfDoc} bind:pageNum />
              </div>
            {/if}
          </div>
        {/if}

        <!-- COLUMN 3: Main Stage Hero Canvas -->
        <div class="main-stage">
          {#if appState.showDiffView}
            <PdfVisualDiff onClose={() => appState.showDiffView = false} />
          {:else if appState.selectedDocumentIndex !== null && isSpreadsheetFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
            {@const selectedDoc = appState.documents[appState.selectedDocumentIndex]}
            <SpreadsheetViewer file={selectedDoc} />
          {:else if appState.selectedDocumentIndex !== null && isImageFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
            {@const selectedDoc = appState.documents[appState.selectedDocumentIndex]}
            <ImageViewer file={selectedDoc} fileUrl={currentFileUrl} />
          {:else if appState.selectedDocumentIndex !== null && isJsonFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
            {@const selectedDoc = appState.documents[appState.selectedDocumentIndex]}
            <JsonViewer file={selectedDoc} />
          {:else if appState.selectedDocumentIndex !== null && isMarkdownFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
            {@const selectedDoc = appState.documents[appState.selectedDocumentIndex]}
            <MarkdownViewer file={selectedDoc} />
          {:else if appState.selectedDocumentIndex !== null && isHtmlFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
            {@const selectedDoc = appState.documents[appState.selectedDocumentIndex]}
            <HtmlViewer file={selectedDoc} />
          {:else if appState.selectedDocumentIndex !== null && isDocxFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
            {@const selectedDoc = appState.documents[appState.selectedDocumentIndex]}
            <DocxViewer file={selectedDoc} />
          {:else if appState.selectedDocumentIndex !== null && isPdfFile(appState.documents[appState.selectedDocumentIndex]?.name ?? '')}
            {@const selectedDoc = appState.documents[appState.selectedDocumentIndex]}
            <div class="viewer-wrapper">
              <div class="viewer-main">
                <div class="pdf-wrapper">
                  <PdfViewer
                    fileUrl={currentFileUrl}
                    file={selectedDoc}
                    bind:pdfDoc
                    bind:pageNum
                    bind:scale
                    activeTool={activeTool}
                    bind:annotations
                    bind:armedSignature
                    bind:placedStamps
                    onStampPlaced={(stamp: PlacedStamp) => {
                      toastState.info(`Signature placed on Page ${stamp.page}. Click 'Apply to Document' to bake it.`);
                    }}
                  />
                  <PdfFormLayer {pdfDoc} {pageNum} {scale} bind:formValues bind:hasFormFields />
                </div>
              </div>

              <!-- Floating Action Pill Wrapper with Spec 152 Signature Bar & Popover -->
              <div class="floating-action-pill-wrapper">
                {#if placedStamps.length > 0}
                  <div class="signature-action-bar" role="region" aria-label="Signature Action Bar">
                    <span class="sig-status-text">
                      ✍️ {placedStamps.length} signature{placedStamps.length > 1 ? 's' : ''} placed on Page {pageNum}
                    </span>
                    <div class="sig-action-btns">
                      <button type="button" class="btn-cancel-stamps" onclick={() => { placedStamps = []; armedSignature = null; }}>
                        Cancel
                      </button>
                      <button
                        type="button"
                        class="btn-apply-stamps"
                        onclick={handleApplySignatureToDocument}
                        disabled={isBakingSignature}
                      >
                        {#if isBakingSignature}
                          ⏳ Applying...
                        {:else}
                          Apply to Document
                        {/if}
                      </button>
                    </div>
                  </div>
                {/if}

                {#if showSignatureMenu}
                  <div class="signature-menu-popover" role="menu">
                    <div class="menu-header">Sign & Stamp</div>
                    <button
                      type="button"
                      class="sig-menu-item"
                      role="menuitem"
                      onclick={handlePlaceSignatureClick}
                    >
                      <span class="item-icon">✍</span>
                      <div class="item-text">
                        <span class="item-title">Place Signature</span>
                        <span class="item-subtitle">Stamp saved full signature</span>
                      </div>
                    </button>
                    <button
                      type="button"
                      class="sig-menu-item"
                      role="menuitem"
                      onclick={handlePlaceInitialsClick}
                    >
                      <span class="item-icon">📝</span>
                      <div class="item-text">
                        <span class="item-title">Place Initials</span>
                        <span class="item-subtitle">Stamp saved initials</span>
                      </div>
                    </button>
                    <button
                      type="button"
                      class="sig-menu-item"
                      role="menuitem"
                      onclick={handleFreehandPenClick}
                    >
                      <span class="item-icon">✏️</span>
                      <div class="item-text">
                        <span class="item-title">Freehand Pen</span>
                        <span class="item-subtitle">Draw annotations on document</span>
                      </div>
                    </button>
                    <div class="menu-divider"></div>
                    <button
                      type="button"
                      class="sig-menu-item primary-action"
                      role="menuitem"
                      onclick={() => { showSignatureMenu = false; signatureModalInitialTab = 'type'; showSignatureModal = true; }}
                    >
                      <span class="item-icon">➕</span>
                      <div class="item-text">
                        <span class="item-title">Create New Signature...</span>
                        <span class="item-subtitle">Type, draw, or upload</span>
                      </div>
                    </button>
                    {#if placedStamps.length > 0}
                      <div class="menu-divider"></div>
                      <button
                        type="button"
                        class="sig-menu-item danger-action"
                        role="menuitem"
                        onclick={() => { placedStamps = []; armedSignature = null; showSignatureMenu = false; }}
                      >
                        <span class="item-icon">🗑️</span>
                        <div class="item-text">
                          <span class="item-title">Clear Placed Stamps</span>
                        </div>
                      </button>
                    {/if}
                  </div>
                {/if}

                <div class="floating-action-pill">
                  <button title="Zoom Out" onclick={() => window.dispatchEvent(new CustomEvent('paperpilot:zoom-out'))}>🔍 -</button>
                  <button title="Zoom In" onclick={() => window.dispatchEvent(new CustomEvent('paperpilot:zoom-in'))}>🔍 +</button>
                  <div class="divider"></div>
                  <button title="Rotate 90°" onclick={() => window.dispatchEvent(new CustomEvent('paperpilot:rotate-single-page', { detail: { page: pageNum, angle: 90 } }))}>↻</button>
                  <div class="divider"></div>
                  <button
                    id="btn-sign-annotate"
                    title="Sign / Annotate"
                    class:active={showSignatureMenu || !!armedSignature || activeTool === 'pen'}
                    onclick={handleSignPillClick}
                  >
                    ✍
                  </button>
                  <div class="divider"></div>
                  <button title="Tool Settings" class:active={isOperationsDrawerOpen} onclick={() => appState.toggleOperationsDrawer()}>⚙️ Tools</button>
                </div>
              </div>
            </div>
          {:else}
            <!-- Empty state / Dropzone when no document is active -->
            <div class="empty-stage">
              <div class="documents-view">
                <header class="view-header">
                  <h2>Workspace</h2>
                  <p>
                    {#if appState.documents.length === 0}
                      Drop PDF or Office files to get started
                    {:else}
                      Select a document from the library
                    {/if}
                  </p>
                </header>
                <DropZone ondrop={handleFilesDropped} />
              </div>
            </div>
          {/if}
        </div>

        <!-- SLIDE-OUT RIGHT DRAWER -->
        <div class="right-drawer" class:open={isOperationsDrawerOpen}>
          <div class="right-drawer-header">
            <h3>Tools & Actions</h3>
            <button class="close-btn" onclick={() => appState.toggleOperationsDrawer(false)} title="Close tools">✕</button>
          </div>
          <div class="right-drawer-content">
            <OperationsPanel embedded={true} />
          </div>
        </div>
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

{#if appState.isMobile && (appState.activeTool || appState.activeTab === 'tools')}
  <OperationsPanel />
{/if}

<SignatureModal
  isOpen={showSignatureModal}
  initialTab={signatureModalInitialTab}
  onAdopt={handleAdoptSignature}
  onClose={() => showSignatureModal = false}
/>

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

  .context-drawer {
    width: 260px;
    background-color: var(--bg-surface);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-shrink: 0;
    transition: all 0.3s ease;
  }
  .context-drawer-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    border-bottom: 1px solid var(--border-color);
    gap: 12px;
  }
  .drawer-mode-tabs {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--bg-primary);
    padding: 4px;
    border-radius: 8px;
  }
  .drawer-tab-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 12px;
    font-weight: 500;
    padding: 5px 12px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s ease;
  }
  .drawer-tab-btn:hover:not(:disabled) {
    color: var(--text-primary);
  }
  .drawer-tab-btn.active {
    background: var(--bg-surface);
    color: var(--text-primary);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }
  .drawer-tab-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }
  .collapse-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 1.5rem;
    cursor: pointer;
    line-height: 1;
    padding: 4px 8px;
    border-radius: 4px;
  }
  .collapse-btn:hover {
    background: var(--bg-primary);
    color: var(--text-primary);
  }
  .drawer-library, .drawer-thumbnails {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
  }

  .main-stage {
    flex: 1;
    position: relative;
    background-color: var(--bg-primary);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .empty-stage {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 24px;
  }

  .floating-action-pill-wrapper {
    position: absolute;
    bottom: 24px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    z-index: 100;
    pointer-events: none;
  }

  .floating-action-pill-wrapper > * {
    pointer-events: auto;
  }

  .signature-action-bar {
    background: var(--bg-surface, #1e1e24);
    color: var(--text-primary, #ffffff);
    padding: 8px 16px;
    border-radius: 24px;
    border: 1px solid var(--border-color, rgba(255, 255, 255, 0.2));
    box-shadow: var(--shadow-lg, 0 10px 25px -5px rgba(0, 0, 0, 0.4));
    display: flex;
    align-items: center;
    gap: 16px;
    animation: pillSlideUp 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes pillSlideUp {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .sig-status-text {
    font-size: 13px;
    font-weight: 500;
    letter-spacing: 0.01em;
  }

  .sig-action-btns {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .btn-cancel-stamps {
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.25);
    color: #cbd5e1;
    border-radius: 14px;
    padding: 4px 12px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-cancel-stamps:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }

  .btn-apply-stamps {
    background: #2563eb;
    border: 1px solid #1d4ed8;
    color: #ffffff;
    border-radius: 14px;
    padding: 5px 14px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    box-shadow: 0 2px 6px rgba(37, 99, 235, 0.4);
  }

  .btn-apply-stamps:hover:not(:disabled) {
    background: #1d4ed8;
  }

  .btn-apply-stamps:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .signature-menu-popover {
    background: #ffffff;
    color: #0f172a;
    border-radius: 12px;
    box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.25), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
    border: 1px solid #cbd5e1;
    width: 240px;
    overflow: hidden;
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    animation: pillSlideUp 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .menu-header {
    font-size: 11px;
    font-weight: 600;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 6px 10px 4px;
  }

  .sig-menu-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 8px;
    border: none;
    background: transparent;
    cursor: pointer;
    text-align: left;
    width: 100%;
    transition: background 0.15s ease;
  }

  .sig-menu-item:hover {
    background: #f1f5f9;
  }

  .item-icon {
    font-size: 16px;
    width: 22px;
    text-align: center;
  }

  .item-text {
    display: flex;
    flex-direction: column;
  }

  .item-title {
    font-size: 13px;
    font-weight: 500;
    color: #0f172a;
  }

  .item-subtitle {
    font-size: 11px;
    color: #64748b;
  }

  .menu-divider {
    height: 1px;
    background: #e2e8f0;
    margin: 4px 0;
  }

  .primary-action:hover {
    background: #eff6ff;
  }

  .primary-action .item-title {
    color: #2563eb;
    font-weight: 600;
  }

  .danger-action:hover {
    background: #fee2e2;
  }

  .danger-action .item-title {
    color: #dc2626;
  }

  .floating-action-pill {
    position: relative;
    display: flex;
    align-items: center;
    background: rgba(30, 30, 30, 0.85);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;
    padding: 4px 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  }

  .floating-action-pill button {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 8px 12px;
    margin: 0 4px;
    border-radius: 16px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }
  .floating-action-pill button:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.1);
  }
  .floating-action-pill button.active {
    color: var(--accent-primary);
    background: rgba(59, 130, 246, 0.15);
  }

  .divider {
    width: 1px;
    height: 20px;
    background: rgba(255, 255, 255, 0.15);
    margin: 0 8px;
  }

  .right-drawer {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    width: 360px;
    background-color: var(--bg-surface);
    border-left: 1px solid var(--border-color);
    box-shadow: -4px 0 15px rgba(0, 0, 0, 0.2);
    transform: translateX(100%);
    transition: transform 0.3s ease;
    display: flex;
    flex-direction: column;
    z-index: 200;
  }
  .right-drawer.open {
    transform: translateX(0);
  }
  .right-drawer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
  }
  .right-drawer-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  .right-drawer-header .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 1.25rem;
    cursor: pointer;
    padding: 4px;
  }
  .right-drawer-header .close-btn:hover {
    color: var(--text-primary);
  }
  .right-drawer-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    padding: 0;
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
    gap: 24px;
    max-width: 900px;
    margin: 0 auto;
  }

  @media (max-width: 768px) {
    .documents-view {
      padding: 16px 12px 100px 12px;
    }
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
    min-height: 0;
    min-width: 0;
    position: relative;
  }

  .pdf-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    overflow: hidden;
    position: relative;
    min-height: 0;
    min-width: 0;
  }

  .toolbars-container {
    display: flex;
    flex-direction: column;
  }

  .status-pills {
    display: flex;
    gap: 12px;
    margin-bottom: 24px;
    justify-content: flex-start;
  }
  .status-pill {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .studio-hub-layout {
    display: flex;
    flex-direction: column;
    gap: 24px;
    width: 100%;
  }

  .documents-layout .documents-view {
    max-width: 100%;
  }
  .operations-panel-wrapper {
    display: flex;
    flex-shrink: 0;
  }

  .studio-operations-grid {
    display: flex;
    flex-direction: column;
    gap: 32px;
    margin-top: 16px;
  }
  .studio-category h3 {
    font-size: 1.1rem;
    color: var(--text-secondary);
    margin-bottom: 12px;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 8px;
  }
  .studio-cards {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
  }
  .studio-card {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    padding: 16px;
    border-radius: 8px;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  .studio-card:hover {
    border-color: var(--accent-primary, #3b82f6);
    background: var(--bg-primary);
    transform: translateY(-2px);
  }
  .studio-card-icon {
    font-size: 1.5rem;
  }
  .studio-card-content h4 {
    margin: 0 0 4px 0;
    font-size: 1rem;
    color: var(--text-primary);
  }
  .studio-card-content p {
    margin: 0;
    font-size: 0.8rem;
    color: var(--text-muted);
  }
  .batch-summary {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    margin-top: 8px;
  }
  .clear-btn {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.3);
    padding: 6px 12px;
    border-radius: 4px;
    font-size: 0.8rem;
    cursor: pointer;
  }
  .clear-btn:hover {
    background: rgba(239, 68, 68, 0.2);
  }
  .smart-badge-studio {
    display: inline-block;
    margin-left: 8px;
    padding: 2px 6px;
    border-radius: 10px;
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.3);
    vertical-align: middle;
  }

  .format-pill {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 6px;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 4px;
    font-size: 9px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 8px;
  }
  .fmt-in {
    color: var(--text-secondary, #9ca3af);
  }
  .fmt-arrow {
    color: var(--accent-primary, #5e6ad2);
    font-size: 8px;
  }
  .fmt-out {
    color: var(--text-primary, #ffffff);
  }

</style>
