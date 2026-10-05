<script lang="ts">
  import { appState } from '$lib/state/app.svelte';
  import { toastState } from '$lib/state/toast.svelte';
  import { jobsState } from '$lib/state/jobs.svelte';
  import { invoke } from '@tauri-apps/api/core';

  interface ToolDefinition {
    id: string;
    title: string;
    description: string;
    icon: string;
    category: 'quick' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert' | 'ai';
    tags: string[];
  }

  const allTools: ToolDefinition[] = [
    // Quick / Popular
    { id: 'compress', title: 'Compress PDF', description: 'Reduce document file size with image optimization', icon: '🗜️', category: 'quick', tags: ['shrink', 'size', 'optimize'] },
    { id: 'merge', title: 'Merge PDFs', description: 'Combine multiple PDF files into one continuous document', icon: '📑', category: 'quick', tags: ['combine', 'join', 'append'] },
    { id: 'rotate', title: 'Rotate Pages', description: 'Rotate pages by 90, 180, or 270 degrees', icon: '🔄', category: 'quick', tags: ['orientation', 'turn'] },
    { id: 'compare', title: 'Compare PDFs', description: 'Diff two documents and highlight changes', icon: '🔍', category: 'quick', tags: ['diff', 'compare', 'difference'] },
    { id: 'ocr', title: 'OCR Text Recognition', description: 'Extract text from scanned pages using local OCR', icon: '👁️', category: 'quick', tags: ['scan', 'text', 'extract'] },
    { id: 'pdf_to_docx', title: 'PDF → Word', description: 'Convert document into editable DOCX format', icon: '📝', category: 'quick', tags: ['word', 'docx', 'convert', 'export'] },

    // Page Operations
    { id: 'merge', title: 'Merge Documents', description: 'Combine open tabs into a single PDF', icon: '📑', category: 'pages', tags: ['combine', 'join', 'concat'] },
    { id: 'split', title: 'Split PDF', description: 'Divide document into separate pages or ranges', icon: '✂️', category: 'pages', tags: ['divide', 'cut', 'separate'] },
    { id: 'compare', title: 'Compare Documents', description: 'Side-by-side comparison between two open tabs', icon: '🔍', category: 'pages', tags: ['diff', 'compare'] },
    { id: 'extract_pages', title: 'Extract Pages', description: 'Save specific pages into a new PDF document', icon: '📤', category: 'pages', tags: ['select', 'isolate'] },
    { id: 'rotate', title: 'Rotate Document', description: 'Orient landscape or portrait pages', icon: '🔄', category: 'pages', tags: ['orientation', 'turn', 'flip'] },

    // Edit & Markup
    { id: 'watermark', title: 'Add Watermark', description: 'Stamp text or branding across all pages', icon: '💧', category: 'edit', tags: ['stamp', 'text', 'brand', 'copyright'] },
    { id: 'flatten', title: 'Flatten Annotations', description: 'Bake annotations & forms permanently into page graphics', icon: '📄', category: 'edit', tags: ['bake', 'lock', 'rasterize'] },

    // Optimize & Repair
    { id: 'compress', title: 'Compress PDF', description: 'Lossy & lossless file size reduction', icon: '🗜️', category: 'optimize', tags: ['shrink', 'size', 'dpi'] },
    { id: 'repair', title: 'Repair PDF', description: 'Recover and rebuild corrupted or unreadable documents', icon: '🔧', category: 'optimize', tags: ['corrupt', 'fix', 'recover'] },

    // Security & Privacy
    { id: 'encrypt', title: 'Encrypt with Password', description: 'Protect document with AES-256 password encryption', icon: '🔒', category: 'security', tags: ['protect', 'password', 'lock'] },
    { id: 'decrypt', title: 'Remove Password', description: 'Remove password restrictions from unlocked PDF', icon: '🔓', category: 'security', tags: ['unlock', 'open'] },
    { id: 'metadata', title: 'Edit / Sanitize Metadata', description: 'Inspect and clean author, title, and creation timestamps', icon: '📋', category: 'security', tags: ['sanitize', 'privacy', 'info'] },

    // Convert & Export
    { id: 'pdf_to_docx', title: 'PDF to Word (DOCX)', description: 'Export document as editable Microsoft Word file', icon: '📝', category: 'convert', tags: ['word', 'docx'] },
    { id: 'pdf_to_xlsx', title: 'PDF to Excel (XLSX)', description: 'Extract tabular data into spreadsheets', icon: '📊', category: 'convert', tags: ['excel', 'sheet', 'table'] },
    { id: 'pdf_to_markdown', title: 'PDF to Markdown', description: 'Convert clean layout into LLM-ready markdown text', icon: '📑', category: 'convert', tags: ['llm', 'markdown', 'md'] },
    { id: 'extract_text', title: 'Extract Plain Text', description: 'Extract all raw textual content to a .txt file', icon: '📄', category: 'convert', tags: ['text', 'dump', 'raw'] },

    // AI & Intelligence
    { id: 'ocr', title: 'OCR Recognition', description: 'Run optical character recognition on scanned pages', icon: '👁️', category: 'ai', tags: ['ocr', 'scan', 'tesseract'] },
    { id: 'extract_images', title: 'Extract Embedded Images', description: 'Save all images found inside the document', icon: '🖼️', category: 'ai', tags: ['images', 'photos', 'export'] },
    // Stirling Parity Tools
    { id: 'remove_blank', title: 'Remove Blank Pages', description: 'Remove empty or nearly empty pages', icon: '🧹', category: 'pages', tags: ['blank', 'empty', 'clean', 'delete'] },
    { id: 'page_numbers', title: 'Add Page Numbers', description: 'Add dynamic page numbering', icon: '🔢', category: 'pages', tags: ['numbering', 'bates', 'header', 'footer', 'page'] },
    { id: 'md_to_pdf', title: 'Markdown to PDF', description: 'Convert markdown text to PDF', icon: '📝', category: 'convert', tags: ['markdown', 'convert', 'pdf', 'generate'] },
    { id: 'html_to_pdf', title: 'HTML to PDF', description: 'Convert HTML string to styled PDF', icon: '🌐', category: 'convert', tags: ['html', 'web', 'convert', 'pdf'] },
    { id: 'img_to_pdf', title: 'Images to PDF', description: 'Combine multiple images into a PDF', icon: '🖼️', category: 'convert', tags: ['image', 'jpg', 'png', 'combine', 'photo'] },

    // Additional 44 Tools Required Additions
    // Organize & Pages
    { id: 'delete_pages', title: 'Delete Pages', description: 'Remove specific pages from the document', icon: '🗑️', category: 'pages', tags: ['delete', 'remove', 'pages'] },
    { id: 'reorder_pages', title: 'Reorder Pages', description: 'Change the order of pages in the document', icon: '🔁', category: 'pages', tags: ['reorder', 'sort', 'arrange'] },
    { id: 'burst', title: 'Burst PDF', description: 'Split document into single pages', icon: '💥', category: 'pages', tags: ['burst', 'split', 'single'] },
    { id: 'crop', title: 'Crop Pages', description: 'Crop pages to a specific rectangular area', icon: '✂️', category: 'pages', tags: ['crop', 'trim', 'cut'] },

    // Security & Integrity
    { id: 'redact', title: 'Redact Text', description: 'Permanently remove sensitive information', icon: '⬛', category: 'security', tags: ['redact', 'hide', 'censor', 'blackout'] },
    { id: 'sign', title: 'Sign Document', description: 'Apply digital signature to document', icon: '✒️', category: 'security', tags: ['sign', 'signature', 'cert'] },
    { id: 'validate', title: 'Validate PDF', description: 'Verify document structure and standards', icon: '✅', category: 'security', tags: ['validate', 'verify', 'check'] },
    { id: 'hash', title: 'Generate Hash', description: 'Calculate document checksums', icon: '#️⃣', category: 'security', tags: ['hash', 'checksum', 'md5', 'sha'] },

    // Content & OCR
    { id: 'search', title: 'Search Text', description: 'Search for text across the document', icon: '🔍', category: 'ai', tags: ['search', 'find', 'text'] },
    { id: 'bates', title: 'Bates Numbering', description: 'Apply Bates stamps to pages', icon: '🔢', category: 'edit', tags: ['bates', 'stamp', 'legal'] },
    { id: 'header_footer', title: 'Header & Footer', description: 'Add headers and footers to pages', icon: '📏', category: 'edit', tags: ['header', 'footer', 'header/footer', 'margin'] },
    { id: 'render', title: 'Render Page', description: 'Render page to image format', icon: '🖼️', category: 'convert', tags: ['render', 'image', 'png'] },

    // Forms & Optimization
    { id: 'read_form', title: 'Read Form Data', description: 'Extract data from PDF forms', icon: '📋', category: 'edit', tags: ['form', 'read', 'extract'] },
    { id: 'fill_form', title: 'Fill Form Data', description: 'Fill PDF form fields automatically', icon: '✍️', category: 'edit', tags: ['form', 'fill', 'data'] },
    { id: 'create_form_field', title: 'Add Form Field', description: 'Create new interactive form fields', icon: '➕', category: 'edit', tags: ['form', 'create', 'field'] },
    { id: 'bookmarks', title: 'Extract Bookmarks', description: 'Read document outline and bookmarks', icon: '🔖', category: 'pages', tags: ['bookmarks', 'outline', 'toc'] },
    { id: 'linearize', title: 'Fast Web View', description: 'Optimize PDF for fast web viewing (linearize)', icon: '⚡', category: 'optimize', tags: ['web', 'fast', 'linearize'] },

    // Conversions & Intelligence
    { id: 'pdf_to_pptx', title: 'PDF to PowerPoint', description: 'Export document as PPTX presentation', icon: '📊', category: 'convert', tags: ['powerpoint', 'pptx', 'presentation'] },
    { id: 'pdf_to_pdf_a', title: 'Convert to PDF/A', description: 'Archive-ready PDF format conversion', icon: '🏛️', category: 'convert', tags: ['archive', 'pdfa', 'long-term'] },
    { id: 'classify_type', title: 'Classify Document', description: 'AI classification of document type', icon: '🏷️', category: 'ai', tags: ['classify', 'type', 'classify pdf', 'ai'] },
    { id: 'annotate', title: 'Add Annotations', description: 'Programmatically add annotations', icon: '✏️', category: 'edit', tags: ['annotate', 'draw', 'markup'] },
    { id: 'pdf_convert_excel', title: 'CSV to PDF', description: 'Convert tabular data to PDF', icon: '📄', category: 'convert', tags: ['csv', 'excel', 'convert'] }
  ];

  const categories = [
    { id: 'quick', title: 'Quick Actions', icon: '⚡' },
    { id: 'pages', title: 'Page Management', icon: '📑' },
    { id: 'edit', title: 'Edit & Markup', icon: '✍️' },
    { id: 'optimize', title: 'Optimize & Repair', icon: '🗜️' },
    { id: 'security', title: 'Security & Privacy', icon: '🔒' },
    { id: 'convert', title: 'Convert & Export', icon: '🔄' },
    { id: 'ai', title: 'AI & Intelligence', icon: '🧠' }
  ];

  const allCategories = [
    { id: 'all', title: 'All', icon: '✨' },
    ...categories
  ];

  import { onMount } from 'svelte';

  let searchQuery = $state('');
  let selectedCategory = $state('all');
  let activeTool = $state<ToolDefinition | null>(null);
  let isCollapsed = $state(false);

  onMount(() => {
    const stored = localStorage.getItem('operations-panel-collapsed');
    if (stored === 'true') isCollapsed = true;
  });

  $effect(() => {
    localStorage.setItem('operations-panel-collapsed', String(isCollapsed));
  });

  // Tool parameter states
  let splitPoints = $state('');
  let compressQuality = $state(80);
  let rotateAngle = $state('90');
  let watermarkText = $state('');
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

  // Filter tools based on search query
  let filteredTools = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    let tools = allTools;

    if (q) {
      tools = tools.filter(t =>
        t.title.toLowerCase().includes(q) ||
        t.description.toLowerCase().includes(q) ||
        t.tags.some(tag => tag.toLowerCase().includes(q))
      );
    } else if (selectedCategory !== 'all') {
      tools = tools.filter(t => t.category === selectedCategory);
    }
    return tools;
  });

  function selectTool(tool: ToolDefinition) {
    activeTool = tool;
  }

  function backToDirectory() {
    activeTool = null;
  }

  let customOutputPath = $state('');

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
    const base = docName.replace(/\.pdf$/i, '');

    switch (activeTool.id) {
      case 'merge':
      case 'split':
        customOutputPath = ''; // handled by native dialog
        break;
      case 'compress':      customOutputPath = getOutputPath(`${base}_compressed.pdf`);  break;
      case 'rotate':        customOutputPath = getOutputPath(`${base}_rotated.pdf`);     break;
      case 'watermark':     customOutputPath = getOutputPath(`${base}_watermarked.pdf`); break;
      case 'encrypt':       customOutputPath = getOutputPath(`${base}_encrypted.pdf`);   break;
      case 'decrypt':       customOutputPath = getOutputPath(`${base}_decrypted.pdf`);   break;
      case 'metadata':      customOutputPath = getOutputPath(`${base}_metadata.pdf`);    break;
      case 'extract_pages': customOutputPath = getOutputPath(`${base}_extracted.pdf`);  break;
      case 'extract_text':
      case 'ocr':           customOutputPath = getOutputPath(`${base}_text.txt`);        break;
      case 'pdf_to_docx':     customOutputPath = getOutputPath(`${base}.docx`);           break;
      case 'pdf_to_xlsx':     customOutputPath = getOutputPath(`${base}.xlsx`);           break;
      case 'pdf_to_markdown': customOutputPath = getOutputPath(`${base}.md`);             break;
      case 'pdf_to_pptx':     customOutputPath = getOutputPath(`${base}.pptx`);           break;
      case 'extract_images':  customOutputPath = getOutputPath(`${base}_images`);          break;
      default:              customOutputPath = getOutputPath(`${base}_output.pdf`);      break;
    }
  });

  async function handleRunOperation() {
    if (!activeTool) return;

    if (activeTool.id === 'compare') {
      if (appState.documents.length < 2) {
        toastState.error('Need at least 2 documents loaded to compare.');
        return;
      }
      appState.toggleDiffView(true);
      return;
    }

    const isCreationTool = ['md_to_pdf', 'html_to_pdf', 'img_to_pdf'].includes(activeTool.id);
    if (!isCreationTool && appState.documents.length === 0) {
      toastState.error('No documents available. Add or drop a PDF first.');
      return;
    }

    if (activeTool.id === 'merge' && appState.documents.length < 2) {
      toastState.error('Need at least 2 documents in tabs to merge.');
      return;
    }

    const docIndex = appState.selectedDocumentIndex ?? 0;
    const docName = appState.documents[docIndex]?.name || appState.documents[0].name;
    const docPath = appState.documentPaths[docIndex] || docName;
    const docDir = docPath.includes('/') ? docPath.substring(0, docPath.lastIndexOf('/')) : '';
    const getOutputPath = (filename: string) => docDir ? `${docDir}/${filename}` : filename;

    let toolName = `pdf_${activeTool.id}`;
    if (activeTool.id === 'md_to_pdf') toolName = 'pdf_convert_markdown';
    if (activeTool.id === 'html_to_pdf') toolName = 'pdf_convert_html';
    if (activeTool.id === 'img_to_pdf') toolName = 'pdf_images_to_pdf';

    // Forms and conversions mapping
    if (['create_form_field', 'fill_form', 'read_form'].includes(activeTool.id)) {
        toolName = `pdf_${activeTool.id}`;
    } else if (['pdf_to_docx', 'pdf_to_xlsx', 'pdf_to_pptx', 'pdf_to_pdf_a'].includes(activeTool.id)) {
        toolName = activeTool.id;
    } else if (activeTool.id === 'pdf_convert_excel') {
        toolName = 'pdf_convert_excel';
    }

    let args: Record<string, any> = {};

    switch (activeTool.id) {
      case 'merge': {
        let outputPath: string | null = customOutputPath;
        const suggested = getOutputPath(`${docName.replace(/\.pdf$/i, '')}_merged.pdf`);
        if (!outputPath) {
          try {
            const { save } = await import('@tauri-apps/plugin-dialog');
            outputPath = await save({
              title: 'Save Merged PDF As...',
              defaultPath: suggested,
              filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
            });
          } catch(err) {
             console.warn('Tauri dialog unavailable, falling back:', err);
          }
        }
        const finalPath = outputPath || suggested;
        args = {
          inputs: appState.documents.map((d, i) => appState.documentPaths[i] || d.name),
          output: finalPath
        };
        break;
      }
      case 'split': {
        let outputPath: string | null = customOutputPath;
        const suggested = getOutputPath(`${docName.replace(/\.pdf$/i, '')}_split`);
        if (!outputPath) {
          try {
            const { save } = await import('@tauri-apps/plugin-dialog');
            outputPath = await save({
              title: 'Save Split PDFs to Folder (choose base filename)...',
              defaultPath: suggested,
              filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
            });
          } catch(err) {
            console.warn('Tauri dialog unavailable, falling back:', err);
          }
        }

        const finalPath = outputPath || suggested;
        const outputDir = finalPath.replace(/\.pdf$/i, '');
        args = {
          input: docPath,
          output_pattern: outputDir + "_p%d.pdf",
          ranges: splitPoints
        };
        break;
      }
      case 'md_to_pdf': {
        const { open, save } = await import('@tauri-apps/plugin-dialog');
        const selected = await open({
          title: 'Select Markdown File',
          multiple: false,
          filters: [{ name: 'Markdown', extensions: ['md', 'markdown'] }]
        });
        if (!selected) return;
        const inputPath = Array.isArray(selected) ? selected[0] : selected;

        const suggested = getOutputPath('output.pdf');
        const outputPath = await save({
          title: 'Save PDF As...',
          defaultPath: suggested,
          filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
        });
        if (!outputPath) return;

        args = {
          input: inputPath,
          output: outputPath,
          preset: convertPreset
        };
        break;
      }
      case 'html_to_pdf': {
        const { open, save } = await import('@tauri-apps/plugin-dialog');
        const selected = await open({
          title: 'Select HTML File',
          multiple: false,
          filters: [{ name: 'HTML', extensions: ['html', 'htm'] }]
        });
        if (!selected) return;
        const inputPath = Array.isArray(selected) ? selected[0] : selected;

        const suggested = getOutputPath('output.pdf');
        const outputPath = await save({
          title: 'Save PDF As...',
          defaultPath: suggested,
          filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
        });
        if (!outputPath) return;

        args = {
          input: inputPath,
          output: outputPath,
          preset: convertPreset
        };
        break;
      }
      case 'img_to_pdf': {
        const { open, save } = await import('@tauri-apps/plugin-dialog');
        const selected = await open({
          title: 'Select Images',
          multiple: true,
          filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg'] }]
        });
        if (!selected || selected.length === 0) return;
        const inputPaths = Array.isArray(selected) ? selected : [selected];

        const suggested = getOutputPath('output.pdf');
        const outputPath = await save({
          title: 'Save PDF As...',
          defaultPath: suggested,
          filters: [{ name: 'PDF Document', extensions: ['pdf'] }]
        });
        if (!outputPath) return;

        args = {
          inputs: inputPaths,
          output: outputPath
        };
        break;
      }
      case 'compress':
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_compressed.pdf`)
        };
        break;
      case 'rotate':
        args = {
          input: docPath,
          pages: 'all',
          angle: parseInt(rotateAngle, 10),
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_rotated.pdf`)
        };
        break;
      case 'watermark':
        args = {
          input: docPath,
          text: watermarkText || 'CONFIDENTIAL',
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_watermarked.pdf`)
        };
        break;
      case 'delete_pages':
        args = {
          input: docPath,
          pages: '1',
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_deleted.pdf`)
        };
        break;
      case 'reorder_pages':
        args = {
          input: docPath,
          order: reorderList || '1',
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_reordered.pdf`)
        };
        break;
      case 'burst':
        args = {
          input: docPath,
          output_dir: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_burst`)
        };
        break;
      case 'crop':
        args = {
          input: docPath,
          box: cropBox || '0,0,100,100',
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_cropped.pdf`)
        };
        break;
      case 'redact':
        args = {
          input: docPath,
          regions: '0,0,100,100', // Provide a default/simple region or add an input if needed
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_redacted.pdf`)
        };
        break;
      case 'sign':
        args = {
          input: docPath,
          cert: signCertPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_signed.pdf`)
        };
        break;
      case 'validate':
      case 'hash':
      case 'search':
      case 'read_form':
      case 'bookmarks':
      case 'classify_type':
        args = {
          input: docPath
        };
        if (activeTool.id === 'search') {
          args.query = 'text'; // Replace with a bound value if needed later
        }
        break;
      case 'bates':
        args = {
          input: docPath,
          prefix: batesPrefix,
          start_number: batesStart,
          padding: batesPadding,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_bates.pdf`)
        };
        break;
      case 'header_footer':
        args = {
          input: docPath,
          header_left: headerText,
          footer_center: footerText,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_header.pdf`)
        };
        break;
      case 'render':
        args = {
          input: docPath,
          page: 1,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_rendered.png`)
        };
        break;
      case 'fill_form':
        args = {
          input: docPath,
          values: {}, // You'd likely want to pass actual values here or add UI for it
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_filled.pdf`)
        };
        break;
      case 'create_form_field':
        args = {
          input: docPath,
          field_name: 'new_field', // Add to UI if needed
          field_type: 'text',
          x: 50.0,
          y: 50.0,
          width: 100.0,
          height: 30.0,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_field.pdf`)
        };
        break;
      case 'linearize':
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_linearized.pdf`)
        };
        break;
      case 'annotate':
        args = {
          input: docPath,
          annotations: [], // Placeholder for annotations
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_annotated.pdf`)
        };
        break;
      case 'encrypt':
        args = {
          input: docPath,
          password: password || 'paperpilot',
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_encrypted.pdf`)
        };
        break;
      case 'decrypt':
        args = {
          input: docPath,
          password: password || '',
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_decrypted.pdf`)
        };
        break;
      case 'metadata':
        args = {
          input: docPath,
          title: metadataTitle || docName,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_metadata.pdf`)
        };
        break;
      case 'extract_pages':
        args = {
          input: docPath,
          pages: '1',
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_extracted.pdf`)
        };
        break;
      case 'extract_text':
      case 'ocr':
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_text.txt`)
        };
        break;
      case 'pdf_to_docx':
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.docx`)
        };
        break;
      case 'pdf_to_xlsx':
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.xlsx`)
        };
        break;
      case 'pdf_to_markdown':
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.md`)
        };
        break;
      case 'pdf_to_pptx':
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}.pptx`)
        };
        break;
      case 'extract_images':
        args = {
          input: docPath,
          output_dir: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_images`)
        };
        break;
      default:
        args = {
          input: docPath,
          output: customOutputPath || getOutputPath(`${docName.replace(/\.pdf$/i, '')}_output.pdf`)
        };
        break;
    }

    appState.setLoading(true);
    try {
      const result = await invoke('invoke_mcp_tool', { toolName, arguments: args });
      const resObj = result as any;
      if (resObj && resObj.success) {
        const outDest = resObj.output_path || customOutputPath || args.output || args.output_dir;
        const msg = outDest ? `✅ ${activeTool.title} saved to: ${outDest}` : (resObj.message || `${activeTool.title} completed successfully`);
        toastState.success(msg);
        jobsState.addJob(toolName, 'success', msg);
      } else {
        toastState.error(resObj?.message || `${activeTool.title} failed`);
        jobsState.addJob(toolName, 'error', resObj?.message);
      }
    } catch (error) {
      const errMsg = typeof error === 'string' ? error : (error as Error).message || 'Unknown error occurred';
      toastState.error(errMsg);
      jobsState.addJob(toolName, 'error', errMsg);
    } finally {
      appState.setLoading(false);
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

<div class="operations-panel" class:collapsed={isCollapsed} id="operations-panel">
  <button
    class="collapse-toggle"
    id="operations-panel-toggle"
    onclick={() => isCollapsed = !isCollapsed}
    title={isCollapsed ? 'Expand tools panel' : 'Collapse tools panel'}
    aria-label={isCollapsed ? 'Expand tools panel' : 'Collapse tools panel'}
  >
    {isCollapsed ? '‹' : '›'}
  </button>

  {#if !isCollapsed}
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
      {:else if activeTool.id === 'watermark'}
        <div class="operation-config">
          <label for="watermarkText" class="section-desc">Watermark Text:</label>
          <input id="watermarkText" type="text" class="form-input" bind:value={watermarkText} placeholder="e.g. CONFIDENTIAL" />
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
      {:else if activeTool.id === 'pdf_to_pptx' || activeTool.id === 'pdf_to_pdf_a'}
        <div class="operation-config">
          <p class="section-desc">Export destination can be modified below.</p>
        </div>
      {:else}
        <div class="operation-config">
          <p class="section-desc">Applies directly to the active document.</p>
        </div>
      {/if}

      {#if activeTool && activeTool.id !== 'merge' && activeTool.id !== 'split' && activeTool.id !== 'md_to_pdf' && activeTool.id !== 'html_to_pdf' && activeTool.id !== 'img_to_pdf'}
        <div class="operation-config" id="output-path-config">
          <label for="output-path-input" class="section-desc">Output file:</label>
          <div class="output-path-row">
            <input
              id="output-path-input"
              type="text"
              class="form-input output-path-input"
              bind:value={customOutputPath}
              placeholder="Output file path..."
              title="Edit the output file path"
            />
          </div>
        </div>
      {/if}

      <div class="action-area">
        <button
          class="run-btn"
          id="btn-run-operation"
          disabled={!activeTool || (!['md_to_pdf', 'html_to_pdf', 'img_to_pdf'].includes(activeTool.id) && appState.documents.length === 0)}
          onclick={handleRunOperation}
        >
          Run {activeTool.title}
        </button>
      </div>
    </div>
  {:else}
    <!-- DIRECTORY MODE (STIRLING PDF STYLE) -->
    <div class="panel-header">
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
    </div>

    <div class="panel-content">
      {#if searchQuery.trim() || selectedCategory !== 'all'}
        <!-- Search Results View -->
        <div class="search-results-list">
          <span class="category-header">{searchQuery.trim() ? 'Search Results' : allCategories.find(c => c.id === selectedCategory)?.title} ({filteredTools.length})</span>
          {#if filteredTools.length === 0}
            <div class="empty-state">No matching tools found</div>
          {:else}
            {#each filteredTools as tool (tool.id + tool.category)}
              <button class="tool-card" onclick={() => selectTool(tool)} id="tool-btn-{tool.id}">
                <span class="tool-card-icon">{tool.icon}</span>
                <div class="tool-card-info">
                  <span class="tool-card-title">{tool.title}</span>
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
                      <span class="tool-card-title">{tool.title}</span>
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
</style>
