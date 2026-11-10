<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { jobsState } from '$lib/state/jobs.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { formatUserFriendlyError } from '$lib/utils/errorFormatter';

  import type { ToolDefinition } from "$lib/types/tools";
  import { allTools, categories, allCategories } from "$lib/types/toolList";

  import { resolveToolName, rankToolsByContext } from '$lib/services/toolRegistry';
import { validatePreconditions } from '$lib/services/operationValidator';
import { buildOperationArgs, type OperationContext } from '$lib/services/operationArgsBuilder';
import { onMount } from 'svelte';
import type { SnippetTarget } from '$lib/utils/snippetGenerator';
import AuditReportViewer from '../ui/AuditReportViewer.svelte';

  let { embedded = false } = $props<{ embedded?: boolean }>();

  let searchQuery = $state('');
  let selectedCategory = $state('all');
  let activeTool = $derived(appState.activeTool);
  let isCollapsed = $state(false);
  let panelWidth = $state(320);
  let isDragging = $state(false);

  onMount(() => {
    if (!embedded) {
      const stored = localStorage.getItem('operations-panel-collapsed');
      if (stored === 'true') isCollapsed = true;
    }

    const storedWidth = localStorage.getItem('operations-panel-width');
    if (storedWidth) {
      const parsed = parseInt(storedWidth, 10);
      if (!isNaN(parsed) && parsed >= 260 && parsed <= 700) {
        panelWidth = parsed;
      }
    }
  });

  $effect(() => {
    if (!embedded) {
      localStorage.setItem('operations-panel-collapsed', String(isCollapsed));
    }
  });

  $effect(() => {
    localStorage.setItem('operations-panel-width', String(panelWidth));
  });

    // Mobile OCR downloader state
  let isOcrDownloaded = $state(true);
  let ocrDownloadProgress = $state(0);
  let isOcrDownloading = $state(false);
  let ocrDownloadUnlisten = $state<any>(null);

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
  let nupPages = $state(2);
  let nupOrdering = $state('horizontal');
  let nupMargin = $state(18);
  let nupSpacing = $state(10);
  let nupDrawBorders = $state(false);
  let blankSensitivity = $state(99);
  let textSearchQuery = $state('');
  let searchResultDisplay = $state<string | null>(null);
  let workspaceSearchQuery = $state('');
  let workspaceSearchDir = $state('');
  let semanticSearchQuery = $state('');
  let semanticSearchLimit = $state(5);
  let classifyDetail = $state(false);
  let pageNumberPos = $state('bottom-right');
  let pageNumberFormat = $state('Page 1 of N');
  let convertPreset = $state('academic');
  let scaleTargetSize = $state('A4');
  let scaleFactor = $state(1.0);
  let autoRotateOrientation = $state('portrait');

  // PDF Attachments parameter states
  let attachmentAction = $state('list');
  let attachmentFileName = $state('');
  let attachmentDescription = $state('');
  let attachmentBase64 = $state('');

  // Tools 16-20 parameter states
  let bookletBindingEdge = $state('left');
  let bookletSheetOrdering = $state('standard');
  let posterGrid = $state('2x2');
  let posterScale = $state(1.0);
  let posterTargetSheet = $state('a4');
  let posterOverlap = $state(14.17);
  let posterGuides = $state(true);
  let stitchDirection = $state('vertical');
  let stitchSpacing = $state(0.0);
  let stitchMargin = $state(0.0);
  let attachmentFilePath = $state('');

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

  // Pattern & Keyword Extraction parameter states
  let extractPatternsKeywords = $state('');
  let extractPatternsRegex = $state('');
  let extractPatternsCaseSensitive = $state(false);
  let extractPatternsContextLen = $state(30);

  // Security, Forms & Metadata parameter states (Tools 31-35)
  let metadataAuthor = $state('');
  let metadataSubject = $state('');
  let metadataKeywords = $state('');
  let sanitizeScripts = $state(true);
  let sanitizeMetadata = $state(true);
  let sanitizeAttachments = $state(true);
  let tsaServerUrl = $state('http://timestamp.digicert.com');
  let allowPrinting = $state(false);
  let allowCopying = $state(false);
  let allowModifying = $state(false);

  // Digital Signatures, Hash & Conformance parameter states (Tools 36-40)
  let signPassword = $state('test');
  let signFieldName = $state('Signature1');
  let signReason = $state('Approved and Verified');
  let signLocation = $state('Local Secure Terminal');
  let hashAlgorithm = $state('sha256');

  // Stationery, Stamp, PII, Forms & Redaction parameter states (Tools 41-45)
  let stationeryPath = $state('tests/e2e_fixtures/real/stationery.pdf');
  let overlayMode = $state('underlay');
  let overlayPages = $state('1');
  let overlayOpacity = $state(1.0);
  let stampImagePath = $state('tests/e2e_fixtures/real/img1.png');
  let stampX = $state(100.0);
  let stampY = $state(100.0);
  let stampWidth = $state(150.0);
  let stampHeight = $state(50.0);
  let stampOpacity = $state(1.0);
  let stampPages = $state('all');
  let piiScanSsn = $state(true);
  let piiScanCreditCards = $state(true);
  let piiScanEmails = $state(true);
  let piiScanPhones = $state(true);
  let piiScanIban = $state(true);
  let redactQuery = $state('');
  let redactRect = $state('0,0,100,100');

  // Stamping & Annotation Parameter States (Tools 51-55)
  let annotationType = $state('highlight');
  let annotationContent = $state('Review Notice');
  let annotationColor = $state('#FFFF00');
  let annotationRect = $state('50,50,150,20');
  let annotationPage = $state(1);

  // Form Field, Fill & Batch Parameter States (Tools 46-50)
  let formDataPath = $state('tests/e2e_fixtures/real/form_data.json');
  let formFieldName = $state('ApplicantName');
  let formFieldType = $state('text');
  let formFieldPage = $state(1);
  let formFieldRect = $state('50,50,200,80');
  let batchCsvPath = $state('tests/e2e_fixtures/real/batch_data.csv');

  // Conversions & Packaging parameter states (Tools 61-65)
  let conversionDpi = $state(150);
  let imageFormat = $state('png');
  let tableFormat = $state('json');
  let cbzArchiveName = $state('');
  let pdfaConformance = $state('PDF/A-1b');
  let excelSheet = $state('');
  let hasHeaders = $state(true);

  // Alternating Merge parameter states
  let mergeAlternateDocA = $state('');
  let mergeAlternateDocB = $state('');
  let mergeAlternateReverseB = $state(true);
  let mergeAlternateStep = $state(1);
  let mergeAlternateStepB = $state(1);

  // Initialize or synchronize mergeAlternate inputs when documents change
  $effect(() => {
    if (appState.documentPaths.length > 0 && !mergeAlternateDocA) {
      mergeAlternateDocA = appState.documentPaths[0];
    }
    if (appState.documentPaths.length > 1 && !mergeAlternateDocB) {
      mergeAlternateDocB = appState.documentPaths[1];
    }
  });

  let toolContext = $state<'pdf' | 'office' | 'ai'>('pdf');

    async function checkOcrStatus() {
    try {
      const models = await invoke<{id: string, is_downloaded: boolean}[]>('get_available_ocr_models');
      if (Array.isArray(models)) {
        const tinyModel = models.find(m => m.id === 'ppocrv6-tiny');
        if (tinyModel) {
          isOcrDownloaded = tinyModel.is_downloaded;
        }
      }
    } catch (e) {
      console.warn('Failed to check OCR status:', e);
    }
  }

  $effect(() => {
    if (activeTool?.id === 'ocr' || appState.isMobile) {
      checkOcrStatus();
    }
  });

  async function startOcrDownload() {
    isOcrDownloading = true;
    ocrDownloadProgress = 0;
    try {
      ocrDownloadUnlisten = await listen('ocr-download-progress', (event: any) => {
        if (event.payload?.model_id === 'ppocrv6-tiny') {
          ocrDownloadProgress = event.payload.percent;
          if (event.payload.percent >= 100) {
             isOcrDownloaded = true;
             isOcrDownloading = false;
             if (ocrDownloadUnlisten) ocrDownloadUnlisten();
          }
        }
      });
      await invoke('download_ocr_model', { modelId: 'ppocrv6-tiny' });
    } catch (e) {
      console.error('Failed to download OCR:', e);
      isOcrDownloading = false;
    }
  }

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
    Array.isArray(appState.featurePacks)
      ? appState.featurePacks.filter((p: any) => p && p.is_enabled).map((p: any) => p.id)
      : []
  );

  let ranked = $derived(
    rankToolsByContext(allTools, appState.documents, activePackIds)
  );

  // Unique list of tools for UI presentation (prevents duplicate alias cards in All/Ready)
  let uniqueAllTools = $derived.by(() => {
    const seen = new Set<string>();
    const res: ToolDefinition[] = [];
    for (const tool of allTools) {
      const canonicalId = resolveToolName(tool.id);
      if (!seen.has(canonicalId)) {
        seen.add(canonicalId);
        res.push(tool);
      }
    }
    return res;
  });

  let readyToolsCount = $derived(
    uniqueAllTools.filter(t => t.releaseStatus !== 'preview').length
  );

  let contextFilter = $state('recommended'); // 'recommended', 'ready', 'all', 'pdf', 'office', 'ai'

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
      const seen = new Set<string>();
      for (const t of allTools) {
        if (
          t.title.toLowerCase().includes(q) ||
          t.description.toLowerCase().includes(q) ||
          t.tags.some(tag => tag.toLowerCase().includes(q))
        ) {
          const canonicalId = resolveToolName(t.id);
          if (!seen.has(canonicalId)) {
            seen.add(canonicalId);
            tools.push(t);
          }
        }
      }
    } else {
       if (contextFilter === 'recommended') {
         tools = ranked.recommended;
       } else if (contextFilter === 'ready') {
         // Shows all production-ready tools (v0.5-core, and future wave-core releases)
         tools = uniqueAllTools.filter(t => t.releaseStatus !== 'preview');
       } else if (contextFilter === 'all') {
         tools = uniqueAllTools;
       } else if (contextFilter === 'pdf') {
         tools = uniqueAllTools.filter(t => t.context === 'pdf');
       } else if (contextFilter === 'office') {
         tools = uniqueAllTools.filter(t => t.context === 'office');
       } else if (contextFilter === 'ai') {
         tools = uniqueAllTools.filter(t => t.context === 'ai' || t.category === 'ai' || t.tags.includes('ocr') || t.tags.includes('ai'));
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
    report?: Record<string, any>;
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
      case 'office_extract_text': customOutputPath = getOutputPath(`${base}.txt`);      break;
      case 'office_modernize_legacy': {
        const srcExt = docPath.split('.').pop()?.toLowerCase() ?? 'docx';
        const modernExt = srcExt === 'doc' ? 'docx' : srcExt === 'xls' ? 'xlsx' : srcExt === 'ppt' ? 'pptx' : srcExt;
        customOutputPath = getOutputPath(`${base}_modern.${modernExt}`);
        break;
      }
      case 'excel_to_json':       customOutputPath = getOutputPath(`${base}.json`);     break;
      case 'excel_to_csv':        customOutputPath = getOutputPath(`${base}.csv`);      break;
      case 'merge':
        customOutputPath = ''; // handled by native dialog
        break;
      case 'split':
      case 'pdf_split':
        customOutputPath = getOutputPath(`${base}_split`);
        break;
      case 'burst':
      case 'pdf_burst':
        customOutputPath = getOutputPath(`${base}_burst`);
        break;
      case 'compress':
      case 'pdf_compress':  customOutputPath = getOutputPath(`${base}_compressed.pdf`);  break;
      case 'rotate':
      case 'pdf_rotate':        customOutputPath = getOutputPath(`${base}_rotated.pdf`);     break;
      case 'crop':
      case 'pdf_crop':          customOutputPath = getOutputPath(`${base}_cropped.pdf`);     break;
      case 'reorder_pages':
      case 'pdf_reorder_pages': customOutputPath = getOutputPath(`${base}_reordered.pdf`);   break;
      case 'watermark':
      case 'pdf_watermark':     customOutputPath = getOutputPath(`${base}_watermarked.pdf`); break;
      case 'encrypt':
      case 'pdf_encrypt':       customOutputPath = getOutputPath(`${base}_encrypted.pdf`);   break;
      case 'decrypt':
      case 'pdf_decrypt':       customOutputPath = getOutputPath(`${base}_decrypted.pdf`);   break;
      case 'sanitize':
      case 'pdf_sanitize':      customOutputPath = getOutputPath(`${base}_sanitized.pdf`);   break;
      case 'metadata':
      case 'pdf_metadata':      customOutputPath = getOutputPath(`${base}_metadata.pdf`);    break;
      case 'change_permissions':
      case 'pdf_change_permissions': customOutputPath = getOutputPath(`${base}_permissions.pdf`); break;
      case 'timestamp':
      case 'pdf_timestamp':     customOutputPath = getOutputPath(`${base}_timestamped.pdf`); break;
      case 'sign':
      case 'pdf_sign':          customOutputPath = getOutputPath(`${base}_signed.pdf`);      break;
      case 'validate_signatures':
      case 'pdf_validate_signatures': customOutputPath = getOutputPath(`${base}_signature_report.json`); break;
      case 'remove_signatures':
      case 'pdf_remove_signatures': customOutputPath = getOutputPath(`${base}_unsigned.pdf`); break;
      case 'hash':
      case 'pdf_hash':          customOutputPath = getOutputPath(`${base}_checksums.txt`);   break;
      case 'validate':
      case 'pdf_validate':      customOutputPath = getOutputPath(`${base}_validate_report.json`); break;
      case 'document_pii_scan':
      case 'pii_scan':
      case 'pdf_document_pii_scan': customOutputPath = getOutputPath(`${base}_pii_report.json`); break;
      case 'bates':
      case 'pdf_bates':         customOutputPath = getOutputPath(`${base}_bates.pdf`);       break;
      case 'page_numbers':
      case 'pdf_page_numbers':  customOutputPath = getOutputPath(`${base}_page_numbered.pdf`); break;
      case 'header_footer':
      case 'pdf_header_footer': customOutputPath = getOutputPath(`${base}_header.pdf`);      break;
      case 'annotate':
      case 'pdf_annotate':      customOutputPath = getOutputPath(`${base}_annotated.pdf`);   break;
      case 'redact':
      case 'pdf_redact':        customOutputPath = getOutputPath(`${base}_redacted.pdf`);    break;
      case 'overlay_underlay':
      case 'pdf_overlay_underlay': customOutputPath = getOutputPath(`${base}_stationery.pdf`); break;
      case 'stamp':
      case 'pdf_stamp':         customOutputPath = getOutputPath(`${base}_stamped.pdf`);     break;
      case 'flatten':
      case 'pdf_flatten':       customOutputPath = getOutputPath(`${base}_flattened.pdf`);   break;
      case 'fill_form':
      case 'pdf_fill_form':     customOutputPath = getOutputPath(`${base}_filled.pdf`);      break;
      case 'create_form_field':
      case 'pdf_create_form_field': customOutputPath = getOutputPath(`${base}_with_field.pdf`); break;
      case 'batch_fill':
      case 'cli_fill_batch':    customOutputPath = getOutputPath(`${base}_batch_receipt.json`); break;
      case 'read_form':
      case 'pdf_read_form':     customOutputPath = getOutputPath(`${base}_form_fields.json`); break;
      case 'extract_pages':
      case 'pdf_extract_pages': customOutputPath = getOutputPath(`${base}_extracted.pdf`);  break;
      case 'delete_pages':
      case 'pdf_delete_pages':  customOutputPath = getOutputPath(`${base}_deleted.pdf`);    break;
      case 'remove_blank':
      case 'pdf_remove_blank':  customOutputPath = getOutputPath(`${base}_cleaned.pdf`);    break;
      case 'pdf_scale_pages':   customOutputPath = getOutputPath(`${base}_scaled.pdf`);     break;
      case 'pdf_nup':           customOutputPath = getOutputPath(`${base}_nup.pdf`);        break;
      case 'pdf_booklet':       customOutputPath = getOutputPath(`${base}_booklet.pdf`);    break;
      case 'pdf_posterize':     customOutputPath = getOutputPath(`${base}_poster.pdf`);     break;
      case 'pdf_stitch_canvas': customOutputPath = getOutputPath(`${base}_stitched.pdf`);   break;
      case 'bookmarks':
      case 'pdf_bookmarks':     customOutputPath = getOutputPath(`${base}_bookmarks.json`);  break;
      case 'pdf_attachments':   customOutputPath = attachmentAction === 'add' ? getOutputPath(`${base}_with_attachment.pdf`) : getOutputPath(`${base}_attachments.json`); break;
      case 'pdf_auto_rotate':   customOutputPath = getOutputPath(`${base}_autorotated.pdf`); break;
      case 'repair':
      case 'pdf_repair':        customOutputPath = getOutputPath(`${base}_repaired.pdf`);    break;
      case 'invert_colors':
      case 'pdf_invert_colors': customOutputPath = getOutputPath(`${base}_inverted.pdf`);    break;
      case 'render':
      case 'pdf_render':        customOutputPath = getOutputPath(`${base}_rendered.png`);    break;
      case 'compare':
      case 'pdf_compare':       customOutputPath = getOutputPath(`${base}_diff_report.json`); break;
      case 'extract_text':
      case 'pdf_extract_text':  customOutputPath = getOutputPath(`${base}_text.txt`);        break;
      case 'ocr':
      case 'pdf_ocr':
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
      case 'pdf_to_json':     customOutputPath = getOutputPath(`${base}.json`);           break;
      case 'pdf_to_pptx':     customOutputPath = getOutputPath(`${base}.pptx`);           break;
      case 'pdf_extract_tables': customOutputPath = getOutputPath(`${base}_tables.${tableFormat === 'csv' ? 'csv' : 'json'}`); break;
      case 'extract_images':
      case 'pdf_extract_images': customOutputPath = getOutputPath(`${base}_images`);       break;
      case 'pdf_comic_archive': customOutputPath = getOutputPath(cbzArchiveName || `${base}_output.cbz`); break;
      case 'pdf_to_epub':     customOutputPath = getOutputPath(`${base}_output.epub`);     break;
      case 'pdf_to_pdf_a':    customOutputPath = getOutputPath(`${base}_pdfa.pdf`);        break;
      case 'pdf_convert_excel': customOutputPath = getOutputPath(`${base}_table.pdf`);     break;
      case 'extract_patterns':
      case 'pdf_extract_patterns':
        customOutputPath = getOutputPath(`${base}_extract_report.json`);
        break;
      case 'search':
      case 'pdf_search':
        customOutputPath = getOutputPath(`${base}_search.json`);
        break;
      case 'workspace_search':
        customOutputPath = getOutputPath(`${base}_workspace_search.json`);
        break;
      case 'semantic_search':
        customOutputPath = getOutputPath(`${base}_semantic_search.json`);
        break;
      case 'classify':
      case 'classify_type':
      case 'pdf_classify_type':
        customOutputPath = getOutputPath(`${base}_classified.json`);
        break;
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

    if (activeTool.id === 'compare' || activeTool.id === 'pdf_compare') {
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
            pageNumberPosition: pageNumberPos,
            pageNumberStart: 1,
            pageNumberFormat,
            convertPreset,
            scaleTargetSize,
            scaleFactor,
            reorderList,
            signCertPath,
            headerText,
            footerText,
            metadataTitle,
            compressQuality,
            blankSensitivity,
            searchQuery: textSearchQuery,
            workspaceQuery: workspaceSearchQuery,
            workspaceDir: workspaceSearchDir,
            semanticQuery: semanticSearchQuery,
            semanticLimit: semanticSearchLimit,
            classifyDetail,
            nupPages,
            nupOrdering,
            nupMargin,
            nupSpacing,
            nupDrawBorders,
            action: attachmentAction,
            file_name: attachmentFileName,
            description: attachmentDescription,
            file_data_base64: attachmentBase64,
            extractPatternsKeywords,
            extractPatternsRegex,
            extractPatternsCaseSensitive,
            extractPatternsContextLen,
            mergeAlternateDocA,
            mergeAlternateDocB,
            mergeAlternateReverseB,
            mergeAlternateStep,
            mergeAlternateStepB,
            bookletBindingEdge,
            bookletSheetOrdering,
            posterGrid,
            posterScale,
            posterTargetSheet,
            posterOverlap,
            posterGuides,
            stitchDirection,
            stitchSpacing,
            stitchMargin,
            filePath: attachmentFilePath,
            metadataAuthor,
            metadataSubject,
            metadataKeywords,
            sanitizeScripts,
            sanitizeMetadata,
            sanitizeAttachments,
            tsaServerUrl,
            allowPrinting,
            allowCopying,
            allowModifying,
            signPassword,
            signFieldName,
            signReason,
            signLocation,
            hashAlgorithm,
            stationeryPath,
            overlayMode,
            overlayPages,
            overlayOpacity,
            stampImagePath,
            stampX,
            stampY,
            stampWidth,
            stampHeight,
            stampOpacity,
            stampPages,
            piiScanSsn,
            piiScanCreditCards,
            piiScanEmails,
            piiScanPhones,
            piiScanIban,
            redactQuery,
            redactRect,
            annotationType,
            annotationContent,
            annotationColor,
            annotationRect,
            annotationPage,
            formDataPath,
            formFieldName,
            formFieldType,
            formFieldPage,
            formFieldRect,
            batchCsvPath,
            dpi: conversionDpi,
            imageFormat,
            tableFormat,
            cbzArchiveName,
            pdfaConformance,
            excelSheet: excelSheet.trim() || undefined,
            hasHeaders
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

               const { WasmPdfClientImpl } = await import('$lib/wasm/index');
               const client = new WasmPdfClientImpl();
               let fileData = new Uint8Array();

               if (doc && (doc as any)._isLoaded) {
                   const arrayBuf = await doc.arrayBuffer();
                   fileData = new Uint8Array(arrayBuf);
               } else if (docPath) {
                   const rawBytes = await invoke<number[]>('read_file_bytes', { path: docPath }).catch(() => []);
                   fileData = new Uint8Array(rawBytes);
               }

               // Remove pdf_ prefix for engine compatibility if needed
               let opType = activeTool.id;
               if (opType.startsWith('pdf_') && opType !== 'pdf_to_docx') {
                   opType = opType.replace('pdf_', '');
               }

               let opPayload: any = { ...mobileParams, file: fileData };
               result = await (client as any).runOperation(opType, opPayload);

               // Re-structure to match expected MCP/REST response for single execution
               result = {
                   success: true,
                   fileName: getDocOutputPath(customOutputPath || `${docName}_out.pdf`),
                   dataBase64: btoa(String.fromCharCode.apply(null, Array.from(new Uint8Array(result))))
               };
            } else {
                if (appState.zeroTrace) {
                  args._zero_trace = true;
                }
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
          pageNumberPosition: pageNumberPos,
          pageNumberStart: 1,

          pageNumberFormat,

          convertPreset,
          scaleTargetSize,
          scaleFactor,
          reorderList,
          signCertPath,
          headerText,
          footerText,
          metadataTitle,
          compressQuality,
          blankSensitivity,
          searchQuery: textSearchQuery,
          workspaceQuery: workspaceSearchQuery,
          workspaceDir: workspaceSearchDir,
          semanticQuery: semanticSearchQuery,
          semanticLimit: semanticSearchLimit,
          classifyDetail,
          nupPages,
          nupOrdering,
          nupMargin,
          nupSpacing,
          nupDrawBorders,
          action: attachmentAction,
          file_name: attachmentFileName,
          description: attachmentDescription,
          file_data_base64: attachmentBase64,
          extractPatternsKeywords,
          extractPatternsRegex,
          extractPatternsCaseSensitive,
          extractPatternsContextLen,
          mergeAlternateDocA,
          mergeAlternateDocB,
          mergeAlternateReverseB,
          mergeAlternateStep,
          mergeAlternateStepB,
          bookletBindingEdge,
          bookletSheetOrdering,
          posterGrid,
          posterScale,
          posterTargetSheet,
          posterOverlap,
          posterGuides,
          stitchDirection,
          stitchSpacing,
          stitchMargin,
          filePath: attachmentFilePath,
          metadataAuthor,
          metadataSubject,
          metadataKeywords,
          sanitizeScripts,
          sanitizeMetadata,
          sanitizeAttachments,
          tsaServerUrl,
          allowPrinting,
          allowCopying,
          allowModifying,
          signPassword,
          signFieldName,
          signReason,
          signLocation,
          hashAlgorithm,
          stationeryPath,
          overlayMode,
          overlayPages,
          overlayOpacity,
          stampImagePath,
          stampX,
          stampY,
          stampWidth,
          stampHeight,
          stampOpacity,
          stampPages,
          piiScanSsn,
          piiScanCreditCards,
          piiScanEmails,
          piiScanPhones,
          piiScanIban,
          redactQuery,
          redactRect,
          annotationType,
          annotationContent,
          annotationColor,
          annotationRect,
          annotationPage,
          formDataPath,
          formFieldName,
          formFieldType,
          formFieldPage,
          formFieldRect,
          batchCsvPath,
          dpi: conversionDpi,
          imageFormat,
          tableFormat,
          cbzArchiveName,
          pdfaConformance,
          excelSheet: excelSheet.trim() || undefined,
          hasHeaders
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

               const { WasmPdfClientImpl } = await import('$lib/wasm/index');
               const client = new WasmPdfClientImpl();
               let fileData = new Uint8Array();

               const currentDoc = appState.documents[docIndex];
               if (currentDoc && (currentDoc as any)._isLoaded) {
                   const arrayBuf = await currentDoc.arrayBuffer();
                   fileData = new Uint8Array(arrayBuf);
               } else if (docPath) {
                   const rawBytes = await invoke<number[]>('read_file_bytes', { path: docPath }).catch(() => []);
                   fileData = new Uint8Array(rawBytes);
               }

               let opType = activeTool.id;
               if (opType.startsWith('pdf_') && opType !== 'pdf_to_docx') {
                   opType = opType.replace('pdf_', '');
               }

               let opPayload: any = { ...mobileParams, file: fileData };

               if (opType === 'merge') {
                   const filePayloads = [];
                   for (let idx = 0; idx < appState.documents.length; idx++) {
                       const d = appState.documents[idx];
                       const buf = await d.arrayBuffer();
                       filePayloads.push(new Uint8Array(buf));
                   }
                   opPayload = { files: filePayloads };
               }

               result = await (client as any).runOperation(opType, opPayload);

               result = {
                   success: true,
                   fileName: getOutputPath(customOutputPath || `${docName}_out.pdf`),
                   dataBase64: btoa(String.fromCharCode.apply(null, Array.from(new Uint8Array(result))))
               };
            } else {
                result = await invoke('invoke_mcp_tool', { toolName, arguments: args });
            }

          const durationMs = Math.round(performance.now() - startTime);
          const durationFormatted = durationMs < 1000 ? `${durationMs}ms` : `${(durationMs / 1000).toFixed(1)}s`;

          const resObj = result as any;
          if (resObj && resObj.success) {
            let msg = '';
            if ((activeTool.id === 'ocr' || activeTool.id === 'pdf_ocr') && ocrOutputFormat === 'clipboard') {
              const textToCopy = resObj.data?.text || '';
              if (navigator.clipboard && textToCopy) {
                await navigator.clipboard.writeText(textToCopy);
              }
              msg = `📋 OCR text copied to clipboard! (${textToCopy.length} characters)`;
            } else if (['search', 'pdf_search', 'workspace_search', 'semantic_search'].includes(activeTool.id)) {
              msg = resObj.message || 'Search completed';
              searchResultDisplay = resObj.message || 'No matches found.';
            } else if (['classify', 'classify_type', 'pdf_classify_type'].includes(activeTool.id)) {
              msg = resObj.message || 'Classification completed';
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
              message: msg,
              report: (customOutputPath || args.output)?.toLowerCase().endsWith('.json') && resObj.data ? resObj.data : undefined
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

  let currentSnippetTarget = $derived.by(() => {
    if (!activeTool) return null;
    const docIndex = appState.selectedDocumentIndex ?? 0;
    const docName = appState.documents.length > 0 ? (appState.documents[docIndex]?.name || appState.documents[0].name) : 'input.pdf';
    const docPath = appState.documents.length > 0 ? (appState.documentPaths[docIndex] || docName) : 'input.pdf';
    const docDir = docPath.includes('/') ? docPath.substring(0, docPath.lastIndexOf('/')) : '';
    const getOutputPath = (filename: string) => docDir ? `${docDir}/${filename}` : filename;

    const rawToolId = activeTool.id;
    const toolId = rawToolId.startsWith('pdf_') ? rawToolId : `pdf_${rawToolId}`;
    let ext = '.pdf';
    if (['extract_text', 'pdf_extract_text', 'office_extract_text'].includes(rawToolId) || ['extract_text', 'pdf_extract_text'].includes(toolId)) ext = '.txt';
    else if (['pdf_to_json', 'excel_to_json', 'pdf_extract_patterns', 'extract_patterns', 'pdf_search', 'search', 'workspace_search', 'semantic_search', 'pdf_classify_type', 'classify_type', 'classify'].includes(rawToolId) || ['pdf_to_json', 'excel_to_json', 'pdf_extract_patterns', 'pdf_search', 'workspace_search', 'semantic_search', 'pdf_classify_type'].includes(toolId)) ext = '.json';
    else if (['pdf_to_markdown', 'office_to_markdown'].includes(rawToolId) || ['pdf_to_markdown', 'office_to_markdown'].includes(toolId)) ext = '.md';
    else if (['pdf_to_html', 'excel_to_html'].includes(rawToolId) || ['pdf_to_html', 'excel_to_html'].includes(toolId)) ext = '.html';
    else if (['pdf_to_docx'].includes(rawToolId) || ['pdf_to_docx'].includes(toolId)) ext = '.docx';
    else if (['pdf_to_xlsx', 'excel_to_csv'].includes(rawToolId) || ['pdf_to_xlsx', 'excel_to_csv'].includes(toolId)) ext = rawToolId.includes('csv') ? '.csv' : '.xlsx';
    else if (['pdf_extract_tables'].includes(toolId) || rawToolId === 'pdf_extract_tables') ext = tableFormat === 'csv' ? '.csv' : '.json';
    else if (rawToolId === 'office_modernize_legacy') {
      const srcExt = docPath.split('.').pop()?.toLowerCase() ?? 'docx';
      ext = srcExt === 'doc' ? '.docx' : srcExt === 'xls' ? '.xlsx' : srcExt === 'ppt' ? '.pptx' : '.docx';
    }

    const defaultOutput = docPath ? docPath.replace(/\.[^/.]+$/, `_${activeTool.id.replace('pdf_', '')}${ext}`) : `output${ext}`;
    const output = customOutputPath || defaultOutput;

    const params: Record<string, any> = {
      input: docPath,
      output,
      pages: splitPoints || extractPages || deletePages || rotatePages || 'all',
      angle: parseInt(rotateAngle, 10) || 90,
      text: watermarkText || 'CONFIDENTIAL',
      opacity: watermarkOpacity / 100,
      password: password || 'secure123',
      prefix: batesPrefix || 'BATES-',
      start_number: batesStart || 1,
      padding: batesPadding || 6,
      quality: compressQuality || 80
    };

    return {
      toolId,
      intent: activeTool.title,
      params,
      inputFile: docPath,
      outputFile: output,
      inputs: appState.documentPaths.length > 0 ? appState.documentPaths : [docPath]
    } as SnippetTarget;
  });

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

  async function handleBrowseOutputPath() {
    if (!activeTool) return;
    try {
      const isFolderOutput = activeTool.id === 'split' || activeTool.id === 'pdf_split' || activeTool.id === 'burst' || activeTool.id === 'pdf_burst' || activeTool.id === 'extract_images' || activeTool.id === 'pdf_extract_images';
      if (isFolderOutput) {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const selected = await open({
          directory: true,
          multiple: false,
          defaultPath: customOutputPath || undefined,
          title: `Select ${activeTool.title} Output Folder`
        });
        if (selected && typeof selected === 'string') {
          customOutputPath = selected;
        }
      } else {
        const { save } = await import('@tauri-apps/plugin-dialog');
        // Determine file extensions based on active tool
        const rawToolId = activeTool.id;
        const toolId = rawToolId.startsWith('pdf_') ? rawToolId : `pdf_${rawToolId}`;
        let filters = [{ name: 'PDF Document', extensions: ['pdf'] }];
        if (['extract_text', 'pdf_extract_text', 'office_extract_text'].includes(rawToolId) || ['extract_text', 'pdf_extract_text'].includes(toolId)) {
          filters = [{ name: 'Text File', extensions: ['txt'] }];
        } else if (['pdf_to_json', 'excel_to_json', 'pdf_extract_patterns', 'extract_patterns', 'pdf_search', 'search', 'workspace_search', 'semantic_search', 'pdf_classify_type', 'classify_type', 'classify'].includes(rawToolId) || ['pdf_to_json', 'excel_to_json', 'pdf_extract_patterns', 'pdf_search', 'workspace_search', 'semantic_search', 'pdf_classify_type'].includes(toolId)) {
          filters = [{ name: 'JSON Document', extensions: ['json'] }, { name: 'Text File', extensions: ['txt'] }];
        } else if (['pdf_to_markdown', 'office_to_markdown'].includes(rawToolId) || ['pdf_to_markdown', 'office_to_markdown'].includes(toolId)) {
          filters = [{ name: 'Markdown File', extensions: ['md'] }];
        } else if (['pdf_to_html', 'excel_to_html'].includes(rawToolId) || ['pdf_to_html', 'excel_to_html'].includes(toolId)) {
          filters = [{ name: 'HTML Document', extensions: ['html'] }];
        } else if (['pdf_to_docx'].includes(rawToolId) || ['pdf_to_docx'].includes(toolId)) {
          filters = [{ name: 'Word Document', extensions: ['docx'] }];
        } else if (['pdf_to_xlsx'].includes(rawToolId) || ['pdf_to_xlsx'].includes(toolId)) {
          filters = [{ name: 'Excel Spreadsheet', extensions: ['xlsx'] }];
        } else if (['excel_to_csv'].includes(rawToolId) || ['excel_to_csv'].includes(toolId)) {
          filters = [{ name: 'CSV File', extensions: ['csv'] }];
        } else if (rawToolId === 'office_modernize_legacy') {
          filters = [{ name: 'Modern Office Document', extensions: ['docx', 'xlsx', 'pptx'] }];
        } else if (['pdf_extract_tables'].includes(toolId) || rawToolId === 'pdf_extract_tables') {
          filters = [{ name: 'JSON Document', extensions: ['json'] }, { name: 'CSV File', extensions: ['csv'] }];
        } else if (['pdf_comic_archive'].includes(toolId) || rawToolId === 'pdf_comic_archive') {
          filters = [{ name: 'Comic Book Archive', extensions: ['cbz'] }];
        } else if (['pdf_to_epub'].includes(toolId) || rawToolId === 'pdf_to_epub') {
          filters = [{ name: 'EPUB E-Book', extensions: ['epub'] }];
        }

        const selected = await save({
          defaultPath: customOutputPath || undefined,
          title: `Save ${activeTool.title} Output As`,
          filters
        });
        if (selected) {
          customOutputPath = selected;
        }
      }
    } catch (err) {
      console.warn('File save dialog error:', err);
    }
  }
</script>

<div
  class="operations-panel"
  class:embedded={embedded}
  class:collapsed={!embedded && isCollapsed && !appState.isMobile}
  class:dragging={isDragging && !appState.isMobile}
  class:mobile-bottom-sheet={appState.isMobile}
  id="operations-panel"
  style={!embedded && !appState.isMobile && !isCollapsed ? `width: ${panelWidth}px; min-width: ${panelWidth}px;` : ''}
>
  {#if !isCollapsed && !embedded}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="resize-handle"
      id="operations-panel-resize-handle"
      title="Drag to resize panel, double-click to toggle width"
      onmousedown={startResize}
      ondblclick={handleResizeDblClick}
    ></div>
  {/if}

  {#if !embedded}
    <button
      class="collapse-toggle"
      id="operations-panel-toggle"
      onclick={() => isCollapsed = !isCollapsed}
      title={isCollapsed ? 'Expand tools panel' : 'Collapse tools panel'}
      aria-label={isCollapsed ? 'Expand tools panel' : 'Collapse tools panel'}
    >
      {isCollapsed ? '‹' : '›'}
    </button>
  {/if}

  {#if embedded || !isCollapsed || appState.isMobile}
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
      <button
        class="code-export-btn"
        id="btn-tool-export-code"
        onclick={() => appState.openExportCode(currentSnippetTarget)}
        title="Export operation as cURL, CLI, MCP prompt, or Embed JS"
      >
        &lt;/&gt; Code / API
      </button>
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
      <div class="tool-format-badges" data-testid="tool-format-badges">
        <span class="format-badge input-badge" title="Input document requirements">
          <span class="badge-label">Input</span>
          <span class="badge-value">{activeTool.inputFormat}</span>
        </span>
        <span class="badge-arrow" aria-hidden="true">→</span>
        <span class="format-badge output-badge" title="Generated artifact format">
          <span class="badge-label">Output</span>
          <span class="badge-value">{activeTool.outputFormat}</span>
        </span>
      </div>

      <p class="tool-desc">{activeTool.description}</p>
      {#if activeTool.detailedDescription}
        <p class="tool-detailed-desc">{activeTool.detailedDescription}</p>
      {/if}

      {#if activeTool.id === 'delete_pages' || activeTool.id === 'pdf_delete_pages' || activeTool.id === 'redact' || activeTool.id === 'pdf_redact'}
        <div class="destructive-warning-callout" role="alert" data-testid="destructive-warning">
          <div class="warning-callout-header">
            <span class="warning-callout-icon">⚠️</span>
            <span class="warning-callout-title">
              {activeTool.id.includes('redact') ? 'Irreversible Content Excision' : 'Destructive Page Removal'}
            </span>
          </div>
          <p class="warning-callout-text">
            {#if activeTool.id.includes('redact')}
              Permanent excision: underlying text glyphs, font operators, and vector paths will be permanently purged from the PDF stream and cannot be recovered.
            {:else}
              Permanent removal: selected pages will be omitted from the generated PDF output. Your original source file remains safe and untouched on disk.
            {/if}
          </p>
        </div>
      {/if}

      {#if activeTool.id === 'merge' || activeTool.id === 'pdf_merge'}
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
      {:else if activeTool.id === 'pdf_merge_alternate'}
        <div class="operation-config">
          <label for="mergeAlternateDocA" class="section-desc">Document A (Front Pages):</label>
          <select id="mergeAlternateDocA" bind:value={mergeAlternateDocA} class="form-input" style="margin-bottom: 12px;">
            {#if appState.documents.length === 0}
              <option value="">No documents loaded</option>
            {:else}
              {#each appState.documents as doc, i}
                <option value={appState.documentPaths[i] || doc.name}>{doc.name} ({appState.documentPaths[i] || doc.name})</option>
              {/each}
            {/if}
          </select>

          <label for="mergeAlternateDocB" class="section-desc">Document B (Back Pages):</label>
          <select id="mergeAlternateDocB" bind:value={mergeAlternateDocB} class="form-input" style="margin-bottom: 12px;">
            {#if appState.documents.length === 0}
              <option value="">No documents loaded</option>
            {:else}
              {#each appState.documents as doc, i}
                <option value={appState.documentPaths[i] || doc.name}>{doc.name} ({appState.documentPaths[i] || doc.name})</option>
              {/each}
            {/if}
          </select>

          <label class="toggle-label" style="display: flex; align-items: center; gap: 8px; margin-top: 8px; cursor: pointer;">
            <input id="mergeAlternateReverseB" type="checkbox" bind:checked={mergeAlternateReverseB} />
            <span class="section-desc" style="margin: 0;">Reverse Doc B (Duplex Backside)</span>
          </label>
          <span class="field-hint">Collate odd pages from Doc A and even pages from Doc B. If Doc B was scanned in reverse stack order, keep reversed checked.</span>
        </div>
      {:else if activeTool.id === 'split' || activeTool.id === 'pdf_split'}
        <div class="operation-config">
          <label for="splitPoints" class="section-desc">Split Page Ranges (comma separated):</label>
          <input id="splitPoints" type="text" class="form-input" bind:value={splitPoints} placeholder="e.g. 1-3, 4-8" />
          <span class="field-hint">Specify discrete pages or intervals (e.g. 1-3, 5, 8-10) to generate separate output files.</span>
        </div>
      {:else if activeTool.id === 'delete_pages' || activeTool.id === 'pdf_delete_pages'}
        <div class="operation-config">
          <label for="deletePages" class="section-desc">Pages to Delete (comma separated or ranges):</label>
          <input id="deletePages" type="text" class="form-input" bind:value={deletePages} placeholder="e.g. 1, 3, 5-7" />
          <span class="field-hint">Specify 1-based page numbers or ranges to permanently remove (e.g. 1, 3, 5-7).</span>
        </div>
      {:else if activeTool.id === 'extract_pages' || activeTool.id === 'pdf_extract_pages'}
        <div class="operation-config">
          <label for="extractPages" class="section-desc">Pages to Extract (comma separated or ranges):</label>
          <input id="extractPages" type="text" class="form-input" bind:value={extractPages} placeholder="e.g. 1-3, 5" />
          <span class="field-hint">Specify discrete pages or intervals (e.g. 1-3, 5) to isolate into a new document.</span>
        </div>
      {:else if activeTool.id === 'compress' || activeTool.id === 'pdf_compress'}
        <div class="operation-config">
          <label for="compressQuality" class="section-desc">Image Quality: {compressQuality}%</label>
          <input id="compressQuality" type="range" min="10" max="100" bind:value={compressQuality} class="range-input" />
          <div class="range-labels">
            <span>Smaller file</span>
            <span>Higher quality</span>
          </div>
          <span class="field-hint">Higher quality retains fine detail at larger file sizes; 80% is recommended for standard printing (300 DPI).</span>
        </div>
      {:else if activeTool.id === 'rotate' || activeTool.id === 'pdf_rotate'}
        <div class="operation-config">
          <label for="rotateAngle" class="section-desc">Rotation Angle:</label>
          <select id="rotateAngle" bind:value={rotateAngle} class="form-input">
            <option value="90">90° Clockwise</option>
            <option value="180">180° Flip</option>
            <option value="270">270° Counter-Clockwise</option>
          </select>
          <span class="field-hint">Rotate clockwise in 90° increments (90°, 180°, 270°).</span>
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
          <span class="field-hint">Specify discrete pages or intervals (e.g. 1, 3, 5-10), or leave empty for all pages.</span>
        </div>
      {:else if activeTool.id === 'watermark' || activeTool.id === 'pdf_watermark'}
        <div class="operation-config">
          <label for="watermarkText" class="section-desc">Watermark Text:</label>
          <input id="watermarkText" type="text" class="form-input" bind:value={watermarkText} placeholder="e.g. CONFIDENTIAL" />
          <span class="field-hint">Text stamped across pages (e.g. CONFIDENTIAL, DRAFT, INTERNAL ONLY).</span>
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
      {:else if activeTool.id === 'pdf_extract_patterns' || activeTool.id === 'extract_patterns'}
        <div class="operation-config">
          <label for="extractPatternsKeywords" class="section-desc">Keywords (comma-separated):</label>
          <input id="extractPatternsKeywords" type="text" class="form-input" bind:value={extractPatternsKeywords} placeholder="e.g. Confidential, NDA, Urgent" />
          <span class="field-hint">Extracts all occurrences across all pages.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="extractPatternsRegex" class="section-desc">Custom Regex (label=regex per line or comma):</label>
          <textarea id="extractPatternsRegex" class="form-input" style="height: 60px; font-family: monospace; font-size: 11px;" bind:value={extractPatternsRegex} placeholder={"ssn=\\d{3}-\\d{2}-\\d{4}\nemail=[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"}></textarea>
          <span class="field-hint">Captures all matching instances with page numbers and text snippets.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px; display: flex; align-items: center; justify-content: space-between;">
          <label for="extractPatternsCase" class="section-desc" style="margin-bottom: 0;">Case Sensitive:</label>
          <input id="extractPatternsCase" type="checkbox" bind:checked={extractPatternsCaseSensitive} />
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="extractPatternsContextLen" class="section-desc">Context Length: {extractPatternsContextLen} chars</label>
          <input id="extractPatternsContextLen" type="range" min="10" max="100" bind:value={extractPatternsContextLen} class="range-input" />
        </div>
      {:else if activeTool.id === 'remove_blank' || activeTool.id === 'pdf_remove_blank'}
        <div class="operation-config">
          <label for="blankSensitivity" class="section-desc">Sensitivity (0-100): {blankSensitivity}%</label>
          <input id="blankSensitivity" type="range" min="0" max="100" bind:value={blankSensitivity} class="range-input" />
          <span class="field-hint">Higher sensitivity removes pages with even minimal or faint ink traces.</span>
        </div>
      {:else if activeTool.id === 'page_numbers' || activeTool.id === 'pdf_page_numbers'}
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
          <span class="field-hint">Dynamic template placeholders: &#123;n&#125; for current page, &#123;total&#125; for total page count.</span>
        </div>
      {:else if activeTool.id === 'md_to_pdf' || activeTool.id === 'html_to_pdf' || activeTool.id === 'pdf_convert_markdown' || activeTool.id === 'pdf_convert_html'}
        <div class="operation-config">
          <label for="convertPreset" class="section-desc">Style Preset:</label>
          <select id="convertPreset" bind:value={convertPreset} class="form-input">
            <option value="academic">Academic</option>
            <option value="corporate">Corporate</option>
            <option value="minimalist">Minimalist</option>
          </select>
          <span class="field-hint">Applies typography, margins, and header styles tailored to the selected publication preset.</span>
        </div>
      {:else if activeTool.id === 'json_to_pdf'}
        <div class="operation-config">
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Synthesize a publication-grade PDF from structured JSON text pages and inline base64 / file-path images. Click <strong>Run</strong> to select your JSON file.
          </p>
          <span class="field-hint">JSON schema must define pages, typography, and optional embedded image assets.</span>
        </div>
      {:else if activeTool.id === 'encrypt' || activeTool.id === 'pdf_encrypt' || activeTool.id === 'decrypt' || activeTool.id === 'pdf_decrypt'}
        <div class="operation-config">
          <label for="passwordField" class="section-desc">Password:</label>
          <input id="passwordField" type="password" class="form-input" bind:value={password} placeholder="Enter document password" />
          <input id="password" type="hidden" value={password} />
          <span class="field-hint">Used to encrypt output with 256-bit AES encryption (or unlock protected documents).</span>
        </div>
      {:else if activeTool.id === 'bates' || activeTool.id === 'pdf_bates'}
        <div class="operation-config">
          <label for="batesPrefix" class="section-desc">Prefix:</label>
          <input id="batesPrefix" type="text" class="form-input" bind:value={batesPrefix} placeholder="e.g. CONF-" />
          <span class="field-hint">Leading alphanumeric prefix for legal discovery tracking (e.g. CONF-).</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="batesStart" class="section-desc">Start Number:</label>
          <input id="batesStart" type="number" class="form-input" bind:value={batesStart} min="1" />
          <span class="field-hint">Initial sequential integer index for numbering.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="batesPadding" class="section-desc">Padding:</label>
          <input id="batesPadding" type="number" class="form-input" bind:value={batesPadding} min="1" />
          <span class="field-hint">Total digit count with zero padding (e.g. 6 produces 000001).</span>
        </div>
      {:else if activeTool.id === 'reorder_pages' || activeTool.id === 'pdf_reorder_pages'}
        <div class="operation-config">
          <label for="reorderList" class="section-desc">Order Permutation (comma separated):</label>
          <input id="reorderList" type="text" class="form-input" bind:value={reorderList} placeholder="e.g. 2,1,3,4,5" />
          <span class="field-hint">Specify full 1-based page order (e.g. 2,1,3,4,5 to swap the first two pages).</span>
        </div>
      {:else if activeTool.id === 'crop' || activeTool.id === 'pdf_crop'}
        <div class="operation-config">
          <label for="cropBox" class="section-desc">Crop Box (x,y,w,h):</label>
          <input id="cropBox" type="text" class="form-input" bind:value={cropBox} placeholder="e.g. 10,10,200,200" />
          <span class="field-hint">Specify boundaries as x,y,width,height in points (72 points = 1 inch).</span>
        </div>
      {:else if activeTool.id === 'sign' || activeTool.id === 'pdf_sign'}
        <div class="operation-config">
          <label for="signCertPath" class="section-desc">Certificate Path (.p12 / .pfx):</label>
          <input id="signCertPath" type="text" class="form-input" bind:value={signCertPath} placeholder="Path to .p12 cert (e.g. tests/e2e_fixtures/real/dummy.p12)" />
          <span class="field-hint">Select an RFC 3161 PKCS#12 (.p12 / .pfx) cryptographic certificate file.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="signPassword" class="section-desc">Certificate Password:</label>
          <input id="signPassword" type="password" class="form-input" bind:value={signPassword} placeholder="Enter certificate password" />
          <span class="field-hint">Password to decrypt the PKCS#12 private key container.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="signFieldName" class="section-desc">Signature Field Name:</label>
          <input id="signFieldName" type="text" class="form-input" bind:value={signFieldName} placeholder="Signature1" />
          <span class="field-hint">Target form widget field name for the cryptographic signature dictionary.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="signReason" class="section-desc">Signing Reason:</label>
          <input id="signReason" type="text" class="form-input" bind:value={signReason} placeholder="Approved and Verified" />
          <span class="field-hint">Reason for signing recorded in the signature dictionary /Reason entry.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="signLocation" class="section-desc">Signing Location:</label>
          <input id="signLocation" type="text" class="form-input" bind:value={signLocation} placeholder="Local Secure Terminal" />
          <span class="field-hint">Physical or digital location recorded in the signature dictionary /Location entry.</span>
        </div>
      {:else if activeTool.id === 'header_footer' || activeTool.id === 'pdf_header_footer'}
        <div class="operation-config">
          <label for="headerText" class="section-desc">Header Text:</label>
          <input id="headerText" type="text" class="form-input" bind:value={headerText} placeholder="e.g. Confidential" />
          <span class="field-hint">Text placed at top margin across all pages.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="footerText" class="section-desc">Footer Text:</label>
          <input id="footerText" type="text" class="form-input" bind:value={footerText} placeholder="e.g. Page" />
          <span class="field-hint">Text placed at bottom margin across all pages.</span>
        </div>
            {:else if activeTool.id === 'ocr' || activeTool.id === 'pdf_ocr'}
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

          {#if !isOcrDownloaded}
            <div style="margin-top: 16px; padding: 12px; background: rgba(59, 130, 246, 0.1); border: 1px solid rgba(59, 130, 246, 0.3); border-radius: 6px;">
               <p style="margin: 0 0 8px 0; font-size: 13px; color: #e2e8f0; display: flex; align-items: center; gap: 6px;">
                 <span style="color: #60a5fa;">○</span> Model not downloaded (~6 MB required)
               </p>
               {#if isOcrDownloading}
                 <div style="width: 100%; background: rgba(255,255,255,0.1); height: 6px; border-radius: 3px; overflow: hidden; margin-top: 8px;">
                    <div style="width: {ocrDownloadProgress}%; height: 100%; background: #3b82f6; transition: width 0.2s;"></div>
                 </div>
                 <p style="margin: 4px 0 0 0; font-size: 11px; color: #94a3b8; text-align: right;">{ocrDownloadProgress.toFixed(1)}%</p>
               {:else}
                 <button class="run-btn" style="background: #3b82f6; padding: 6px 12px; font-size: 12px; width: auto;" onclick={startOcrDownload}>
                   Download Offline OCR Engine
                 </button>
               {/if}
            </div>
          {:else}
             <div style="margin-top: 16px; font-size: 12px; color: #10b981; display: flex; align-items: center; gap: 6px;">
                <span>●</span> OCR Ready (Offline)
             </div>
          {/if}
        </div>
      {:else if activeTool.id === 'search' || activeTool.id === 'pdf_search'}
        <div class="operation-config">
          <label for="textSearchQueryInput" class="section-desc">Search Text:</label>
          <input
            id="textSearchQueryInput"
            type="text"
            class="form-input"
            bind:value={textSearchQuery}
            placeholder="Enter word or phrase to find in PDF..."
          />
          <p class="section-desc" style="font-size: 0.78rem; color: #a1a1aa; margin-top: 6px;">
            Scans document decoded content streams and identifies all pages matching the query text.
          </p>
          {#if searchResultDisplay}
            <div style="margin-top: 10px; padding: 10px 12px; background: rgba(56, 189, 248, 0.12); border: 1px solid rgba(56, 189, 248, 0.3); border-radius: 6px; font-size: 0.85rem; color: #38bdf8;">
              🔍 {searchResultDisplay}
            </div>
          {/if}
          <span class="field-hint" style="margin-top: 8px; display: block;">Output: <code>.json</code> — matched pages and occurrence counts.</span>
        </div>
      {:else if activeTool.id === 'pdf_nup'}
        <div class="operation-config">
          <label class="section-desc">Pages Per Sheet:</label>
          <div class="segmented" style="display: flex; gap: 8px; margin-bottom: 12px;">
            {#each [2, 4, 9, 16] as n}
              <button
                type="button"
                class="btn-segment"
                style="flex: 1; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--border-color); background: {nupPages === n ? 'var(--accent-color)' : 'var(--bg-surface)'}; color: {nupPages === n ? '#fff' : 'var(--text-primary)'}; font-size: 13px;"
                onclick={() => nupPages = n}
              >
                {n}-Up
              </button>
            {/each}
          </div>

          <label for="nupOrdering" class="section-desc">Ordering Direction:</label>
          <select id="nupOrdering" bind:value={nupOrdering} class="form-input" style="margin-bottom: 12px;">
            <option value="horizontal">Horizontal (Left to Right)</option>
            <option value="vertical">Vertical (Top to Bottom)</option>
          </select>

          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin-bottom: 12px;">
            <div>
              <label for="nupMargin" class="section-desc">Margin ({nupMargin} pt):</label>
              <input id="nupMargin" type="range" min="0" max="72" bind:value={nupMargin} class="range-input" />
            </div>
            <div>
              <label for="nupSpacing" class="section-desc">Spacing ({nupSpacing} pt):</label>
              <input id="nupSpacing" type="range" min="0" max="36" bind:value={nupSpacing} class="range-input" />
            </div>
          </div>

          <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; cursor: pointer;">
            <input type="checkbox" bind:checked={nupDrawBorders} />
            Draw borders around each page
          </label>
        </div>
      {:else if activeTool.id === 'pdf_scale_pages'}
        <div class="operation-config">
          <label for="scaleTargetSize" class="section-desc">Target Paper Size:</label>
          <select id="scaleTargetSize" bind:value={scaleTargetSize} class="form-input" style="margin-bottom: 12px;">
            <option value="A4">A4 (210 × 297 mm)</option>
            <option value="Letter">US Letter (8.5 × 11 in)</option>
            <option value="Legal">US Legal (8.5 × 14 in)</option>
            <option value="A3">A3 (297 × 420 mm)</option>
            <option value="A5">A5 (148 × 210 mm)</option>
            <option value="Tabloid">Tabloid (11 × 17 in)</option>
            <option value="Custom">Custom Scale</option>
          </select>
          <div style="margin-bottom: 12px;">
            <label for="scaleFactor" class="section-desc">Scale Factor: {scaleFactor}x</label>
            <input id="scaleFactor" type="range" min="0.25" max="3.0" step="0.05" bind:value={scaleFactor} class="range-input" />
          </div>
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Scales page dimensions uniformly while preserving original aspect ratio and centering contents.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_remove_signatures'}
        <div class="operation-config">
          <p class="section-desc" style="font-size: 0.85rem; color: #f59e0b; line-height: 1.4; margin-bottom: 8px;">
            ⚠️ Strips cryptographic signature dictionaries, locks, and widget annotations to enable editing.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_stamp' || activeTool.id === 'stamp'}
        <div class="operation-config">
          <label for="stampImagePath" class="section-desc">Stamp Image Path:</label>
          <input id="stampImagePath" type="text" bind:value={stampImagePath} class="form-input" placeholder="e.g. tests/e2e_fixtures/real/img1.png" style="margin-bottom: 8px;" />
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 8px;">
            <div>
              <label for="stampX" class="section-desc">X Origin (pt):</label>
              <input id="stampX" type="number" bind:value={stampX} class="form-input" />
            </div>
            <div>
              <label for="stampY" class="section-desc">Y Origin (pt):</label>
              <input id="stampY" type="number" bind:value={stampY} class="form-input" />
            </div>
          </div>
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 8px;">
            <div>
              <label for="stampWidth" class="section-desc">Width (pt):</label>
              <input id="stampWidth" type="number" bind:value={stampWidth} class="form-input" />
            </div>
            <div>
              <label for="stampHeight" class="section-desc">Height (pt):</label>
              <input id="stampHeight" type="number" bind:value={stampHeight} class="form-input" />
            </div>
          </div>
          <label for="stampPages" class="section-desc">Target Pages:</label>
          <input id="stampPages" type="text" bind:value={stampPages} class="form-input" placeholder="all or 1,3,5" style="margin-bottom: 8px;" />
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa;">
            Applies coordinate-exact visual stamps and branding onto target pages.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_overlay_underlay' || activeTool.id === 'overlay_underlay'}
        <div class="operation-config">
          <label for="stationeryPath" class="section-desc">Stationery / Letterhead PDF Path:</label>
          <input id="stationeryPath" type="text" bind:value={stationeryPath} class="form-input" placeholder="e.g. tests/e2e_fixtures/real/stationery.pdf" style="margin-bottom: 8px;" />
          <div style="margin-bottom: 8px;">
            <label for="overlayMode" class="section-desc">Layering Mode:</label>
            <select id="overlayMode" class="form-input" bind:value={overlayMode}>
              <option value="underlay">Underlay (Background Stationery)</option>
              <option value="overlay">Overlay (Foreground Watermark)</option>
            </select>
          </div>
          <div style="margin-bottom: 8px;">
            <label for="overlayPages" class="section-desc">Target Pages:</label>
            <input id="overlayPages" type="text" bind:value={overlayPages} class="form-input" placeholder="all, first, last, or 1" />
          </div>
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Applies stationery letterheads or templates beneath or above target page content streams.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_booklet'}
        <div class="operation-config">
          <label for="bookletBindingEdge" class="section-desc">Binding Edge:</label>
          <select id="bookletBindingEdge" bind:value={bookletBindingEdge} class="form-input" style="margin-bottom: 12px;">
            <option value="left">Left Edge (Standard Book / Magazine)</option>
            <option value="right">Right Edge (Manga / RTL)</option>
            <option value="top">Top Edge (Calendar / Notepad)</option>
          </select>

          <label for="bookletSheetOrdering" class="section-desc">Sheet Ordering:</label>
          <select id="bookletSheetOrdering" bind:value={bookletSheetOrdering} class="form-input" style="margin-bottom: 12px;">
            <option value="standard">Standard Saddle-Stitch</option>
            <option value="consecutive">Consecutive Sheets</option>
          </select>
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Rearranges and pairs pages into 2-up spreads so folding down the center creates a correctly ordered booklet.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_posterize'}
        <div class="operation-config">
          <label for="posterGrid" class="section-desc">Poster Split Grid:</label>
          <select id="posterGrid" bind:value={posterGrid} class="form-input" style="margin-bottom: 12px;">
            <option value="2x2">2 × 2 Grid (4 printable sheets)</option>
            <option value="3x3">3 × 3 Grid (9 printable sheets)</option>
            <option value="4x4">4 × 4 Grid (16 printable sheets)</option>
            <option value="2x1">2 × 1 Grid (2 horizontal sheets)</option>
            <option value="1x2">1 × 2 Grid (2 vertical sheets)</option>
          </select>

          <label for="posterTargetSheet" class="section-desc">Target Paper Size:</label>
          <select id="posterTargetSheet" bind:value={posterTargetSheet} class="form-input" style="margin-bottom: 12px;">
            <option value="a4">A4 (210 × 297 mm)</option>
            <option value="letter">US Letter (8.5 × 11 in)</option>
            <option value="legal">US Legal (8.5 × 14 in)</option>
            <option value="a3">A3 (297 × 420 mm)</option>
          </select>

          <div style="margin-bottom: 12px;">
            <label for="posterScale" class="section-desc">Scale Factor: {posterScale}x</label>
            <input id="posterScale" type="range" min="0.5" max="5.0" step="0.1" bind:value={posterScale} class="range-input" />
          </div>

          <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 12px;">
            <input id="posterGuides" type="checkbox" bind:checked={posterGuides} />
            <label for="posterGuides" class="section-desc" style="margin: 0; cursor: pointer;">Include Cut Guides & Overlap Margins</label>
          </div>
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Slices large documents or maps across a grid of printable sheets with alignment marks.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_stitch_canvas'}
        <div class="operation-config">
          <label class="section-desc">Stitching Direction:</label>
          <div style="display: flex; gap: 8px; margin-bottom: 12px;">
            <button
              type="button"
              class="form-button"
              style="flex: 1; padding: 6px 10px; font-size: 13px; {stitchDirection === 'vertical' ? 'background: #2563eb; color: white;' : 'background: #27272a; color: #a1a1aa;'}"
              onclick={() => stitchDirection = 'vertical'}
            >
              ⬇️ Vertical Banner
            </button>
            <button
              type="button"
              class="form-button"
              style="flex: 1; padding: 6px 10px; font-size: 13px; {stitchDirection === 'horizontal' ? 'background: #2563eb; color: white;' : 'background: #27272a; color: #a1a1aa;'}"
              onclick={() => stitchDirection = 'horizontal'}
            >
              ➡️ Horizontal Strip
            </button>
          </div>

          <div style="margin-bottom: 12px;">
            <label for="stitchSpacing" class="section-desc">Spacing Between Pages: {stitchSpacing} pt</label>
            <input id="stitchSpacing" type="number" min="0" max="100" step="1" bind:value={stitchSpacing} class="form-input" />
          </div>
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Stitches multiple consecutive pages end-to-end into a single continuous-flow canvas page.
          </p>
        </div>
      {:else if activeTool.id === 'bookmarks' || activeTool.id === 'pdf_bookmarks'}
        <div class="operation-config">
          <p class="section-desc" style="font-size: 0.85rem; color: #38bdf8; line-height: 1.4; margin-bottom: 8px;">
            📑 Extracts document outline bookmarks and TOC hierarchy into structured JSON format.
          </p>
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Outputs outline entries including title, destination page, and indentation level.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_attachments'}
        <div class="operation-config">
          <label class="section-desc">Action:</label>
          <div class="segmented" style="display: flex; gap: 8px; margin-bottom: 12px;">
            <button type="button" class="btn-segment" style="flex: 1; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--border-color); background: {attachmentAction === 'list' ? 'var(--accent-color)' : 'var(--bg-surface)'}; color: {attachmentAction === 'list' ? '#fff' : 'var(--text-primary)'}; font-size: 13px;" onclick={() => attachmentAction = 'list'}>📋 List</button>
            <button type="button" class="btn-segment" style="flex: 1; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--border-color); background: {attachmentAction === 'add' ? 'var(--accent-color)' : 'var(--bg-surface)'}; color: {attachmentAction === 'add' ? '#fff' : 'var(--text-primary)'}; font-size: 13px;" onclick={() => attachmentAction = 'add'}>➕ Add</button>
            <button type="button" class="btn-segment" style="flex: 1; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--border-color); background: {attachmentAction === 'extract' ? 'var(--accent-color)' : 'var(--bg-surface)'}; color: {attachmentAction === 'extract' ? '#fff' : 'var(--text-primary)'}; font-size: 13px;" onclick={() => attachmentAction = 'extract'}>📥 Extract</button>
            <button type="button" class="btn-segment" style="flex: 1; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--border-color); background: {attachmentAction === 'remove' ? 'var(--accent-color)' : 'var(--bg-surface)'}; color: {attachmentAction === 'remove' ? '#fff' : 'var(--text-primary)'}; font-size: 13px;" onclick={() => attachmentAction = 'remove'}>❌ Remove</button>
          </div>

          {#if attachmentAction !== 'list'}
            <label for="attachmentFileName" class="section-desc">File Name in Portfolio:</label>
            <input id="attachmentFileName" type="text" class="form-input" bind:value={attachmentFileName} placeholder="e.g. secret.txt" style="margin-bottom: 12px;" />
          {/if}

          {#if attachmentAction === 'add'}
            <label for="attachmentDescription" class="section-desc">Description (Optional):</label>
            <input id="attachmentDescription" type="text" class="form-input" bind:value={attachmentDescription} placeholder="Confidential report..." style="margin-bottom: 12px;" />

            <label for="attachmentFile" class="section-desc">File to Embed:</label>
            <input id="attachmentFile" type="file" style="margin-bottom: 12px; font-size: 13px;" onchange={async (e) => {
              const file = (e.target as HTMLInputElement).files?.[0];
              if (file) {
                 const buffer = await file.arrayBuffer();
                 const uint8 = new Uint8Array(buffer);
                 let binary = '';
                 for (let i = 0; i < uint8.byteLength; i++) {
                     binary += String.fromCharCode(uint8[i]);
                 }
                 attachmentBase64 = window.btoa(binary);
                 if (!attachmentFileName) attachmentFileName = file.name;
              }
            }} />
          {/if}
        </div>
      {:else if activeTool.id === 'pdf_auto_rotate'}
        <div class="operation-config">
          <label for="autoRotateTarget" class="section-desc">Target Upright Orientation:</label>
          <div style="display: flex; gap: 8px; margin-bottom: 12px;">
            <button
              type="button"
              class="form-button"
              style="flex: 1; padding: 6px 10px; font-size: 13px; {autoRotateOrientation === 'portrait' ? 'background: #2563eb; color: white;' : 'background: #27272a; color: #a1a1aa;'}"
              onclick={() => autoRotateOrientation = 'portrait'}
            >
              📄 Portrait (Default)
            </button>
            <button
              type="button"
              class="form-button"
              style="flex: 1; padding: 6px 10px; font-size: 13px; {autoRotateOrientation === 'landscape' ? 'background: #2563eb; color: white;' : 'background: #27272a; color: #a1a1aa;'}"
              onclick={() => autoRotateOrientation = 'landscape'}
            >
              🖼️ Landscape
            </button>
          </div>
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Heuristically inspects dimensions and reading baselines to automatically rotate sideways or upside-down pages into upright orientation.
          </p>
        </div>
      {:else if activeTool.id === 'burst' || activeTool.id === 'pdf_burst'}
        <div class="operation-config">
          <p class="section-desc" style="font-size: 0.82rem; color: #a1a1aa; line-height: 1.4;">
            Bursts every single page into an individual standalone PDF file saved into an output directory.
          </p>
        </div>
      {:else if activeTool.id === 'compare' || activeTool.id === 'pdf_compare'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Interactive comparison tool to visually inspect differences, slide between document revisions, or generate pixel discrepancy maps between two open PDFs.
          </p>
        </div>
      {:else if activeTool.id === 'annotate' || activeTool.id === 'pdf_annotate'}
        <div class="operation-config">
          <label for="annotationType" class="section-desc">Annotation Type:</label>
          <select id="annotationType" bind:value={annotationType} class="form-input">
            <option value="highlight">Highlight</option>
            <option value="underline">Underline</option>
            <option value="strikethrough">Strikethrough</option>
            <option value="note">Sticky Note</option>
            <option value="pen">Ink / Pen</option>
          </select>
          <span class="field-hint">PDF standard annotation dictionary /Subtype entry.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="annotationContent" class="section-desc">Note / Comment Content:</label>
          <input id="annotationContent" type="text" class="form-input" bind:value={annotationContent} placeholder="e.g. Legal review requested" />
          <span class="field-hint">Content text attached to the note /Contents attribute.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="annotationColor" class="section-desc">Color Hex:</label>
          <input id="annotationColor" type="text" class="form-input" bind:value={annotationColor} placeholder="#FFFF00" />
          <span class="field-hint">Hex color code formatted as #RRGGBB.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="annotationRect" class="section-desc">Bounding Coordinates (x,y,w,h):</label>
          <input id="annotationRect" type="text" class="form-input" bind:value={annotationRect} placeholder="50,50,150,20" />
          <span class="field-hint">Rectangular boundary [x, y, width, height] in PDF points.</span>
        </div>
      {:else if activeTool.id === 'redact' || activeTool.id === 'pdf_redact'}
        <div class="operation-config">
          <label for="redactQuery" class="section-desc">Search Query / Pattern to Redact:</label>
          <input id="redactQuery" type="text" class="form-input" bind:value={redactQuery} placeholder="e.g. SSN, CONFIDENTIAL, or regex pattern" style="margin-bottom: 8px;" />
          <div style="margin-top: 8px;">
            <label for="redactRect" class="section-desc">Redaction Box (x,y,w,h):</label>
            <input id="redactRect" type="text" class="form-input" bind:value={redactRect} placeholder="0,0,100,100" />
          </div>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1; margin-top: 8px;">
            Permanently redacts matching sensitive patterns, text glyphs, and rectangular zones from the document stream.
          </p>
          <span class="field-hint">Redacted text is permanently stripped and cannot be recovered from the output file.</span>
        </div>
      {:else if activeTool.id === 'metadata' || activeTool.id === 'pdf_metadata'}
        <div class="operation-config">
          <label for="metadataTitle" class="section-desc">Document Title:</label>
          <input id="metadataTitle" type="text" class="form-input" bind:value={metadataTitle} placeholder="e.g. Verified Release 2026" />
          <div style="margin-top: 10px;">
            <label for="metadataAuthor" class="section-desc">Author:</label>
            <input id="metadataAuthor" type="text" class="form-input" bind:value={metadataAuthor} placeholder="e.g. Author Name" />
          </div>
          <div style="margin-top: 10px;">
            <label for="metadataSubject" class="section-desc">Subject:</label>
            <input id="metadataSubject" type="text" class="form-input" bind:value={metadataSubject} placeholder="e.g. Document Subject" />
          </div>
          <div style="margin-top: 10px;">
            <label for="metadataKeywords" class="section-desc">Keywords (comma-separated):</label>
            <input id="metadataKeywords" type="text" class="form-input" bind:value={metadataKeywords} placeholder="e.g. report, annual, verified" />
          </div>
          <span class="field-hint">Updates the document /Info dictionary and XMP metadata stream.</span>
        </div>
      {:else if activeTool.id === 'sanitize' || activeTool.id === 'pdf_sanitize'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Deeply sanitizes the document by purging scripts, unreferenced objects, and hidden metadata streams.
          </p>
          <div style="margin-top: 10px; display: flex; flex-direction: column; gap: 8px;">
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: #e2e8f0; cursor: pointer;">
              <input id="sanitizeScripts" type="checkbox" bind:checked={sanitizeScripts} />
              <span>Strip all JavaScript actions and launch triggers</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: #e2e8f0; cursor: pointer;">
              <input id="sanitizeMetadata" type="checkbox" bind:checked={sanitizeMetadata} />
              <span>Remove identifying metadata streams</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: #e2e8f0; cursor: pointer;">
              <input id="sanitizeAttachments" type="checkbox" bind:checked={sanitizeAttachments} />
              <span>Remove embedded file attachments</span>
            </label>
          </div>
          <span class="field-hint">Guarantees zero active scripts or private tracking elements remain.</span>
        </div>
      {:else if activeTool.id === 'change_permissions' || activeTool.id === 'pdf_change_permissions'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Configure granular document permissions and access control flags.
          </p>
          <div style="margin-top: 10px; display: flex; flex-direction: column; gap: 8px;">
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: #e2e8f0; cursor: pointer;">
              <input id="allowPrinting" type="checkbox" bind:checked={allowPrinting} />
              <span>Allow high-resolution printing</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: #e2e8f0; cursor: pointer;">
              <input id="allowCopying" type="checkbox" bind:checked={allowCopying} />
              <span>Allow copying text and graphics</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: #e2e8f0; cursor: pointer;">
              <input id="allowModifying" type="checkbox" bind:checked={allowModifying} />
              <span>Allow document modifications</span>
            </label>
          </div>
          <span class="field-hint">Configures standard PDF security /P bitmask to enforce read-only policies.</span>
        </div>
      {:else if activeTool.id === 'timestamp' || activeTool.id === 'pdf_timestamp'}
        <div class="operation-config">
          <label for="tsaServerUrl" class="section-desc">RFC 3161 TSA Server URL:</label>
          <input id="tsaServerUrl" type="text" class="form-input" bind:value={tsaServerUrl} placeholder="http://timestamp.digicert.com" />
          <span class="field-hint">Applies an authentic RFC 3161 cryptographic timestamp token to ensure non-repudiation.</span>
        </div>
      {:else if activeTool.id === 'validate_signatures' || activeTool.id === 'pdf_validate_signatures'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Cryptographically verify digital signatures, certificate chains, and document byte-range integrity.
          </p>
          <span class="field-hint">Inspects all signature fields, PKCS#7 detached envelopes, and generates a validation audit report.</span>
        </div>
      {:else if activeTool.id === 'remove_signatures' || activeTool.id === 'pdf_remove_signatures'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Strip digital signature fields, cryptographic locks, and widget annotations to enable editing.
          </p>
          <span class="field-hint">Removes /Sig dictionaries and /ByteRange blocks from the document catalog, producing a clean unsigned PDF.</span>
        </div>
      {:else if activeTool.id === 'hash' || activeTool.id === 'pdf_hash'}
        <div class="operation-config">
          <label for="hashAlgorithm" class="section-desc">Checksum Algorithm:</label>
          <select id="hashAlgorithm" class="form-input" bind:value={hashAlgorithm}>
            <option value="sha256">SHA-256 (Secure Hash Algorithm 256-bit)</option>
            <option value="md5">MD5 (Message Digest 5 - 128-bit)</option>
          </select>
          <span class="field-hint">Streams file bytes into cryptographic digest engine for tamper-evident verification.</span>
        </div>
      {:else if activeTool.id === 'validate' || activeTool.id === 'pdf_validate'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Validate PDF syntax, dictionary structures, cross-reference tables, and ISO 32000 standards conformance.
          </p>
          <span class="field-hint">Scans xref tables, page trees, stream lengths, and encryption dictionary flags to report syntax conformance.</span>
        </div>
      {:else if activeTool.id === 'document_pii_scan' || activeTool.id === 'pii_scan' || activeTool.id === 'pdf_document_pii_scan'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1; margin-bottom: 8px;">
            Scan document for sensitive PII (SSN, credit cards, emails, phones, IBANs) and generate audit telemetry.
          </p>
          <div style="display: flex; flex-direction: column; gap: 6px;">
            <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; cursor: pointer;">
              <input type="checkbox" bind:checked={piiScanSsn} />
              <span>Scan Social Security Numbers (SSN)</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; cursor: pointer;">
              <input type="checkbox" bind:checked={piiScanCreditCards} />
              <span>Scan Credit Cards (Luhn validated)</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; cursor: pointer;">
              <input type="checkbox" bind:checked={piiScanEmails} />
              <span>Scan Email Addresses</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; cursor: pointer;">
              <input type="checkbox" bind:checked={piiScanPhones} />
              <span>Scan Phone Numbers</span>
            </label>
            <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; cursor: pointer;">
              <input type="checkbox" bind:checked={piiScanIban} />
              <span>Scan International Bank Account Numbers (IBAN)</span>
            </label>
          </div>
        </div>
      {:else if activeTool.id === 'flatten' || activeTool.id === 'pdf_flatten'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Permanently flatten interactive form fields, widget appearances, and annotations into static page graphics.
          </p>
          <span class="field-hint">Removes /AcroForm dictionaries; form fields become uneditable static page text.</span>
        </div>
      {:else if activeTool.id === 'read_form' || activeTool.id === 'pdf_read_form'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Extract all interactive AcroForm field values and widget definitions into structured JSON.
          </p>
          <span class="field-hint">Parses text fields, checkboxes, radio groups, and signature widgets across all pages.</span>
        </div>
      {:else if activeTool.id === 'fill_form' || activeTool.id === 'pdf_fill_form'}
        <div class="operation-config">
          <label for="formDataPath" class="section-desc">Form Data JSON Path:</label>
          <input id="formDataPath" type="text" class="form-input" bind:value={formDataPath} placeholder="e.g. tests/e2e_fixtures/real/form_data.json" />
          <span class="field-hint">JSON mapping field names to values (e.g. &#123;"ApplicantName": "Jane Doe"&#125;).</span>
        </div>
      {:else if activeTool.id === 'create_form_field' || activeTool.id === 'pdf_create_form_field'}
        <div class="operation-config">
          <label for="formFieldName" class="section-desc">Field Name:</label>
          <input id="formFieldName" type="text" class="form-input" bind:value={formFieldName} placeholder="ApplicantName" />
          <span class="field-hint">Unique identifier for the form widget annotation.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="formFieldType" class="section-desc">Field Type:</label>
          <select id="formFieldType" bind:value={formFieldType} class="form-input">
            <option value="text">Text Box (Tx)</option>
            <option value="checkbox">Checkbox (Btn)</option>
            <option value="signature">Signature (Sig)</option>
          </select>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="formFieldPage" class="section-desc">Target Page Number:</label>
          <input id="formFieldPage" type="number" class="form-input" bind:value={formFieldPage} min="1" />
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <label for="formFieldRect" class="section-desc">Bounding Rect (llx,lly,urx,ury):</label>
          <input id="formFieldRect" type="text" class="form-input" bind:value={formFieldRect} placeholder="50,50,200,80" />
          <span class="field-hint">Coordinates in points: lower-left X, lower-left Y, upper-right X, upper-right Y.</span>
        </div>
      {:else if activeTool.id === 'batch_fill' || activeTool.id === 'cli_fill_batch'}
        <div class="operation-config">
          <label for="batchCsvPath" class="section-desc">Dataset Path (.csv or .json):</label>
          <input id="batchCsvPath" type="text" class="form-input" bind:value={batchCsvPath} placeholder="e.g. tests/e2e_fixtures/real/batch_data.csv" />
          <span class="field-hint">Tabular dataset containing row records to generate individual filled documents.</span>
        </div>
        <div class="operation-config" style="margin-top: 10px;">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            High-speed automated AcroForm batch engine with machine-readable audit receipt generation.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_comic_archive' || activeTool.id === 'comic_archive'}
        <div class="operation-config">
          <label for="cbzArchiveName" class="section-desc">Archive File Name (Optional):</label>
          <input id="cbzArchiveName" type="text" class="form-input" bind:value={cbzArchiveName} placeholder="e.g. comic_volume_1.cbz" style="margin-bottom: 8px;" />
          <div style="margin-top: 8px;">
            <label for="conversionDpi" class="section-desc">Raster Resolution (DPI): {conversionDpi} DPI</label>
            <input id="conversionDpi" type="range" min="72" max="600" step="10" bind:value={conversionDpi} class="range-input" />
          </div>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1; margin-top: 8px;">
            Packages rendered sequential page image frames into a standard CBZ (ZIP) comic book archive container.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_to_epub' || activeTool.id === 'to_epub'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Transforms fixed-layout PDF pages into a valid reflowable EPUB 3.0 publication package with mimetype and container XML.
          </p>
          <span class="field-hint">Extracts text and logical structure into semantic XHTML documents inside an EPUB container.</span>
        </div>
      {:else if activeTool.id === 'pdf_to_pdf_a' || activeTool.id === 'to_pdf_a' || activeTool.id === 'pdf_a'}
        <div class="operation-config">
          <label for="pdfaConformance" class="section-desc">PDF/A Conformance Profile:</label>
          <select id="pdfaConformance" class="form-input" bind:value={pdfaConformance}>
            <option value="PDF/A-1b">PDF/A-1b (ISO 19005-1 Basic Conformance)</option>
            <option value="PDF/A-2b">PDF/A-2b (ISO 19005-2 Basic Conformance)</option>
            <option value="PDF/A-3b">PDF/A-3b (ISO 19005-3 with File Attachments)</option>
          </select>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1; margin-top: 8px;">
            Embeds GTS_PDFXVersion / OutputIntents conformance tags and verifies long-term archival suitability.
          </p>
        </div>
      {:else if activeTool.id === 'extract_images' || activeTool.id === 'pdf_extract_images'}
        <div class="operation-config">
          <label for="imageFormat" class="section-desc">Extracted Image Format:</label>
          <select id="imageFormat" class="form-input" bind:value={imageFormat}>
            <option value="png">PNG (Lossless Portable Network Graphics)</option>
            <option value="jpeg">JPEG (Compressed Joint Photographic Experts Group)</option>
          </select>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1; margin-top: 8px;">
            Extracts all embedded raster image XObjects from PDF page resources into individual image files.
          </p>
        </div>
      {:else if activeTool.id === 'pdf_to_docx'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Converts the active PDF into a fully editable Microsoft Word document (.docx) using an OpenXML
            paragraph-mapping engine. Text blocks, headings, and inline formatting are preserved as
            native Word paragraph styles.
          </p>
          <span class="field-hint">Output: <code>.docx</code> — compatible with Microsoft Word 2016 and later (OOXML ECMA-376).</span>
        </div>
      {:else if activeTool.id === 'pdf_to_xlsx'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Extracts tabular content from the active PDF and writes it into a Microsoft Excel spreadsheet
            (.xlsx). Each detected table region is mapped to a worksheet with its original row/column
            structure intact.
          </p>
          <span class="field-hint">Output: <code>.xlsx</code> — compatible with Microsoft Excel 2016 and later (Office Open XML).</span>
        </div>
      {:else if activeTool.id === 'pdf_to_pptx'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Converts each PDF page into a Microsoft PowerPoint slide (.pptx). Page graphics are rasterized
            at high DPI and embedded as full-bleed slide backgrounds, preserving the visual layout of the
            original document.
          </p>
          <span class="field-hint">Output: <code>.pptx</code> — compatible with Microsoft PowerPoint 2016 and later (ECMA-376 PresentationML).</span>
        </div>
      {:else if activeTool.id === 'pdf_to_html'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Converts the active PDF into a semantic HTML document. Text is extracted with positional
            layout awareness and wrapped in semantic heading and paragraph elements. Images are embedded
            as inline base-64 data URIs for single-file portability.
          </p>
          <span class="field-hint">Output: <code>.html</code> — self-contained responsive page renderable in any modern browser.</span>
        </div>
      {:else if activeTool.id === 'pdf_to_markdown' || activeTool.id === 'pdf_convert_markdown'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Converts the active PDF document into structured Markdown (.md). Heading hierarchies,
            lists, code blocks, and data tables are recognized and transcribed into clean
            CommonMark format for documentation and LLM prompts.
          </p>
          <span class="field-hint">Output: <code>.md</code> — standard Markdown text file.</span>
        </div>
      {:else if activeTool.id === 'pdf_to_json'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Extracts document content and structure into an organized JSON hierarchy. Captures per-page text blocks,
            bounding box coordinates, font metadata, and table structures for programmatic ingestion and analysis.
          </p>
          <span class="field-hint">Output: <code>.json</code> — structured JSON document data.</span>
        </div>
      {:else if activeTool.id === 'extract_text' || activeTool.id === 'pdf_extract_text'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Extracts all raw textual content from the active PDF in natural reading order. Decodes character
            glyphs using embedded font cmaps and normalizes unicode whitespace across pages.
          </p>
          <span class="field-hint">Output: <code>.txt</code> — clean UTF-8 plain text file.</span>
        </div>
      {:else if activeTool.id === 'pdf_extract_tables'}
        <div class="operation-config">
          <label for="tableFormat" class="section-desc">Extracted Table Format:</label>
          <select id="tableFormat" class="form-input" bind:value={tableFormat}>
            <option value="json">JSON (Structured records & cells)</option>
            <option value="csv">CSV (Comma-separated values)</option>
          </select>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1; margin-top: 8px;">
            Detects grid lines and cell alignments within PDF pages to extract structured tabular data into
            reusable JSON records or spreadsheet CSV rows.
          </p>
          <span class="field-hint">Output: <code>.{tableFormat}</code> — structured tabular data.</span>
        </div>
      {:else if activeTool.id === 'pdf_convert_excel' || activeTool.id === 'convert_excel'}
        <div class="operation-config">
          <label for="convertPreset" class="section-desc">Visual Table Styling Preset:</label>
          <select id="convertPreset" class="form-input" bind:value={convertPreset}>
            <option value="academic">Academic (Crisp borders, Serif headers)</option>
            <option value="corporate">Corporate (Modern blue banner, zebra striping)</option>
            <option value="minimalist">Minimalist (Clean borders, airy layout)</option>
          </select>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1; margin-top: 8px;">
            Parses tabular matrix cells, computes row bandings, and renders publication-ready PDF tables.
          </p>
        </div>
      {:else if activeTool.id === 'office_to_markdown'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Converts Microsoft Office documents (Word DOCX, PowerPoint PPTX, Excel XLSX) into structured Markdown (.md).
            Headings, bullet lists, tables, and formatted text paragraphs are recognized and transcribed into clean CommonMark.
          </p>
          <span class="field-hint">Output: <code>.md</code> — standard Markdown text file.</span>
        </div>
      {:else if activeTool.id === 'office_extract_text'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Rapidly extracts all plain text content from Microsoft Office documents (.docx, .xlsx, .pptx).
            Strips internal XML formatting tags and layout schemas while preserving document paragraphs in natural reading order.
          </p>
          <span class="field-hint">Output: <code>.txt</code> — clean UTF-8 plain text file.</span>
        </div>
      {:else if activeTool.id === 'office_modernize_legacy'}
        <div class="operation-config">
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Upgrades legacy binary Microsoft Office files (.doc, .xls, .ppt) to modern OpenXML formats (.docx, .xlsx, .pptx).
            Repairs vintage binary streams and modernizes file structure without requiring Microsoft Office installation.
          </p>
          <span class="field-hint">Output: <code>.docx</code> / <code>.xlsx</code> / <code>.pptx</code> — modern Office OpenXML container.</span>
        </div>
      {:else if activeTool.id === 'excel_to_json'}
        <div class="operation-config">
          <div class="form-group" style="margin-bottom: 10px;">
            <label for="excelSheet" class="section-desc">Target Worksheet (Optional):</label>
            <input
              id="excelSheet"
              type="text"
              class="form-input"
              placeholder="All sheets (or enter e.g. Sheet1)"
              bind:value={excelSheet}
            />
          </div>
          <div class="form-group" style="margin-bottom: 10px;">
            <label class="scope-option" style="display: flex; align-items: center; gap: 8px; cursor: pointer; color: #cbd5e1;">
              <input type="checkbox" bind:checked={hasHeaders} />
              <span>First row contains column headers</span>
            </label>
          </div>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Ingests Microsoft Excel spreadsheets (.xlsx, .xls) and generates structured JSON records.
            Maps worksheet rows to key-value objects or arrays suitable for programmatic ingestion and LLM context.
          </p>
          <span class="field-hint">Output: <code>.json</code> — structured JSON document data.</span>
        </div>
      {:else if activeTool.id === 'excel_to_csv'}
        <div class="operation-config">
          <div class="form-group" style="margin-bottom: 10px;">
            <label for="excelCsvSheet" class="section-desc">Target Worksheet (Optional):</label>
            <input
              id="excelCsvSheet"
              type="text"
              class="form-input"
              placeholder="First sheet (or enter e.g. Sheet1)"
              bind:value={excelSheet}
            />
          </div>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Exports Microsoft Excel spreadsheet worksheets (.xlsx, .xls) into standard RFC 4180 comma-separated values (CSV).
            Preserves tabular data grids, cell strings, and numeric records with proper quoting and UTF-8 encoding.
          </p>
          <span class="field-hint">Output: <code>.csv</code> — standard comma-separated values file.</span>
        </div>
      {:else if activeTool.id === 'workspace_search'}
        <div class="operation-config">
          <div class="form-group" style="margin-bottom: 10px;">
            <label for="workspaceSearchQueryInput" class="section-desc">Directory Search Query:</label>
            <input
              id="workspaceSearchQueryInput"
              type="text"
              class="form-input"
              bind:value={workspaceSearchQuery}
              placeholder="Query across all files in workspace..."
            />
          </div>
          <div class="form-group" style="margin-bottom: 10px;">
            <label for="workspaceSearchDirInput" class="section-desc">Target Directory (Optional):</label>
            <input
              id="workspaceSearchDirInput"
              type="text"
              class="form-input"
              bind:value={workspaceSearchDir}
              placeholder="Leave empty for current document directory"
            />
          </div>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Recursively scans PDF, Word, Excel, and Markdown documents across your workspace directory, returning matched files and page locations.
          </p>
          <span class="field-hint">Output: <code>.json</code> — multi-document search matches and page indices.</span>
        </div>
      {:else if activeTool.id === 'semantic_search'}
        <div class="operation-config">
          <div class="form-group" style="margin-bottom: 10px;">
            <label for="semanticSearchQueryInput" class="section-desc">Semantic Concept Query:</label>
            <input
              id="semanticSearchQueryInput"
              type="text"
              class="form-input"
              bind:value={semanticSearchQuery}
              placeholder="e.g. financial liability clauses, termination policies..."
            />
          </div>
          <div class="form-group" style="margin-bottom: 10px;">
            <label for="semanticSearchLimitInput" class="section-desc">Max Results: {semanticSearchLimit}</label>
            <input
              id="semanticSearchLimitInput"
              type="range"
              min="1"
              max="20"
              class="range-input"
              bind:value={semanticSearchLimit}
            />
          </div>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Performs neural vector embedding similarity search across indexed document segments, returning conceptually related passages beyond literal keyword matching.
          </p>
          <span class="field-hint">Output: <code>.json</code> — ranked semantic matches with similarity scores.</span>
        </div>
      {:else if activeTool.id === 'pdf_classify_type' || activeTool.id === 'classify_type' || activeTool.id === 'classify'}
        <div class="operation-config">
          <div class="form-group" style="margin-bottom: 10px;">
            <label class="scope-option" style="display: flex; align-items: center; gap: 8px; cursor: pointer; color: #cbd5e1;">
              <input type="checkbox" bind:checked={classifyDetail} />
              <span>Deep structural and font feature analysis</span>
            </label>
          </div>
          <p class="section-desc" style="line-height: 1.5; color: #cbd5e1;">
            Analyzes document layout, typography, metadata, and form streams to classify the document type (e.g. invoice, contract, academic paper, presentation, form, scanned image) with confidence score.
          </p>
          <span class="field-hint">Output: <code>.json</code> — document classification category, features, and confidence rating.</span>
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

      {#if activeTool && activeTool.id !== 'merge' && activeTool.id !== 'pdf_merge' && activeTool.id !== 'compare' && activeTool.id !== 'pdf_compare' && activeTool.id !== 'img_to_pdf' && activeTool.id !== 'pdf_images_to_pdf' && activeTool.id !== 'search' && !((activeTool.id === 'ocr' || activeTool.id === 'pdf_ocr') && ocrOutputFormat === 'clipboard') && !(applyToAllFiles && activeTool.rule?.isBatchable && appState.documents.length > 1)}
        <div class="operation-config" id="output-path-config">
          <label for="output-path-input" class="section-desc">
            {activeTool.id === 'split' || activeTool.id === 'pdf_split' || activeTool.id === 'burst' || activeTool.id === 'pdf_burst' ? 'Output folder:' : 'Output file:'}
          </label>
          <div class="output-path-row">
            <input
              id="output-path-input"
              type="text"
              class="form-input output-path-input"
              bind:value={customOutputPath}
              placeholder={activeTool.id === 'split' || activeTool.id === 'pdf_split' || activeTool.id === 'burst' || activeTool.id === 'pdf_burst' ? 'Output folder path...' : 'Output file path...'}
              title="Edit the output path"
            />
            <button
              type="button"
              class="output-browse-btn"
              title={activeTool.id === 'split' || activeTool.id === 'pdf_split' || activeTool.id === 'burst' || activeTool.id === 'pdf_burst' ? 'Browse destination folder' : 'Browse destination file'}
              onclick={handleBrowseOutputPath}
            >
              📁 Browse
            </button>
          </div>
        </div>
      {/if}

      <div class="action-area">
        <button
          class="run-btn"
          class:run-btn--loading={isRunning}
          id="btn-run-operation"
          disabled={isRunning || !activeTool || (!['md_to_pdf', 'html_to_pdf', 'img_to_pdf', 'pdf_images_to_pdf', 'json_to_pdf', 'pdf_convert_excel', 'convert_excel', 'excel_to_pdf'].includes(activeTool.id) && appState.documents.length === 0) || ((activeTool.id === 'ocr' || activeTool.id === 'pdf_ocr') && !isOcrDownloaded)}
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
          {#if lastExecution.success && lastExecution.report}
            <AuditReportViewer title={lastExecution.toolTitle} report={lastExecution.report} />
          {/if}
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
      
      {#if appState.isMobile}
        <div class="mobile-quick-actions" style="margin-top: 16px; margin-bottom: 16px;">
          <h4 style="margin: 0 0 10px 16px; font-size: 13px; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.05em;">⚡ Quick Actions</h4>
          <div class="quick-actions-grid" style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px; padding: 0 16px;">
            {#each [
              { id: 'img_to_pdf', title: 'Scan', icon: '📷', tool: allTools.find(t => t.id === 'img_to_pdf') },
              { id: 'sign', title: 'Sign', icon: '✍️', tool: allTools.find(t => t.id === 'sign') },
              { id: 'compress', title: 'Compress', icon: '⚡', tool: allTools.find(t => t.id === 'compress') },
              { id: 'pdf_sanitize', title: 'Sanitize', icon: '🧹', tool: allTools.find(t => t.id === 'document_pii_scan') || allTools.find(t => t.id === 'pdf_sanitize') }
            ] as action}
              {#if action.tool}
                <button class="quick-action-card" onclick={() => selectTool(action.tool as import('$lib/types/tools').ToolDefinition)} style="background: var(--bg-surface); border: 1px solid var(--border-color); border-radius: 12px; padding: 16px 12px; display: flex; flex-direction: column; align-items: center; gap: 8px;">
                  <span style="font-size: 24px;">{action.icon}</span>
                  <span style="font-size: 13px; font-weight: 500; color: var(--text-primary);">{action.title}</span>
                </button>
              {/if}
            {/each}
          </div>
        </div>
      {/if}

      <!-- Context Filter Chip Bar -->
      <div class="context-bar" role="tablist" aria-label="Filter context">
        {#each [
          { id: 'recommended', label: `⚡ Recommended (${ranked.recommended.length})` },
          { id: 'ready', label: `✅ Ready (${readyToolsCount})` },
          { id: 'all', label: `All Tools (${uniqueAllTools.length})` },
          { id: 'pdf', label: 'PDF' },
          { id: 'office', label: 'Office' },
          { id: 'ai', label: 'AI & OCR' }
        ] as ctx (ctx.id)}
          <button
            class="context-btn {ctx.id === 'ready' ? 'context-btn-ready' : ''}"
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
                    {#if tool.releaseStatus === 'v0.5-core'}
                      <span class="status-badge-v1">v0.5 Core</span>
                    {:else if tool.releaseStatus === 'v0.6-core'}
                      <span class="status-badge-v06">v0.6 Core</span>
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
                          {#if tool.releaseStatus === 'v0.5-core'}
                            <span class="status-badge-v1">v0.5 Core</span>
                          {:else if tool.releaseStatus === 'v0.6-core'}
                            <span class="status-badge-v06">v0.6 Core</span>
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
  .code-export-btn {
    margin-left: auto;
    background: var(--bg-surface, #1e1e24);
    border: 1px solid var(--border-color, #2a2a35);
    color: var(--text-primary, #ffffff);
    padding: 4px 10px;
    border-radius: var(--border-radius-sm, 4px);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s ease;
  }

  .code-export-btn:hover {
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.1));
    border-color: var(--accent-primary, #5e6ad2);
    color: var(--accent-primary, #5e6ad2);
  }
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

  .operations-panel.embedded {
    width: 100% !important;
    min-width: 0 !important;
    max-width: 100% !important;
    border-left: none;
    background-color: transparent;
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
  .context-btn-ready {
    border-color: rgba(34, 197, 94, 0.3);
    color: #4ade80;
  }
  .context-btn-ready.active {
    background: #16a34a;
    border-color: #16a34a;
    color: #ffffff;
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

  .tool-format-badges {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 2px;
  }

  .format-badge {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 3px 8px;
    border-radius: 5px;
    font-size: 11px;
    font-weight: 500;
  }

  .format-badge.input-badge {
    background: rgba(59, 130, 246, 0.12);
    border: 1px solid rgba(59, 130, 246, 0.3);
    color: #93c5fd;
  }

  .format-badge.output-badge {
    background: rgba(16, 185, 129, 0.12);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: #6ee7b7;
  }

  .badge-label {
    text-transform: uppercase;
    font-size: 9px;
    font-weight: 700;
    opacity: 0.8;
    letter-spacing: 0.05em;
  }

  .badge-value {
    font-family: var(--font-mono, monospace);
    font-weight: 600;
  }

  .badge-arrow {
    color: var(--text-muted, #71717a);
    font-size: 12px;
  }

  .tool-detailed-desc {
    font-size: 11.5px;
    color: var(--text-muted, #94a3b8);
    line-height: 1.45;
  }

  .destructive-warning-callout {
    background: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.35);
    border-radius: var(--border-radius-md, 6px);
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .warning-callout-header {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .warning-callout-icon {
    font-size: 14px;
  }

  .warning-callout-title {
    font-size: 12px;
    font-weight: 600;
    color: #f87171;
  }

  .warning-callout-text {
    font-size: 11px;
    color: #fca5a5;
    line-height: 1.4;
    margin: 0;
  }

  .field-hint {
    display: block;
    font-size: 11px;
    color: var(--text-muted, #94a3b8);
    margin-top: 4px;
    line-height: 1.35;
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
    color-scheme: dark;
  }

  select.form-input option {
    background-color: var(--bg-surface, #1e1e24);
    color: var(--text-primary, #ffffff);
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

  .output-browse-btn {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 6px 12px;
    font-size: 11px;
    font-weight: 500;
    color: var(--text-muted, #9ca3af);
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.05));
    border: 1px solid var(--border-color, #2a2a35);
    border-radius: 6px;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s ease;
  }

  .output-browse-btn:hover {
    color: var(--text-primary, #ffffff);
    background: var(--bg-surface-active, rgba(255, 255, 255, 0.1));
    border-color: var(--accent-primary, #5e6ad2);
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
  .status-badge-v06 {
    display: inline-block;
    padding: 1px 6px;
    border-radius: 9999px;
    font-size: 8.5px;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.35);
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
