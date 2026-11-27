<script lang="ts">
  import { onMount } from 'svelte';
  import { wasmPdfClient } from '../lib/wasm';
  import { ALL_101_TOOLS } from '../lib/toolsCatalog';
  import DropZone from '../components/DropZone.svelte';

  let activeTool = $state('merge');
  let isProcessing = $state(false);

  $effect(() => {
    if (typeof window !== 'undefined' && activeTool) {
      history.replaceState(null, '', '?tool=' + activeTool);
    }
  });

  onMount(() => {
    const toolParam = new URLSearchParams(window.location.search).get('tool');
    if (toolParam) {
      let matched = tools.find(t => t.id === toolParam);
      if (!matched) {
        const catalogEntry = ALL_101_TOOLS.find(t => t.id === toolParam);
        if (catalogEntry?.wasmId) {
          matched = tools.find(t => t.id === catalogEntry.wasmId);
        }
      }
      if (matched) {
        activeTool = matched.id;
      }
    }

    const handleSelectTool = (e: CustomEvent<{ toolId: string }>) => {
      if (e.detail?.toolId) {
        activeTool = e.detail.toolId;
        // Reset category filter if tool is not in current filter
        const target = tools.find(t => t.id === e.detail.toolId);
        if (target && selectedCategory !== 'all' && target.category !== selectedCategory) {
          selectedCategory = 'all';
        }
      }
    };
    window.addEventListener('select-tool' as any, handleSelectTool);
    return () => {
      window.removeEventListener('select-tool' as any, handleSelectTool);
    };
  });

  interface WebTool {
    id: string;
    title: string;
    icon: string;
    category: 'pages' | 'edit' | 'optimize' | 'security' | 'convert';
    description?: string;
    releaseStatus: 'v0.5-core' | 'preview';
  }

  // Tools configuration
  const tools: WebTool[] = [
    // Pages & Imposition
    { id: 'merge', title: 'Merge PDFs', icon: '📑', category: 'pages', releaseStatus: 'v0.5-core' },
    { id: 'split', title: 'Split PDF', icon: '✂️', category: 'pages', releaseStatus: 'v0.5-core' },
    { id: 'rotate', title: 'Rotate Pages', icon: '🔄', category: 'pages', releaseStatus: 'v0.5-core' },
    { id: 'delete', title: 'Delete Pages', icon: '🗑️', category: 'pages', releaseStatus: 'v0.5-core' },
    { id: 'extract', title: 'Extract Pages', icon: '📤', category: 'pages', releaseStatus: 'v0.5-core' },
    { id: 'reorder', title: 'Reorder Pages', icon: '🔀', category: 'pages', releaseStatus: 'v0.5-core' },
    { id: 'crop', title: 'Crop PDF', icon: '📐', category: 'pages', releaseStatus: 'preview' },
    { id: 'pdf_nup', title: 'N-Up', icon: '🎛️', category: 'pages', releaseStatus: 'preview' },
    { id: 'pdf_booklet', title: 'Booklet', icon: '📖', category: 'pages', releaseStatus: 'preview' },
    { id: 'pdf_scale_pages', title: 'Scale Pages', icon: '📏', category: 'pages', releaseStatus: 'preview' },
    { id: 'pdf_auto_rotate', title: 'Auto Rotate', icon: '📐', category: 'pages', releaseStatus: 'preview' },

    // Edit & Stationery
    { id: 'watermark', title: 'Watermark', icon: '©️', category: 'edit', releaseStatus: 'v0.5-core' },
    { id: 'flatten', title: 'Flatten Forms', icon: '📄', category: 'edit', releaseStatus: 'preview' },
    { id: 'metadata', title: 'Edit Metadata', icon: '🏷️', category: 'edit', releaseStatus: 'preview' },
    { id: 'header_footer', title: 'Header & Footer', icon: '📝', category: 'edit', releaseStatus: 'preview' },
    { id: 'page_numbers', title: 'Page Numbers', icon: '🔢', category: 'edit', releaseStatus: 'preview' },
    { id: 'pdf_stamp', title: 'Add Stamp', icon: '💮', category: 'edit', releaseStatus: 'preview' },
    { id: 'pdf_overlay_underlay', title: 'Overlay/Underlay', icon: '🥪', category: 'edit', releaseStatus: 'preview' },

    // Optimize
    { id: 'compress', title: 'Compress PDF', icon: '🗜️', category: 'optimize', releaseStatus: 'v0.5-core' },
    { id: 'pdf_hash', title: 'Hash PDF', icon: '#️⃣', category: 'optimize', releaseStatus: 'preview' },
    { id: 'pdf_info', title: 'PDF Info', icon: 'ℹ️', category: 'optimize', releaseStatus: 'preview' },

    // Security
    { id: 'encrypt', title: 'Encrypt PDF', icon: '🔒', category: 'security', releaseStatus: 'preview' },
    { id: 'decrypt', title: 'Decrypt PDF', icon: '🔓', category: 'security', releaseStatus: 'preview' },
    { id: 'sanitize', title: 'Sanitize', icon: '🧹', category: 'security', releaseStatus: 'preview' },
    { id: 'pii_scan', title: 'PII Scan', icon: '🔍', category: 'security', releaseStatus: 'preview' },
    { id: 'pdf_validate_signatures', title: 'Validate Sigs', icon: '✅', category: 'security', releaseStatus: 'preview' },
    { id: 'pdf_remove_signatures', title: 'Remove Sigs', icon: '❌', category: 'security', releaseStatus: 'preview' },

    // Convert
    { id: 'images_to_pdf', title: 'Images to PDF', icon: '🖼️', category: 'convert', releaseStatus: 'preview' },
    { id: 'extract_images', title: 'Extract Images', icon: '📸', category: 'convert', releaseStatus: 'preview' },
    { id: 'extract_text', title: 'Extract Text', icon: '📝', category: 'convert', releaseStatus: 'preview' },
    { id: 'render_page', title: 'Render Page', icon: '🖼️', category: 'convert', releaseStatus: 'preview' },
    { id: 'json_to_pdf', title: 'JSON to PDF', icon: '📜', category: 'convert', releaseStatus: 'preview' },
    { id: 'pdf_to_docx', title: 'PDF to DOCX', icon: '📝', category: 'convert', releaseStatus: 'preview' },
    { id: 'pdf_to_gif', title: 'PDF to Animated GIF', icon: '🎞️', category: 'convert', releaseStatus: 'preview', description: 'Convert multi-page PDF into an animated looping GIF preview' },
    { id: 'ocr', title: 'OCR PDF', icon: '👁️', category: 'convert', releaseStatus: 'preview' },

    // Additional Core & Preview Operations
    { id: 'pdf_stitch_canvas', title: 'Stitch Canvas', icon: '🖼️', category: 'pages', releaseStatus: 'preview', description: 'Stitch multi-page PDF into a single continuous vertical or horizontal strip' },
    { id: 'pdf_split_scanned_book', title: 'Split Scanned Book', icon: '📖', category: 'pages', releaseStatus: 'preview', description: 'Split two-page scanned book spreads down the center gutter into individual pages' },
    { id: 'pdf_split_by_outline', title: 'Split by Outline', icon: '📑', category: 'pages', releaseStatus: 'preview', description: 'Split document into chapters or sections based on its outline/table of contents' },
    { id: 'pdf_merge_alternate', title: 'Alternate Merge', icon: '🔀', category: 'pages', releaseStatus: 'preview', description: 'Interleave pages alternately from two documents (e.g. duplex scanner front/back)' },
    { id: 'pdf_posterize', title: 'Posterize Print', icon: '🪟', category: 'pages', releaseStatus: 'preview', description: 'Tile a large single PDF page across a multi-sheet grid for poster printing' },

    { id: 'pdf_attachments', title: 'Attachments', icon: '📎', category: 'edit', releaseStatus: 'preview', description: 'View, extract, add, or remove embedded PDF file attachments' },
    { id: 'pdf_edit_text', title: 'Edit PDF Text', icon: '✏️', category: 'edit', releaseStatus: 'preview', description: 'Find and replace text typos in-place inside PDF content streams' },

    { id: 'pdf_dedup_streams', title: 'Dedup Streams', icon: '🗜️', category: 'optimize', releaseStatus: 'preview', description: 'Deduplicate identical font, image, and content streams to optimize file size' },

    { id: 'pdf_timestamp', title: 'Cryptographic Timestamp', icon: '⏱️', category: 'security', releaseStatus: 'preview', description: 'Apply cryptographic document timestamping to prove existence at a specific time' },
    { id: 'pdf_change_permissions', title: 'Permissions', icon: '🛡️', category: 'security', releaseStatus: 'preview', description: 'Configure fine-grained PDF permission flags (printing, copying, annotations)' },
    { id: 'img_redact', title: 'Redact Image', icon: '⬛', category: 'security', releaseStatus: 'preview', description: 'Burn permanent opaque redaction boxes over sensitive image regions' },

    { id: 'pdf_to_epub', title: 'PDF to EPUB', icon: '📚', category: 'convert', releaseStatus: 'preview', description: 'Convert PDF text and structure into a reflowable EPUB eBook format' },
    { id: 'pdf_extract_patterns', title: 'Extract Patterns', icon: '🔎', category: 'convert', releaseStatus: 'preview', description: 'Extract structured data, regex patterns (emails, URLs, phones), and keywords' },
    { id: 'pdf_comic_archive', title: 'Comic Archive (CBZ)', icon: '📙', category: 'convert', releaseStatus: 'preview', description: 'Convert PDF pages into an archived Comic Book ZIP (CBZ) format' },
    { id: 'showcase_gif', title: 'Showcase GIF', icon: '✨', category: 'convert', releaseStatus: 'preview', description: 'Generate an animated showcase demo GIF with smooth transitions' },
    { id: 'images_to_gif', title: 'Images to Animated GIF', icon: '🎞️', category: 'convert', releaseStatus: 'preview', description: 'Combine multiple photos or drawings into a looping animated GIF' },
    { id: 'img_optimize_for_llm', title: 'Optimize for Vision LLM', icon: '🤖', category: 'convert', releaseStatus: 'preview', description: 'Sharpen document text, auto-downscale, and optimize image for Vision LLM input' },
    { id: 'img_deskew', title: 'Deskew Image', icon: '📐', category: 'convert', releaseStatus: 'preview', description: 'Automatically detect and correct rotational skew in scanned document images' },
    { id: 'img_convert', title: 'Convert Image', icon: '🔄', category: 'convert', releaseStatus: 'preview', description: 'Convert image formats between PNG, JPEG, WebP, BMP, and TIFF' },
    { id: 'img_crop_to_content', title: 'Crop to Content', icon: '✂️', category: 'convert', releaseStatus: 'preview', description: 'Automatically trim empty borders and margins around visual content' }
  ];

  // Tool specific states (batch array support)
  let mergeFiles: File[] = $state([]);
  let splitFiles: File[] = $state([]);
  let splitRanges = $state('');

  let rotateFiles: File[] = $state([]);
  let rotateAngle = $state(90);
  let rotatePages = $state('all');

  let compressFiles: File[] = $state([]);

  let encryptFiles: File[] = $state([]);
  let encryptPassword = $state('');

  let watermarkFiles: File[] = $state([]);
  let watermarkText = $state('');

  let deleteFiles: File[] = $state([]);
  let deletePagesStr = $state('');

  let extractFiles: File[] = $state([]);
  let extractPagesStr = $state('');

  let reorderFiles: File[] = $state([]);
  let reorderPagesStr = $state('');

  let cropFile: File | null = $state(null);
  let cropLeft = $state(0);
  let cropBottom = $state(0);
  let cropRight = $state(0);
  let cropTop = $state(0);

  let flattenFile: File | null = $state(null);

let metadataFile: File | null = $state(null);
  let metaTitle = $state('');
  let metaAuthor = $state('');
  let metaSubject = $state('');
  let metaKeywords = $state('');

  let imagesToPdfFiles: File[] = $state([]);
  let extractImagesFile: File | null = $state(null);
  let pdfHashFile: File | null = $state(null);
  let computedPdfHash: string | null = $state(null);

  let renderFile: File | null = $state(null);
  let renderPageIndex: number = $state(0);
  let renderScale: number = $state(2.0);
  let renderOutputUrl: string | null = $state(null);

  let extractTextFile: File | null = $state(null);
  let extractedText: string = $state("");

  let decryptFile: File | null = $state(null);
  let decryptPassword: string = $state("");

  let pageNumbersFile: File | null = $state(null);
  let pageNumbersFormat: string = $state("Page {n} of {total}");
  let pageNumbersPosition: string = $state("bottom-center");

  let headerFooterFile: File | null = $state(null);
  let headerText: string = $state("");
  let footerText: string = $state("");

  let pdfInfoFile: File | null = $state(null);
  let pdfInfoData: string = $state("");

  let ocrFile: File | null = $state(null);
  let ocrData: string = $state("");

  // New Tool States
  let nupFile: File | null = $state(null);
  let nupGrid: number = $state(2);
  let nupOrdering: string = $state('horizontal');

  let bookletFile: File | null = $state(null);
  let bookletPageFormat: string = $state('a4');

  let scalePagesFile: File | null = $state(null);
  let scalePagesTarget: string = $state('a4');
  let scalePagesPercentage: number = $state(100);

  let autoRotateFile: File | null = $state(null);
  let autoRotateOrientation: string = $state('portrait');

  let stampFile: File | null = $state(null);
  let stampImageFile: File | null = $state(null);
  let stampPosition: string = $state('bottom-right');

  let overlayUnderlayFile: File | null = $state(null);
  let overlayUnderlaySecondaryFile: File | null = $state(null);
  let overlayUnderlayMode: string = $state('overlay');

  let removeSignaturesFile: File | null = $state(null);

  let validateSignaturesFile: File | null = $state(null);
  let validateSignaturesBaselineFile: File | null = $state(null);
  let validateSignaturesResult: string | null = $state(null);

  let sanitizeFile: File | null = $state(null);
  let piiScanFile: File | null = $state(null);
  let piiScanResult: string | null = $state(null);
  let pdfToDocxFile: File | null = $state(null);
  let pdfToGifFile: File | null = $state(null);
  let gifDelay = $state(100);
  let gifWidth = $state(800);
  let gifMaxPages = $state<number | undefined>(undefined);

  // New tool states
  let stitchCanvasFile: File | null = $state(null);
  let stitchDirection = $state('vertical');
  let stitchSpacing = $state(0);

  let splitScannedBookFile: File | null = $state(null);
  let scannedBookReadingDirection = $state('ltr');

  let splitByOutlineFile: File | null = $state(null);
  let splitOutlineMaxDepth = $state(1);

  let mergeAlternatePrimary: File | null = $state(null);
  let mergeAlternateSecondary: File | null = $state(null);

  let posterizeFile: File | null = $state(null);
  let posterizeCols = $state(2);
  let posterizeRows = $state(2);

  let attachmentsFile: File | null = $state(null);
  let dedupStreamsFile: File | null = $state(null);
  let timestampFile: File | null = $state(null);

  let changePermissionsFile: File | null = $state(null);
  let allowPrinting = $state(true);
  let allowCopying = $state(false);
  let allowModifying = $state(false);

  let toEpubFile: File | null = $state(null);
  let epubTitle = $state('');
  let epubAuthor = $state('');

  let extractPatternsFile: File | null = $state(null);
  let patternsResult: string | null = $state(null);

  let comicArchiveFile: File | null = $state(null);
  let comicDpi = $state(150);

  let showcaseGifFile: File | null = $state(null);
  let showcaseGifFps = $state(4);
  let showcaseGifWidth = $state(800);

  let imagesToGifFiles: File[] = $state([]);
  let imagesToGifFps = $state(4);
  let imagesToGifWidth = $state(800);

  let optimizeLlmFile: File | null = $state(null);
  let optimizeLlmMaxDim = $state(1024);

  let deskewFile: File | null = $state(null);

  let convertImageFile: File | null = $state(null);
  let convertImageFormat = $state('webp');
  let convertImageQuality = $state(90);

  let cropContentFile: File | null = $state(null);
  let cropPadding = $state(10);

  let redactImageFile: File | null = $state(null);
  let redactBoxX = $state(50);
  let redactBoxY = $state(50);
  let redactBoxW = $state(200);
  let redactBoxH = $state(100);

  let editTextFile: File | null = $state(null);
  let editTextFind = $state('');
  let editTextReplace = $state('');
  let editTextPage = $state<number | string | null | undefined>('');
  let editTextOccurrence = $state<number | string | null | undefined>('');
  let editTextMatchCase = $state(true);


  async function readAsUint8Array(file: File): Promise<Uint8Array> {
      const buffer = await file.arrayBuffer();
      return new Uint8Array(buffer);
  }

  function download(data: Uint8Array, filename: string) {
      const blob = new Blob([data.buffer as ArrayBuffer], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
  }

  // Handlers
  async function handleMerge() {
      if (mergeFiles.length < 2) return;
      isProcessing = true;
      try {
          const buffers = await Promise.all(mergeFiles.map(f => readAsUint8Array(f)));
          const result = await wasmPdfClient.merge(buffers);
          download(result, 'merged.pdf');
      } catch (e: any) {
          alert('Merge failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleSplit() {
      if (splitFiles.length === 0 || !splitRanges) return;
      isProcessing = true;
      try {
          for (let fIdx = 0; fIdx < splitFiles.length; fIdx++) {
              const file = splitFiles[fIdx];
              const buffer = await readAsUint8Array(file);
              const results = await wasmPdfClient.split(buffer, splitRanges);
              results.forEach((res: Uint8Array, i: number) => {
                  setTimeout(() => {
                      download(res, `split_part_${i+1}_${file.name}`);
                  }, (fIdx * results.length + i) * 200);
              });
          }
      } catch (e: any) {
          alert('Split failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleRotate() {
      if (rotateFiles.length === 0) return;
      isProcessing = true;
      try {
          for (let i = 0; i < rotateFiles.length; i++) {
              const file = rotateFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.rotate(buffer, rotateAngle, rotatePages);
              setTimeout(() => {
                  download(result, `rotated_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Rotate failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleCompress() {
      if (compressFiles.length === 0) return;
      isProcessing = true;
      try {
          for (let i = 0; i < compressFiles.length; i++) {
              const file = compressFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.compress(buffer);
              setTimeout(() => {
                  download(result, `compressed_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Compress failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleEncrypt() {
      if (encryptFiles.length === 0 || !encryptPassword) return;
      isProcessing = true;
      try {
          for (let i = 0; i < encryptFiles.length; i++) {
              const file = encryptFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.encrypt(buffer, encryptPassword);
              setTimeout(() => {
                  download(result, `encrypted_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Encrypt failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleWatermark() {
      if (watermarkFiles.length === 0 || !watermarkText) return;
      isProcessing = true;
      try {
          for (let i = 0; i < watermarkFiles.length; i++) {
              const file = watermarkFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.watermark(buffer, watermarkText);
              setTimeout(() => {
                  download(result, `watermarked_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Watermark failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleDelete() {
      if (deleteFiles.length === 0 || !deletePagesStr) return;
      isProcessing = true;
      try {
          for (let i = 0; i < deleteFiles.length; i++) {
              const file = deleteFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.delete_pages(buffer, deletePagesStr);
              setTimeout(() => {
                  download(result, `deleted_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Delete failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleExtract() {
      if (extractFiles.length === 0 || !extractPagesStr) return;
      isProcessing = true;
      try {
          for (let i = 0; i < extractFiles.length; i++) {
              const file = extractFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.extract_pages(buffer, extractPagesStr);
              setTimeout(() => {
                  download(result, `extracted_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Extract failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleReorder() {
      if (reorderFiles.length === 0 || !reorderPagesStr) return;
      isProcessing = true;
      try {
          const orderArr = reorderPagesStr.split(',').map(s => parseInt(s.trim())).filter(n => !isNaN(n));
          for (let i = 0; i < reorderFiles.length; i++) {
              const file = reorderFiles[i];
              const buffer = await readAsUint8Array(file);
              const result = await wasmPdfClient.reorder_pages(buffer, orderArr);
              setTimeout(() => {
                  download(result, `reordered_${file.name}`);
              }, i * 250);
          }
      } catch (e: any) {
          alert('Reorder failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleCrop() {
      if (!cropFile) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(cropFile);
          const result = await wasmPdfClient.crop(buffer, cropLeft, cropBottom, cropRight, cropTop);
          download(result, 'cropped.pdf');
      } catch (e: any) {
          alert('Crop failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }

  async function handleFlatten() {
      if (!flattenFile) return;
      isProcessing = true;
      try {
          const buffer = await readAsUint8Array(flattenFile);
          const result = await wasmPdfClient.flatten(buffer);
          download(result, 'flattened.pdf');
      } catch (e: any) {
          alert('Flatten failed: ' + e.message);
      } finally {
          isProcessing = false;
      }
  }


  async function handleMetadata() {
    if (!metadataFile) return;
    try {
      isProcessing = true;
      const buffer = new Uint8Array(await metadataFile.arrayBuffer());
      const result = await wasmPdfClient.set_metadata(buffer, metaTitle, metaAuthor, metaSubject, metaKeywords);
      download(result, `metadata_${metadataFile.name}`);
    } catch (e) {
      alert(`Error updating metadata: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleImagesToPdf() {
    if (imagesToPdfFiles.length === 0) return;
    try {
      isProcessing = true;
      const buffers = await Promise.all(imagesToPdfFiles.map(async (f) => new Uint8Array(await f.arrayBuffer())));
      const result = await wasmPdfClient.images_to_pdf(buffers);
      download(result, `images_to_pdf.pdf`);
    } catch (e) {
      alert(`Error converting images: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleExtractImages() {
    if (!extractImagesFile) return;
    try {
      isProcessing = true;
      const buffer = new Uint8Array(await extractImagesFile.arrayBuffer());
      const result = await wasmPdfClient.extract_images(buffer);
      for (let i = 0; i < result.length; i++) {
        const ext = (result[i][0] === 0xFF && result[i][1] === 0xD8) ? 'jpg' : 'png';
        download(result[i], `image_${i + 1}.${ext}`);
      }
    } catch (e) {
      alert(`Error extracting images: ${e}`);
    } finally {
      isProcessing = false;
    }
  }


  async function handleRenderPage() {
    if (!renderFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await renderFile.arrayBuffer());
      const resBytes = await wasmPdfClient.render_page(bytes, renderPageIndex, renderScale);
      const blob = new Blob([resBytes as any], { type: 'image/png' });
      renderOutputUrl = URL.createObjectURL(blob);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleExtractText() {
    if (!extractTextFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await extractTextFile.arrayBuffer());
      extractedText = await wasmPdfClient.extract_text(bytes);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleDecrypt() {
    if (!decryptFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await decryptFile.arrayBuffer());
      const resBytes = await wasmPdfClient.decrypt(bytes, decryptPassword);
      download(resBytes, `decrypted_${decryptFile.name}`);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handlePageNumbers() {
    if (!pageNumbersFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await pageNumbersFile.arrayBuffer());
      const resBytes = await wasmPdfClient.page_numbers(bytes, pageNumbersFormat, pageNumbersPosition);
      download(resBytes, `numbered_${pageNumbersFile.name}`);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleHeaderFooter() {
    if (!headerFooterFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await headerFooterFile.arrayBuffer());
      const resBytes = await wasmPdfClient.header_footer(bytes, headerText, footerText);
      download(resBytes, `header_footer_${headerFooterFile.name}`);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handlePdfInfo() {
    if (!pdfInfoFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await pdfInfoFile.arrayBuffer());
      const jsonStr = await wasmPdfClient.pdf_info(bytes);
      pdfInfoData = JSON.stringify(JSON.parse(jsonStr), null, 2);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handleOcr() {
    if (!ocrFile) return;
    isProcessing = true;
    try {
      const bytes = new Uint8Array(await ocrFile.arrayBuffer());
      ocrData = await wasmPdfClient.ocr(bytes);
    } catch (e) {
      alert("Error: " + e);
    }
    isProcessing = false;
  }

  async function handlePdfHash() {
    if (!pdfHashFile) return;
    try {
      isProcessing = true;
      const buffer = new Uint8Array(await pdfHashFile.arrayBuffer());
      computedPdfHash = await wasmPdfClient.pdf_hash(buffer);
    } catch (e) {
      alert(`Error hashing pdf: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleNup() {
    if (!nupFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await nupFile.arrayBuffer());
      const result = await wasmPdfClient.pdf_nup(buffer, { grid: nupGrid, ordering: nupOrdering });
      download(result, `nup_${nupFile.name}`);
    } catch (e) {
      alert(`Error applying N-Up: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleBooklet() {
    if (!bookletFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await bookletFile.arrayBuffer());
      const result = await wasmPdfClient.pdf_booklet(buffer, { page_format: bookletPageFormat });
      download(result, `booklet_${bookletFile.name}`);
    } catch (e) {
      alert(`Error generating booklet: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleScalePages() {
    if (!scalePagesFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await scalePagesFile.arrayBuffer());
      let options: any = {};
      if (scalePagesTarget === 'percentage') {
          options.percentage = scalePagesPercentage;
      } else {
          options.format = scalePagesTarget;
      }
      const result = await wasmPdfClient.pdf_scale_pages(buffer, options);
      download(result, `scaled_${scalePagesFile.name}`);
    } catch (e) {
      alert(`Error scaling pages: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleAutoRotate() {
    if (!autoRotateFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await autoRotateFile.arrayBuffer());
      const result = await wasmPdfClient.pdf_auto_rotate(buffer, { target_orientation: autoRotateOrientation });
      download(result, `autorotated_${autoRotateFile.name}`);
    } catch (e) {
      alert(`Error auto-rotating pages: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleStamp() {
    if (!stampFile || !stampImageFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await stampFile.arrayBuffer());
      const reader = new FileReader();
      reader.readAsDataURL(stampImageFile);
      await new Promise<void>((resolve, reject) => {
        reader.onload = () => resolve();
        reader.onerror = error => reject(error);
      });
      const dataUrl = reader.result as string;
      const base64Image = dataUrl.split(',')[1];
      const result = await wasmPdfClient.pdf_stamp(buffer, { image_data: base64Image, position: stampPosition });
      download(result, `stamped_${stampFile.name}`);
    } catch (e) {
      alert(`Error applying stamp: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleOverlayUnderlay() {
    if (!overlayUnderlayFile || !overlayUnderlaySecondaryFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await overlayUnderlayFile.arrayBuffer());
      const reader = new FileReader();
      reader.readAsDataURL(overlayUnderlaySecondaryFile);
      await new Promise<void>((resolve, reject) => {
        reader.onload = () => resolve();
        reader.onerror = error => reject(error);
      });
      const dataUrl = reader.result as string;
      const base64Secondary = dataUrl.split(',')[1];
      const result = await wasmPdfClient.pdf_overlay_underlay(buffer, { mode: overlayUnderlayMode, secondary_pdf: base64Secondary });
      download(result, `overlayed_${overlayUnderlayFile.name}`);
    } catch (e) {
      alert(`Error applying overlay/underlay: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleRemoveSignatures() {
    if (!removeSignaturesFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await removeSignaturesFile.arrayBuffer());
      const result = await wasmPdfClient.pdf_remove_signatures(buffer, {});
      download(result, `unlocked_${removeSignaturesFile.name}`);
    } catch (e) {
      alert(`Error removing signatures: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleValidateSignatures() {
    if (!validateSignaturesFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await validateSignaturesFile.arrayBuffer());
      let baselineBase64 = undefined;
      if (validateSignaturesBaselineFile) {
        const reader = new FileReader();
        reader.readAsDataURL(validateSignaturesBaselineFile);
        await new Promise<void>((resolve, reject) => {
          reader.onload = () => resolve();
          reader.onerror = error => reject(error);
        });
        const dataUrl = reader.result as string;
        baselineBase64 = dataUrl.split(',')[1];
      }
      const result = await wasmPdfClient.pdf_validate_signatures(buffer, { baseline_pdf: baselineBase64 });
      validateSignaturesResult = result;
    } catch (e) {
      alert(`Error validating signatures: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleSanitize() {
    if (!sanitizeFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await sanitizeFile.arrayBuffer());
      const result = await wasmPdfClient.sanitize(buffer, {});
      download(result, `sanitized_${sanitizeFile.name}`);
    } catch (e) {
      alert(`Error sanitizing PDF: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handlePiiScan() {
    if (!piiScanFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await piiScanFile.arrayBuffer());
      const result = await wasmPdfClient.pii_scan(buffer, {});
      piiScanResult = result;
    } catch (e) {
      alert(`Error scanning for PII: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handlePdfToDocx() {
    if (!pdfToDocxFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await pdfToDocxFile.arrayBuffer());
      const result = await wasmPdfClient.pdf_to_docx(buffer);
      const blob = new Blob([result.buffer as ArrayBuffer], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `converted_${pdfToDocxFile.name}.docx`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (e) {
      alert(`Error converting to DOCX: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handlePdfToGif() {
    if (!pdfToGifFile) return;
    isProcessing = true;
    try {
      const buffer = new Uint8Array(await pdfToGifFile.arrayBuffer());
      const options: any = {
        delay_centisecs: Number(gifDelay),
        width: Number(gifWidth),
      };
      if (gifMaxPages && Number(gifMaxPages) > 0) {
        options.max_pages = Number(gifMaxPages);
      }
      const result = await wasmPdfClient.pdf_to_gif(buffer, options);
      const blob = new Blob([result.buffer as ArrayBuffer], { type: 'image/gif' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      const base = pdfToGifFile.name.replace(/\.[^.]+$/i, '');
      a.download = `${base}_preview.gif`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (e) {
      alert(`Error converting to GIF: ${e}`);
    } finally {
      isProcessing = false;
    }
  }


  function downloadBlob(buffer: Uint8Array, filename: string, mimeType: string = 'application/pdf') {
    const blob = new Blob([buffer.buffer as ArrayBuffer], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  async function handleStitchCanvas() {
    if (!stitchCanvasFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(stitchCanvasFile);
      const res = await wasmPdfClient.pdf_stitch_canvas(buffer, { direction: stitchDirection, spacing_pt: Number(stitchSpacing) });
      downloadBlob(res, `${stitchCanvasFile.name.replace(/\.pdf$/i, '')}_stitched.pdf`);
    } catch (e) {
      alert(`Error stitching canvas: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleSplitScannedBook() {
    if (!splitScannedBookFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(splitScannedBookFile);
      const res = await wasmPdfClient.pdf_split_scanned_book(buffer, { reading_direction: scannedBookReadingDirection });
      downloadBlob(res, `${splitScannedBookFile.name.replace(/\.pdf$/i, '')}_split_pages.pdf`);
    } catch (e) {
      alert(`Error splitting scanned book: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleSplitByOutline() {
    if (!splitByOutlineFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(splitByOutlineFile);
      const res = await wasmPdfClient.pdf_split_by_outline(buffer, { max_depth: Number(splitOutlineMaxDepth), sanitize_names: true });
      downloadBlob(res, `${splitByOutlineFile.name.replace(/\.pdf$/i, '')}_sections.zip`, 'application/zip');
    } catch (e) {
      alert(`Error splitting by outline: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleMergeAlternate() {
    if (!mergeAlternatePrimary || !mergeAlternateSecondary) return;
    isProcessing = true;
    try {
      const bufA = await readAsUint8Array(mergeAlternatePrimary);
      const bufB = await readAsUint8Array(mergeAlternateSecondary);
      const res = await wasmPdfClient.pdf_merge_alternate(bufA, bufB, { step: 1, step_b: 1, reverse_a: false, reverse_b: false });
      downloadBlob(res, `${mergeAlternatePrimary.name.replace(/\.pdf$/i, '')}_interleaved.pdf`);
    } catch (e) {
      alert(`Error interleaving PDFs: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handlePosterize() {
    if (!posterizeFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(posterizeFile);
      const res = await wasmPdfClient.pdf_posterize(buffer, { grid_cols: Number(posterizeCols), grid_rows: Number(posterizeRows), include_guides: true });
      downloadBlob(res, `${posterizeFile.name.replace(/\.pdf$/i, '')}_poster_${posterizeCols}x${posterizeRows}.pdf`);
    } catch (e) {
      alert(`Error generating poster: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleAttachments() {
    if (!attachmentsFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(attachmentsFile);
      const res = await wasmPdfClient.pdf_attachments(buffer, { action: 'extract' });
      downloadBlob(res, `${attachmentsFile.name.replace(/\.pdf$/i, '')}_attachments.zip`, 'application/zip');
    } catch (e) {
      alert(`Error processing attachments: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleDedupStreams() {
    if (!dedupStreamsFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(dedupStreamsFile);
      const res = await wasmPdfClient.pdf_dedup_streams(buffer, {});
      downloadBlob(res, `${dedupStreamsFile.name.replace(/\.pdf$/i, '')}_deduped.pdf`);
    } catch (e) {
      alert(`Error deduplicating streams: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleTimestamp() {
    if (!timestampFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(timestampFile);
      const res = await wasmPdfClient.pdf_timestamp(buffer, {});
      downloadBlob(res, `${timestampFile.name.replace(/\.pdf$/i, '')}_timestamped.pdf`);
    } catch (e) {
      alert(`Error applying timestamp: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleChangePermissions() {
    if (!changePermissionsFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(changePermissionsFile);
      const res = await wasmPdfClient.pdf_change_permissions(buffer, {
        allow_printing: allowPrinting,
        allow_copying: allowCopying,
        allow_modifying: allowModifying
      });
      downloadBlob(res, `${changePermissionsFile.name.replace(/\.pdf$/i, '')}_permissions.pdf`);
    } catch (e) {
      alert(`Error modifying permissions: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleToEpub() {
    if (!toEpubFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(toEpubFile);
      const res = await wasmPdfClient.pdf_to_epub(buffer, {
        title: epubTitle || toEpubFile.name.replace(/\.pdf$/i, ''),
        author: epubAuthor || 'PaperPilot'
      });
      downloadBlob(res, `${toEpubFile.name.replace(/\.pdf$/i, '')}.epub`, 'application/epub+zip');
    } catch (e) {
      alert(`Error converting to EPUB: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleExtractPatterns() {
    if (!extractPatternsFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(extractPatternsFile);
      const res = await wasmPdfClient.pdf_extract_patterns(buffer, {
        keywords: ['invoice', 'total', 'due', 'amount'],
        patterns: {
          email: '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}',
          url: 'https?://[^\\s/$.?#].[^\\s]*'
        },
        case_sensitive: false,
        context_len: 40
      });
      const text = new TextDecoder().decode(res);
      patternsResult = text;
    } catch (e) {
      alert(`Error extracting patterns: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleComicArchive() {
    if (!comicArchiveFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(comicArchiveFile);
      const res = await wasmPdfClient.pdf_comic_archive(buffer, { dpi: Number(comicDpi) });
      downloadBlob(res, `${comicArchiveFile.name.replace(/\.pdf$/i, '')}.cbz`, 'application/vnd.comicbook+zip');
    } catch (e) {
      alert(`Error converting to CBZ: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleShowcaseGif() {
    if (!showcaseGifFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(showcaseGifFile);
      const res = await wasmPdfClient.showcase_gif(buffer, {
        fps: Number(showcaseGifFps),
        width: Number(showcaseGifWidth)
      });
      downloadBlob(res, `${showcaseGifFile.name.replace(/\.[^.]+$/i, '')}_showcase.gif`, 'image/gif');
    } catch (e) {
      alert(`Error generating showcase GIF: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleImagesToGif() {
    if (imagesToGifFiles.length === 0) return;
    isProcessing = true;
    try {
      const buffers: Uint8Array[] = [];
      for (const f of imagesToGifFiles) {
        buffers.push(await readAsUint8Array(f));
      }
      const res = await wasmPdfClient.images_to_gif(buffers, {
        fps: Number(imagesToGifFps),
        width: Number(imagesToGifWidth)
      });
      downloadBlob(res, 'animated_sequence.gif', 'image/gif');
    } catch (e) {
      alert(`Error generating GIF from images: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleOptimizeLlm() {
    if (!optimizeLlmFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(optimizeLlmFile);
      const res = await wasmPdfClient.img_optimize_for_llm(buffer, {
        max_dim: Number(optimizeLlmMaxDim),
        enhance_text: true,
        quality: 85
      });
      downloadBlob(res, `${optimizeLlmFile.name.replace(/\.[^.]+$/i, '')}_llm_opt.jpg`, 'image/jpeg');
    } catch (e) {
      alert(`Error optimizing image for LLM: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleDeskew() {
    if (!deskewFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(deskewFile);
      const res = await wasmPdfClient.img_deskew(buffer, { auto_detect: true });
      downloadBlob(res, `${deskewFile.name.replace(/\.[^.]+$/i, '')}_deskewed.png`, 'image/png');
    } catch (e) {
      alert(`Error deskewing image: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleConvertImage() {
    if (!convertImageFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(convertImageFile);
      const res = await wasmPdfClient.img_convert(buffer, {
        format: convertImageFormat,
        quality: Number(convertImageQuality)
      });
      const mime = convertImageFormat === 'webp' ? 'image/webp' : convertImageFormat === 'png' ? 'image/png' : 'image/jpeg';
      downloadBlob(res, `${convertImageFile.name.replace(/\.[^.]+$/i, '')}.${convertImageFormat}`, mime);
    } catch (e) {
      alert(`Error converting image: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleCropContent() {
    if (!cropContentFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(cropContentFile);
      const res = await wasmPdfClient.img_crop_to_content(buffer, {
        threshold: 0.1,
        padding: Number(cropPadding)
      });
      downloadBlob(res, `${cropContentFile.name.replace(/\.[^.]+$/i, '')}_cropped.png`, 'image/png');
    } catch (e) {
      alert(`Error auto-cropping image: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handleRedactImage() {
    if (!redactImageFile) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(redactImageFile);
      const res = await wasmPdfClient.img_redact(buffer, {
        rects: [{ x: Number(redactBoxX), y: Number(redactBoxY), width: Number(redactBoxW), height: Number(redactBoxH) }],
        color: '#000000'
      });
      downloadBlob(res, `${redactImageFile.name.replace(/\.[^.]+$/i, '')}_redacted.png`, 'image/png');
    } catch (e) {
      alert(`Error redacting image: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  async function handlePdfEditText() {
    if (!editTextFile || !editTextFind) return;
    isProcessing = true;
    try {
      const buffer = await readAsUint8Array(editTextFile);
      const parsePositiveInt = (val: number | string | null | undefined): number | undefined => {
        const trimmed = val == null ? '' : String(val).trim();
        if (trimmed === '') return undefined;
        const parsed = Number(trimmed);
        return Number.isFinite(parsed) && parsed >= 1 ? Math.trunc(parsed) : undefined;
      };
      const options: Record<string, any> = {
        find: editTextFind,
        replace: editTextReplace,
        match_case: editTextMatchCase
      };
      const parsedPage = parsePositiveInt(editTextPage);
      if (parsedPage !== undefined) {
        options.page = parsedPage;
      }
      const parsedOccurrence = parsePositiveInt(editTextOccurrence);
      if (parsedOccurrence !== undefined) {
        options.occurrence = parsedOccurrence;
      }
      const res = await wasmPdfClient.pdf_edit_text(buffer, options);
      downloadBlob(res, `${editTextFile.name.replace(/\.pdf$/i, '')}_edited.pdf`);
    } catch (e) {
      alert(`Error editing PDF text: ${e}`);
    } finally {
      isProcessing = false;
    }
  }

  // Helpers for merge files
  function removeMergeFile(index: number) {
      mergeFiles = mergeFiles.filter((_, i) => i !== index);
  }

  function moveMergeFileUp(index: number) {
      if (index > 0) {
          const temp = mergeFiles[index];
          mergeFiles[index] = mergeFiles[index - 1];
          mergeFiles[index - 1] = temp;
      }
  }

  function moveMergeFileDown(index: number) {
      if (index < mergeFiles.length - 1) {
          const temp = mergeFiles[index];
          mergeFiles[index] = mergeFiles[index + 1];
          mergeFiles[index + 1] = temp;
      }
  }

  let filterMode = $state<'all' | 'ready'>('all');
  let selectedCategory = $state<'all' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert'>('all');
  let searchQuery = $state('');

  const categories = [
    { id: 'all', label: 'All Tools', icon: '⭐' },
    { id: 'pages', label: 'Pages & Imposition', icon: '📑' },
    { id: 'edit', label: 'Edit & Stationery', icon: '🎨' },
    { id: 'optimize', label: 'Optimize', icon: '🗜️' },
    { id: 'security', label: 'Security', icon: '🔒' },
    { id: 'convert', label: 'Convert', icon: '🔁' }
  ] as const;

  function getCategoryCount(catId: string) {
    if (catId === 'all') return tools.length;
    return tools.filter(t => t.category === catId).length;
  }

  let displayedTools = $derived(
    tools.filter(t => {
      const modeMatch = filterMode === 'ready' ? t.releaseStatus !== 'preview' : true;
      const categoryMatch = selectedCategory === 'all' || t.category === selectedCategory;
      const searchMatch = t.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          (t.description && t.description.toLowerCase().includes(searchQuery.toLowerCase()));
      return modeMatch && categoryMatch && searchMatch;
    })
  );
</script>

<div class="operations-view">
  <div class="tools-sidebar">
    <div class="sidebar-header-row">
      <h3>Tools</h3>
      <div class="filter-toggle">
        <button
          class="toggle-btn"
          class:active={filterMode === 'all'}
          onclick={() => filterMode = 'all'}
        >All</button>
        <button
          class="toggle-btn toggle-ready"
          class:active={filterMode === 'ready'}
          onclick={() => filterMode = 'ready'}
        >✅ Ready</button>
      </div>
    </div>

    <div class="search-box" style="margin-bottom: 12px;">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search tools..."
        style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);"
      />
    </div>

    <div class="category-tabs" style="display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 16px;">
      {#each categories as cat}
        <button
          class="cat-tab"
          class:active={selectedCategory === cat.id}
          onclick={() => selectedCategory = cat.id}
          style="flex: 1; min-width: 45%; display: flex; align-items: center; gap: 4px; padding: 6px; font-size: 0.8rem; background: {selectedCategory === cat.id ? 'var(--primary-color)' : 'transparent'}; border: 1px solid {selectedCategory === cat.id ? 'var(--primary-color)' : 'var(--border-color)'}; border-radius: var(--border-radius-sm); color: {selectedCategory === cat.id ? 'white' : 'var(--text-secondary)'}; cursor: pointer;"
        >
          <span>{cat.icon}</span>
          <span style="flex: 1; text-align: left;">{cat.label}</span>
          <span class="count" style="font-size: 0.7rem; opacity: 0.7;">{getCategoryCount(cat.id)}</span>
        </button>
      {/each}
    </div>

    <div class="tools-list">
      {#each displayedTools as tool}
        <button
          class="tool-btn"
          class:active={activeTool === tool.id}
          onclick={() => activeTool = tool.id}
        >
          <span class="tool-icon">{tool.icon}</span>
          <span class="tool-title-text">{tool.title}</span>
        </button>
      {/each}
    </div>
  </div>

  <div class="tool-content">
    {#if activeTool === 'merge'}
      <div class="tool-pane">
        <h2>Merge PDFs</h2>
        <p>Combine multiple PDF files into one.</p>

        <DropZone ondrop={(files: File[]) => mergeFiles = [...mergeFiles, ...files]} multiple={true} />

        {#if mergeFiles.length > 0}
          <div class="file-list">
            {#each mergeFiles as file, i}
              <div class="file-item">
                <span class="file-name">{file.name}</span>
                <div class="file-controls">
                    <button onclick={() => moveMergeFileUp(i)} disabled={i === 0}>↑</button>
                    <button onclick={() => moveMergeFileDown(i)} disabled={i === mergeFiles.length - 1}>↓</button>
                    <button onclick={() => removeMergeFile(i)}>✕</button>
                </div>
              </div>
            {/each}
          </div>
          <button class="action-btn" onclick={handleMerge} disabled={isProcessing || mergeFiles.length < 2}>
            {isProcessing ? 'Processing...' : 'Merge & Download'}
          </button>
        {/if}
      </div>

    {:else if activeTool === 'split'}
      <div class="tool-pane">
        <h2>Split PDF</h2>
        <p>Extract pages from one or more PDFs.</p>

        <DropZone ondrop={(files: File[]) => splitFiles = [...splitFiles, ...files]} multiple={true} />

        {#if splitFiles.length > 0}
            <div class="file-list">
              {#each splitFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => splitFiles = splitFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="splitRanges">Page Ranges (e.g. 1-2, 4)</label>
                <input id="splitRanges" type="text" bind:value={splitRanges} placeholder="1-2, 4" />
            </div>

            <button class="action-btn" onclick={handleSplit} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Split & Download (${splitFiles.length} file${splitFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'rotate'}
      <div class="tool-pane">
        <h2>Rotate Pages</h2>
        <p>Rotate pages in one or more PDFs by 90, 180, or 270 degrees.</p>

        <DropZone ondrop={(files: File[]) => rotateFiles = [...rotateFiles, ...files]} multiple={true} />

        {#if rotateFiles.length > 0}
            <div class="file-list">
              {#each rotateFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => rotateFiles = rotateFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="rotateAngle">Angle</label>
                <select id="rotateAngle" bind:value={rotateAngle}>
                    <option value={90}>90° Clockwise</option>
                    <option value={180}>180°</option>
                    <option value={270}>270° Clockwise (90° CCW)</option>
                </select>
            </div>

            <div class="input-group">
                <label for="rotatePages">Pages (e.g. all, 1, 2-5)</label>
                <input id="rotatePages" type="text" bind:value={rotatePages} placeholder="all" />
            </div>

            <button class="action-btn" onclick={handleRotate} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Rotate & Download (${rotateFiles.length} file${rotateFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'compress'}
      <div class="tool-pane">
        <h2>Compress PDF</h2>
        <p>Reduce file size across one or multiple documents.</p>

        <DropZone ondrop={(files: File[]) => compressFiles = [...compressFiles, ...files]} multiple={true} />

        {#if compressFiles.length > 0}
            <div class="file-list">
              {#each compressFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => compressFiles = compressFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <button class="action-btn" onclick={handleCompress} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Compress & Download (${compressFiles.length} file${compressFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'encrypt'}
      <div class="tool-pane">
        <h2>Encrypt PDF</h2>
        <p>Add a password to protect your document(s).</p>

        <DropZone ondrop={(files: File[]) => encryptFiles = [...encryptFiles, ...files]} multiple={true} />

        {#if encryptFiles.length > 0}
            <div class="file-list">
              {#each encryptFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => encryptFiles = encryptFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="encryptPassword">Password</label>
                <input id="encryptPassword" type="password" bind:value={encryptPassword} />
            </div>

            <button class="action-btn" onclick={handleEncrypt} disabled={isProcessing || !encryptPassword}>
                {isProcessing ? 'Processing...' : `Encrypt & Download (${encryptFiles.length} file${encryptFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'watermark'}
      <div class="tool-pane">
        <h2>Watermark PDF</h2>
        <p>Add a text watermark to your document(s).</p>

        <DropZone ondrop={(files: File[]) => watermarkFiles = [...watermarkFiles, ...files]} multiple={true} />

        {#if watermarkFiles.length > 0}
            <div class="file-list">
              {#each watermarkFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => watermarkFiles = watermarkFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="watermarkText">Watermark Text</label>
                <input id="watermarkText" type="text" bind:value={watermarkText} />
            </div>

            <button class="action-btn" onclick={handleWatermark} disabled={isProcessing || !watermarkText}>
                {isProcessing ? 'Processing...' : `Watermark & Download (${watermarkFiles.length} file${watermarkFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'delete'}
      <div class="tool-pane">
        <h2>Delete Pages</h2>
        <p>Remove specific pages from your document(s).</p>

        <DropZone ondrop={(files: File[]) => deleteFiles = [...deleteFiles, ...files]} multiple={true} />

        {#if deleteFiles.length > 0}
            <div class="file-list">
              {#each deleteFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => deleteFiles = deleteFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="deletePagesStr">Pages to Delete (e.g. 2, 4-6)</label>
                <input id="deletePagesStr" type="text" bind:value={deletePagesStr} placeholder="2, 4-6" />
            </div>

            <button class="action-btn" onclick={handleDelete} disabled={isProcessing || !deletePagesStr}>
                {isProcessing ? 'Processing...' : `Delete Pages & Download (${deleteFiles.length} file${deleteFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'extract'}
      <div class="tool-pane">
        <h2>Extract Pages</h2>
        <p>Extract specific pages across your document(s).</p>

        <DropZone ondrop={(files: File[]) => extractFiles = [...extractFiles, ...files]} multiple={true} />

        {#if extractFiles.length > 0}
            <div class="file-list">
              {#each extractFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => extractFiles = extractFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="extractPagesStr">Target Pages (e.g. 1-3)</label>
                <input id="extractPagesStr" type="text" bind:value={extractPagesStr} placeholder="1-3" />
            </div>

            <button class="action-btn" onclick={handleExtract} disabled={isProcessing || !extractPagesStr}>
                {isProcessing ? 'Processing...' : `Extract Pages & Download (${extractFiles.length} file${extractFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'reorder'}
      <div class="tool-pane">
        <h2>Reorder Pages</h2>
        <p>Rearrange the pages in your document(s).</p>

        <DropZone ondrop={(files: File[]) => reorderFiles = [...reorderFiles, ...files]} multiple={true} />

        {#if reorderFiles.length > 0}
            <div class="file-list">
              {#each reorderFiles as file, i}
                <div class="file-item">
                  <span class="file-name">{file.name}</span>
                  <div class="file-controls">
                    <button onclick={() => reorderFiles = reorderFiles.filter((_, idx) => idx !== i)}>✕</button>
                  </div>
                </div>
              {/each}
            </div>

            <div class="input-group">
                <label for="reorderPagesStr">New Order (comma separated, e.g. 3, 1, 2)</label>
                <input id="reorderPagesStr" type="text" bind:value={reorderPagesStr} placeholder="3, 1, 2" />
            </div>

            <button class="action-btn" onclick={handleReorder} disabled={isProcessing || !reorderPagesStr}>
                {isProcessing ? 'Processing...' : `Reorder & Download (${reorderFiles.length} file${reorderFiles.length > 1 ? 's' : ''})`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'crop'}
      <div class="tool-pane">
        <h2>Crop PDF</h2>
        <p>Crop the pages in your document.</p>

        {#if !cropFile}
            <DropZone ondrop={(files: File[]) => cropFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{cropFile.name}</span>
                <button onclick={() => cropFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="cropLeft">Left</label>
                <input id="cropLeft" type="number" bind:value={cropLeft} />
            </div>
            <div class="input-group">
                <label for="cropBottom">Bottom</label>
                <input id="cropBottom" type="number" bind:value={cropBottom} />
            </div>
            <div class="input-group">
                <label for="cropRight">Right</label>
                <input id="cropRight" type="number" bind:value={cropRight} />
            </div>
            <div class="input-group">
                <label for="cropTop">Top</label>
                <input id="cropTop" type="number" bind:value={cropTop} />
            </div>

            <button class="action-btn" onclick={handleCrop} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Crop & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'flatten'}
      <div class="tool-pane">
        <h2>Flatten Forms</h2>
        <p>Remove interactive form elements and flatten them onto the pages.</p>

        {#if !flattenFile}
            <DropZone ondrop={(files: File[]) => flattenFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{flattenFile.name}</span>
                <button onclick={() => flattenFile = null}>✕</button>
            </div>

            <button class="action-btn" onclick={handleFlatten} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Flatten & Download'}
            </button>
        {/if}
      </div>

{:else if activeTool === 'metadata'}
      <div class="tool-pane">
        <h2>Edit Metadata</h2>
        <p>Update the metadata of your document.</p>

        {#if !metadataFile}
            <DropZone ondrop={(files: File[]) => metadataFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{metadataFile.name}</span>
                <button onclick={() => metadataFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="metaTitle">Title</label>
                <input id="metaTitle" type="text" bind:value={metaTitle} />
            </div>
            <div class="input-group">
                <label for="metaAuthor">Author</label>
                <input id="metaAuthor" type="text" bind:value={metaAuthor} />
            </div>
            <div class="input-group">
                <label for="metaSubject">Subject</label>
                <input id="metaSubject" type="text" bind:value={metaSubject} />
            </div>
            <div class="input-group">
                <label for="metaKeywords">Keywords</label>
                <input id="metaKeywords" type="text" bind:value={metaKeywords} />
            </div>

            <button class="action-btn" onclick={handleMetadata} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Update Metadata & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'images_to_pdf'}
      <div class="tool-pane">
        <h2>Images to PDF</h2>
        <p>Convert multiple images (PNG/JPEG) into a single PDF document.</p>

        {#if imagesToPdfFiles.length === 0}
            <DropZone ondrop={(files: File[]) => imagesToPdfFiles = [...imagesToPdfFiles, ...files]} multiple={true} />
        {:else}
            <div class="file-list">
                {#each imagesToPdfFiles as file, i}
                    <div class="file-item">
                        <span class="file-name">{file.name}</span>
                        <div class="file-controls">
                            <button onclick={() => {
                                const newFiles = [...imagesToPdfFiles];
                                newFiles.splice(i, 1);
                                imagesToPdfFiles = newFiles;
                            }}>✕</button>
                        </div>
                    </div>
                {/each}
            </div>
            <DropZone ondrop={(files: File[]) => imagesToPdfFiles = [...imagesToPdfFiles, ...files]} multiple={true} />
            <button class="action-btn" onclick={handleImagesToPdf} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Convert Images & Download PDF'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'extract_images'}
      <div class="tool-pane">
        <h2>Extract Images</h2>
        <p>Extract embedded images from a PDF.</p>

        {#if !extractImagesFile}
            <DropZone ondrop={(files: File[]) => extractImagesFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{extractImagesFile.name}</span>
                <button onclick={() => extractImagesFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleExtractImages} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Extract Images & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_hash'}
      <div class="tool-pane">
        <h2>PDF Hash / Checksum</h2>
        <p>Calculate the cryptographic SHA-256 integrity hash of your PDF document locally.</p>

        {#if !pdfHashFile}
            <DropZone ondrop={(files: File[]) => { pdfHashFile = files[0]; computedPdfHash = null; }} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pdfHashFile.name}</span>
                <button onclick={() => { pdfHashFile = null; computedPdfHash = null; }}>✕</button>
            </div>
            <button class="action-btn" onclick={handlePdfHash} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Calculate Hash'}
            </button>

            {#if computedPdfHash}
                <div class="hash-result" style="margin-top: 24px; padding: 16px; background-color: var(--bg-surface); border-radius: var(--border-radius-md); border: 1px solid var(--border-color);">
                    <h3 style="margin-top: 0; font-size: 1rem; color: var(--text-secondary);">SHA-256 Hash</h3>
                    <div style="display: flex; gap: 8px; align-items: center;">
                        <input type="text" readonly value={computedPdfHash} style="flex: 1; padding: 8px; font-family: monospace; font-size: 0.9rem; background-color: var(--bg-primary); border: 1px solid var(--border-color); border-radius: 4px;" />
                        <button onclick={() => navigator.clipboard.writeText(computedPdfHash || '')} style="padding: 8px 16px; background-color: var(--accent-primary); color: white; border: none; border-radius: 4px; cursor: pointer;">Copy</button>
                    </div>
                </div>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'render_page'}
      <div class="tool-pane">
        <h2>Render Page</h2>
        <p>Render a PDF page to PNG.</p>
        {#if !renderFile}
            <DropZone ondrop={(files: File[]) => renderFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{renderFile.name}</span>
                <button onclick={() => renderFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="renderPageIndex">Page Index (0-based)</label>
                <input id="renderPageIndex" type="number" bind:value={renderPageIndex} min="0" />
            </div>
            <div class="input-group">
                <label for="renderScale">Scale</label>
                <input id="renderScale" type="number" step="0.1" bind:value={renderScale} />
            </div>
            <button class="action-btn" onclick={handleRenderPage} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Render'}
            </button>
            {#if renderOutputUrl}
                <div style="margin-top: 16px;">
                    <img alt="Rendered PDF Page" src={renderOutputUrl} style="max-width: 100%; border: 1px solid var(--border-color); border-radius: var(--border-radius-md);" />
                </div>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'extract_text'}
      <div class="tool-pane">
        <h2>Extract Text</h2>
        <p>Extract all text content from the PDF.</p>
        {#if !extractTextFile}
            <DropZone ondrop={(files: File[]) => extractTextFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{extractTextFile.name}</span>
                <button onclick={() => extractTextFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleExtractText} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Extract'}
            </button>
            {#if extractedText}
                <textarea readonly rows="10" style="width: 100%; margin-top: 16px; font-family: monospace; padding: 8px;">{extractedText}</textarea>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'decrypt'}
      <div class="tool-pane">
        <h2>Decrypt</h2>
        <p>Decrypt a password-protected PDF.</p>
        {#if !decryptFile}
            <DropZone ondrop={(files: File[]) => decryptFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{decryptFile.name}</span>
                <button onclick={() => decryptFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="decryptPassword">Password</label>
                <input id="decryptPassword" type="password" bind:value={decryptPassword} />
            </div>
            <button class="action-btn" onclick={handleDecrypt} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Decrypt & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'page_numbers'}
      <div class="tool-pane">
        <h2>Page Numbers</h2>
        <p>Add page numbers to the PDF.</p>
        {#if !pageNumbersFile}
            <DropZone ondrop={(files: File[]) => pageNumbersFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pageNumbersFile.name}</span>
                <button onclick={() => pageNumbersFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="pageNumbersFormat">Format</label>
                <input id="pageNumbersFormat" type="text" bind:value={pageNumbersFormat} />
            </div>
            <div class="input-group">
                <label for="pageNumbersPosition">Position</label>
                <select id="pageNumbersPosition" bind:value={pageNumbersPosition}>
                    <option value="bottom-center">Bottom Center</option>
                    <option value="bottom-right">Bottom Right</option>
                    <option value="top-center">Top Center</option>
                    <option value="top-right">Top Right</option>
                </select>
            </div>
            <button class="action-btn" onclick={handlePageNumbers} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Add Numbers & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'header_footer'}
      <div class="tool-pane">
        <h2>Header & Footer</h2>
        <p>Add header and footer text to the PDF.</p>
        {#if !headerFooterFile}
            <DropZone ondrop={(files: File[]) => headerFooterFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{headerFooterFile.name}</span>
                <button onclick={() => headerFooterFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="headerText">Header Text</label>
                <input id="headerText" type="text" bind:value={headerText} />
            </div>
            <div class="input-group">
                <label for="footerText">Footer Text</label>
                <input id="footerText" type="text" bind:value={footerText} />
            </div>
            <button class="action-btn" onclick={handleHeaderFooter} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Add & Download'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_info'}
      <div class="tool-pane">
        <h2>PDF Info</h2>
        <p>Get metadata and info about the PDF.</p>
        {#if !pdfInfoFile}
            <DropZone ondrop={(files: File[]) => pdfInfoFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pdfInfoFile.name}</span>
                <button onclick={() => pdfInfoFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handlePdfInfo} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Get Info'}
            </button>
            {#if pdfInfoData}
                <pre style="margin-top: 16px; padding: 16px; background: var(--bg-surface); border-radius: var(--border-radius-md); overflow-x: auto;">{pdfInfoData}</pre>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'ocr'}
      <div class="tool-pane">
        <h2>OCR</h2>
        <p>Extract text from images using local WASM OCR.</p>
        {#if !ocrFile}
            <DropZone ondrop={(files: File[]) => ocrFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{ocrFile.name}</span>
                <button onclick={() => ocrFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleOcr} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Run OCR'}
            </button>
            {#if ocrData}
                <textarea readonly rows="10" style="width: 100%; margin-top: 16px; font-family: monospace; padding: 8px;">{ocrData}</textarea>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'pdf_nup'}
      <div class="tool-pane">
        <h2>N-Up Document Layout</h2>
        <p>Combine multiple pages into a grid on a single page.</p>
        {#if !nupFile}
            <DropZone ondrop={(files: File[]) => nupFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{nupFile.name}</span>
                <button onclick={() => nupFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label>Grid Option:</label>
                <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                    <button style="flex: 1; padding: 8px;" class:active={nupGrid === 2} onclick={() => nupGrid = 2}>2 (2x1)</button>
                    <button style="flex: 1; padding: 8px;" class:active={nupGrid === 4} onclick={() => nupGrid = 4}>4 (2x2)</button>
                    <button style="flex: 1; padding: 8px;" class:active={nupGrid === 9} onclick={() => nupGrid = 9}>9 (3x3)</button>
                    <button style="flex: 1; padding: 8px;" class:active={nupGrid === 16} onclick={() => nupGrid = 16}>16 (4x4)</button>
                </div>
            </div>
            <div class="input-group">
                <label>Ordering:</label>
                <div style="display: flex; gap: 8px;">
                    <button style="flex: 1; padding: 8px;" class:active={nupOrdering === 'horizontal'} onclick={() => nupOrdering = 'horizontal'}>Horizontal</button>
                    <button style="flex: 1; padding: 8px;" class:active={nupOrdering === 'vertical'} onclick={() => nupOrdering = 'vertical'}>Vertical</button>
                </div>
            </div>
            <button class="action-btn" onclick={handleNup} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Apply N-Up'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_booklet'}
      <div class="tool-pane">
        <h2>Create Booklet</h2>
        <p>Rearrange pages into a booklet layout for double-sided printing.</p>
        {#if !bookletFile}
            <DropZone ondrop={(files: File[]) => bookletFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{bookletFile.name}</span>
                <button onclick={() => bookletFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label>Page Format:</label>
                <div style="display: flex; gap: 8px;">
                    <button style="flex: 1; padding: 8px;" class:active={bookletPageFormat === 'a4'} onclick={() => bookletPageFormat = 'a4'}>A4</button>
                    <button style="flex: 1; padding: 8px;" class:active={bookletPageFormat === 'letter'} onclick={() => bookletPageFormat = 'letter'}>Letter</button>
                </div>
            </div>
            <button class="action-btn" onclick={handleBooklet} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Generate Booklet'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_scale_pages'}
      <div class="tool-pane">
        <h2>Scale Pages</h2>
        <p>Resize all pages in a document to a target format.</p>
        {#if !scalePagesFile}
            <DropZone ondrop={(files: File[]) => scalePagesFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{scalePagesFile.name}</span>
                <button onclick={() => scalePagesFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label>Target Format:</label>
                <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                    <button style="flex: 1; padding: 8px;" class:active={scalePagesTarget === 'a4'} onclick={() => scalePagesTarget = 'a4'}>A4</button>
                    <button style="flex: 1; padding: 8px;" class:active={scalePagesTarget === 'letter'} onclick={() => scalePagesTarget = 'letter'}>Letter</button>
                    <button style="flex: 1; padding: 8px;" class:active={scalePagesTarget === 'legal'} onclick={() => scalePagesTarget = 'legal'}>Legal</button>
                    <button style="flex: 1; padding: 8px;" class:active={scalePagesTarget === 'a3'} onclick={() => scalePagesTarget = 'a3'}>A3</button>
                    <button style="flex: 1; padding: 8px;" class:active={scalePagesTarget === 'percentage'} onclick={() => scalePagesTarget = 'percentage'}>Percentage</button>
                </div>
            </div>
            {#if scalePagesTarget === 'percentage'}
                <div class="input-group">
                    <label>Scale Percentage: {scalePagesPercentage}%</label>
                    <input type="range" bind:value={scalePagesPercentage} min="50" max="200" style="width: 100%;" />
                </div>
            {/if}
            <button class="action-btn" onclick={handleScalePages} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Scale Pages'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_auto_rotate'}
      <div class="tool-pane">
        <h2>Auto Rotate</h2>
        <p>Automatically rotate pages to match the target orientation.</p>
        {#if !autoRotateFile}
            <DropZone ondrop={(files: File[]) => autoRotateFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{autoRotateFile.name}</span>
                <button onclick={() => autoRotateFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label>Target Orientation:</label>
                <div style="display: flex; gap: 8px;">
                    <button style="flex: 1; padding: 8px;" class:active={autoRotateOrientation === 'portrait'} onclick={() => autoRotateOrientation = 'portrait'}>Portrait</button>
                    <button style="flex: 1; padding: 8px;" class:active={autoRotateOrientation === 'landscape'} onclick={() => autoRotateOrientation = 'landscape'}>Landscape</button>
                </div>
            </div>
            <button class="action-btn" onclick={handleAutoRotate} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Auto Rotate'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_stamp'}
      <div class="tool-pane">
        <h2>Add Stamp</h2>
        <p>Stamp an image (PNG/JPEG) onto a PDF document.</p>
        {#if !stampFile}
            <DropZone ondrop={(files: File[]) => stampFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{stampFile.name} (PDF)</span>
                <button onclick={() => stampFile = null}>✕</button>
            </div>
            {#if !stampImageFile}
                <div style="margin-top: 16px;">
                    <label>Select Stamp Image:</label>
                    <DropZone ondrop={(files: File[]) => stampImageFile = files[0]} multiple={false} />
                </div>
            {:else}
                <div class="selected-file" style="margin-top: 16px;">
                    <span class="file-name">{stampImageFile.name} (Stamp Image)</span>
                    <button onclick={() => stampImageFile = null}>✕</button>
                </div>
                <div class="input-group">
                    <label>Position:</label>
                    <select bind:value={stampPosition} style="width: 100%; padding: 8px;">
                        <option value="top-left">Top Left</option>
                        <option value="center">Center</option>
                        <option value="bottom-right">Bottom Right</option>
                    </select>
                </div>
                <button class="action-btn" onclick={handleStamp} disabled={isProcessing}>
                    {isProcessing ? 'Processing...' : 'Add Stamp'}
                </button>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'pdf_overlay_underlay'}
      <div class="tool-pane">
        <h2>Overlay / Underlay</h2>
        <p>Apply one PDF over or under another PDF.</p>
        {#if !overlayUnderlayFile}
            <label>Primary PDF:</label>
            <DropZone ondrop={(files: File[]) => overlayUnderlayFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{overlayUnderlayFile.name} (Primary)</span>
                <button onclick={() => overlayUnderlayFile = null}>✕</button>
            </div>
            {#if !overlayUnderlaySecondaryFile}
                <div style="margin-top: 16px;">
                    <label>Secondary PDF:</label>
                    <DropZone ondrop={(files: File[]) => overlayUnderlaySecondaryFile = files[0]} multiple={false} />
                </div>
            {:else}
                <div class="selected-file" style="margin-top: 16px;">
                    <span class="file-name">{overlayUnderlaySecondaryFile.name} (Secondary)</span>
                    <button onclick={() => overlayUnderlaySecondaryFile = null}>✕</button>
                </div>
                <div class="input-group">
                    <label>Mode:</label>
                    <div style="display: flex; gap: 8px;">
                        <button style="flex: 1; padding: 8px;" class:active={overlayUnderlayMode === 'overlay'} onclick={() => overlayUnderlayMode = 'overlay'}>Overlay (On Top)</button>
                        <button style="flex: 1; padding: 8px;" class:active={overlayUnderlayMode === 'underlay'} onclick={() => overlayUnderlayMode = 'underlay'}>Underlay (Behind)</button>
                    </div>
                </div>
                <button class="action-btn" onclick={handleOverlayUnderlay} disabled={isProcessing}>
                    {isProcessing ? 'Processing...' : 'Apply Overlay/Underlay'}
                </button>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'pdf_validate_signatures'}
      <div class="tool-pane">
        <h2>Validate Signatures</h2>
        <p>Check the validity and integrity of signatures in a PDF.</p>
        {#if !validateSignaturesFile}
            <DropZone ondrop={(files: File[]) => { validateSignaturesFile = files[0]; validateSignaturesResult = null; }} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{validateSignaturesFile.name}</span>
                <button onclick={() => { validateSignaturesFile = null; validateSignaturesResult = null; }}>✕</button>
            </div>
            <div style="margin-top: 16px;">
                <label>Baseline PDF (Optional, for tamper detection):</label>
                {#if !validateSignaturesBaselineFile}
                    <DropZone ondrop={(files: File[]) => validateSignaturesBaselineFile = files[0]} multiple={false} />
                {:else}
                    <div class="selected-file">
                        <span class="file-name">{validateSignaturesBaselineFile.name}</span>
                        <button onclick={() => validateSignaturesBaselineFile = null}>✕</button>
                    </div>
                {/if}
            </div>

            <button class="action-btn" onclick={handleValidateSignatures} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Validate Signatures'}
            </button>

            {#if validateSignaturesResult}
                <div style="margin-top: 16px; padding: 16px; background-color: var(--bg-surface); border-radius: var(--border-radius-md); border: 1px solid var(--border-color);">
                    <h3 style="margin-top: 0; font-size: 1rem; color: var(--text-secondary);">Validation Report</h3>
                    <pre style="font-family: monospace; font-size: 0.9rem; white-space: pre-wrap; word-break: break-all;">{validateSignaturesResult}</pre>
                </div>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'pdf_remove_signatures'}
      <div class="tool-pane">
        <h2>Remove Signatures</h2>
        <p>Remove all digital signatures and permissions from a PDF.</p>
        {#if !removeSignaturesFile}
            <DropZone ondrop={(files: File[]) => removeSignaturesFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{removeSignaturesFile.name}</span>
                <button onclick={() => removeSignaturesFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleRemoveSignatures} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Remove Signatures'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'sanitize'}
      <div class="tool-pane">
        <h2>Sanitize Document</h2>
        <p>Remove hidden metadata, embedded files, JavaScript, and other non-visible data.</p>
        {#if !sanitizeFile}
            <DropZone ondrop={(files: File[]) => sanitizeFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{sanitizeFile.name}</span>
                <button onclick={() => sanitizeFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleSanitize} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Sanitize Document'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pii_scan'}
      <div class="tool-pane">
        <h2>PII Scan</h2>
        <p>Scan the document for Personally Identifiable Information (PII) like emails or SSNs.</p>
        {#if !piiScanFile}
            <DropZone ondrop={(files: File[]) => { piiScanFile = files[0]; piiScanResult = null; }} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{piiScanFile.name}</span>
                <button onclick={() => { piiScanFile = null; piiScanResult = null; }}>✕</button>
            </div>
            <button class="action-btn" onclick={handlePiiScan} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Scan Document'}
            </button>
            {#if piiScanResult}
                <div style="margin-top: 16px; padding: 16px; background-color: var(--bg-surface); border-radius: var(--border-radius-md); border: 1px solid var(--border-color);">
                    <h3 style="margin-top: 0; font-size: 1rem; color: var(--text-secondary);">Scan Results</h3>
                    <pre style="font-family: monospace; font-size: 0.9rem; white-space: pre-wrap;">{piiScanResult}</pre>
                </div>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'pdf_to_docx'}
      <div class="tool-pane">
        <h2>Convert PDF to DOCX</h2>
        <p>Convert a PDF document into a Microsoft Word (.docx) file.</p>
        {#if !pdfToDocxFile}
            <DropZone ondrop={(files: File[]) => pdfToDocxFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pdfToDocxFile.name}</span>
                <button onclick={() => pdfToDocxFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handlePdfToDocx} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Convert to DOCX'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_to_gif'}
      <div class="tool-pane">
        <h2>PDF to Animated GIF</h2>
        <p>Convert multi-page PDF into an animated looping GIF preview.</p>
        {#if !pdfToGifFile}
            <DropZone ondrop={(files: File[]) => pdfToGifFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{pdfToGifFile.name}</span>
                <button onclick={() => pdfToGifFile = null}>✕</button>
            </div>

            <div class="input-group">
                <label for="webGifDelay">Frame Delay: {gifDelay} cs ({gifDelay / 100}s)</label>
                <input
                    id="webGifDelay"
                    type="range"
                    min="10"
                    max="500"
                    step="5"
                    bind:value={gifDelay}
                    style="width: 100%;"
                />
            </div>

            <div class="input-group">
                <label for="webGifWidth">Frame Width: {gifWidth}px</label>
                <input
                    id="webGifWidth"
                    type="range"
                    min="320"
                    max="1920"
                    step="20"
                    bind:value={gifWidth}
                    style="width: 100%;"
                />
            </div>

            <div class="input-group">
                <label for="webGifMaxPages">Max Pages (Optional):</label>
                <input
                    id="webGifMaxPages"
                    type="number"
                    min="1"
                    bind:value={gifMaxPages}
                    placeholder="All pages"
                    style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);"
                />
            </div>

            <button class="action-btn" onclick={handlePdfToGif} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Generate Animated GIF'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_stitch_canvas'}
      <div class="tool-pane">
        <h2>Stitch Canvas</h2>
        <p>Stitch all pages of a PDF into a single continuous vertical or horizontal strip.</p>
        {#if !stitchCanvasFile}
            <DropZone ondrop={(files: File[]) => stitchCanvasFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{stitchCanvasFile.name}</span>
                <button onclick={() => stitchCanvasFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="stitchDir">Direction:</label>
                <div style="display: flex; gap: 8px;">
                    <button style="flex: 1; padding: 8px;" class:active={stitchDirection === 'vertical'} onclick={() => stitchDirection = 'vertical'}>Vertical</button>
                    <button style="flex: 1; padding: 8px;" class:active={stitchDirection === 'horizontal'} onclick={() => stitchDirection = 'horizontal'}>Horizontal</button>
                </div>
            </div>
            <div class="input-group">
                <label for="stitchSpacing">Spacing between pages (pt): {stitchSpacing}pt</label>
                <input id="stitchSpacing" type="number" min="0" max="100" bind:value={stitchSpacing} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
            </div>
            <button class="action-btn" onclick={handleStitchCanvas} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Stitch Canvas'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_split_scanned_book'}
      <div class="tool-pane">
        <h2>Split Scanned Book</h2>
        <p>Split two-page scanned spreads down the center gutter into individual pages.</p>
        {#if !splitScannedBookFile}
            <DropZone ondrop={(files: File[]) => splitScannedBookFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{splitScannedBookFile.name}</span>
                <button onclick={() => splitScannedBookFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label>Reading Direction:</label>
                <div style="display: flex; gap: 8px;">
                    <button style="flex: 1; padding: 8px;" class:active={scannedBookReadingDirection === 'ltr'} onclick={() => scannedBookReadingDirection = 'ltr'}>Left-to-Right (Western)</button>
                    <button style="flex: 1; padding: 8px;" class:active={scannedBookReadingDirection === 'rtl'} onclick={() => scannedBookReadingDirection = 'rtl'}>Right-to-Left (Manga)</button>
                </div>
            </div>
            <button class="action-btn" onclick={handleSplitScannedBook} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Split Book Pages'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_split_by_outline'}
      <div class="tool-pane">
        <h2>Split by Outline</h2>
        <p>Extract sections or chapters based on document bookmarks and table of contents.</p>
        {#if !splitByOutlineFile}
            <DropZone ondrop={(files: File[]) => splitByOutlineFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{splitByOutlineFile.name}</span>
                <button onclick={() => splitByOutlineFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="splitOutlineDepth">Maximum Heading Depth: {splitOutlineMaxDepth}</label>
                <input id="splitOutlineDepth" type="range" min="1" max="5" bind:value={splitOutlineMaxDepth} style="width: 100%;" />
            </div>
            <button class="action-btn" onclick={handleSplitByOutline} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Split by Outline'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_merge_alternate'}
      <div class="tool-pane">
        <h2>Alternate Merge</h2>
        <p>Interleave two PDFs page-by-page (ideal for duplex scans with separate odd/even files).</p>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
            <div>
                <label style="display: block; margin-bottom: 8px; font-weight: 500;">Odd / Front Pages:</label>
                {#if !mergeAlternatePrimary}
                    <DropZone ondrop={(files: File[]) => mergeAlternatePrimary = files[0]} multiple={false} />
                {:else}
                    <div class="selected-file">
                        <span class="file-name">{mergeAlternatePrimary.name}</span>
                        <button onclick={() => mergeAlternatePrimary = null}>✕</button>
                    </div>
                {/if}
            </div>
            <div>
                <label style="display: block; margin-bottom: 8px; font-weight: 500;">Even / Back Pages:</label>
                {#if !mergeAlternateSecondary}
                    <DropZone ondrop={(files: File[]) => mergeAlternateSecondary = files[0]} multiple={false} />
                {:else}
                    <div class="selected-file">
                        <span class="file-name">{mergeAlternateSecondary.name}</span>
                        <button onclick={() => mergeAlternateSecondary = null}>✕</button>
                    </div>
                {/if}
            </div>
        </div>
        {#if mergeAlternatePrimary && mergeAlternateSecondary}
            <button class="action-btn" style="margin-top: 16px;" onclick={handleMergeAlternate} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Interleave PDFs'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_posterize'}
      <div class="tool-pane">
        <h2>Posterize Print</h2>
        <p>Tile and scale a large PDF across a multi-sheet page grid for printing large posters.</p>
        {#if !posterizeFile}
            <DropZone ondrop={(files: File[]) => posterizeFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{posterizeFile.name}</span>
                <button onclick={() => posterizeFile = null}>✕</button>
            </div>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                <div class="input-group">
                    <label for="postCols">Grid Columns: {posterizeCols}</label>
                    <input id="postCols" type="number" min="1" max="10" bind:value={posterizeCols} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
                </div>
                <div class="input-group">
                    <label for="postRows">Grid Rows: {posterizeRows}</label>
                    <input id="postRows" type="number" min="1" max="10" bind:value={posterizeRows} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
                </div>
            </div>
            <button class="action-btn" onclick={handlePosterize} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Generate ${posterizeCols}x${posterizeRows} Poster`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_attachments'}
      <div class="tool-pane">
        <h2>Attachments Manager</h2>
        <p>Inspect and extract embedded files and attachments bundled inside the PDF.</p>
        {#if !attachmentsFile}
            <DropZone ondrop={(files: File[]) => attachmentsFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{attachmentsFile.name}</span>
                <button onclick={() => attachmentsFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleAttachments} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Extract Embedded Attachments'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_dedup_streams'}
      <div class="tool-pane">
        <h2>Deduplicate Streams</h2>
        <p>Identify identical font descriptors, embedded images, and data streams to shrink document size.</p>
        {#if !dedupStreamsFile}
            <DropZone ondrop={(files: File[]) => dedupStreamsFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{dedupStreamsFile.name}</span>
                <button onclick={() => dedupStreamsFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleDedupStreams} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Deduplicate Streams'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_timestamp'}
      <div class="tool-pane">
        <h2>Cryptographic Timestamp</h2>
        <p>Embed an immutable timestamp proof certifying document state and creation time.</p>
        {#if !timestampFile}
            <DropZone ondrop={(files: File[]) => timestampFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{timestampFile.name}</span>
                <button onclick={() => timestampFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleTimestamp} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Apply Cryptographic Timestamp'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_change_permissions'}
      <div class="tool-pane">
        <h2>Security & Permissions</h2>
        <p>Enforce or customize security flags to protect sensitive documents from unauthorized changes.</p>
        {#if !changePermissionsFile}
            <DropZone ondrop={(files: File[]) => changePermissionsFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{changePermissionsFile.name}</span>
                <button onclick={() => changePermissionsFile = null}>✕</button>
            </div>
            <div style="display: flex; flex-direction: column; gap: 8px; margin-bottom: 16px;">
                <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                    <input type="checkbox" bind:checked={allowPrinting} /> Allow Printing
                </label>
                <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                    <input type="checkbox" bind:checked={allowCopying} /> Allow Copying Text/Graphics
                </label>
                <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                    <input type="checkbox" bind:checked={allowModifying} /> Allow Document Modification
                </label>
            </div>
            <button class="action-btn" onclick={handleChangePermissions} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Apply Permissions'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_to_epub'}
      <div class="tool-pane">
        <h2>PDF to Reflowable EPUB</h2>
        <p>Extract document flow, hierarchy, and text into standard EPUB eBook format.</p>
        {#if !toEpubFile}
            <DropZone ondrop={(files: File[]) => toEpubFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{toEpubFile.name}</span>
                <button onclick={() => toEpubFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="epubTitle">eBook Title:</label>
                <input id="epubTitle" type="text" placeholder="Title" bind:value={epubTitle} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
            </div>
            <div class="input-group">
                <label for="epubAuthor">Author:</label>
                <input id="epubAuthor" type="text" placeholder="Author" bind:value={epubAuthor} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
            </div>
            <button class="action-btn" onclick={handleToEpub} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Convert to EPUB'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_extract_patterns'}
      <div class="tool-pane">
        <h2>Extract Patterns</h2>
        <p>Mine structured entities (emails, URLs, phones, keywords) across all pages.</p>
        {#if !extractPatternsFile}
            <DropZone ondrop={(files: File[]) => extractPatternsFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{extractPatternsFile.name}</span>
                <button onclick={() => { extractPatternsFile = null; patternsResult = null; }}>✕</button>
            </div>
            <button class="action-btn" onclick={handleExtractPatterns} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Scan & Extract Patterns'}
            </button>
            {#if patternsResult}
                <div style="margin-top: 16px; padding: 12px; background: var(--bg-surface); border-radius: var(--border-radius-md); max-height: 250px; overflow-y: auto;">
                    <pre style="margin: 0; font-family: monospace; font-size: 13px;">{patternsResult}</pre>
                </div>
            {/if}
        {/if}
      </div>

    {:else if activeTool === 'pdf_comic_archive'}
      <div class="tool-pane">
        <h2>Comic Archive (CBZ)</h2>
        <p>Package visual PDF pages into high-fidelity Comic Book ZIP (CBZ) archives.</p>
        {#if !comicArchiveFile}
            <DropZone ondrop={(files: File[]) => comicArchiveFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{comicArchiveFile.name}</span>
                <button onclick={() => comicArchiveFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="comicDpi">Render Resolution: {comicDpi} DPI</label>
                <input id="comicDpi" type="range" min="72" max="300" step="10" bind:value={comicDpi} style="width: 100%;" />
            </div>
            <button class="action-btn" onclick={handleComicArchive} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Create Comic Archive (.cbz)'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'showcase_gif'}
      <div class="tool-pane">
        <h2>Showcase GIF</h2>
        <p>Generate a smooth animated showcase GIF preview with cross-fade transitions.</p>
        {#if !showcaseGifFile}
            <DropZone ondrop={(files: File[]) => showcaseGifFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{showcaseGifFile.name}</span>
                <button onclick={() => showcaseGifFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="sgFps">Frames Per Second: {showcaseGifFps} FPS</label>
                <input id="sgFps" type="range" min="1" max="15" bind:value={showcaseGifFps} style="width: 100%;" />
            </div>
            <div class="input-group">
                <label for="sgWidth">Width: {showcaseGifWidth}px</label>
                <input id="sgWidth" type="range" min="400" max="1200" step="50" bind:value={showcaseGifWidth} style="width: 100%;" />
            </div>
            <button class="action-btn" onclick={handleShowcaseGif} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Generate Showcase GIF'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'images_to_gif'}
      <div class="tool-pane">
        <h2>Images to Animated GIF</h2>
        <p>Combine multiple uploaded photos, drawings, or scans into an animated looping GIF.</p>
        <DropZone ondrop={(files: File[]) => imagesToGifFiles = [...imagesToGifFiles, ...files]} multiple={true} />
        {#if imagesToGifFiles.length > 0}
            <div style="margin-top: 12px; max-height: 150px; overflow-y: auto; background: var(--bg-surface); padding: 8px; border-radius: var(--border-radius-sm);">
                {#each imagesToGifFiles as f, idx}
                    <div style="display: flex; justify-content: space-between; padding: 4px 0; font-size: 13px;">
                        <span>{idx + 1}. {f.name}</span>
                        <button onclick={() => imagesToGifFiles = imagesToGifFiles.filter((_, i) => i !== idx)}>✕</button>
                    </div>
                {/each}
            </div>
            <div class="input-group" style="margin-top: 12px;">
                <label for="i2gFps">Animation Speed: {imagesToGifFps} FPS</label>
                <input id="i2gFps" type="range" min="1" max="24" bind:value={imagesToGifFps} style="width: 100%;" />
            </div>
            <button class="action-btn" onclick={handleImagesToGif} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Assemble GIF (${imagesToGifFiles.length} frames)`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'img_optimize_for_llm'}
      <div class="tool-pane">
        <h2>Optimize Image for Vision LLM</h2>
        <p>Enhance document contrast and resize for ideal token efficiency in Gemini, Claude, and GPT-4o.</p>
        {#if !optimizeLlmFile}
            <DropZone ondrop={(files: File[]) => optimizeLlmFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{optimizeLlmFile.name}</span>
                <button onclick={() => optimizeLlmFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="optMaxDim">Max Dimension: {optimizeLlmMaxDim}px</label>
                <input id="optMaxDim" type="range" min="512" max="2048" step="128" bind:value={optimizeLlmMaxDim} style="width: 100%;" />
            </div>
            <button class="action-btn" onclick={handleOptimizeLlm} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Optimize for Vision LLM'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'img_deskew'}
      <div class="tool-pane">
        <h2>Deskew Document Image</h2>
        <p>Detect document orientation and automatically correct angular skew from mobile photos and scanners.</p>
        {#if !deskewFile}
            <DropZone ondrop={(files: File[]) => deskewFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{deskewFile.name}</span>
                <button onclick={() => deskewFile = null}>✕</button>
            </div>
            <button class="action-btn" onclick={handleDeskew} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Auto-Deskew Image'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'img_convert'}
      <div class="tool-pane">
        <h2>Universal Image Converter</h2>
        <p>Convert document images between WebP, PNG, and JPEG with custom compression quality.</p>
        {#if !convertImageFile}
            <DropZone ondrop={(files: File[]) => convertImageFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{convertImageFile.name}</span>
                <button onclick={() => convertImageFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label>Target Format:</label>
                <div style="display: flex; gap: 8px;">
                    <button style="flex: 1; padding: 8px;" class:active={convertImageFormat === 'webp'} onclick={() => convertImageFormat = 'webp'}>WebP</button>
                    <button style="flex: 1; padding: 8px;" class:active={convertImageFormat === 'png'} onclick={() => convertImageFormat = 'png'}>PNG</button>
                    <button style="flex: 1; padding: 8px;" class:active={convertImageFormat === 'jpeg'} onclick={() => convertImageFormat = 'jpeg'}>JPEG</button>
                </div>
            </div>
            {#if convertImageFormat !== 'png'}
                <div class="input-group">
                    <label for="imgQuality">Quality: {convertImageQuality}%</label>
                    <input id="imgQuality" type="range" min="10" max="100" bind:value={convertImageQuality} style="width: 100%;" />
                </div>
            {/if}
            <button class="action-btn" onclick={handleConvertImage} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : `Convert to ${convertImageFormat.toUpperCase()}`}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'img_crop_to_content'}
      <div class="tool-pane">
        <h2>Auto-Crop to Content</h2>
        <p>Detect borders and automatically trim redundant margins and backgrounds around document content.</p>
        {#if !cropContentFile}
            <DropZone ondrop={(files: File[]) => cropContentFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{cropContentFile.name}</span>
                <button onclick={() => cropContentFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="cropPad">Border Padding: {cropPadding}px</label>
                <input id="cropPad" type="range" min="0" max="50" bind:value={cropPadding} style="width: 100%;" />
            </div>
            <button class="action-btn" onclick={handleCropContent} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Auto-Crop Borders'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'img_redact'}
      <div class="tool-pane">
        <h2>Redact Sensitive Image Regions</h2>
        <p>Burn permanent opaque black redaction rectangles directly into the image pixel data.</p>
        {#if !redactImageFile}
            <DropZone ondrop={(files: File[]) => redactImageFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{redactImageFile.name}</span>
                <button onclick={() => redactImageFile = null}>✕</button>
            </div>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                <div class="input-group">
                    <label for="redX">X: {redactBoxX}px</label>
                    <input id="redX" type="number" min="0" bind:value={redactBoxX} style="width: 100%; padding: 6px;" />
                </div>
                <div class="input-group">
                    <label for="redY">Y: {redactBoxY}px</label>
                    <input id="redY" type="number" min="0" bind:value={redactBoxY} style="width: 100%; padding: 6px;" />
                </div>
                <div class="input-group">
                    <label for="redW">Width: {redactBoxW}px</label>
                    <input id="redW" type="number" min="10" bind:value={redactBoxW} style="width: 100%; padding: 6px;" />
                </div>
                <div class="input-group">
                    <label for="redH">Height: {redactBoxH}px</label>
                    <input id="redH" type="number" min="10" bind:value={redactBoxH} style="width: 100%; padding: 6px;" />
                </div>
            </div>
            <button class="action-btn" onclick={handleRedactImage} disabled={isProcessing}>
                {isProcessing ? 'Processing...' : 'Burn Permanent Redaction'}
            </button>
        {/if}
      </div>

    {:else if activeTool === 'pdf_edit_text'}
      <div class="tool-pane">
        <h2>Edit PDF Text</h2>
        <p>Find and replace literal text typos in-place inside PDF content streams without rasterizing pages.</p>
        {#if !editTextFile}
            <DropZone ondrop={(files: File[]) => editTextFile = files[0]} multiple={false} />
        {:else}
            <div class="selected-file">
                <span class="file-name">{editTextFile.name}</span>
                <button onclick={() => editTextFile = null}>✕</button>
            </div>
            <div class="input-group">
                <label for="webEditTextFind">Find Text:</label>
                <input id="webEditTextFind" type="text" placeholder="Exact text to find..." bind:value={editTextFind} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
            </div>
            <div class="input-group">
                <label for="webEditTextReplace">Replace With:</label>
                <input id="webEditTextReplace" type="text" placeholder="Corrected replacement text..." bind:value={editTextReplace} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
            </div>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; align-items: end;">
                <div class="input-group">
                    <label for="webEditTextPage">Page Number (optional):</label>
                    <input id="webEditTextPage" type="number" min="1" placeholder="All pages" bind:value={editTextPage} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
                </div>
                <div class="input-group">
                    <label for="webEditTextOccurrence">Occurrence (1-indexed, optional):</label>
                    <input id="webEditTextOccurrence" type="number" min="1" placeholder="All occurrences" bind:value={editTextOccurrence} style="width: 100%; padding: 8px; border-radius: var(--border-radius-sm); border: 1px solid var(--border-color); background: var(--bg-surface); color: var(--text-primary);" />
                </div>
            </div>
            <div class="input-group" style="display: flex; align-items: center; gap: 8px; margin-bottom: 16px;">
                <input id="webEditTextMatchCase" type="checkbox" bind:checked={editTextMatchCase} />
                <label for="webEditTextMatchCase" style="margin: 0; cursor: pointer;">Match case</label>
            </div>
            <button class="action-btn" onclick={handlePdfEditText} disabled={isProcessing || !editTextFind}>
                {isProcessing ? 'Processing...' : 'Replace Text in PDF'}
            </button>
        {/if}
      </div>

    {/if}
  </div>
</div>

<style>
  .operations-view {
    display: flex;
    gap: 32px;
    height: 100%;
    max-width: 1400px;
    margin: 0 auto;
  }

  .tools-sidebar {
    width: 350px;
    flex-shrink: 0;
    background-color: var(--bg-secondary);
    border-radius: var(--border-radius-lg);
    border: 1px solid var(--border-color);
    padding: 16px;
    max-height: 80vh;
    overflow-y: auto;
  }

  .sidebar-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .tools-sidebar h3 {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .filter-toggle {
    display: flex;
    gap: 4px;
    background: rgba(255, 255, 255, 0.05);
    padding: 2px;
    border-radius: var(--border-radius-sm, 6px);
    border: 1px solid var(--border-color, #2a2a35);
  }

  .toggle-btn {
    padding: 2px 8px;
    font-size: 0.75rem;
    font-weight: 500;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .toggle-btn.active {
    background: var(--bg-surface-hover, rgba(255, 255, 255, 0.1));
    color: var(--text-primary);
  }

  .toggle-btn.toggle-ready.active {
    background: rgba(34, 197, 94, 0.2);
    color: #4ade80;
  }

  .tools-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .tool-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--border-radius-md);
    color: var(--text-primary);
    cursor: pointer;
    transition: all var(--transition-fast);
    text-align: left;
    font-size: 0.95rem;
    position: relative;
  }

  .tool-title-text {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tool-btn:hover {
    background-color: var(--bg-surface-hover);
  }

  .tool-btn.active {
    background-color: rgba(79, 70, 229, 0.1);
    color: var(--accent-primary);
    border-color: rgba(79, 70, 229, 0.2);
  }

  .tool-icon {
    font-size: 1.2rem;
  }

  .tool-content {
    flex: 1;
    background-color: var(--bg-secondary);
    border-radius: var(--border-radius-lg);
    border: 1px solid var(--border-color);
    padding: 32px;
    overflow-y: auto;
  }

  .tool-pane h2 {
    margin-top: 0;
    margin-bottom: 8px;
  }

  .tool-pane p {
    color: var(--text-secondary);
    margin-bottom: 24px;
  }

  .file-list {
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .file-item, .selected-file {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
  }

  .file-name {
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-controls {
    display: flex;
    gap: 8px;
  }

  .file-controls button, .selected-file button {
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 4px 8px;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .file-controls button:hover:not(:disabled), .selected-file button:hover {
      background: var(--bg-surface-hover);
      color: var(--text-primary);
  }

  .file-controls button:disabled {
      opacity: 0.3;
      cursor: not-allowed;
  }

  .action-btn {
    margin-top: 24px;
    width: 100%;
    padding: 12px;
    background-color: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--border-radius-md);
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color var(--transition-fast);
  }

  .action-btn:hover:not(:disabled) {
    background-color: var(--accent-hover);
  }

  .action-btn:disabled {
    background-color: var(--bg-surface-hover);
    color: var(--text-muted);
    cursor: not-allowed;
  }

  .input-group {
      margin-top: 16px;
      display: flex;
      flex-direction: column;
      gap: 8px;
  }

  .input-group label {
      font-size: 0.9rem;
      font-weight: 500;
      color: var(--text-secondary);
  }

  .input-group input, .input-group select {
      padding: 10px 12px;
      background: var(--bg-surface);
      border: 1px solid var(--border-color);
      border-radius: var(--border-radius-md);
      color: var(--text-primary);
      font-size: 1rem;
  }

  @media (max-width: 768px) {
    .operations-view {
      flex-direction: column;
      gap: 16px;
    }
    .tools-sidebar {
      width: 100%;
      max-height: 180px;
      padding: 12px;
    }
    .tools-list {
      display: flex;
      flex-direction: row;
      flex-wrap: nowrap;
      overflow-x: auto;
      gap: 8px;
      padding-bottom: 6px;
      -webkit-overflow-scrolling: touch;
    }
    .tool-btn {
      flex: 0 0 auto;
      min-width: 120px;
      min-height: 44px;
      padding: 8px 12px;
    }
    .tool-content {
      padding: 16px;
      min-height: 480px;
    }
    .action-btn {
      min-height: 48px;
      font-size: 1.05rem;
    }
    .file-controls button, .selected-file button {
      min-height: 44px;
      min-width: 44px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
    }
  }

  @media (max-width: 480px) {
    .tool-pane h2 {
      font-size: 1.25rem;
    }
    .tools-sidebar {
      max-height: 150px;
    }
    .tool-content {
      padding: 12px;
    }
  }
</style>
