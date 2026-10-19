<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { jobsState } from '$lib/state/jobs.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { formatUserFriendlyError } from '$lib/utils/errorFormatter';

  import type { ToolDefinition } from "$lib/types/tools";
  import { allTools, categories, allCategories } from "$lib/types/toolList";

  import { resolveToolName, rankToolsByContext } from '$lib/services/toolRegistry';
import { validatePreconditions } from '$lib/services/operationValidator';
import { buildOperationArgs, type OperationContext } from '$lib/services/operationArgsBuilder';
import { onMount } from 'svelte';

  let searchQuery = $state('');
  let selectedCategory = $state('all');
  let activeTool = $derived(appState.activeTool);
  let isCollapsed = $state(false);
  let panelWidth = $state(320);
  let isDragging = $state(false);

  onMount(() => {
    const stored = localStorage.getItem('operations-panel-collapsed');
    if (stored === 'true') isCollapsed = true;

    const storedWidth = localStorage.getItem('operations-panel-width');
    if (storedWidth) {
      const parsed = parseInt(storedWidth, 10);
      if (!isNaN(parsed) && parsed >= 260 && parsed <= 700) {
        panelWidth = parsed;
      }
    }
  });

  $effect(() => {
    localStorage.setItem('operations-panel-collapsed', String(isCollapsed));
  });

  $effect(() => {
    localStorage.setItem('operations-panel-width', String(panelWidth));
  });

  function startResize(e: MouseEvent) {
    if (isCollapsed) return;
    isDragging = true;
    const startX = e.clientX;
    const startWidth = panelWidth;

    function onMouseMove(moveEvent: MouseEvent) {
      const delta = startX - moveEvent.clientX;
      const newWidth = Math.min(Math.max(startWidth + delta, 260), 700);
      panelWidth = newWidth;
    }

    function onMouseUp() {
      isDragging = false;
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
    }

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  }

  function handleResizeDblClick() {
    panelWidth = panelWidth > 380 ? 320 : 480;
  }

  // Tool parameter states
  let splitPoints = $state('');
  let deletePages = $state('');
  let extractPages = $state('');
  let compressQuality = $state(80);
  let rotateAngle = $state('90');
  let rotatePages = $state('');
  let watermarkText = $state('');

  let watermarkOpacity = $state(20);
  let watermarkOrientation = $state('diagonal');
  let watermarkColor = $state('gray');

  let password = $state('');
  let metadataTitle = $state('');

  // New tool parameters
  let blankSensitivity = $state(99);
  let pageNumberPos = $state('bottom-right');
  let pageNumberFormat = $state('Page 1 of N');
  let convertPreset = $state('academic');

  // Additional 44 tools parameter states
  let batesPrefix = $state('CONF-');
  let batesStart = $state(1);
  let batesPadding = $state(6);
  let reorderList = $state('');
  let cropBox = $state('');
  let signCertPath = $state('');
  let headerText = $state('');
  let footerText = $state('');
  let ocrOutputFormat = $state<'pdf' | 'txt' | 'clipboard'>('pdf');

  let toolContext = $state<'pdf' | 'office' | 'ai'>('pdf');

  // Auto-switch tool context based on loaded files
  $effect(() => {
    const paths = appState.documentPaths;
    if (!paths || paths.length === 0) return;
    const officeExts = ['docx','doc','xlsx','xls','pptx','ppt'];
    const allOffice = paths.every((p: string) => officeExts.some(e => p.toLowerCase().endsWith(e)));
    const allPdf = paths.every((p: string) => p.toLowerCase().endsWith('.pdf'));
    if (allOffice && toolContext === 'pdf') toolContext = 'office';
    if (allPdf && toolContext === 'office') toolContext = 'pdf';
  });

  // Filter tools based on search query
  let activePackIds = $derived(
    appState.featurePacks.filter((p: any) => p.is_enabled).map((p: any) => p.id)
  );

  let ranked = $derived(
    rankToolsByContext(allTools, appState.documents, activePackIds)
  );

  let contextFilter = $state('recommended'); // 'recommended', 'all', 'pdf', 'office', 'ai'

  $effect(() => {
    if (appState.documents.length === 0 && contextFilter === 'recommended') {
      // Keep it as recommended, which now shows creation tools
    }
  });

  // Filter tools based on search query
  let filteredTools = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    let tools: ToolDefinition[] = [];

    if (q) {
      tools = allTools.filter(t =>
        t.title.toLowerCase().includes(q) ||
        t.description.toLowerCase().includes(q) ||
        t.tags.some(tag => tag.toLowerCase().includes(q))
      );
    } else {
       if (contextFilter === 'recommended') {
         tools = ranked.recommended;
       } else if (contextFilter === 'all') {
         tools = allTools;
       } else if (contextFilter === 'pdf') {
         tools = allTools.filter(t => t.context === 'pdf');
       } else if (contextFilter === 'office') {
         tools = allTools.filter(t => t.context === 'office');
       } else if (contextFilter === 'ai') {
         tools = allTools.filter(t => t.context === 'ai' || t.category === 'ai' || t.tags.includes('ocr') || t.tags.includes('ai'));
       }
    }
    if (selectedCategory !== 'all') {
      tools = tools.filter(t => t.category === selectedCategory);
    }
    return tools;
  });

  function selectTool(tool: ToolDefinition) {
    isCollapsed = false;
    appState.openTool(tool);
  }

  function backToDirectory() {
    appState.closeTool();
  }

  let customOutputPath = $state('');

  let isRunning = $state(false);
  let elapsedSeconds = $state(0);
  let elapsedDisplay = $derived(
    `${String(Math.floor(elapsedSeconds / 60)).padStart(2, '0')}:${String(elapsedSeconds % 60).padStart(2, '0')}`
  );

  interface ExecutionResult {
    success: boolean;
    durationFormatted: string;
    toolTitle: string;
    timestamp: Date;
    message: string;
  }

  let lastExecution = $state<ExecutionResult | null>(null);
  let executionHistory = $state<ExecutionResult[]>([]);

  // Auto-fill the output path whenever the active tool or selected document changes
  $effect(() => {
    if (!activeTool || appState.documents.length === 0) {
      customOutputPath = '';
      return;
    }
    const docIndex = appState.selectedDocumentIndex ?? 0;
    const docName = appState.documents[docIndex]?.name || '';
    const docPath = appState.documentPaths[docIndex] || docName;
    const docDir = docPath.includes('/') ? docPath.substring(0, docPath.lastIndexOf('/')) : '';
    const getOutputPath = (filename: string) => docDir ? `${docDir}/${filename}` : filename;
    const base = docName.replace(/\.[^.]+$/i, '');

    switch (activeTool.id) {
      case 'office_to_markdown':  customOutputPath = getOutputPath(`${base}.md`);       break;
      case 'office_extract_text': customOutputPath = getOutputPath(`${base}_text.txt`); break;
      case 'office_modernize_legacy': {
        const srcExt = docPath.split('.').pop()?.toLowerCase() ?? 'docx';
        const modernExt = srcExt === 'doc' ? 'docx' : srcExt === 'xls' ? 'xlsx' : srcExt === 'ppt' ? 'pptx' : srcExt;
        customOutputPath = getOutputPath(`${base}_modern.${modernExt}`);
        break;
      }
      case 'merge':
        customOutputPath = ''; // handled by native dialog
        break;
      case 'split':
        customOutputPath = getOutputPath(`${base}_split`);
        break;
      case 'compress':      customOutputPath = getOutputPath(`${base}_compressed.pdf`);  break;
      case 'rotate':        customOutputPath = getOutputPath(`${base}_rotated.pdf`);     break;
      case 'watermark':     customOutputPath = getOutputPath(`${base}_watermarked.pdf`); break;
      case 'encrypt':       customOutputPath = getOutputPath(`${base}_encrypted.pdf`);   break;
      case 'decrypt':       customOutputPath = getOutputPath(`${base}_decrypted.pdf`);   break;
      case 'metadata':      customOutputPath = getOutputPath(`${base}_metadata.pdf`);    break;
      case 'extract_pages': customOutputPath = getOutputPath(`${base}_extracted.pdf`);  break;
      case 'delete_pages':  customOutputPath = getOutputPath(`${base}_deleted.pdf`);    break;
      case 'extract_text':  customOutputPath = getOutputPath(`${base}_text.txt`);        break;
      case 'ocr':
        if (ocrOutputFormat === 'txt') {
          customOutputPath = getOutputPath(`${base}_ocr.txt`);
        } else if (ocrOutputFormat === 'clipboard') {
          customOutputPath = '';
        } else {
          customOutputPath = getOutputPath(`${base}_ocr.pdf`);
        }
        break;
      case 'pdf_to_docx':     customOutputPath = getOutputPath(`${base}.docx`);           break;
      case 'pdf_to_xlsx':     customOutputPath = getOutputPath(`${base}.xlsx`);           break;
      case 'pdf_to_markdown': customOutputPath = getOutputPath(`${base}.md`);             break;
      case 'pdf_to_html':     customOutputPath = getOutputPath(`${base}.html`);           break;
      case 'pdf_to_pptx':     customOutputPath = getOutputPath(`${base}.pptx`);           break;
      case 'extract_images':  customOutputPath = getOutputPath(`${base}_images`);          break;
      case 'office_to_markdown': customOutputPath = getOutputPath(`${base}.md`);           break;
      case 'office_extract_text': customOutputPath = getOutputPath(`${base}.txt`);          break;
      case 'excel_to_json':   customOutputPath = getOutputPath(`${base}.json`);            break;
      case 'excel_to_csv':    customOutputPath = getOutputPath(`${base}.csv`);             break;
      default:              customOutputPath = getOutputPath(`${base}_output.pdf`);      break;
    }
  });

  let applyToAllFiles = $state(true);

  async function handleRunOperation() {
    if (isRunning || !activeTool) return;

    const validation = validatePreconditions(activeTool.id, appState.documents.length);
    if (!validation.valid) {
      if (validation.error) toastState.error(validation.error);
      return;
    }

    if (activeTool.id === 'compare') {
      appState.toggleDiffView(true);
      return;
    }

    const isBatchExecution = applyToAllFiles && (activeTool.rule?.isBatchable ?? false) && appState.documents.length > 1;

    isRunning = true;
    elapsedSeconds = 0;
    const timer = setInterval(() => { elapsedSeconds++; }, 1000);

    try {
      const toolName = resolveToolName(activeTool.id);
      appState.setLoading(true);
      const startTime = performance.now();

      if (isBatchExecution) {
        // Run batch operation across all documents
        const totalDocs = appState.documents.length;
        let successCount = 0;
        let failCount = 0;
        const errors: string[] = [];

        for (let i = 0; i < totalDocs; i++) {
          const doc = appState.documents[i];
          const docName = doc?.name || `document_${i + 1}.pdf`;
          const docPath = appState.documentPaths[i] || docName;
          const docDir = docPath.includes('/') ? docPath.substring(0, docPath.lastIndexOf('/')) : '';
          const getDocOutputPath = (filename: string) => docDir ? `${docDir}/${filename}` : filename;

          const context: OperationContext = {
            docName,
            docPath,
            docDir,
            allDocumentPaths: appState.documentPaths,
            allDocuments: appState.documents,
            getOutputPath: getDocOutputPath,
            customOutputPath: '', // Auto per-doc naming in batch mode
            splitPoints,
            deletePages,
            extractPages,
            rotateAngle,
            rotatePages,
            watermarkText,
            watermarkOrientation,
            watermarkOpacity,
            watermarkColor,
            watermarkFontSize: 12,
            password,
            batesPrefix,
            batesStart,
            batesPadding,
            ocrOutputFormat,
            ocrLanguage: "eng",
            cropBox,
            pageNumberPosition: "bottom_right",
            pageNumberStart: 1,
            pageNumberFormat: "1",
            convertPreset,
            reorderList,
            signCertPath,
            headerText,
            footerText,
            metadataTitle,
            compressQuality
          };

          const args = await buildOperationArgs(activeTool.id, context);
          if (!args) continue;

          try {

            let result: any;
            if (appState.isMobile) {
               // Extract params for mobile
               let mobileParams = { ...args };
               // Find original bytes if any (this is tricky since we only have docPath and args)
               // The mobile bridge expects: fileName, inputBytes, operation, params
               // We will pass empty buffer and rely on path/file access or handle in rust if we were to fully bridge.
               // However, as per spec: "route tool execution directly through process_mobile_buffer(file_name, input_bytes, operation, params)"

               let inputBytes: number[] = [];
               if (doc && (doc as any)._isLoaded) {
                   const arrayBuf = await doc.arrayBuffer();
                   inputBytes = Array.from(new Uint8Array(arrayBuf));
               } else if (docPath) {
                   inputBytes = await invoke<number[]>('read_file_bytes', { path: docPath }).catch(() => []);
               }

               result = await invoke('process_mobile_buffer', {
                   fileName: docName,
                   inputBytes: inputBytes,
                   operation: activeTool.id,
                   params: mobileParams
               });
            } else {
                result = await invoke('invoke_mcp_tool', { toolName, arguments: args });
            }

            const resObj = result as any;
            if (resObj && resObj.success) {
              successCount++;
            } else {
              failCount++;
              errors.push(`${docName}: ${resObj?.message || 'Failed'}`);
            }
          } catch (err: any) {
            failCount++;
            errors.push(`${docName}: ${err?.message || String(err)}`);
          }
        }

        const durationMs = Math.round(performance.now() - startTime);
        const durationFormatted = durationMs < 1000 ? `${durationMs}ms` : `${(durationMs / 1000).toFixed(1)}s`;

        if (failCount === 0) {
          const msg = `✅ Batch ${activeTool.title} completed for all ${successCount} files!`;
          toastState.success(msg);
          jobsState.addJob(toolName, 'success', msg);
          lastExecution = {
            success: true,
            durationFormatted,
            toolTitle: activeTool.title,
            timestamp: new Date(),
            message: msg
          };
        } else {
          const msg = `Batch finished: ${successCount} succeeded, ${failCount} failed.`;
          toastState.error(msg, errors.slice(0, 2).join('; '));
          jobsState.addJob(toolName, 'error', msg);
          lastExecution = {
            success: successCount > 0,
            durationFormatted,
            toolTitle: activeTool.title,
            timestamp: new Date(),
            message: msg
          };
        }
        executionHistory = [lastExecution, ...executionHistory].slice(0, 10);
      } else {
        // Single document execution
        const docIndex = appState.selectedDocumentIndex ?? 0;
        const docName = appState.documents.length > 0 ? (appState.documents[docIndex]?.name || appState.documents[0].name) : '';
        const docPath = appState.documents.length > 0 ? (appState.documentPaths[docIndex] || docName) : '';
        const docDir = docPath.includes('/') ? docPath.substring(0, docPath.lastIndexOf('/')) : '';
        const getOutputPath = (filename: string) => docDir ? `${docDir}/${filename}` : filename;

        const context: OperationContext = {
          docName,
          docPath,
          docDir,
          allDocumentPaths: appState.documentPaths,
          allDocuments: appState.documents,
          getOutputPath,
          customOutputPath,
          splitPoints,
          deletePages,
          extractPages,
          rotateAngle,
          rotatePages,
          watermarkText,
          watermarkOrientation,
          watermarkOpacity,
          watermarkColor,
          watermarkFontSize: 12,

          password,
          batesPrefix,
          batesStart,
          batesPadding,
          ocrOutputFormat,
          ocrLanguage: "eng",

          cropBox,
          pageNumberPosition: "bottom_right",
          pageNumberStart: 1,

          pageNumberFormat: "1",

          convertPreset,
          reorderList,
          signCertPath,
          headerText,
          footerText,
          metadataTitle,
          compressQuality
        };

        const args = await buildOperationArgs(activeTool.id, context);
        if (!args) return; // User cancelled file picker

        try {

            let result: any;
            if (appState.isMobile) {
               // Extract params for mobile
               let mobileParams = { ...args };
               // Find original bytes if any (this is tricky since we only have docPath and args)
               // The mobile bridge expects: fileName, inputBytes, operation, params
               // We will pass empty buffer and rely on path/file access or handle in rust if we were to fully bridge.
               // However, as per spec: "route tool execution directly through process_mobile_buffer(file_name, input_bytes, operation, params)"

               let inputBytes: number[] = [];
               const currentDoc = appState.documents[docIndex];
               if (currentDoc && (currentDoc as any)._isLoaded) {
                   const arrayBuf = await currentDoc.arrayBuffer();
                   inputBytes = Array.from(new Uint8Array(arrayBuf));
               } else if (docPath) {
                   inputBytes = await invoke<number[]>('read_file_bytes', { path: docPath }).catch(() => []);
               }

               if ((activeTool.id === 'merge' || activeTool.id === 'pdf_merge') && appState.documents.length > 1) {
                   const docsPayload = [];
                   for (let idx = 0; idx < appState.documents.length; idx++) {
                       const d = appState.documents[idx];
                       const buf = await d.arrayBuffer();
                       const bytes = new Uint8Array(buf);
                       let binary = '';
                       for (let b = 0; b < bytes.length; b++) {
                           binary += String.fromCharCode(bytes[b]);
                       }
                       docsPayload.push({
                           name: d.name || `doc_${idx + 1}.pdf`,
                           dataBase64: btoa(binary)
                       });
                   }
                   mobileParams.documents = docsPayload;
               }

               result = await invoke('process_mobile_buffer', {
                   fileName: docName,
                   inputBytes: inputBytes,
                   operation: activeTool.id,
                   params: mobileParams
               });
            } else {
                result = await invoke('invoke_mcp_tool', { toolName, arguments: args });
            }

          const durationMs = Math.round(performance.now() - startTime);
          const durationFormatted = durationMs < 1000 ? `${durationMs}ms` : `${(durationMs / 1000).toFixed(1)}s`;

          const resObj = result as any;
          if (resObj && resObj.success) {
            let msg = '';
            if (activeTool.id === 'ocr' && ocrOutputFormat === 'clipboard') {
              const textToCopy = resObj.data?.text || '';
              if (navigator.clipboard && textToCopy) {
                await navigator.clipboard.writeText(textToCopy);
              }
              msg = `📋 OCR text copied to clipboard! (${textToCopy.length} characters)`;
            } else {
              const outDest = resObj.output_path || customOutputPath || args.output || args.output_dir;
              msg = outDest ? `✅ ${activeTool.title} saved to: ${outDest}` : (resObj.message || `${activeTool.title} completed successfully`);

              if (appState.isMobile && resObj.dataBase64) {
                try {
                  const byteCharacters = atob(resObj.dataBase64);
                  const byteArray = new Uint8Array(byteCharacters.length);
                  for (let i = 0; i < byteCharacters.length; i++) {
                    byteArray[i] = byteCharacters.charCodeAt(i);
                  }
                  const resultBlob = new Blob([byteArray], { type: 'application/pdf' });
                  const resultFile = new File([resultBlob], resObj.fileName || 'output.pdf', { type: 'application/pdf' });
                  (resultFile as any)._isLoaded = true;
                  appState.addDocuments([resultFile]);
                  appState.selectDocument(appState.documents.length - 1);
                } catch (loadErr) {
                  console.warn('Failed to auto-load output document on mobile:', loadErr);
                }
              }
            }
            toastState.success(msg);
            jobsState.addJob(toolName, 'success', msg);

            lastExecution = {
              success: true,
              durationFormatted,
              toolTitle: activeTool.title,
              timestamp: new Date(),
              message: msg
            };
            executionHistory = [lastExecution, ...executionHistory].slice(0, 10);
          } else {
            const formatted = formatUserFriendlyError(resObj?.message || 'Operation failed', activeTool.title);
            toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
            jobsState.addJob(toolName, 'error', formatted.headline);

            lastExecution = {
              success: false,
              durationFormatted,
              toolTitle: activeTool.title,
              timestamp: new Date(),
              message: formatted.headline
            };
            executionHistory = [lastExecution, ...executionHistory].slice(0, 10);
          }
        } catch (error) {
          const durationMs = Math.round(performance.now() - startTime);
          const durationFormatted = durationMs < 1000 ? `${durationMs}ms` : `${(durationMs / 1000).toFixed(1)}s`;
          const formatted = formatUserFriendlyError(error, activeTool.title);
          toastState.error(formatted.headline, formatted.actionHint, formatted.technicalDetails);
          jobsState.addJob(toolName, 'error', formatted.headline);

          lastExecution = {
            success: false,
            durationFormatted,
            toolTitle: activeTool.title,
            timestamp: new Date(),
            message: formatted.headline
          };
          executionHistory = [lastExecution, ...executionHistory].slice(0, 10);
        }
      }
    } finally {
      appState.setLoading(false);
      isRunning = false;
      clearInterval(timer);
    }
  }

  function moveUp(index: number) {
    if (index > 0) {
      appState.reorderDocuments(index, index - 1);
    }
  }

  function moveDown(index: number) {
    if (index < appState.documents.length - 1) {
      appState.reorderDocuments(index, index + 1);
    }
  }
</script>

<div
  class="operations-panel"
  class:collapsed={isCollapsed && !appState.isMobile}
  class:dragging={isDragging && !appState.isMobile}
  class:mobile-bottom-sheet={appState.isMobile}
  id="operations-panel"
  style={!appState.isMobile && !isCollapsed ? `width: ${panelWidth}px; min-width: ${panelWidth}px;` : ''}
>
  {#if !isCollapsed}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="resize-handle"
      id="operations-panel-resize-handle"
      title="Drag to resize panel, double-click to toggle width"
      onmousedown={startResize}
      ondblclick={handleResizeDblClick}
    ></div>
  {/if}

  <button
    class="collapse-toggle"
    id="operations-panel-toggle"
    onclick={() => isCollapsed = !isCollapsed}
    title={isCollapsed ? 'Expand tools panel' : 'Collapse tools panel'}
    aria-label={isCollapsed ? 'Expand tools panel' : 'Collapse tools panel'}
  >
    {isCollapsed ? '‹' : '›'}
  </button>

  {#if !isCollapsed || appState.isMobile}
  {#if activeTool}
    <!-- INSPECTOR MODE FOR ACTIVE TOOL -->
    <div class="inspector-header">
      <button class="back-btn" onclick={backToDirectory} id="btn-back-tools" title="Back to tool list">
        ‹ Back
      </button>
      <div class="inspector-title-wrap" id="inspector-param-{activeTool.id.replace('pdf_', '')}">
        <span class="tool-icon">{activeTool.icon}</span>
        <h3 class="inspector-title">{activeTool.title}</h3>
      </div>
      {#if appState.isMobile}
        <button class="back-btn close-btn" onclick={() => {
          appState.closeTool();
          if (appState.activeTab === 'tools') appState.setActiveTab('documents');
        }} title="Close">
          ✕
        </button>
      {/if}
    </div>

    <div class="inspector-content">
      <p class="tool-desc">{activeTool.description}</p>

      {#if activeTool.id === 'merge'}
        <div class="operation-config">
          <p class="section-desc">Reorder open documents for merging:</p>
          {#if appState.documents.length === 0}
            <div class="empty-list">No documents added.</div>
          {:else}
            <div class="reorder-list">
              {#each appState.documents as doc, i}
                <div class="reorder-item">
                  <span class="item-name" title={doc.name}>{doc.name}</span>
                  <div class="reorder-controls">
                    <button class="icon-btn" disabled={i === 0} onclick={() => moveUp(i)} title="Move Up">⬆️</button>
                    <button class="icon-btn" disabled={i === appState.documents.length - 1} onclick={() => moveDown(i)} title="Move Down">⬇️</button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {:else if activeTool.id === 'split'}
        <div class="operation-config">
          <label for="splitPoints" class="section-desc">Split Page Ranges (comma separated):</label>
          <input id="splitPoints" type="text" class="form-input" bind:value={splitPoints} placeholder="e.g. 1-3, 4-8" />
        </div>
      {:else if activeTool.id === 'delete_pages'}
        <div class="operation-config">
          <label for="deletePages" class="section-desc">Pages to Delete (comma separated or ranges):</label>
          <input id="deletePages" type="text" class="form-input" bind:value={deletePages} placeholder="e.g. 1, 3, 5-7" />
        </div>
      {:else if activeTool.id === 'extract_pages'}
        <div class="operation-config">
          <label for="extractPages" class="section-desc">Pages to Extract (comma separated or ranges):</label>
          <input id="extractPages" type="text" class="form-input" bind:value={extractPages} placeholder="e.g. 1-3, 5" />
        </div>
      {:else if activeTool.id === 'compress'}
        <div class="operation-config">
          <label for="compressQuality" class="section-desc">Image Quality: {compressQuality}%</label>
          <input id="compressQuality" type="range" min="10" max="100" bind:value={compressQuality} class="range-input" />
          <div class="range-labels">
            <span>Smaller file</span>
            <span>Higher quality</span>
          </div>
        </div>
      {:else if activeTool.id === 'rotate'}
        <div class="operation-config">
          <label for="rotateAngle" class="section-desc">Rotation Angle:</label>
          <select id="rotateAngle" bind:value={rotateAngle} class="form-input">
            <option value="90">90° Clockwise</option>
            <option value="180">180° Flip</option>
            <option value="270">270° Counter-Clockwise</option>
          </select>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="rotatePages" class="section-desc">Pages to Rotate (Optional):</label>
          <input
            id="rotatePages"
            type="text"
            class="form-input"
            bind:value={rotatePages}
            placeholder="e.g. 1, 3, 5-10 (leave empty for all)"
          />
          <div class="range-labels" style="margin-top: 4px;">
            <span style="font-size: 0.72rem; color: #94a3b8;">Empty = All pages</span>
          </div>
        </div>
      {:else if activeTool.id === 'watermark'}
        <div class="operation-config">
          <label for="watermarkText" class="section-desc">Watermark Text:</label>
          <input id="watermarkText" type="text" class="form-input" bind:value={watermarkText} placeholder="e.g. CONFIDENTIAL" />
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="watermarkOrientation" class="section-desc">Orientation:</label>
          <select id="watermarkOrientation" bind:value={watermarkOrientation} class="form-input">
            <option value="diagonal">Diagonal (45°)</option>
            <option value="horizontal">Horizontal (0°)</option>
          </select>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="watermarkOpacity" class="section-desc">Opacity: {watermarkOpacity}%</label>
          <input id="watermarkOpacity" type="range" min="10" max="100" bind:value={watermarkOpacity} class="range-input" />
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="watermarkColor" class="section-desc">Color:</label>
          <select id="watermarkColor" bind:value={watermarkColor} class="form-input">
            <option value="gray">Gray</option>
            <option value="red">Red Alert</option>
            <option value="blue">Blue</option>
          </select>
        </div>
      {:else if activeTool.id === 'remove_blank'}
        <div class="operation-config">
          <label for="blankSensitivity" class="section-desc">Sensitivity (0-100): {blankSensitivity}%</label>
          <input id="blankSensitivity" type="range" min="0" max="100" bind:value={blankSensitivity} class="range-input" />
        </div>
      {:else if activeTool.id === 'page_numbers'}
        <div class="operation-config">
          <label for="pageNumberPos" class="section-desc">Position:</label>
          <select id="pageNumberPos" bind:value={pageNumberPos} class="form-input">
            <option value="bottom-right">Bottom Right</option>
            <option value="bottom-center">Bottom Center</option>
            <option value="top-right">Top Right</option>
            <option value="top-center">Top Center</option>
          </select>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="pageNumberFormat" class="section-desc">Format:</label>
          <input id="pageNumberFormat" type="text" class="form-input" bind:value={pageNumberFormat} placeholder={'Page {n} of {total}'} />
        </div>
      {:else if activeTool.id === 'md_to_pdf' || activeTool.id === 'html_to_pdf'}
        <div class="operation-config">
          <label for="convertPreset" class="section-desc">Style Preset:</label>
          <select id="convertPreset" bind:value={convertPreset} class="form-input">
            <option value="academic">Academic</option>
            <option value="corporate">Corporate</option>
            <option value="minimalist">Minimalist</option>
          </select>
        </div>
      {:else if activeTool.id === 'json_to_pdf'}
        <div class="operation-config">
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Synthesize a publication-grade PDF from structured JSON text pages and inline base64 / file-path images. Click <strong>Run</strong> to select your JSON file.
          </p>
        </div>
      {:else if activeTool.id === 'encrypt' || activeTool.id === 'decrypt'}
        <div class="operation-config">
          <label for="passwordField" class="section-desc">Password:</label>
          <input id="passwordField" type="password" class="form-input" bind:value={password} placeholder="Enter document password" />
        </div>
      {:else if activeTool.id === 'bates'}
        <div class="operation-config">
          <label for="batesPrefix" class="section-desc">Prefix:</label>
          <input id="batesPrefix" type="text" class="form-input" bind:value={batesPrefix} placeholder="e.g. CONF-" />
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="batesStart" class="section-desc">Start Number:</label>
          <input id="batesStart" type="number" class="form-input" bind:value={batesStart} min="1" />
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="batesPadding" class="section-desc">Padding:</label>
          <input id="batesPadding" type="number" class="form-input" bind:value={batesPadding} min="1" />
        </div>
      {:else if activeTool.id === 'reorder_pages'}
        <div class="operation-config">
          <label for="reorderList" class="section-desc">Order Permutation (comma separated):</label>
          <input id="reorderList" type="text" class="form-input" bind:value={reorderList} placeholder="e.g. 2,1,3,4,5" />
        </div>
      {:else if activeTool.id === 'crop'}
        <div class="operation-config">
          <label for="cropBox" class="section-desc">Crop Box (x,y,w,h):</label>
          <input id="cropBox" type="text" class="form-input" bind:value={cropBox} placeholder="e.g. 10,10,200,200" />
        </div>
      {:else if activeTool.id === 'sign'}
        <div class="operation-config">
          <label for="signCertPath" class="section-desc">Certificate Path:</label>
          <input id="signCertPath" type="text" class="form-input" bind:value={signCertPath} placeholder="Path to .p12 cert" />
        </div>
      {:else if activeTool.id === 'header_footer'}
        <div class="operation-config">
          <label for="headerText" class="section-desc">Header Text:</label>
          <input id="headerText" type="text" class="form-input" bind:value={headerText} placeholder="e.g. Confidential" />
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="footerText" class="section-desc">Footer Text:</label>
          <input id="footerText" type="text" class="form-input" bind:value={footerText} placeholder="e.g. Page" />
        </div>
      {:else if activeTool.id === 'ocr'}
        <div class="operation-config">
          <label for="ocrOutputFormat" class="section-desc">Output Destination:</label>
          <select id="ocrOutputFormat" bind:value={ocrOutputFormat} class="form-input">
            <option value="pdf">Searchable PDF (.pdf)</option>
            <option value="txt">Plain Text File (.txt)</option>
            <option value="clipboard">Copy to Clipboard</option>
          </select>
          <p class="section-desc" style="font-size: 0.78rem; color: #a1a1aa; margin-top: 6px;">
            {#if ocrOutputFormat === 'pdf'}
              Injects a transparent text layer into the PDF so text can be selected and searched.
            {:else if ocrOutputFormat === 'txt'}
              Extracts the recognized text and writes it out as a structured .txt text file.
            {:else}
              Copies the recognized text straight to your operating system clipboard!
            {/if}
          </p>
        </div>
      {:else}
        <div class="operation-config">
          <p class="section-desc">Applies directly to the active document.</p>
        </div>
      {/if}

      {#if appState.documents.length > 1 && activeTool.rule?.isBatchable}
        <div class="batch-scope-banner">
          <div class="batch-scope-header">
            <span class="batch-scope-icon">⚡</span>
            <span class="batch-scope-title">Execution Mode</span>
          </div>
          <div class="batch-scope-options">
            <label class="scope-option">
              <input type="radio" name="batchScope" checked={applyToAllFiles} onchange={() => applyToAllFiles = true} />
              <span>Apply to all <strong>{appState.documents.length} files</strong> in queue</span>
            </label>
            <label class="scope-option">
              <input type="radio" name="batchScope" checked={!applyToAllFiles} onchange={() => applyToAllFiles = false} />
              <span>Current file only ({appState.documents[appState.selectedDocumentIndex ?? 0]?.name || 'active'})</span>
            </label>
          </div>
        </div>
      {/if}

      {#if activeTool && activeTool.id !== 'merge' && activeTool.id !== 'md_to_pdf' && activeTool.id !== 'html_to_pdf' && activeTool.id !== 'img_to_pdf' && activeTool.id !== 'json_to_pdf' && !(activeTool.id === 'ocr' && ocrOutputFormat === 'clipboard') && !(applyToAllFiles && activeTool.rule?.isBatchable && appState.documents.length > 1)}
        <div class="operation-config" id="output-path-config">
          <label for="output-path-input" class="section-desc">
            {activeTool.id === 'split' ? 'Output folder:' : 'Output file:'}
          </label>
          <div class="output-path-row">
            <input
              id="output-path-input"
              type="text"
              class="form-input output-path-input"
              bind:value={customOutputPath}
              placeholder={activeTool.id === 'split' ? 'Output folder path...' : 'Output file path...'}
              title="Edit the output path"
            />
          </div>
        </div>
      {/if}

      <div class="action-area">
        <button
          class="run-btn"
          class:run-btn--loading={isRunning}
          id="btn-run-operation"
          disabled={isRunning || !activeTool || (!['md_to_pdf', 'html_to_pdf', 'img_to_pdf', 'json_to_pdf'].includes(activeTool.id) && appState.documents.length === 0)}
          onclick={handleRunOperation}
        >
          {#if isRunning}
            <span class="spinner" aria-hidden="true"></span>
            Running...
          {:else if applyToAllFiles && activeTool.rule?.isBatchable && appState.documents.length > 1}
            Run Batch {activeTool.title} ({appState.documents.length} files)
          {:else}
            Run {activeTool.title}
          {/if}
        </button>
        {#if isRunning}
          <div class="elapsed-timer" aria-live="polite">⏱ {elapsedDisplay} elapsed</div>
        {:else if lastExecution}
          <div class="last-execution-badge" class:badge-success={lastExecution.success} class:badge-error={!lastExecution.success} title={lastExecution.message}>
            <span class="badge-icon">{lastExecution.success ? '✓' : '✗'}</span>
            <span class="badge-text">
              {lastExecution.success ? 'Completed' : 'Failed'} in <strong>{lastExecution.durationFormatted}</strong>
            </span>
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <!-- DIRECTORY MODE (STIRLING PDF STYLE) -->
    <div class="panel-header">
      {#if appState.isMobile}
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
          <h3 style="margin: 0; font-size: 15px; font-weight: 600;">⚡ All Tools</h3>
          <button class="back-btn close-btn" onclick={() => {
            if (appState.activeTab === 'tools') appState.setActiveTab('documents');
          }} title="Close">✕</button>
        </div>
      {/if}
      <!-- Context Filter Chip Bar -->
      <div class="context-bar" role="tablist" aria-label="Filter context">
        {#each [
          { id: 'recommended', label: `⚡ Recommended (${ranked.recommended.length})` },
          { id: 'all', label: `All Tools (${allTools.length})` },
          { id: 'pdf', label: 'PDF' },
          { id: 'office', label: 'Office' },
          { id: 'ai', label: 'AI & OCR' }
        ] as ctx (ctx.id)}
          <button
            class="context-btn"
            class:active={contextFilter === ctx.id}
            role="tab"
            aria-selected={contextFilter === ctx.id}
            onclick={() => { contextFilter = ctx.id; selectedCategory = 'all'; searchQuery = ''; }}
            id={`context-btn-${ctx.id}`}
          >{ctx.label}</button>
        {/each}
      </div>

      <div class="search-box">
        <span class="search-icon">🔍</span>
        <input
          id="tool-search-input"
          class="tool-search-input"
          type="text"
          placeholder="Search tools..."
          bind:value={searchQuery}
        />
        {#if searchQuery}
          <button class="clear-search-btn" onclick={() => searchQuery = ''}>×</button>
        {/if}
      </div>

      {#if contextFilter === 'all' || contextFilter === 'pdf'}
        <!-- Category Filter Pills -->
        <div class="category-pills">
          {#each allCategories as cat}
            <button
              class="pill-btn {selectedCategory === cat.id ? 'active' : ''}"
              onclick={() => selectedCategory = cat.id}
            >
              {cat.icon} {cat.title}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <div class="panel-content">
      <!-- Missing Pack Warning -->
      {#if ranked.packLocked.length > 0 && contextFilter === 'recommended'}
        {#each ranked.packLocked as lockedTool}
          <div class="pack-locked-card">
            <span class="tool-card-icon">{lockedTool.tool.icon}</span>
            <div class="tool-card-info">
              <div class="tool-card-title-row">
                <span class="tool-card-title" style="opacity: 0.6;">{lockedTool.tool.title}</span>
                {#if lockedTool.tool.inputFormat && lockedTool.tool.outputFormat}
                  <span class="format-pill">
                    <span class="fmt-in">{lockedTool.tool.inputFormat}</span>
                    <span class="fmt-arrow">➔</span>
                    <span class="fmt-out">{lockedTool.tool.outputFormat}</span>
                  </span>
                {/if}
              </div>
              <span class="tool-card-desc">📦 Requires {lockedTool.missingPack} Pack</span>
            </div>
            <button class="enable-pack-btn" onclick={(e) => {
               e.stopPropagation();
               appState.toggleFeaturePack(lockedTool.missingPack, true);
            }}>
               Enable Pack
            </button>
          </div>
        {/each}
      {/if}

      {#if searchQuery.trim() || selectedCategory !== 'all' || contextFilter === 'recommended'}
        <!-- Search Results View -->
        <div class="search-results-list">
          <span class="category-header">
             {#if searchQuery.trim()}
                Search Results ({filteredTools.length})
             {:else if contextFilter === 'recommended'}
                Recommended for Selection ({filteredTools.length})
             {:else}
                {allCategories.find(c => c.id === selectedCategory)?.title} ({filteredTools.length})
             {/if}
          </span>
          {#if filteredTools.length === 0}
            <div class="empty-state">No matching tools found</div>
          {:else}
            {#each filteredTools as tool (tool.id + tool.category)}
              <button class="tool-card" onclick={() => selectTool(tool)} id="tool-btn-{tool.id}">
                <span class="tool-card-icon">{tool.icon}</span>
                <div class="tool-card-info">
                  <span class="tool-card-title">
                    {tool.title}
                    {#if tool.releaseStatus === 'v1-core'}
                      <span class="status-badge-v1">v1.0 Core</span>
                    {:else}
                      <span class="status-badge-preview">Preview</span>
                    {/if}
                    {#if appState.documents.length >= 2}
                      {#if tool.rule?.minDocuments === 2}
                        <span class="smart-badge">Multi-File Action</span>
                      {:else if tool.rule?.isBatchable}
                        <span class="smart-badge">Batch Action ({appState.documents.length} files)</span>
                      {/if}
                    {/if}
                  </span>
                  <span class="tool-card-desc">{tool.description}</span>
                </div>
              </button>
            {/each}
          {/if}
        </div>
      {:else}
        <!-- Categorized Sections View -->
        {#each categories as cat}
          {@const toolsInCat = filteredTools.filter(t => t.category === cat.id)}
          {#if toolsInCat.length > 0}
            <div class="category-section">
              <span class="category-header">
                <span class="cat-icon">{cat.icon}</span>
                {cat.title}
              </span>
              <div class="category-grid">
                {#each toolsInCat as tool (tool.id + tool.category)}
                  <button class="tool-card" onclick={() => selectTool(tool)} id="tool-btn-{tool.id}">
                    <span class="tool-card-icon">{tool.icon}</span>
                    <div class="tool-card-info">
                      <div class="tool-card-title-row">
                        <div class="tool-title-with-badge">
                          <span class="tool-card-title">{tool.title}</span>
                          {#if tool.releaseStatus === 'v1-core'}
                            <span class="status-badge-v1">v1.0 Core</span>
                          {:else}
                            <span class="status-badge-preview">Preview</span>
                          {/if}
                        </div>
                        {#if tool.inputFormat && tool.outputFormat}
                          <span class="format-pill">
                            <span class="fmt-in">{tool.inputFormat}</span>
                            <span class="fmt-arrow">➔</span>
                            <span class="fmt-out">{tool.outputFormat}</span>
                          </span>
                        {/if}
                      </div>
                      <span class="tool-card-desc">{tool.description}</span>
                    </div>
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        {/each}
      {/if}
    </div>
  {/if}
  {/if}
</div>

<style>
  .operations-panel {
    width: 320px;
    min-width: 320px;
    background-color: var(--bg-secondary, #141416);
    border-left: 1px solid var(--border-color, #2a2a35);
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-shrink: 0;
    overflow: hidden;
    position: relative;
    transition: width 0.2s ease, min-width 0.2s ease;
  }

  .operations-panel.collapsed {
    width: 32px !important;
    min-width: 32px !important;
  }

  .operations-panel.dragging {
    transition: none; /* smooth real-time drag without transition lag */
    user-select: none;
  }

  .resize-handle {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 6px;
    cursor: col-resize;
    z-index: 20;
    background: transparent;
    transition: background 0.15s ease;
  }

  .resize-handle:hover,
  .operations-panel.dragging .resize-handle {
    background: var(--accent-primary, #5e6ad2);
  }

  .collapse-toggle {
    position: absolute;
    left: 6px;
    top: 50%;
    transform: translateY(-50%);
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 4px;
    color: var(--text-muted, #9ca3af);
    width: 20px;
    height: 44px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 13px;
    z-index: 10;
    transition: all var(--transition-fast, 0.15s ease);
    padding: 0;
  }

  .collapse-toggle:hover {
    color: var(--text-primary, #ffffff);
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    border-color: var(--accent-primary, #5e6ad2);
  }

  .panel-header {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #2a2a35);
    flex-shrink: 0;
  }

  .context-bar {
    display: flex;
    overflow-x: auto;
    gap: 6px;
    padding: 4px 0 0;
    margin-bottom: 12px;
    scrollbar-width: none; /* Firefox */
  }
  .context-bar::-webkit-scrollbar {
    display: none; /* Safari and Chrome */
  }

  .context-btn {
    flex-shrink: 0;
    padding: 6px 12px;
    border-radius: var(--border-radius-sm, 4px);
    font-size: 0.8rem;
    font-weight: 500;
    border: 1px solid var(--border-color, #2a2a35);
    background: transparent;
    color: var(--text-secondary, #9ca3af);
    cursor: pointer;
    transition: all var(--transition-fast, 0.15s ease);
    white-space: nowrap;
  }
  .context-btn.active {
    background: var(--accent-primary, #5e6ad2);
    color: #fff;
    border-color: var(--accent-primary, #5e6ad2);
  }
  .context-btn:hover:not(.active) {
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    color: var(--text-primary, #ffffff);
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
    background-color: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-sm, 4px);
    padding: 0 10px;
  }


  .category-pills {
    display: flex;
    overflow-x: auto;
    gap: 8px;
    padding: 12px 0 0 0;
    scrollbar-width: none; /* Firefox */
  }
  .category-pills::-webkit-scrollbar {
    display: none; /* Safari and Chrome */
  }

  .pill-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 12px;
    background-color: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 16px;
    color: var(--text-muted, #9ca3af);
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
    transition: all var(--transition-fast, 0.15s ease);
  }

  .pill-btn:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    color: var(--text-primary, #ffffff);
  }

  .pill-btn.active {
    background-color: var(--accent-primary, #5e6ad2);
    border-color: var(--accent-primary, #5e6ad2);
    color: #ffffff;
  }



  .search-icon {
    font-size: 13px;
    opacity: 0.6;
    margin-right: 6px;
  }

  .tool-search-input {
    width: 100%;
    height: 32px;
    background: transparent;
    border: none;
    color: var(--text-primary, #ffffff);
    font-size: 13px;
    outline: none;
  }

  .clear-search-btn {
    background: transparent;
    border: none;
    color: var(--text-muted, #9ca3af);
    font-size: 16px;
    cursor: pointer;
    padding: 0 4px;
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    padding: 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .category-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .category-header {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted, #9ca3af);
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 2px;
  }

  .cat-icon {
    font-size: 12px;
  }

  .category-grid, .search-results-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .tool-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--border-radius-sm, 4px);
    cursor: pointer;
    text-align: left;
    transition: all var(--transition-fast, 0.15s ease);
    width: 100%;
  }

  .tool-card:hover {
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    border-color: var(--border-color, #2a2a35);
  }

  .tool-card-icon {
    font-size: 18px;
    flex-shrink: 0;
  }

  .tool-card-info {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .tool-card-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary, #ffffff);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tool-card-desc {
    font-size: 11px;
    color: var(--text-muted, #9ca3af);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* INSPECTOR VIEW */
  .inspector-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #2a2a35);
    background-color: var(--bg-secondary, #141416);
    flex-shrink: 0;
  }

  .back-btn {
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-sm, 4px);
    color: var(--text-secondary, #9ca3af);
    font-size: 12px;
    padding: 4px 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .back-btn:hover {
    color: var(--text-primary, #ffffff);
    background-color: var(--bg-surface-hover, rgba(255, 255, 255, 0.1));
  }

  .inspector-title-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
    overflow: hidden;
  }

  .inspector-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary, #ffffff);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .inspector-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .tool-desc {
    font-size: 12px;
    color: var(--text-secondary, #9ca3af);
    line-height: 1.4;
  }

  .section-desc {
    font-size: 12px;
    color: var(--text-secondary, #9ca3af);
    margin-bottom: 6px;
    display: block;
  }

  .form-input {
    width: 100%;
    padding: 8px 12px;
    background-color: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: var(--border-radius-sm, 4px);
    color: var(--text-primary, #ffffff);
    font-size: 13px;
    outline: none;
  }

  select.form-input {
    appearance: none;
    -webkit-appearance: none;
    background-image: url("data:image/svg+xml;charset=UTF-8,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%239ca3af' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    background-size: 14px;
    padding-right: 30px;
  }

  .range-input {
    width: 100%;
    accent-color: var(--accent-primary, #5e6ad2);
  }

  .range-labels {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    color: var(--text-muted, #9ca3af);
    margin-top: 2px;
  }

  .reorder-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    max-height: 180px;
    overflow-y: auto;
  }

  .reorder-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 10px;
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 4px;
    font-size: 12px;
  }

  .item-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 180px;
  }

  .reorder-controls {
    display: flex;
    gap: 4px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    font-size: 10px;
    cursor: pointer;
    padding: 2px 4px;
  }

  .icon-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .action-area {
    margin-top: auto;
    padding-top: 16px;
  }

  .run-btn {
    width: 100%;
    padding: 10px 16px;
    background-color: var(--accent-primary, #5e6ad2);
    color: #ffffff;
    border: none;
    border-radius: var(--border-radius-sm, 4px);
    font-weight: 500;
    font-size: 13px;
    cursor: pointer;
    transition: background-color var(--transition-fast, 0.15s ease);
  }

  .run-btn:hover:not(:disabled) {
    background-color: var(--accent-hover, #6e79d6);
  }

  .run-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: #ffffff;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-right: 8px;
    vertical-align: middle;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
  .run-btn--loading { opacity: 0.72; cursor: not-allowed; }
  .elapsed-timer {
    font-size: 0.78rem;
    color: var(--text-muted);
    text-align: center;
    margin-top: 6px;
    font-variant-numeric: tabular-nums;
  }

  .last-execution-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    width: 100%;
    margin-top: 8px;
    padding: 6px 10px;
    border-radius: 6px;
    font-size: 11px;
    font-family: var(--font-mono, 'JetBrains Mono', monospace);
    transition: all 0.2s ease;
  }

  .badge-success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.25);
    color: #4ade80;
  }

  .badge-error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.25);
    color: #f87171;
  }

  .last-execution-badge .badge-icon {
    font-weight: 700;
  }

  .last-execution-badge strong {
    color: #ffffff;
  }

  .empty-state {
    padding: 20px 0;
    text-align: center;
    color: var(--text-muted, #9ca3af);
    font-size: 12px;
  }

  .output-path-row {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .output-path-input {
    flex: 1;
    font-size: 11px;
    font-family: var(--font-mono, 'JetBrains Mono', monospace);
    color: var(--text-muted, #9ca3af);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .output-path-input:focus {
    color: var(--text-primary, #ffffff);
  }

  .smart-badge {
    display: inline-block;
    margin-left: 6px;
    padding: 2px 6px;
    border-radius: 10px;
    font-size: 9px;
    font-weight: 600;
    text-transform: uppercase;
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.3);
    vertical-align: middle;
  }

  .pack-locked-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    background: rgba(239, 68, 68, 0.05);
    border: 1px solid rgba(239, 68, 68, 0.2);
    border-radius: var(--border-radius-sm, 4px);
    width: 100%;
  }

  .enable-pack-btn {
    margin-left: auto;
    padding: 4px 8px;
    font-size: 11px;
    background: var(--accent-primary, #5e6ad2);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .enable-pack-btn:hover {
    background: var(--accent-hover, #6e79d6);
  }

  /* Batch Scope Banner */
  .batch-scope-banner {
    background: rgba(94, 106, 210, 0.08);
    border: 1px solid rgba(94, 106, 210, 0.25);
    border-radius: 6px;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .batch-scope-header {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .batch-scope-icon {
    font-size: 13px;
  }

  .batch-scope-title {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--accent-primary, #818cf8);
  }

  .batch-scope-options {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .scope-option {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-secondary, #cbd5e1);
    cursor: pointer;
  }

  .scope-option input[type="radio"] {
    accent-color: var(--accent-primary, #5e6ad2);
    cursor: pointer;
  }

  /* Mobile Bottom Sheet Overrides */
  .operations-panel.mobile-bottom-sheet {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    width: 100% !important;
    min-width: 100% !important;
    height: auto;
    max-height: 85vh;
    border-radius: 20px 20px 0 0;
    border: none;
    border-top: 1px solid var(--border-color, #2a2a35);
    z-index: 1000;
    padding-bottom: env(safe-area-inset-bottom, 0px);
    box-shadow: 0 -4px 12px rgba(0,0,0,0.3);
  }

  .operations-panel.mobile-bottom-sheet .collapse-toggle,
  .operations-panel.mobile-bottom-sheet .resize-handle {
    display: none;
  }

  .operations-panel.mobile-bottom-sheet .form-input,
  .operations-panel.mobile-bottom-sheet .run-btn,
  .operations-panel.mobile-bottom-sheet select {
    min-height: 48px;
    font-size: 16px;
  }

  /* Drag handle indicator */
  .operations-panel.mobile-bottom-sheet::before {
    content: '';
    position: absolute;
    top: 8px;
    left: 50%;
    transform: translateX(-50%);
    width: 40px;
    height: 4px;
    background: var(--border-color, #2a2a35);
    border-radius: 2px;
  }

  .operations-panel.mobile-bottom-sheet .panel-header {
      padding-top: 20px;
  }
  .operations-panel.mobile-bottom-sheet .inspector-header {
      padding-top: 20px;
  }

  .tool-card-title-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
    align-items: flex-start;
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
  }
  .fmt-in {
    color: var(--text-secondary, #9ca3af);
  }
  .fmt-arrow {
    color: var(--accent-primary, #5e6ad2);
    font-size: 8px;
  }
  .tool-title-with-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }
  .status-badge-v1 {
    display: inline-block;
    padding: 1px 6px;
    border-radius: 9999px;
    font-size: 8.5px;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border: 1px solid rgba(34, 197, 94, 0.35);
    vertical-align: middle;
  }
  .status-badge-preview {
    display: inline-block;
    padding: 1px 5px;
    border-radius: 9999px;
    font-size: 8px;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
    background: rgba(147, 51, 234, 0.12);
    color: #c084fc;
    border: 1px solid rgba(147, 51, 234, 0.28);
    vertical-align: middle;
  }
</style>
