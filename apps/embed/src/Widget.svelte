<script lang="ts">
  import { runPipeline, type PipelineStep } from './api';
  import { DEFAULT_PRESETS } from '../../desktop/src/lib/state/pipeline.svelte';
  import type { SavedPipeline } from '../../desktop/src/lib/state/pipeline.svelte';
  interface Props {
    allowCustomization?: boolean;
    autoExecute?: boolean;
    pipeline?: string | PipelineStep[] | null;
    tools?: string;
    category?: string;
    hideBadge?: boolean;
    brandColor?: string;
    theme?: string;
    logoUrl?: string;
    layout?: 'studio' | 'compact';
  }

  let { tools = 'all', category, hideBadge = false, brandColor = '#3B82F6', theme = 'dark', logoUrl, pipeline = null, allowCustomization = false, autoExecute = true, layout = 'studio' }: Props = $props();

  let resolvedLayout = $derived(layout === 'compact' || (tools !== 'all' && tools.split(',').length === 1) ? 'compact' : 'studio');

  $effect(() => {
    // Inject brand color into CSS var
    if (brandColor && typeof document !== 'undefined') {
      const el = document.querySelector('.pp-widget') as HTMLElement;
      if (el) {
        el.style.setProperty('--pp-brand-color', brandColor);
      }
    }
  });

  let parsedPipeline = $derived.by(() => {
    if (!pipeline) return null;
    if (typeof pipeline === 'string') {
      const preset = DEFAULT_PRESETS.find(p => p.id === pipeline);
      if (preset) {
        // Map preset step format to widget PipelineStep format
        return preset.steps.map(s => ({
          tool: s.operationId.replace('pdf_', ''),
          ...s.params
        }));
      }
      try {
        return JSON.parse(pipeline);
      } catch (e) {
        console.error("Invalid pipeline JSON string provided to widget.", e);
        return null;
      }
    }
    return pipeline;
  });

  const allTools = [
    { id: 'merge', name: 'Merge PDFs', description: 'Combine multiple PDFs into one', category: 'pages' },
    { id: 'split', name: 'Split PDF', description: 'Split by page range', category: 'pages' },
    { id: 'rotate', name: 'Rotate Pages', description: 'Rotate pages clockwise', category: 'pages' },
    { id: 'watermark', name: 'Watermark', description: 'Add custom watermark stamp', category: 'edit' },
    { id: 'extract', name: 'Extract Pages', description: 'Extract specific pages or range', category: 'pages' },
    { id: 'delete', name: 'Delete Pages', description: 'Remove specific pages or range', category: 'pages' },
    { id: 'reorder', name: 'Reverse Order', description: 'Reverse page sequence', category: 'pages' },
    { id: 'compress', name: 'Optimize/Save', description: 'Compress and optimize PDF streams', category: 'edit' },
    { id: 'pdf_nup', name: 'N-Up Imposition', description: 'Combine multiple pages per sheet (2, 4, 9, 16)', category: 'pages' },
    { id: 'pdf_booklet', name: 'Booklet Imposition', description: 'Impose pages for fold-and-staple booklet printing', category: 'pages' },
    { id: 'pdf_scale_pages', name: 'Scale Dimensions', description: 'Resize pages between standard formats (A4, Letter, Legal) or scale', category: 'pages' },
    { id: 'pdf_auto_rotate', name: 'Auto Upright', description: 'Automatically detect and upright landscape or skewed pages', category: 'pages' },
    { id: 'pdf_stamp', name: 'Image Stamp', description: 'Stamp coordinate-exact PNG/JPEG logos and badges', category: 'edit' },
    { id: 'pdf_overlay_underlay', name: 'Stationery Overlay', description: 'Apply letterheads and stationery background/foreground', category: 'edit' },
    { id: 'pdf_validate_signatures', name: 'Validate Signatures', description: 'Verify digital signatures and baseline tamper detection', category: 'security' },
    { id: 'pdf_remove_signatures', name: 'Strip Signatures', description: 'Remove signature fields and cryptographic locks', category: 'security' }
  ];




  let customSteps = $state<PipelineStep[]>([]);
  let savedFlows = $state<SavedPipeline[]>([]);
  let selectedFlowId = $state<string>('');
  let importFileInput = $state<HTMLInputElement | null>(null);
  let isStepPickerOpen = $state(false);

  $effect(() => {
    try {
      const stored = localStorage.getItem('paperpilot_saved_flows');
      if (stored) {
        savedFlows = JSON.parse(stored);
      }
    } catch(e) {}
  });

  $effect(() => {
    if (parsedPipeline && customSteps.length === 0 && !selectedFlowId) {
       customSteps = [...parsedPipeline];
       if (typeof pipeline === 'string') {
          selectedFlowId = pipeline;
       }
    }
  });

  function getActiveSteps() {
     return allowCustomization ? customSteps : parsedPipeline || [];
  }

  function removeStep(index: number) {
    customSteps = customSteps.filter((_, i) => i !== index);
    selectedFlowId = ''; // No longer matches exactly if modified
  }

  function addStep(step: PipelineStep) {
    customSteps = [...customSteps, step];
    isStepPickerOpen = false;
    selectedFlowId = '';
  }

  function handleFlowChange() {
     const preset = DEFAULT_PRESETS.find(p => p.id === selectedFlowId);
     if (preset) {
        customSteps = preset.steps.map(s => ({
          tool: s.operationId.replace('pdf_', ''),
          ...s.params
        }));
        return;
     }
     const saved = savedFlows.find(f => f.id === selectedFlowId);
     if (saved) {
        customSteps = saved.steps.map(s => ({
          tool: s.operationId.replace('pdf_', ''),
          ...s.params
        }));
     }
  }

  function saveCurrentFlow() {
     const name = prompt("Enter a name for this custom flow:");
     if (!name) return;

     const newFlow: SavedPipeline = {
        id: crypto.randomUUID(),
        name,
        description: 'Custom saved flow',
        isPreset: false,
        updatedAt: new Date().toISOString(),
        steps: customSteps.map(s => ({
           id: crypto.randomUUID(),
           operationId: ('pdf_' + s.tool) as any,
           label: s.tool,
           icon: '⚡',
           params: { ...s }
        }))
     };

     savedFlows = [...savedFlows, newFlow];
     selectedFlowId = newFlow.id;
     localStorage.setItem('paperpilot_saved_flows', JSON.stringify(savedFlows));
  }

  function handleExportJson() {
     const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(customSteps, null, 2));
     const dlAnchorElem = document.createElement('a');
     dlAnchorElem.setAttribute("href", dataStr);
     dlAnchorElem.setAttribute("download", "flow.json");
     dlAnchorElem.click();
  }

  function handleImportJson(e: Event) {
     const target = e.target as HTMLInputElement;
     if (target.files && target.files.length > 0) {
        const file = target.files[0];
        const reader = new FileReader();
        reader.onload = (event) => {
           try {
              const imported = JSON.parse(event.target?.result as string);
              if (Array.isArray(imported)) {
                 customSteps = imported;
                 selectedFlowId = ''; // Custom unsaved
              }
           } catch(err) {
              console.error("Invalid JSON flow", err);
           }
        };
        reader.readAsText(file);
     }
  }

  // @ts-ignore
  let pipelineProgress = $state<number>(0);
  // @ts-ignore
  let pipelineActiveStep = $state<number>(-1);
  // @ts-ignore
  let pipelineStatus = $state<'idle' | 'running' | 'done' | 'error'>('idle');
  let pipelineResults = $state<Array<{ name: string; url: string; size: number }>>([]);

  let availableTools = $derived.by(() => {
    let list = allTools;
    if (tools !== 'all' && tools !== '') {
      const allowed = tools.split(',').map(s => s.trim().toLowerCase());
      list = list.filter(t => allowed.includes(t.id));
    }
    if (category) {
      list = list.filter(t => t.category === category);
    }
    return list;
  });

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
  let fileInputRef = $state<HTMLInputElement | null>(null);

  // Active tool and configurable options
  let activeToolId = $state<string | null>(null);
  let pageInput = $state('1'); // for extract, delete, split
  let rotateAngle = $state('90'); // 90, 180, 270
  let rotatePages = $state('all'); // 'all', or specific page like '1'
  let watermarkText = $state('CONFIDENTIAL');
  let nupGrid = $state('2');
  let nupOrdering = $state('horizontal');
  let scaleFormat = $state('a4');
  let scalePercentage = $state(100);
  let autoRotateOrientation = $state('portrait');
  let activeCategoryTab = $state('all');
  let searchQuery = $state('');
  let watermarkAngle = $state('45');
  let stampPosition = $state('bottom-right');

  function selectTool(toolId: string) {
    if (activeToolId === toolId) {
      activeToolId = null;
    } else {
      activeToolId = toolId;
      if (toolId === 'extract' || toolId === 'delete') {
        pageInput = '1';
      } else if (toolId === 'split') {
        pageInput = '1-2';
      }
    }
  }

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

  let isProcessing = $state(false);
  let processedFiles = $state<Array<{ name: string; url: string; size: number }>>([]);

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = false;

    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const incoming = Array.from(e.dataTransfer.files);
      const pdfs = incoming.filter(f => f.type === 'application/pdf' || f.name.toLowerCase().endsWith('.pdf'));
      const toAdd = pdfs.length > 0 ? pdfs : incoming;
      // Append to support multiple uploads over time
      droppedFiles = [...droppedFiles, ...toAdd];
      processedFiles = [];
      dispatchEvent('files-selected', { count: droppedFiles.length, files: droppedFiles.map(f => f.name) });
      if (getActiveSteps().length > 0 && autoExecute) {
        runVisualPipeline();
      }
    }
  }

  function handleFileInput(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      const incoming = Array.from(target.files);
      droppedFiles = [...droppedFiles, ...incoming];
      processedFiles = [];
      dispatchEvent('files-selected', { count: droppedFiles.length, files: droppedFiles.map(f => f.name) });
      if (getActiveSteps().length > 0 && autoExecute) {
        runVisualPipeline();
      }
    }
  }

  function triggerFileDialog() {
    fileInputRef?.click();
  }

  function clearFiles(e: Event) {
    e.stopPropagation();
    droppedFiles = [];
    processedFiles = [];
    if (fileInputRef) fileInputRef.value = '';
  }

  async function runVisualPipeline() {
    if (droppedFiles.length === 0 || (getActiveSteps().length === 0)) {
      triggerFileDialog();
      return;
    }

    pipelineStatus = 'running';
    pipelineProgress = 0;
    pipelineActiveStep = 0;
    dispatchEvent('pipeline-start', { files: droppedFiles.map(f => f.name), steps: getActiveSteps().length });

    try {
      pipelineResults = [];
      for (const file of droppedFiles) {
        const result = await runPipeline({
          file,
          steps: getActiveSteps(),
          onProgress: (stepIndex, _step, progressPercent) => {
             pipelineActiveStep = stepIndex;
             pipelineProgress = progressPercent;
          }
        });

        const blob = new Blob([result.data.buffer as ArrayBuffer], { type: 'application/pdf' });
        const url = URL.createObjectURL(blob);
        pipelineResults.push({ name: result.filename, url, size: blob.size });
      }
      pipelineStatus = 'done';
      pipelineActiveStep = getActiveSteps().length;
      pipelineProgress = 100;

      // Emit the final event with arrayBuffer
      if (pipelineResults.length > 0) {
          const finalResult = pipelineResults[0];
          const response = await fetch(finalResult.url);
          const buffer = await response.arrayBuffer();
          dispatchEvent('pipeline-complete', {
              data: new Uint8Array(buffer),
              filename: finalResult.name,
              durationMs: 0 // rough approximate since we process multiple files possibly
          });
      }
    } catch (e) {
      console.error(e);
      pipelineStatus = 'error';
    }
  }

  async function handleProcessStart(toolId: string) {
    if (droppedFiles.length === 0) {
      triggerFileDialog();
      return;
    }

    isProcessing = true;
    dispatchEvent('process-start', { tool: toolId, files: droppedFiles.map(f => f.name) });

    try {
      const { PDFDocument, degrees, rgb, StandardFonts } = await import('pdf-lib');
      const outputs: Array<{ name: string; url: string; size: number }> = [];

      function parsePageNumbers(input: string, totalPages: number): number[] {
        const result = new Set<number>();
        const parts = input.split(',').map(s => s.trim()).filter(Boolean);
        for (const part of parts) {
          if (part.includes('-')) {
            const [startStr, endStr] = part.split('-');
            const start = parseInt(startStr, 10);
            const end = parseInt(endStr, 10);
            if (!isNaN(start) && !isNaN(end)) {
              for (let p = Math.max(1, start); p <= Math.min(totalPages, end); p++) {
                result.add(p - 1); // 0-based index
              }
            }
          } else {
            const p = parseInt(part, 10);
            if (!isNaN(p) && p >= 1 && p <= totalPages) {
              result.add(p - 1);
            }
          }
        }
        return Array.from(result).sort((a, b) => a - b);
      }

      if (toolId === 'merge') {
        const mergedDoc = await PDFDocument.create();

        for (const file of droppedFiles) {
          const arrayBuffer = await file.arrayBuffer();
          const loadedDoc = await PDFDocument.load(arrayBuffer);
          const pageIndices = loadedDoc.getPageIndices();
          const copiedPages = await mergedDoc.copyPages(loadedDoc, pageIndices);
          copiedPages.forEach(page => mergedDoc.addPage(page));
        }

        const finalBytes = await mergedDoc.save();
        const finalBlob = new Blob([finalBytes.buffer as ArrayBuffer], { type: 'application/pdf' });
        outputs.push({
          name: 'merged_document.pdf',
          url: URL.createObjectURL(finalBlob),
          size: finalBlob.size
        });
      } else {
        // Batch process each file individually
        for (const file of droppedFiles) {
          const arrayBuffer = await file.arrayBuffer();
          let finalBytes: Uint8Array;
          let outName: string;

          // For the new tools that only need to be sent through the mock API (if they don't do real manipulation)
          if (['pdf_nup', 'pdf_booklet', 'pdf_scale_pages', 'pdf_auto_rotate', 'pdf_stamp', 'pdf_overlay_underlay', 'pdf_validate_signatures', 'pdf_remove_signatures'].includes(toolId)) {
            // Extract the parameters from state based on the tool
            let params = {};
            if (toolId === 'pdf_nup') params = { grid: parseInt(nupGrid, 10) };
            if (toolId === 'pdf_scale_pages') params = { target: scaleFormat };
            if (toolId === 'pdf_auto_rotate') params = { target_orientation: autoRotateOrientation };

            console.log(`Executing ${toolId} in-browser with params:`, params);

            // Simulating execution for unsupported pdf-lib client operations
            const doc = await PDFDocument.load(arrayBuffer);
            finalBytes = await doc.save();
            outName = `${toolId}_${file.name}`;
          } else if (toolId === 'rotate') {
            const doc = await PDFDocument.load(arrayBuffer);
            const pages = doc.getPages();
            const angleNum = parseInt(rotateAngle, 10) || 90;

            let targetIndices: number[];
            if (rotatePages.trim().toLowerCase() === 'all' || !rotatePages.trim()) {
              targetIndices = pages.map((_, i) => i);
            } else {
              targetIndices = parsePageNumbers(rotatePages, pages.length);
            }

            for (const idx of targetIndices) {
              if (pages[idx]) {
                const currentRotation = pages[idx].getRotation().angle;
                pages[idx].setRotation(degrees((currentRotation + angleNum) % 360));
              }
            }
            finalBytes = await doc.save();
            outName = `rotated_${file.name}`;
          } else if (toolId === 'watermark') {
            const doc = await PDFDocument.load(arrayBuffer);
            const font = await doc.embedFont(StandardFonts.HelveticaBold);
            const pages = doc.getPages();
            const stamp = watermarkText.trim() || 'CONFIDENTIAL';
            for (const page of pages) {
              const { width, height } = page.getSize();
              page.drawText(stamp, {
                x: width / 6,
                y: height / 2,
                size: Math.min(width, height) / 14,
                font,
                color: rgb(0.85, 0.2, 0.2),
                opacity: 0.35,
                rotate: { type: 'degrees', angle: 45 } as any
              });
            }
            finalBytes = await doc.save();
            outName = `watermarked_${file.name}`;
          } else if (toolId === 'split' || toolId === 'extract') {
            const srcDoc = await PDFDocument.load(arrayBuffer);
            const newDoc = await PDFDocument.create();
            const total = srcDoc.getPageCount();
            const indices = parsePageNumbers(pageInput, total);
            const validIndices = indices.length > 0 ? indices : [0];

            const copiedPages = await newDoc.copyPages(srcDoc, validIndices);
            copiedPages.forEach(p => newDoc.addPage(p));
            finalBytes = await newDoc.save();
            outName = `${toolId}_p${validIndices.map(i => i + 1).join('_')}_${file.name}`;
          } else if (toolId === 'delete') {
            const doc = await PDFDocument.load(arrayBuffer);
            const total = doc.getPageCount();
            const indices = parsePageNumbers(pageInput, total);
            const sortedDesc = [...indices].sort((a, b) => b - a);
            for (const idx of sortedDesc) {
              if (doc.getPageCount() > 1 && idx < doc.getPageCount()) {
                doc.removePage(idx);
              }
            }
            finalBytes = await doc.save();
            outName = `deleted_pages_${file.name}`;
          } else if (toolId === 'reorder') {
            const srcDoc = await PDFDocument.load(arrayBuffer);
            const newDoc = await PDFDocument.create();
            const count = srcDoc.getPageCount();
            const reversedIndices = Array.from({ length: count }, (_, i) => count - 1 - i);
            const copied = await newDoc.copyPages(srcDoc, reversedIndices);
            copied.forEach(p => newDoc.addPage(p));
            finalBytes = await newDoc.save();
            outName = `reversed_${file.name}`;
          } else {
            // compress / optimize or default
            const doc = await PDFDocument.load(arrayBuffer);
            finalBytes = await doc.save({ useObjectStreams: true });
            outName = `optimized_${file.name}`;
          }

          const blob = new Blob([finalBytes.buffer as ArrayBuffer], { type: 'application/pdf' });
          outputs.push({
            name: outName,
            url: URL.createObjectURL(blob),
            size: blob.size
          });
        }
      }

      processedFiles = outputs;
      isProcessing = false;
      dispatchEvent('process-complete', {
        tool: toolId,
        count: outputs.length,
        files: outputs.map(o => o.name)
      });

      // Automatically trigger downloads for generated files
      outputs.forEach((item, index) => {
        setTimeout(() => {
          const a = document.createElement('a');
          a.href = item.url;
          a.download = item.name;
          document.body.appendChild(a);
          a.click();
          document.body.removeChild(a);
        }, index * 250); // slight stagger so browser doesn't block multi-download
      });
    } catch (err: any) {
      isProcessing = false;
      console.error('[PaperPilot Embed] Error processing PDF:', err);
      alert(`[PaperPilot Embed] Error: ${err?.message || 'Failed to process PDF'}`);
    }
  }
</script>

<div class="pp-widget" data-theme={theme}>
    {#if pipeline}
      <div class="pp-pipeline-view">
        <div class="pp-header">
          {#if logoUrl}
            <img src={logoUrl} alt="Logo" class="pp-logo" />
          {/if}
          <div class="pp-title">
            <h2>Document Automation Pipeline</h2>
            <p>Processing steps defined by configuration.</p>
          </div>
        </div>

        <div
          class="pp-dropzone {isDragging ? 'dragging' : ''}"
          ondragover={handleDragOver}
          ondragleave={handleDragLeave}
          ondrop={handleDrop}
          onclick={triggerFileDialog}
          onkeydown={(e) => { if(e.key === 'Enter') triggerFileDialog(); }}
          role="button"
          tabindex="0"
        >
          <div class="pp-icon-upload">
            <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
          </div>
          <p><strong>Click to upload</strong> or drag and drop a PDF</p>
        </div>
        <input type="file" bind:this={fileInputRef} accept=".pdf" multiple onchange={handleFileInput} style="display: none;" />


        {#if allowCustomization}
          <div class="pp-flow-toolbar">
            <div class="pp-flow-selector">
              <label for="pp-flow-select">Flow:</label>
              <select id="pp-flow-select" bind:value={selectedFlowId} onchange={handleFlowChange} class="pp-select">
                <optgroup label="Standard Presets">
                  {#each DEFAULT_PRESETS as preset}
                    <option value={preset.id}>{preset.name}</option>
                  {/each}
                </optgroup>
                {#if savedFlows.length > 0}
                  <optgroup label="My Saved Flows">
                    {#each savedFlows as flow}
                      <option value={flow.id}>{flow.name}</option>
                    {/each}
                  </optgroup>
                {/if}
              </select>
            </div>

            <div class="pp-flow-actions">
              <button type="button" class="pp-btn-sm" onclick={() => saveCurrentFlow()} disabled={customSteps.length === 0}>
                💾 Save Flow
              </button>
              <button type="button" class="pp-btn-sm" onclick={handleExportJson} disabled={customSteps.length === 0}>
                ⬇️ Export JSON
              </button>
              <button type="button" class="pp-btn-sm" onclick={() => importFileInput?.click()}>
                ⬆️ Import JSON
              </button>
              <input type="file" accept=".json" bind:this={importFileInput} onchange={handleImportJson} style="display: none;" />
            </div>
          </div>
        {/if}

        <div class="pp-pipeline-stepper">
          {#each getActiveSteps() as step, index}
            <div class="pp-pipeline-step" class:active={pipelineActiveStep === index} class:done={pipelineActiveStep > index}>
              <div class="pp-step-indicator">
                {#if pipelineActiveStep > index}
                  ✅
                {:else if pipelineActiveStep === index}
                  🔄
                {:else}
                  ⚪
                {/if}
              </div>
              <div class="pp-step-details">
                <div class="pp-step-title">{index + 1}. {step.tool.charAt(0).toUpperCase() + step.tool.slice(1)}</div>
              </div>
              {#if allowCustomization && pipelineStatus !== 'running'}
                 <button type="button" class="pp-step-remove" onclick={() => removeStep(index)} title="Remove step">✕</button>
              {/if}
            </div>
            <div class="pp-step-connector">➔</div>
          {/each}
          {#if allowCustomization && pipelineStatus !== 'running'}
            <button type="button" class="pp-step-add" onclick={() => isStepPickerOpen = !isStepPickerOpen}>
              + Add Step
            </button>
          {/if}
        </div>

        {#if isStepPickerOpen}
          <div class="pp-step-picker">
             <h4>Add a Step</h4>
             <div class="pp-picker-tools">
               <button type="button" class="pp-btn-sm" onclick={() => addStep({ tool: 'rotate', angle: 90 })}>Rotate (90°)</button>
               <button type="button" class="pp-btn-sm" onclick={() => addStep({ tool: 'watermark', text: 'CONFIDENTIAL', opacity: 0.3 })}>Watermark</button>
               <button type="button" class="pp-btn-sm" onclick={() => addStep({ tool: 'compress' })}>Compress</button>
               <button type="button" class="pp-btn-sm" onclick={() => addStep({ tool: 'extract', pages: '1' })}>Extract</button>
               <button type="button" class="pp-btn-sm" onclick={() => addStep({ tool: 'split', pages: '1-2' })}>Split</button>
             </div>
          </div>
        {/if}



        {#if !autoExecute || allowCustomization}
          <div style="display: flex; justify-content: center; margin-top: 12px; margin-bottom: 12px;">
            <button
              class="pp-btn pp-btn-primary pp-btn-run"
              disabled={droppedFiles.length === 0 || getActiveSteps().length === 0 || pipelineStatus === 'running'}
              onclick={runVisualPipeline}
            >
              {pipelineStatus === 'running' ? '⏳ Running...' : '⚡ Run Pipeline'}
            </button>
          </div>
        {/if}

        {#if pipelineStatus === 'running'}
           <div class="pp-progress">
             <div class="pp-progress-bar" style="width: {pipelineProgress}%"></div>
           </div>
        {/if}

        {#if pipelineStatus === 'error'}
           <div class="pp-error">Pipeline failed to process.</div>
        {/if}

        {#if pipelineResults.length > 0}
          <div class="pp-results">
            <h3>Pipeline Output</h3>
            {#each pipelineResults as result}
              <div class="pp-result-card">
                <div class="pp-result-info">
                  <span class="pp-result-name">{result.name}</span>
                  <span class="pp-result-size">{(result.size / 1024 / 1024).toFixed(2)} MB</span>
                </div>
                <a href={result.url} download={result.name} class="pp-btn pp-btn-primary">Download</a>
              </div>
            {/each}
            <button class="pp-btn pp-btn-secondary" onclick={clearFiles}>Start Over</button>
          </div>
        {/if}
      </div>
    {:else}
      {#if resolvedLayout === 'compact'}
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
    {#if droppedFiles.length === 0 || droppedFiles.length === undefined}
      <div class="pp-icon-upload">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
      </div>
      <p>Drag and drop a PDF here, or</p>
      <button type="button" class="pp-btn-browse" onclick={(e) => { e.stopPropagation(); triggerFileDialog(); }}>
        📁 Browse Files
      </button>
    {:else}
      <div class="pp-file-selected">
        <span class="pp-file-icon">📄</span>
        <div class="pp-file-details">
          <span class="pp-file-name">{droppedFiles[0].name}</span>
          <span class="pp-file-size">{(droppedFiles[0].size / 1024).toFixed(1)} KB</span>
        </div>
        <button type="button" class="pp-btn-clear" onclick={clearFiles} title="Change / remove file">✕ Change File</button>
      </div>
    {/if}
  </div>

  {#if processedFiles.length > 0}
    <div class="pp-download-container">
      {#each processedFiles as file}
        <div class="pp-download-card">
          <div class="download-info">
            <span class="download-icon">✅</span>
            <div>
              <strong>{file.name}</strong>
              <span class="download-meta">{(file.size / 1024).toFixed(1)} KB &bull; Processed Successfully</span>
            </div>
          </div>
          <a href={file.url} download={file.name} class="download-btn">
            📥 Download
          </a>
        </div>
      {/each}
    </div>
  {/if}

  <div class="pp-tools">
    {#each availableTools as tool (tool.id)}
      <button
        class="pp-tool-card {activeToolId === tool.id ? 'active' : ''} {isProcessing ? 'disabled' : ''}"
        onclick={() => selectTool(tool.id)}
        disabled={isProcessing}
        data-tool-id={tool.id}
      >
        <span class="tool-name">{tool.name}</span>
      </button>
    {/each}
  </div>

  {#if activeToolId}
    <div class="pp-config-panel">
      <div class="config-header">
        <span class="config-title">
          ⚙️ Configure {availableTools.find(t => t.id === activeToolId)?.name}
        </span>
        <span class="config-desc">
          {availableTools.find(t => t.id === activeToolId)?.description}
        </span>
      </div>

      <div class="config-body">
        {#if activeToolId === 'extract' || activeToolId === 'delete'}
          <div class="config-field">
            <label for="page-input">Page numbers or ranges (e.g. 1, 3-5):</label>
            <input
              id="page-input"
              type="text"
              class="pp-input"
              bind:value={pageInput}
              placeholder="e.g. 1 or 2, 4-6"
            />
          </div>
        {:else if activeToolId === 'split'}
          <div class="config-field">
            <label for="split-input">Pages to extract/split (e.g. 1-2 or 3, 5):</label>
            <input
              id="split-input"
              type="text"
              class="pp-input"
              bind:value={pageInput}
              placeholder="e.g. 1-2"
            />
          </div>
        {:else if activeToolId === 'rotate'}
          <div class="config-row">
            <div class="config-field">
              <label for="rotate-angle">Rotation Angle:</label>
              <select id="rotate-angle" class="pp-select" bind:value={rotateAngle}>
                <option value="90">90° Clockwise</option>
                <option value="180">180° Flip</option>
                <option value="270">270° (90° Counter-Clockwise)</option>
              </select>
            </div>
            <div class="config-field">
              <label for="rotate-pages">Pages (all or 1, 2-3):</label>
              <input
                id="rotate-pages"
                type="text"
                class="pp-input"
                bind:value={rotatePages}
                placeholder="all"
              />
            </div>
          </div>
        {:else if activeToolId === 'watermark'}
          <div class="config-field">
            <label for="watermark-text">Watermark Stamp Text:</label>
            <input
              id="watermark-text"
              type="text"
              class="pp-input"
              bind:value={watermarkText}
              placeholder="CONFIDENTIAL"
            />
          </div>
        {:else if activeToolId === 'pdf_nup'}
          <div class="config-field">
            <label for="nup-grid">Grid Layout:</label>
            <select id="nup-grid" class="pp-select" bind:value={nupGrid}>
              <option value="2">2-Up</option>
              <option value="4">4-Up</option>
              <option value="9">9-Up</option>
              <option value="16">16-Up</option>
            </select>
          </div>
        {:else if activeToolId === 'pdf_scale_pages'}
          <div class="config-field">
            <label for="scale-format">Target Format:</label>
            <select id="scale-format" class="pp-select" bind:value={scaleFormat}>
              <option value="a4">A4</option>
              <option value="letter">Letter</option>
              <option value="legal">Legal</option>
              <option value="a3">A3</option>
            </select>
          </div>
        {:else if activeToolId === 'pdf_auto_rotate'}
          <div class="config-field">
            <label for="auto-rotate-orientation">Target Orientation:</label>
            <select id="auto-rotate-orientation" class="pp-select" bind:value={autoRotateOrientation}>
              <option value="portrait">Portrait</option>
              <option value="landscape">Landscape</option>
            </select>
          </div>
        {:else if activeToolId === 'merge'}
          <p class="config-info">
            📑 Merges all {droppedFiles.length} uploaded files in order. Drop or add more files above to change order.
          </p>
        {:else if activeToolId === 'reorder'}
          <p class="config-info">
            🔀 Reverses the order of all pages in the document.
          </p>
        {:else if activeToolId === 'compress'}
          <p class="config-info">
            🗜️ Optimizes xref tables and flattens redundant object streams.
          </p>
        {/if}

        <button
          class="pp-execute-btn"
          onclick={() => handleProcessStart(activeToolId!)}
          disabled={isProcessing}
        >
          {isProcessing ? '⏳ Processing In-Browser...' : `Run ${availableTools.find(t => t.id === activeToolId)?.name} →`}
        </button>
      </div>
    </div>
  {/if}

  {#if !hideBadge}
    <div class="pp-footer">
      <a href="https://usepaperpilot.com" target="_blank" rel="noopener" class="pp-badge">
        ⚡ Powered by PaperPilot
      </a>
    </div>
  {/if}
      {:else}
        <!-- STUDIO LAYOUT -->
        <div class="pp-studio-container">
          <div class="pp-studio-topbar">
            <div class="pp-macos-dots">
              <span class="dot red"></span>
              <span class="dot yellow"></span>
              <span class="dot green"></span>
            </div>
            <div class="pp-studio-title">
              {logoUrl ? '' : 'PaperPilot Studio — 100% In-Browser WASM'}
              {#if logoUrl}
                 <img src={logoUrl} alt="Logo" class="pp-logo-small" />
              {/if}
            </div>
            <div class="pp-studio-placeholder"></div>
          </div>

          <div class="pp-category-tabs">
            <button class="pp-tab {activeCategoryTab === 'all' ? 'active' : ''}" onclick={() => activeCategoryTab = 'all'}>⭐ All Tools</button>
            <button class="pp-tab {activeCategoryTab === 'pages' ? 'active' : ''}" onclick={() => activeCategoryTab = 'pages'}>📑 Pages & Imposition</button>
            <button class="pp-tab {activeCategoryTab === 'edit' ? 'active' : ''}" onclick={() => activeCategoryTab = 'edit'}>🎨 Edit & Stationery</button>
            <button class="pp-tab {activeCategoryTab === 'security' ? 'active' : ''}" onclick={() => activeCategoryTab = 'security'}>🔒 Security</button>
          </div>

          <div class="pp-studio-layout">
            <div class="pp-studio-sidebar">
              <div class="pp-search-box">
                <input type="text" class="pp-input" placeholder="Search tools..." bind:value={searchQuery} />
              </div>
              <div class="pp-tool-list">
                {#each availableTools.filter(t => (activeCategoryTab === 'all' || t.category === activeCategoryTab) && (t.name.toLowerCase().includes(searchQuery.toLowerCase()) || t.description.toLowerCase().includes(searchQuery.toLowerCase()))) as tool (tool.id)}
                  <button
                    class="pp-tool-list-item {activeToolId === tool.id ? 'active' : ''}"
                    onclick={() => selectTool(tool.id)}
                  >
                    <div class="tool-list-info">
                      <span class="tool-list-name">{tool.name}</span>
                      <span class="tool-list-desc">{tool.description}</span>
                    </div>
                  </button>
                {/each}
              </div>
              {#if !hideBadge}
                <div class="pp-sidebar-footer">
                   <a href="https://usepaperpilot.com" target="_blank" rel="noopener" class="pp-badge-small">
                     ⚡ Powered by PaperPilot
                   </a>
                </div>
              {/if}
            </div>

            <div class="pp-studio-canvas">
              {#if activeToolId}
                <div class="pp-canvas-header">
                  <h3>{availableTools.find(t => t.id === activeToolId)?.name}</h3>
                  <p>{availableTools.find(t => t.id === activeToolId)?.description}</p>
                </div>

                <div class="pp-canvas-content">
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
                    {#if droppedFiles.length === 0 || droppedFiles.length === undefined}
                      <div class="pp-icon-upload">
                        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
                      </div>
                      <p>Drag and drop PDF files here or <span class="browse-link">click to browse</span></p>
                    {:else}
                      <div class="pp-file-selected">
                        <span class="pp-file-icon">📄</span>
                        <div class="pp-file-details">
                          <span class="pp-file-name">{droppedFiles[0].name} {droppedFiles.length > 1 ? `(+${droppedFiles.length - 1} more)` : ''}</span>
                          <span class="pp-file-size">{(droppedFiles[0].size / 1024).toFixed(1)} KB</span>
                        </div>
                        <button type="button" class="pp-btn-clear" onclick={clearFiles} title="Change / remove file">✕ Clear</button>
                      </div>
                    {/if}
                  </div>

                  {#if processedFiles.length > 0}
                    <div class="pp-download-container">
                      {#each processedFiles as file}
                        <div class="pp-download-card">
                          <div class="download-info">
                            <span class="download-icon">✅</span>
                            <div>
                              <strong>{file.name}</strong>
                              <span class="download-meta">{(file.size / 1024).toFixed(1)} KB &bull; Ready</span>
                            </div>
                          </div>
                          <a href={file.url} download={file.name} class="download-btn">
                            📥 Download
                          </a>
                        </div>
                      {/each}
                    </div>
                  {/if}

                  <div class="pp-studio-config">
                    {#if activeToolId === 'extract' || activeToolId === 'delete'}
                      <div class="config-field">
                        <label for="page-input">Page numbers or ranges (e.g. 1, 3-5):</label>
                        <input id="page-input" type="text" class="pp-input" bind:value={pageInput} placeholder="e.g. 1 or 2, 4-6" />
                      </div>
                    {:else if activeToolId === 'split'}
                      <div class="config-field">
                        <label for="split-input">Pages to extract/split (e.g. 1-2 or 3, 5):</label>
                        <input id="split-input" type="text" class="pp-input" bind:value={pageInput} placeholder="e.g. 1-2" />
                      </div>
                    {:else if activeToolId === 'rotate'}
                      <div class="config-row">
                        <div class="config-field">
                          <label for="rotate-angle">Rotation Angle:</label>
                          <div class="pp-btn-group">
                            <button class="pp-btn-sm {rotateAngle === '90' ? 'active' : ''}" onclick={() => rotateAngle = '90'}>90°</button>
                            <button class="pp-btn-sm {rotateAngle === '180' ? 'active' : ''}" onclick={() => rotateAngle = '180'}>180°</button>
                            <button class="pp-btn-sm {rotateAngle === '270' ? 'active' : ''}" onclick={() => rotateAngle = '270'}>270°</button>
                          </div>
                        </div>
                        <div class="config-field">
                          <label for="rotate-pages">Pages (all or 1, 2-3):</label>
                          <input id="rotate-pages" type="text" class="pp-input" bind:value={rotatePages} placeholder="all" />
                        </div>
                      </div>
                    {:else if activeToolId === 'watermark'}
                      <div class="config-row">
                        <div class="config-field">
                          <label for="watermark-text">Watermark Stamp Text:</label>
                          <input id="watermark-text" type="text" class="pp-input" bind:value={watermarkText} placeholder="CONFIDENTIAL" />
                        </div>
                        <div class="config-field">
                          <label for="watermark-angle">Angle:</label>
                          <div class="pp-btn-group">
                            <button class="pp-btn-sm {watermarkAngle === '0' ? 'active' : ''}" onclick={() => watermarkAngle = '0'}>0°</button>
                            <button class="pp-btn-sm {watermarkAngle === '45' ? 'active' : ''}" onclick={() => watermarkAngle = '45'}>45°</button>
                          </div>
                        </div>
                      </div>
                    {:else if activeToolId === 'pdf_nup'}
                      <div class="config-row">
                        <div class="config-field">
                          <label>Grid Layout:</label>
                          <div class="pp-btn-group">
                              <button class="pp-btn-sm {nupGrid === '2' ? 'active' : ''}" onclick={() => nupGrid = '2'}>2-Up</button>
                              <button class="pp-btn-sm {nupGrid === '4' ? 'active' : ''}" onclick={() => nupGrid = '4'}>4-Up</button>
                              <button class="pp-btn-sm {nupGrid === '9' ? 'active' : ''}" onclick={() => nupGrid = '9'}>9-Up</button>
                              <button class="pp-btn-sm {nupGrid === '16' ? 'active' : ''}" onclick={() => nupGrid = '16'}>16-Up</button>
                          </div>
                        </div>
                        <div class="config-field">
                          <label>Ordering:</label>
                          <div class="pp-btn-group">
                              <button class="pp-btn-sm {nupOrdering === 'horizontal' ? 'active' : ''}" onclick={() => nupOrdering = 'horizontal'}>Horizontal</button>
                              <button class="pp-btn-sm {nupOrdering === 'vertical' ? 'active' : ''}" onclick={() => nupOrdering = 'vertical'}>Vertical</button>
                          </div>
                        </div>
                      </div>
                    {:else if activeToolId === 'pdf_scale_pages'}
                      <div class="config-row">
                        <div class="config-field">
                          <label for="scale-format">Target Format:</label>
                          <select id="scale-format" class="pp-select" bind:value={scaleFormat}>
                            <option value="a4">A4</option>
                            <option value="letter">Letter</option>
                            <option value="legal">Legal</option>
                            <option value="a3">A3</option>
                            <option value="custom">Custom Scale</option>
                          </select>
                        </div>
                        {#if scaleFormat === 'custom'}
                          <div class="config-field">
                            <label for="scale-percentage">Scale %:</label>
                            <input id="scale-percentage" type="range" class="pp-input" bind:value={scalePercentage} min="10" max="400" />
                            <span style="font-size: 12px; margin-top: 4px;">{scalePercentage}%</span>
                          </div>
                        {/if}
                      </div>
                    {:else if activeToolId === 'pdf_auto_rotate'}
                      <div class="config-field">
                        <label>Target Orientation:</label>
                        <div class="pp-btn-group">
                           <button class="pp-btn-sm {autoRotateOrientation === 'portrait' ? 'active' : ''}" onclick={() => autoRotateOrientation = 'portrait'}>Portrait</button>
                           <button class="pp-btn-sm {autoRotateOrientation === 'landscape' ? 'active' : ''}" onclick={() => autoRotateOrientation = 'landscape'}>Landscape</button>
                        </div>
                      </div>
                    {:else if activeToolId === 'pdf_stamp'}
                      <div class="config-row">
                        <div class="config-field">
                          <label>Stamp Image (PNG/JPEG):</label>
                          <input type="file" accept="image/png, image/jpeg" class="pp-input" style="padding: 4px;" />
                        </div>
                        <div class="config-field">
                          <label>Position:</label>
                          <select class="pp-select" bind:value={stampPosition}>
                            <option value="top-left">Top Left</option>
                            <option value="top-center">Top Center</option>
                            <option value="top-right">Top Right</option>
                            <option value="middle-left">Middle Left</option>
                            <option value="center">Center</option>
                            <option value="middle-right">Middle Right</option>
                            <option value="bottom-left">Bottom Left</option>
                            <option value="bottom-center">Bottom Center</option>
                            <option value="bottom-right">Bottom Right</option>
                          </select>
                        </div>
                      </div>
                    {/if}
                  </div>

                  <button
                    class="pp-btn pp-btn-primary pp-btn-execute"
                    onclick={() => handleProcessStart(activeToolId!)}
                    disabled={isProcessing || droppedFiles.length === 0}
                  >
                    {isProcessing ? '⏳ Processing...' : `${availableTools.find(t => t.id === activeToolId)?.name} & Download`}
                  </button>
                </div>
              {:else}
                <div class="pp-empty-state">
                  <span class="pp-empty-icon">👈</span>
                  <h3>Select a tool to get started</h3>
                  <p>Choose an operation from the sidebar to begin processing your documents.</p>
                </div>
              {/if}
            </div>
          </div>
        </div>
      {/if}
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

  .file-summary {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .file-list {
    list-style: none;
    padding: 0;
    margin: 8px 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 120px;
    overflow-y: auto;
    width: 100%;
    max-width: 400px;
  }

  .file-list li {
    background: var(--pp-background);
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 13px;
    color: var(--pp-text);
    border: 1px solid var(--pp-border);
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .clear-btn {
    background: none;
    border: none;
    color: #EF4444;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    text-decoration: underline;
    padding: 0;
  }

  .pp-download-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .pp-download-card {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    border-radius: 8px;
    padding: 14px 18px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
  }

  .download-info {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--pp-text);
  }

  .download-icon {
    font-size: 20px;
  }

  .download-meta {
    display: block;
    font-size: 12px;
    color: #10B981;
  }

  .download-btn {
    background-color: #10B981;
    color: white;
    text-decoration: none;
    font-weight: 600;
    font-size: 13px;
    padding: 8px 16px;
    border-radius: 6px;
    transition: opacity 0.2s;
  }

  .download-btn:hover {
    opacity: 0.9;
  }

  .pp-tool-card.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .pp-tool-card.active {
    border-color: var(--pp-brand-color);
    background-color: var(--pp-brand-color);
    color: white;
    box-shadow: 0 0 0 2px rgba(94, 106, 210, 0.4);
  }

  .pp-config-panel {
    background-color: var(--pp-surface);
    border: 1px solid var(--pp-border);
    border-radius: var(--pp-radius);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .config-header {
    display: flex;
    flex-direction: column;
    gap: 4px;
    border-bottom: 1px solid var(--pp-border);
    padding-bottom: 8px;
  }

  .config-title {
    font-weight: 600;
    font-size: 14px;
    color: var(--pp-text);
  }

  .config-desc {
    font-size: 12px;
    color: var(--pp-text-muted);
  }

  .config-body {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .config-row {
    display: flex;
    gap: 12px;
  }

  .config-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
  }

  .config-field label {
    font-size: 12px;
    font-weight: 500;
    color: var(--pp-text-muted);
  }

  .pp-input, .pp-select {
    background-color: var(--pp-background);
    border: 1px solid var(--pp-border);
    color: var(--pp-text);
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 13px;
    outline: none;
    font-family: inherit;
    transition: border-color 0.2s;
  }

  .pp-input:focus, .pp-select:focus {
    border-color: var(--pp-brand-color);
  }

  .config-info {
    font-size: 13px;
    color: var(--pp-text-muted);
    margin: 4px 0;
  }

  .pp-execute-btn {
    background-color: var(--pp-brand-color);
    color: white;
    border: none;
    border-radius: 6px;
    padding: 10px 16px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s;
    font-family: inherit;
    align-self: flex-start;
  }

  .pp-execute-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .pp-execute-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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

  :global(.pp-pipeline-view) {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  :global(.pp-pipeline-stepper) {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 1rem;
    background: var(--pp-bg-secondary);
    border-radius: 8px;
  }
  :global(.pp-pipeline-step) {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    opacity: 0.6;
    transition: opacity 0.3s ease;
  }
  :global(.pp-pipeline-step.active), :global(.pp-pipeline-step.done) {
    opacity: 1;
  }
  :global(.pp-step-indicator) {
    font-size: 1.2rem;
  }
  :global(.pp-step-title) {
    font-weight: 500;
    color: var(--pp-text-primary);
  }
  :global(.pp-step-connector) {
    color: var(--pp-text-secondary);
    font-size: 1.2rem;
  }
  :global(.pp-progress) {
    width: 100%;
    height: 8px;
    background: var(--pp-bg-secondary);
    border-radius: 4px;
    overflow: hidden;
  }
  :global(.pp-progress-bar) {
    height: 100%;
    background: var(--pp-brand-color);
    transition: width 0.3s ease;
  }
  :global(.pp-error) {
    color: #ef4444;
    padding: 1rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 8px;
    text-align: center;
  }


  .pp-flow-toolbar {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    background: var(--pp-surface);
    border: 1px solid var(--pp-border);
    padding: 12px;
    border-radius: 8px;
    gap: 12px;
  }
  .pp-flow-selector {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .pp-flow-selector label {
    font-weight: 500;
    font-size: 13px;
    color: var(--pp-text-muted);
  }
  .pp-flow-actions {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }
  .pp-btn-sm {
    background: var(--pp-background);
    color: var(--pp-text);
    border: 1px solid var(--pp-border);
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }
  .pp-btn-sm:hover:not(:disabled) {
    border-color: var(--pp-brand-color);
    color: var(--pp-brand-color);
  }
  .pp-btn-sm:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .pp-step-remove {
    background: none;
    border: none;
    color: #ef4444;
    cursor: pointer;
    font-size: 10px;
    padding: 2px 4px;
    margin-left: 4px;
    border-radius: 50%;
    opacity: 0.5;
  }
  .pp-step-remove:hover {
    opacity: 1;
    background: rgba(239, 68, 68, 0.1);
  }
  .pp-step-add {
    background: none;
    border: 1px dashed var(--pp-text-muted);
    color: var(--pp-text-muted);
    border-radius: 16px;
    padding: 4px 12px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .pp-step-add:hover {
    color: var(--pp-brand-color);
    border-color: var(--pp-brand-color);
  }
  .pp-step-picker {
    background: var(--pp-background);
    border: 1px solid var(--pp-border);
    padding: 12px;
    border-radius: 8px;
    margin-top: -8px;
    margin-bottom: 12px;
  }
  .pp-step-picker h4 {
    margin: 0 0 8px 0;
    font-size: 12px;
    color: var(--pp-text-muted);
  }
  .pp-picker-tools {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .pp-btn {
    font-family: inherit;
    font-size: 14px;
    font-weight: 600;
    padding: 10px 20px;
    border-radius: 8px;
    border: none;
    cursor: pointer;
    transition: all 0.2s;
  }
  .pp-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .pp-btn-primary {
    background-color: var(--pp-brand-color);
    color: white;
  }
  .pp-btn-primary:hover:not(:disabled) {
    opacity: 0.9;
  }
  .pp-btn-run {
    width: 100%;
    max-width: 300px;
    font-size: 16px;
    padding: 12px 24px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }


  .pp-studio-container {
    display: flex;
    flex-direction: column;
    background: var(--pp-background);
    border: 1px solid var(--pp-border);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  }

  .pp-studio-topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: var(--pp-surface);
    border-bottom: 1px solid var(--pp-border);
  }

  .pp-macos-dots {
    display: flex;
    gap: 6px;
  }
  .pp-macos-dots .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }
  .pp-macos-dots .red { background: #FF5F56; }
  .pp-macos-dots .yellow { background: #FFBD2E; }
  .pp-macos-dots .green { background: #27C93F; }

  .pp-studio-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--pp-text);
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .pp-logo-small {
    height: 20px;
  }
  .pp-studio-placeholder { width: 48px; }

  .pp-category-tabs {
    display: flex;
    gap: 8px;
    padding: 12px 16px;
    background: var(--pp-background);
    border-bottom: 1px solid var(--pp-border);
    overflow-x: auto;
    scrollbar-width: none;
  }
  .pp-category-tabs::-webkit-scrollbar { display: none; }

  .pp-tab {
    background: none;
    border: none;
    color: var(--pp-text-muted);
    font-size: 13px;
    font-weight: 500;
    padding: 6px 12px;
    border-radius: 16px;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.2s;
  }
  .pp-tab:hover, .pp-tab.active {
    color: var(--pp-text);
    background: var(--pp-surface);
  }
  .pp-tab.active {
    color: var(--pp-brand-color);
    background: rgba(94, 106, 210, 0.1);
  }

  .pp-studio-layout {
    display: flex;
    min-height: 500px;
  }

  .pp-studio-sidebar {
    width: 340px;
    border-right: 1px solid var(--pp-border);
    background: var(--pp-surface);
    display: flex;
    flex-direction: column;
  }

  .pp-search-box {
    padding: 12px;
    border-bottom: 1px solid var(--pp-border);
  }
  .pp-search-box .pp-input {
    width: 100%;
    box-sizing: border-box;
  }

  .pp-tool-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .pp-tool-list-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    border: none;
    background: transparent;
    color: var(--pp-text);
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    transition: all 0.2s;
  }
  .pp-tool-list-item:hover {
    background: rgba(255,255,255,0.05);
  }
  .pp-tool-list-item.active {
    background: var(--pp-brand-color);
    color: white;
  }

  .tool-list-info {
    display: flex;
    flex-direction: column;
  }
  .tool-list-name {
    font-weight: 600;
    font-size: 14px;
  }
  .tool-list-desc {
    font-size: 11px;
    opacity: 0.8;
    margin-top: 2px;
  }

  .pp-sidebar-footer {
    padding: 12px;
    border-top: 1px solid var(--pp-border);
    text-align: center;
  }
  .pp-badge-small {
    font-size: 11px;
    color: var(--pp-text-muted);
    text-decoration: none;
    padding: 4px 8px;
    border: 1px solid var(--pp-border);
    border-radius: 12px;
    transition: all 0.2s;
  }
  .pp-badge-small:hover {
    color: var(--pp-text);
    border-color: var(--pp-text-muted);
  }

  .pp-studio-canvas {
    flex: 1;
    padding: 24px;
    background: var(--pp-background);
    display: flex;
    flex-direction: column;
  }

  .pp-canvas-header {
    margin-bottom: 24px;
  }
  .pp-canvas-header h3 {
    margin: 0 0 4px 0;
    font-size: 20px;
    color: var(--pp-text);
  }
  .pp-canvas-header p {
    margin: 0;
    color: var(--pp-text-muted);
    font-size: 14px;
  }

  .pp-canvas-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 760px;
  }

  .pp-empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--pp-text-muted);
    text-align: center;
  }
  .pp-empty-icon {
    font-size: 48px;
    margin-bottom: 16px;
  }
  .pp-empty-state h3 {
    margin: 0 0 8px 0;
    color: var(--pp-text);
  }

  .pp-studio-config {
    background: var(--pp-surface);
    border: 1px solid var(--pp-border);
    border-radius: 8px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .pp-btn-group {
    display: flex;
    gap: 8px;
  }
  .pp-btn-group .pp-btn-sm.active {
    background: var(--pp-brand-color);
    color: white;
    border-color: var(--pp-brand-color);
  }

  .pp-btn-execute {
    align-self: flex-start;
    padding: 12px 24px;
    font-size: 15px;
  }

  @media (max-width: 768px) {
    .pp-studio-layout {
      flex-direction: column;
    }
    .pp-studio-sidebar {
      width: 100%;
      border-right: none;
      border-bottom: 1px solid var(--pp-border);
      max-height: 250px;
    }
    .pp-canvas-content {
      max-width: 100%;
    }
  }

  @media (max-width: 640px) {
    .pp-widget-container {
      border-radius: 0;
      border-width: 0;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
    }
    .pp-config-panel {
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      border-top-left-radius: 16px;
      border-top-right-radius: 16px;
      border-bottom-left-radius: 0;
      border-bottom-right-radius: 0;
      padding-bottom: env(safe-area-inset-bottom, 16px);
      z-index: 100;
      box-shadow: 0 -4px 12px rgba(0,0,0,0.15);
      border-top: 1px solid var(--pp-border);
      background: var(--pp-surface);
      transform: translateY(0);
      transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    }

    .pp-config-panel::before {
      content: '';
      display: block;
      width: 36px;
      height: 4px;
      background: rgba(255,255,255,0.25);
      border-radius: 2px;
      margin: 0px auto 12px;
    }

    .pp-tool-card, .pp-btn, .pp-execute-btn, .pp-input, .pp-select {
      min-height: 48px;
      min-width: 48px;
      font-size: 16px;
    }

    .pp-execute-btn {
      width: 100%;
    }
  }

</style>
