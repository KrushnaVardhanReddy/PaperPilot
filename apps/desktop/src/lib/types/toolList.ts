import type { ToolDefinition } from './tools';

export const allTools: ToolDefinition[] = [
  { id: 'pdf_nup', name: 'N-Up Page Imposition', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Combine multiple pages per sheet in a 2, 4, 9, or 16-up grid layout.', icon: '📑', category: 'organize', context: 'pdf', tags: ['pdf_nup'], releaseStatus: 'preview' },
  { id: 'pdf_booklet', title: 'Booklet Imposition', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Reorder and impose pages for 2-sided folding booklet printing.', icon: '📖', category: 'pages', context: 'pdf', tags: ['pdf_booklet'], releaseStatus: 'preview' },
  { id: 'pdf_validate_signatures', title: 'Validate Digital Signatures', inputFormat: 'PDF', outputFormat: '.JSON', description: 'Cryptographically verify digital signatures, certificate chains, and document integrity.', icon: '🔏', category: 'security', context: 'pdf', tags: ['pdf_validate_signatures'], releaseStatus: 'preview' },
  // Quick / Popular
  { id: 'compress', title: 'Compress PDF', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Reduce document file size with image optimization', icon: '🗜️', category: 'quick', context: 'pdf', tags: ['shrink', 'size', 'optimize'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], isBatchable: true } },
  { id: 'merge', title: 'Merge PDFs', inputFormat: '2+ PDFs', outputFormat: '.PDF', description: 'Combine multiple PDF files into one continuous document', icon: '📑', category: 'quick', context: 'pdf', tags: ['combine', 'join', 'append'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], minDocuments: 2 } },
  { id: 'rotate', title: 'Rotate Pages', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Rotate pages by 90, 180, or 270 degrees', icon: '🔄', category: 'quick', context: 'pdf', tags: ['orientation', 'turn'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], isBatchable: true } },
  { id: 'compare', title: 'Compare PDFs', inputFormat: '2 PDFs', outputFormat: '.PDF', description: 'Diff two documents and highlight changes', icon: '🔍', category: 'quick', context: 'pdf', tags: ['diff', 'compare', 'difference'], releaseStatus: 'preview', rule: { supportedExtensions: ['pdf'], minDocuments: 2 } },
  { id: 'ocr', title: 'OCR Text Recognition', inputFormat: 'Scanned PDF / Image', outputFormat: '.TXT / PDF', description: 'Extract text from scanned pages using local OCR', icon: '👁️', category: 'quick', context: 'pdf', tags: ['scan', 'text', 'extract'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], isBatchable: true } },
  { id: 'pdf_to_docx', title: 'PDF to Word (DOCX)', inputFormat: 'PDF', outputFormat: '.DOCX', description: 'Convert document into editable DOCX format', icon: '📝', category: 'quick', context: 'pdf', tags: ['word', 'docx', 'convert', 'export'], releaseStatus: 'preview', rule: { supportedExtensions: ['pdf'], isBatchable: true } },

  // Page Operations
  { id: 'merge', title: 'Merge Documents', inputFormat: '2+ PDFs', outputFormat: '.PDF', description: 'Combine open tabs into a single PDF', icon: '📑', category: 'pages', context: 'pdf', tags: ['combine', 'join', 'concat'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], minDocuments: 2 } },
  { id: 'split', title: 'Split PDF', inputFormat: 'PDF', outputFormat: 'Multiple .PDFs', description: 'Divide document into separate pages or ranges', icon: '✂️', category: 'pages', context: 'pdf', tags: ['divide', 'cut', 'separate'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], maxDocuments: 1 } },
  { id: 'compare', title: 'Compare Documents', inputFormat: '2 PDFs', outputFormat: '.PDF', description: 'Side-by-side comparison between two open tabs', icon: '🔍', category: 'pages', context: 'pdf', tags: ['diff', 'compare'], releaseStatus: 'preview', rule: { supportedExtensions: ['pdf'], minDocuments: 2 } },
  { id: 'extract_pages', title: 'Extract PDF Pages', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Pull specific pages into a new document', icon: '📤', category: 'pages', context: 'pdf', tags: ['extract', 'select', 'isolate'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], maxDocuments: 1 } },
  { id: 'rotate', title: 'Rotate Document', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Orient landscape or portrait pages', icon: '🔄', category: 'pages', context: 'pdf', tags: ['orientation', 'turn', 'flip'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], isBatchable: true } },

  // Edit & Optimize
  { id: 'watermark', title: 'Add Watermark', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Stamp text or branding across all pages', icon: '💧', category: 'edit', context: 'pdf', tags: ['stamp', 'text', 'brand', 'copyright'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], isBatchable: true } },
  { id: 'flatten', title: 'Flatten Annotations', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Bake annotations & forms permanently into page graphics', icon: '📄', category: 'edit', context: 'pdf', tags: ['bake', 'lock', 'rasterize'], releaseStatus: 'preview' },

  { id: 'compress', title: 'Compress PDF', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Lossy & lossless file size reduction', icon: '🗜️', category: 'optimize', context: 'pdf', tags: ['shrink', 'size', 'dpi'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], isBatchable: true } },
  { id: 'repair', title: 'Repair PDF', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Recover and rebuild corrupted or unreadable documents', icon: '🔧', category: 'optimize', context: 'pdf', tags: ['corrupt', 'fix', 'recover'], releaseStatus: 'preview' },

  // Security
  { id: 'encrypt', title: 'Encrypt with Password', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Protect document with AES-256 password encryption', icon: '🔒', category: 'security', context: 'pdf', tags: ['protect', 'password', 'lock'], releaseStatus: 'preview' },
  { id: 'decrypt', title: 'Remove Password', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Remove password restrictions from unlocked PDF', icon: '🔓', category: 'security', context: 'pdf', tags: ['unlock', 'open'], releaseStatus: 'preview' },
  { id: 'metadata', title: 'Edit / Sanitize Metadata', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Inspect and clean author, title, and creation timestamps', icon: '📋', category: 'security', context: 'pdf', tags: ['sanitize', 'privacy', 'info'], releaseStatus: 'preview' },

  // Convert & Export
  { id: 'pdf_to_docx', title: 'PDF to Word (DOCX)', inputFormat: 'PDF', outputFormat: '.DOCX', description: 'Export document as editable Microsoft Word file', icon: '📝', category: 'convert', context: 'pdf', tags: ['word', 'docx'], releaseStatus: 'preview', rule: { supportedExtensions: ['pdf'], isBatchable: true } },
  { id: 'pdf_to_xlsx', title: 'PDF to Excel (XLSX)', inputFormat: 'PDF', outputFormat: '.XLSX', description: 'Extract tabular data into spreadsheets', icon: '📊', category: 'convert', context: 'pdf', tags: ['excel', 'sheet', 'table'], releaseStatus: 'preview' },
  { id: 'pdf_to_markdown', title: 'PDF to Markdown', inputFormat: 'PDF', outputFormat: '.MD', description: 'Convert clean layout into LLM-ready markdown text', icon: '📑', category: 'convert', context: 'pdf', tags: ['llm', 'markdown', 'md'], releaseStatus: 'preview' },
  { id: 'pdf_to_html', title: 'PDF to HTML', inputFormat: 'PDF', outputFormat: '.HTML', description: 'Export document as responsive, styled HTML5 web page', icon: '🌐', category: 'convert', context: 'pdf', tags: ['html', 'web', 'export', 'responsive'], releaseStatus: 'preview' },
  { id: 'pdf_to_json', title: 'PDF to JSON', inputFormat: 'PDF', outputFormat: '.JSON', description: 'Multimodal layout & text JSON extraction with bounding boxes', icon: '⚡', category: 'convert', context: 'pdf', tags: ['json', 'ai', 'layout', 'extract', 'llm'], releaseStatus: 'preview' },
  { id: 'extract_text', title: 'PDF to Plain Text', inputFormat: 'PDF', outputFormat: '.TXT', description: 'Extract all raw textual content to a .txt file', icon: '📄', category: 'convert', context: 'pdf', tags: ['text', 'dump', 'raw'], releaseStatus: 'v0.5-core' },

  // AI & Intelligence
  { id: 'ocr', title: 'OCR Text Recognition', inputFormat: 'Scanned PDF / Image', outputFormat: '.TXT / PDF', description: 'Run optical character recognition on scanned pages', icon: '👁️', category: 'ai', context: 'pdf', tags: ['ocr', 'scan', 'tesseract'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], isBatchable: true } },
  { id: 'extract_images', title: 'Extract Embedded Images', inputFormat: 'PDF', outputFormat: 'PNG Images', description: 'Save all images found inside the document', icon: '🖼️', category: 'ai', context: 'pdf', tags: ['images', 'photos', 'export'], releaseStatus: 'preview' },

  // Stirling Parity Tools
  { id: 'remove_blank', title: 'Remove Blank Pages', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Remove empty or nearly empty pages', icon: '🧹', category: 'pages', context: 'pdf', tags: ['blank', 'empty', 'clean', 'delete'], releaseStatus: 'preview' },
  { id: 'page_numbers', title: 'Add Page Numbers', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Add dynamic page numbering', icon: '🔢', category: 'pages', context: 'pdf', tags: ['numbering', 'bates', 'header', 'footer', 'page'], releaseStatus: 'preview' },
  { id: 'md_to_pdf', title: 'Markdown to PDF', inputFormat: 'Markdown (.md)', outputFormat: '.PDF', description: 'Convert markdown text to PDF', icon: '📝', category: 'convert', context: 'pdf', tags: ['markdown', 'convert', 'pdf', 'generate'], releaseStatus: 'preview', rule: { supportedExtensions: ['*'], minDocuments: 0 } },
  { id: 'html_to_pdf', title: 'HTML to PDF', inputFormat: 'HTML', outputFormat: '.PDF', description: 'Convert HTML string to styled PDF', icon: '🌐', category: 'convert', context: 'pdf', tags: ['html', 'web', 'convert', 'pdf'], releaseStatus: 'preview' },
  { id: 'img_to_pdf', title: 'Images to PDF', inputFormat: 'JPG, PNG, WebP', outputFormat: '.PDF', description: 'Combine multiple images into a PDF', icon: '🖼️', category: 'convert', context: 'pdf', tags: ['image', 'jpg', 'png', 'combine', 'photo'], releaseStatus: 'preview', rule: { supportedExtensions: ['*'], minDocuments: 0 } },
  { id: 'json_to_pdf', title: 'JSON to PDF', inputFormat: 'JSON', outputFormat: '.PDF', description: 'Synthesize PDF from structured JSON text and embedded images', icon: '⚡', category: 'convert', context: 'pdf', tags: ['json', 'synthesis', 'generate', 'pdf', 'ai'], releaseStatus: 'preview', rule: { supportedExtensions: ['*'], minDocuments: 0 } },
  { id: 'office_modernize_legacy', title: 'Modernize Legacy Office', inputFormat: 'DOC, XLS, PPT', outputFormat: '.DOCX, .XLSX, .PPTX', description: 'Upgrade .doc/.xls/.ppt to modern .docx/.xlsx/.pptx format', icon: '🔄', category: 'convert', context: 'office', tags: ['legacy', 'doc', 'xls', 'ppt', 'modernize', 'upgrade'], releaseStatus: 'preview', rule: { supportedExtensions: ['doc', 'xls', 'ppt'], requiredPack: 'office', isBatchable: true } },

  // Additional 44 Tools Required Additions
  // Organize & Pages
  { id: 'delete_pages', title: 'Delete Pages', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Remove specific pages from the document', icon: '🗑️', category: 'pages', context: 'pdf', tags: ['delete', 'remove', 'pages'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], maxDocuments: 1 } },
  { id: 'reorder_pages', title: 'Reorder Pages', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Change the order of pages in the document', icon: '🔁', category: 'pages', context: 'pdf', tags: ['reorder', 'sort', 'arrange'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['pdf'], maxDocuments: 1 } },
  { id: 'burst', title: 'Burst PDF', inputFormat: 'PDF', outputFormat: 'Multiple .PDFs', description: 'Split document into single pages', icon: '💥', category: 'pages', context: 'pdf', tags: ['burst', 'split', 'single'], releaseStatus: 'preview' },
  { id: 'crop', title: 'Crop Pages', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Crop pages to a specific rectangular area', icon: '✂️', category: 'pages', context: 'pdf', tags: ['crop', 'trim', 'cut'], releaseStatus: 'preview' },

  // Security & Integrity
  { id: 'document_pii_scan', title: 'PII Scanner', inputFormat: 'PDF', outputFormat: '.JSON', description: 'Scan document for SSNs, credit cards, emails, and sensitive PII', icon: '🔍', category: 'security', context: 'pdf', tags: ['pii', 'security', 'privacy', 'ssn', 'compliance'], releaseStatus: 'preview' },
  { id: 'pdf_sanitize', title: 'Sanitize Document', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Deeply sanitize a PDF document for enterprise security and privacy', icon: '🛡️', category: 'security', context: 'pdf', tags: ['sanitize', 'security', 'metadata', 'remove', 'clean'], releaseStatus: 'preview' },
  { id: 'redact', title: 'Redact Text', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Permanently remove sensitive information', icon: '⬛', category: 'security', context: 'pdf', tags: ['redact', 'hide', 'censor', 'blackout'], releaseStatus: 'preview' },
  { id: 'sign', title: 'Sign Document', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Apply digital signature to document', icon: '✒️', category: 'security', context: 'pdf', tags: ['sign', 'signature', 'cert'], releaseStatus: 'preview' },
  { id: 'validate', title: 'Validate PDF', inputFormat: 'PDF', outputFormat: '.TXT', description: 'Verify document structure and standards', icon: '✅', category: 'security', context: 'pdf', tags: ['validate', 'verify', 'check'], releaseStatus: 'preview' },
  { id: 'hash', title: 'Generate Hash', inputFormat: 'PDF', outputFormat: '.TXT', description: 'Calculate document checksums', icon: '#️⃣', category: 'security', context: 'pdf', tags: ['hash', 'checksum', 'md5', 'sha'], releaseStatus: 'preview' },

  // Content & OCR
  { id: 'search', title: 'Search Text', inputFormat: 'PDF', outputFormat: '.JSON', description: 'Search for text across the document', icon: '🔍', category: 'ai', context: 'pdf', tags: ['search', 'find', 'text'], releaseStatus: 'preview' },
  { id: 'bates', title: 'Bates Numbering', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Apply Bates stamps to pages', icon: '🔢', category: 'edit', context: 'pdf', tags: ['bates', 'stamp', 'legal'], releaseStatus: 'preview' },
  { id: 'header_footer', title: 'Header & Footer', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Add headers and footers to pages', icon: '📏', category: 'edit', context: 'pdf', tags: ['header', 'footer', 'header/footer', 'margin'], releaseStatus: 'preview' },
  { id: 'render', title: 'Render Page', inputFormat: 'PDF', outputFormat: 'PNG Images', description: 'Render page to image format', icon: '🖼️', category: 'convert', context: 'pdf', tags: ['render', 'image', 'png'], releaseStatus: 'preview' },

  // Forms & Optimization
  { id: 'read_form', title: 'Read Form Data', inputFormat: 'PDF', outputFormat: '.JSON', description: 'Extract data from PDF forms', icon: '📋', category: 'edit', context: 'pdf', tags: ['form', 'read', 'extract'], releaseStatus: 'preview' },
  { id: 'fill_form', title: 'Fill Form Data', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Fill PDF form fields automatically', icon: '✍️', category: 'edit', context: 'pdf', tags: ['form', 'fill', 'data'], releaseStatus: 'preview' },
  { id: 'create_form_field', title: 'Add Form Field', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Create new interactive form fields', icon: '➕', category: 'edit', context: 'pdf', tags: ['form', 'create', 'field'], releaseStatus: 'preview' },
  { id: 'bookmarks', title: 'Extract Bookmarks', inputFormat: 'PDF', outputFormat: '.JSON', description: 'Read document outline and bookmarks', icon: '🔖', category: 'pages', context: 'pdf', tags: ['bookmarks', 'outline', 'toc'], releaseStatus: 'preview' },
  { id: 'linearize', title: 'Fast Web View', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Optimize PDF for fast web viewing (linearize)', icon: '⚡', category: 'optimize', context: 'pdf', tags: ['web', 'fast', 'linearize'], releaseStatus: 'preview' },

  // Conversions & Intelligence
  { id: 'pdf_to_pptx', title: 'PDF to PowerPoint', inputFormat: 'PDF', outputFormat: '.PPTX', description: 'Export document as PPTX presentation', icon: '📊', category: 'convert', context: 'pdf', tags: ['powerpoint', 'pptx', 'presentation'], releaseStatus: 'preview' },
  { id: 'pdf_to_pdf_a', title: 'Convert to PDF/A', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Archive-ready PDF format conversion', icon: '🏛️', category: 'convert', context: 'pdf', tags: ['archive', 'pdfa', 'long-term'], releaseStatus: 'preview' },
  { id: 'classify_type', title: 'Classify Document', inputFormat: 'PDF', outputFormat: '.JSON', description: 'AI classification of document type', icon: '🏷️', category: 'ai', context: 'pdf', tags: ['classify', 'type', 'classify pdf', 'ai'], releaseStatus: 'preview' },
  { id: 'annotate', title: 'Add Annotations', inputFormat: 'PDF', outputFormat: '.PDF', description: 'Programmatically add annotations', icon: '✏️', category: 'edit', context: 'pdf', tags: ['annotate', 'draw', 'markup'], releaseStatus: 'preview' },
  { id: 'pdf_convert_excel', title: 'CSV to PDF', inputFormat: 'CSV', outputFormat: '.PDF', description: 'Convert tabular data to PDF', icon: '📄', category: 'convert', context: 'pdf', tags: ['csv', 'excel', 'convert'], releaseStatus: 'preview' },

  // Office Tools
  { id: 'office_to_markdown', title: 'Convert to Markdown', inputFormat: 'DOCX, XLSX, PPTX', outputFormat: '.MD', description: 'Convert Word, Excel, or PowerPoint to clean Markdown text', icon: '📑', category: 'convert', context: 'office', tags: ['office', 'markdown', 'docx', 'xlsx', 'pptx', 'convert'], releaseStatus: 'preview', rule: { supportedExtensions: ['docx', 'xlsx', 'pptx'], requiredPack: 'office', isBatchable: true } },
  { id: 'office_extract_text', title: 'Office to Plain Text', inputFormat: 'DOCX, XLSX, PPTX', outputFormat: '.TXT', description: 'Extract all raw text from Office documents into plain text', icon: '📄', category: 'convert', context: 'office', tags: ['office', 'text', 'extract', 'docx', 'xlsx'], releaseStatus: 'preview', rule: { supportedExtensions: ['docx', 'xlsx', 'pptx'], requiredPack: 'office', isBatchable: true } },
  { id: 'pdf_extract_tables', title: 'Extract Tables', inputFormat: 'PDF', outputFormat: '.JSON/.CSV', description: 'Extract tables as JSON matrices or CSV', icon: '📊', category: 'ai', context: 'pdf', tags: ['table', 'excel', 'csv', 'matrix', 'financial'], releaseStatus: 'preview' },
  { id: 'excel_to_json', title: 'Excel to JSON', inputFormat: 'XLSX, XLS, CSV', outputFormat: '.JSON', description: 'Convert spreadsheet sheets into structured JSON data', icon: '📊', category: 'convert', context: 'office', tags: ['office', 'excel', 'xlsx', 'xls', 'json', 'convert'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['xlsx', 'xls', 'csv'], requiredPack: 'office', isBatchable: true } },
  { id: 'excel_to_csv', title: 'Excel to CSV', inputFormat: 'XLSX, XLS', outputFormat: '.CSV', description: 'Export spreadsheet sheet to standard RFC 4180 CSV', icon: '📑', category: 'convert', context: 'office', tags: ['office', 'excel', 'xlsx', 'xls', 'csv', 'export'], releaseStatus: 'v0.5-core', rule: { supportedExtensions: ['xlsx', 'xls', 'csv'], requiredPack: 'office', isBatchable: true } }
];

export const categories = [
  { id: 'quick', title: 'Quick Actions', icon: '⚡' },
  { id: 'pages', title: 'Page Management', icon: '📑' },
  { id: 'edit', title: 'Edit & Markup', icon: '✍️' },
  { id: 'optimize', title: 'Optimize & Repair', icon: '🗜️' },
  { id: 'security', title: 'Security & Privacy', icon: '🔒' },
  { id: 'convert', title: 'Convert & Export', icon: '🔄' },
  { id: 'ai', title: 'AI & Intelligence', icon: '🧠' }
];

export const allCategories = [
  { id: 'all', title: 'All', icon: '✨' },
  ...categories
];
