import type { ToolDefinition } from './tools';

export const allTools: ToolDefinition[] = [
  // Quick / Popular
  { id: 'compress', title: 'Compress PDF', description: 'Reduce document file size with image optimization', icon: '🗜️', category: 'quick', context: 'pdf', tags: ['shrink', 'size', 'optimize'] },
  { id: 'merge', title: 'Merge PDFs', description: 'Combine multiple PDF files into one continuous document', icon: '📑', category: 'quick', context: 'pdf', tags: ['combine', 'join', 'append'] },
  { id: 'rotate', title: 'Rotate Pages', description: 'Rotate pages by 90, 180, or 270 degrees', icon: '🔄', category: 'quick', context: 'pdf', tags: ['orientation', 'turn'] },
  { id: 'compare', title: 'Compare PDFs', description: 'Diff two documents and highlight changes', icon: '🔍', category: 'quick', context: 'pdf', tags: ['diff', 'compare', 'difference'] },
  { id: 'ocr', title: 'OCR Text Recognition', description: 'Extract text from scanned pages using local OCR', icon: '👁️', category: 'quick', context: 'pdf', tags: ['scan', 'text', 'extract'] },
  { id: 'pdf_to_docx', title: 'PDF → Word', description: 'Convert document into editable DOCX format', icon: '📝', category: 'quick', context: 'pdf', tags: ['word', 'docx', 'convert', 'export'] },

  // Page Operations
  { id: 'merge', title: 'Merge Documents', description: 'Combine open tabs into a single PDF', icon: '📑', category: 'pages', context: 'pdf', tags: ['combine', 'join', 'concat'] },
  { id: 'split', title: 'Split PDF', description: 'Divide document into separate pages or ranges', icon: '✂️', category: 'pages', context: 'pdf', tags: ['divide', 'cut', 'separate'] },
  { id: 'compare', title: 'Compare Documents', description: 'Side-by-side comparison between two open tabs', icon: '🔍', category: 'pages', context: 'pdf', tags: ['diff', 'compare'] },
  { id: 'extract_pages', title: 'Extract Pages', description: 'Pull specific pages into a new document', icon: '📤', category: 'pages', context: 'pdf', tags: ['extract', 'select', 'isolate'] },
  { id: 'rotate', title: 'Rotate Document', description: 'Orient landscape or portrait pages', icon: '🔄', category: 'pages', context: 'pdf', tags: ['orientation', 'turn', 'flip'] },

  // Edit & Optimize
  { id: 'watermark', title: 'Add Watermark', description: 'Stamp text or branding across all pages', icon: '💧', category: 'edit', context: 'pdf', tags: ['stamp', 'text', 'brand', 'copyright'] },
  { id: 'flatten', title: 'Flatten Annotations', description: 'Bake annotations & forms permanently into page graphics', icon: '📄', category: 'edit', context: 'pdf', tags: ['bake', 'lock', 'rasterize'] },

  { id: 'compress', title: 'Compress PDF', description: 'Lossy & lossless file size reduction', icon: '🗜️', category: 'optimize', context: 'pdf', tags: ['shrink', 'size', 'dpi'] },
  { id: 'repair', title: 'Repair PDF', description: 'Recover and rebuild corrupted or unreadable documents', icon: '🔧', category: 'optimize', context: 'pdf', tags: ['corrupt', 'fix', 'recover'] },

  // Security
  { id: 'encrypt', title: 'Encrypt with Password', description: 'Protect document with AES-256 password encryption', icon: '🔒', category: 'security', context: 'pdf', tags: ['protect', 'password', 'lock'] },
  { id: 'decrypt', title: 'Remove Password', description: 'Remove password restrictions from unlocked PDF', icon: '🔓', category: 'security', context: 'pdf', tags: ['unlock', 'open'] },
  { id: 'metadata', title: 'Edit / Sanitize Metadata', description: 'Inspect and clean author, title, and creation timestamps', icon: '📋', category: 'security', context: 'pdf', tags: ['sanitize', 'privacy', 'info'] },

  // Convert & Export
  { id: 'pdf_to_docx', title: 'PDF to Word (DOCX)', description: 'Export document as editable Microsoft Word file', icon: '📝', category: 'convert', context: 'pdf', tags: ['word', 'docx'] },
  { id: 'pdf_to_xlsx', title: 'PDF to Excel (XLSX)', description: 'Extract tabular data into spreadsheets', icon: '📊', category: 'convert', context: 'pdf', tags: ['excel', 'sheet', 'table'] },
  { id: 'pdf_to_markdown', title: 'PDF to Markdown', description: 'Convert clean layout into LLM-ready markdown text', icon: '📑', category: 'convert', context: 'pdf', tags: ['llm', 'markdown', 'md'] },
  { id: 'pdf_to_html', title: 'PDF to HTML', description: 'Export document as responsive, styled HTML5 web page', icon: '🌐', category: 'convert', context: 'pdf', tags: ['html', 'web', 'export', 'responsive'] },
  { id: 'extract_text', title: 'Extract Plain Text', description: 'Extract all raw textual content to a .txt file', icon: '📄', category: 'convert', context: 'pdf', tags: ['text', 'dump', 'raw'] },

  // AI & Intelligence
  { id: 'ocr', title: 'OCR Recognition', description: 'Run optical character recognition on scanned pages', icon: '👁️', category: 'ai', context: 'pdf', tags: ['ocr', 'scan', 'tesseract'] },
  { id: 'extract_images', title: 'Extract Embedded Images', description: 'Save all images found inside the document', icon: '🖼️', category: 'ai', context: 'pdf', tags: ['images', 'photos', 'export'] },

  // Stirling Parity Tools
  { id: 'remove_blank', title: 'Remove Blank Pages', description: 'Remove empty or nearly empty pages', icon: '🧹', category: 'pages', context: 'pdf', tags: ['blank', 'empty', 'clean', 'delete'] },
  { id: 'page_numbers', title: 'Add Page Numbers', description: 'Add dynamic page numbering', icon: '🔢', category: 'pages', context: 'pdf', tags: ['numbering', 'bates', 'header', 'footer', 'page'] },
  { id: 'md_to_pdf', title: 'Markdown to PDF', description: 'Convert markdown text to PDF', icon: '📝', category: 'convert', context: 'pdf', tags: ['markdown', 'convert', 'pdf', 'generate'] },
  { id: 'html_to_pdf', title: 'HTML to PDF', description: 'Convert HTML string to styled PDF', icon: '🌐', category: 'convert', context: 'pdf', tags: ['html', 'web', 'convert', 'pdf'] },
  { id: 'img_to_pdf', title: 'Images to PDF', description: 'Combine multiple images into a PDF', icon: '🖼️', category: 'convert', context: 'pdf', tags: ['image', 'jpg', 'png', 'combine', 'photo'] },
  { id: 'json_to_pdf', title: 'JSON to PDF', description: 'Synthesize PDF from structured JSON text and embedded images', icon: '⚡', category: 'convert', context: 'pdf', tags: ['json', 'synthesis', 'generate', 'pdf', 'ai'] },
  { id: 'office_modernize_legacy', title: 'Modernize Legacy Office', description: 'Upgrade .doc/.xls/.ppt to modern .docx/.xlsx/.pptx format', icon: '🔄', category: 'convert', context: 'office', tags: ['legacy', 'doc', 'xls', 'ppt', 'modernize', 'upgrade'] },

  // Additional 44 Tools Required Additions
  // Organize & Pages
  { id: 'delete_pages', title: 'Delete Pages', description: 'Remove specific pages from the document', icon: '🗑️', category: 'pages', context: 'pdf', tags: ['delete', 'remove', 'pages'] },
  { id: 'reorder_pages', title: 'Reorder Pages', description: 'Change the order of pages in the document', icon: '🔁', category: 'pages', context: 'pdf', tags: ['reorder', 'sort', 'arrange'] },
  { id: 'burst', title: 'Burst PDF', description: 'Split document into single pages', icon: '💥', category: 'pages', context: 'pdf', tags: ['burst', 'split', 'single'] },
  { id: 'crop', title: 'Crop Pages', description: 'Crop pages to a specific rectangular area', icon: '✂️', category: 'pages', context: 'pdf', tags: ['crop', 'trim', 'cut'] },

  // Security & Integrity
  { id: 'redact', title: 'Redact Text', description: 'Permanently remove sensitive information', icon: '⬛', category: 'security', context: 'pdf', tags: ['redact', 'hide', 'censor', 'blackout'] },
  { id: 'sign', title: 'Sign Document', description: 'Apply digital signature to document', icon: '✒️', category: 'security', context: 'pdf', tags: ['sign', 'signature', 'cert'] },
  { id: 'validate', title: 'Validate PDF', description: 'Verify document structure and standards', icon: '✅', category: 'security', context: 'pdf', tags: ['validate', 'verify', 'check'] },
  { id: 'hash', title: 'Generate Hash', description: 'Calculate document checksums', icon: '#️⃣', category: 'security', context: 'pdf', tags: ['hash', 'checksum', 'md5', 'sha'] },

  // Content & OCR
  { id: 'search', title: 'Search Text', description: 'Search for text across the document', icon: '🔍', category: 'ai', context: 'pdf', tags: ['search', 'find', 'text'] },
  { id: 'bates', title: 'Bates Numbering', description: 'Apply Bates stamps to pages', icon: '🔢', category: 'edit', context: 'pdf', tags: ['bates', 'stamp', 'legal'] },
  { id: 'header_footer', title: 'Header & Footer', description: 'Add headers and footers to pages', icon: '📏', category: 'edit', context: 'pdf', tags: ['header', 'footer', 'header/footer', 'margin'] },
  { id: 'render', title: 'Render Page', description: 'Render page to image format', icon: '🖼️', category: 'convert', context: 'pdf', tags: ['render', 'image', 'png'] },

  // Forms & Optimization
  { id: 'read_form', title: 'Read Form Data', description: 'Extract data from PDF forms', icon: '📋', category: 'edit', context: 'pdf', tags: ['form', 'read', 'extract'] },
  { id: 'fill_form', title: 'Fill Form Data', description: 'Fill PDF form fields automatically', icon: '✍️', category: 'edit', context: 'pdf', tags: ['form', 'fill', 'data'] },
  { id: 'create_form_field', title: 'Add Form Field', description: 'Create new interactive form fields', icon: '➕', category: 'edit', context: 'pdf', tags: ['form', 'create', 'field'] },
  { id: 'bookmarks', title: 'Extract Bookmarks', description: 'Read document outline and bookmarks', icon: '🔖', category: 'pages', context: 'pdf', tags: ['bookmarks', 'outline', 'toc'] },
  { id: 'linearize', title: 'Fast Web View', description: 'Optimize PDF for fast web viewing (linearize)', icon: '⚡', category: 'optimize', context: 'pdf', tags: ['web', 'fast', 'linearize'] },

  // Conversions & Intelligence
  { id: 'pdf_to_pptx', title: 'PDF to PowerPoint', description: 'Export document as PPTX presentation', icon: '📊', category: 'convert', context: 'pdf', tags: ['powerpoint', 'pptx', 'presentation'] },
  { id: 'pdf_to_pdf_a', title: 'Convert to PDF/A', description: 'Archive-ready PDF format conversion', icon: '🏛️', category: 'convert', context: 'pdf', tags: ['archive', 'pdfa', 'long-term'] },
  { id: 'classify_type', title: 'Classify Document', description: 'AI classification of document type', icon: '🏷️', category: 'ai', context: 'pdf', tags: ['classify', 'type', 'classify pdf', 'ai'] },
  { id: 'annotate', title: 'Add Annotations', description: 'Programmatically add annotations', icon: '✏️', category: 'edit', context: 'pdf', tags: ['annotate', 'draw', 'markup'] },
  { id: 'pdf_convert_excel', title: 'CSV to PDF', description: 'Convert tabular data to PDF', icon: '📄', category: 'convert', context: 'pdf', tags: ['csv', 'excel', 'convert'] },

  // Office Tools
  { id: 'office_to_markdown', title: 'Convert to Markdown', description: 'Convert Word, Excel, or PowerPoint to clean Markdown text', icon: '📑', category: 'convert', context: 'office', tags: ['office', 'markdown', 'docx', 'xlsx', 'pptx', 'convert'] },
  { id: 'office_extract_text', title: 'Extract Text', description: 'Extract all raw text from Office documents into plain text', icon: '📄', category: 'convert', context: 'office', tags: ['office', 'text', 'extract', 'docx', 'xlsx'] },
  { id: 'excel_to_json', title: 'Excel to JSON', description: 'Convert spreadsheet sheets into structured JSON data', icon: '📊', category: 'convert', context: 'office', tags: ['office', 'excel', 'xlsx', 'xls', 'json', 'convert'] },
  { id: 'excel_to_csv', title: 'Excel to CSV', description: 'Export spreadsheet sheet to standard RFC 4180 CSV', icon: '📑', category: 'convert', context: 'office', tags: ['office', 'excel', 'xlsx', 'xls', 'csv', 'export'] }
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
