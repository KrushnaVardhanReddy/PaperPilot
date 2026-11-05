// Type definitions
export type OperationId =
  | 'batch_fill'
  | 'cli_ask'
  | 'cli_fill_batch'
  | 'document_pii_scan'
  | 'document_workflow'
  | 'excel_to_csv'
  | 'excel_to_json'
  | 'extract_images'
  | 'html_to_pdf'
  | 'img_to_pdf'
  | 'json_to_pdf'
  | 'md_to_pdf'
  | 'office_extract_text'
  | 'office_modernize_legacy'
  | 'office_to_markdown'
  | 'pdf_annotate'
  | 'pdf_attachments'
  | 'pdf_auto_rotate'
  | 'pdf_bates'
  | 'pdf_booklet'
  | 'pdf_bookmarks'
  | 'pdf_burst'
  | 'pdf_change_permissions'
  | 'pdf_classify_type'
  | 'pdf_comic_archive'
  | 'pdf_compare'
  | 'pdf_compress'
  | 'pdf_convert_excel'
  | 'pdf_convert_html'
  | 'pdf_convert_markdown'
  | 'pdf_create_form_field'
  | 'pdf_crop'
  | 'pdf_decrypt'
  | 'pdf_dedup_streams'
  | 'pdf_delete_pages'
  | 'pdf_encrypt'
  | 'pdf_extract_images'
  | 'pdf_extract_pages'
  | 'pdf_extract_tables'
  | 'pdf_extract_text'
  | 'pdf_fill_form'
  | 'pdf_flatten'
  | 'pdf_hash'
  | 'pdf_header_footer'
  | 'pdf_images_to_pdf'
  | 'pdf_invert_colors'
  | 'pdf_linearize'
  | 'pdf_merge'
  | 'pdf_merge_alternate'
  | 'pdf_metadata'
  | 'pdf_nup'
  | 'pdf_ocr'
  | 'pdf_overlay_underlay'
  | 'pdf_page_numbers'
  | 'pdf_posterize'
  | 'pdf_rag_chunks'
  | 'pdf_read_form'
  | 'pdf_redact'
  | 'pdf_remove_blank'
  | 'pdf_remove_signatures'
  | 'pdf_render'
  | 'pdf_reorder_pages'
  | 'pdf_repair'
  | 'pdf_rotate'
  | 'pdf_sanitize'
  | 'pdf_scale_pages'
  | 'pdf_search'
  | 'pdf_sign'
  | 'pdf_split'
  | 'pdf_split_by_outline'
  | 'pdf_split_scanned_book'
  | 'pdf_stamp'
  | 'pdf_stitch_canvas'
  | 'pdf_timestamp'
  | 'pdf_to_docx'
  | 'pdf_to_epub'
  | 'pdf_to_html'
  | 'pdf_to_json'
  | 'pdf_to_markdown'
  | 'pdf_to_pdf_a'
  | 'pdf_to_pptx'
  | 'pdf_to_xlsx'
  | 'pdf_validate'
  | 'pdf_validate_signatures'
  | 'pdf_watermark'
  | 'recipe_run'
  | 'semantic_search'
  | 'watcher_start'
  | 'watcher_stop'
  | 'workspace_search';

export interface WorkflowConditionLeaf {
  if_scanned?: boolean;
  if_page_count_gt?: number;
  comparator?: 'Greater' | 'GreaterOrEqual' | 'Equal' | 'Less' | 'LessOrEqual';
  value?: number;
  text_contains?: string;
  has_form_fields?: boolean;
  is_signed?: boolean;
  is_encrypted?: boolean;
  file_size_bytes?: {
    between?: [number, number];
    eq?: number;
    gt?: number;
    gte?: number;
    lt?: number;
    lte?: number;
  };
}

export type WorkflowCondition =
  | { all: WorkflowCondition[] }
  | { any: WorkflowCondition[] }
  | { not: WorkflowCondition }
  | WorkflowConditionLeaf;

export type OnNoMatch = 'skip' | 'archive_original' | 'error';

export interface PipelineStep {
  id: string;           // crypto.randomUUID()
  operationId: OperationId;
  label: string;        // Human-readable name, e.g. "Compress PDF"
  icon: string;         // emoji or svg name, e.g. "🗜️"
  params: Record<string, unknown>; // operation-specific params
  condition?: WorkflowCondition;
  depends_on?: string[];
  on_no_match?: OnNoMatch;
  status?: 'pending' | 'running' | 'completed' | 'skipped' | 'error';
  position?: { x: number; y: number };
}

export interface SavedPipeline {
  id: string;               // e.g. "preset-legal-bates" or crypto.randomUUID()
  name: string;             // User-visible flow title
  description?: string;     // Purpose / notes
  steps: PipelineStep[];    // Array of configured steps with tool operationId & params
  updatedAt: string;        // ISO 8601 timestamp
  isPreset?: boolean;       // System default vs user-created
  teamId?: string;          // Optional workspace/team ID
}

export class PipelineState {
  steps = $state<PipelineStep[]>([]);
  selectedStepId = $state<string | null>(null);
  isRunning = $state(false);
  currentStepIndex = $state<number | null>(null); // which step is currently running
  lastError = $state<string | null>(null);
  lastSuccessOutput = $state<string | null>(null);

  savedFlows = $state<SavedPipeline[]>([]);
  activeFlowId = $state<string | null>(null);

  initStorage() {
    if (typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
      const stored = localStorage.getItem('paperpilot_saved_flows');
      if (stored) {
        try {
          const parsed = JSON.parse(stored) as SavedPipeline[];
          // Merge user flows with DEFAULT_PRESETS, ensuring presets are always present
          const userFlows = parsed.filter(f => !f.isPreset);
          this.savedFlows = [...DEFAULT_PRESETS, ...userFlows];
        } catch (err) {
          console.error('Failed to parse saved flows:', err);
          this.savedFlows = [...DEFAULT_PRESETS];
        }
      } else {
        this.savedFlows = [...DEFAULT_PRESETS];
      }
    } else {
      this.savedFlows = [...DEFAULT_PRESETS];
    }
  }

  loadFlow(flowId: string): boolean {
    const flow = this.savedFlows.find(f => f.id === flowId);
    if (!flow) return false;

    // deep clone steps to prevent reference mutations
    this.steps = JSON.parse(JSON.stringify(flow.steps));
    // Apply fallback positions if not present
    this.steps.forEach((step, index) => {
      if (!step.position) {
        step.position = { x: 100 + index * 320, y: 200 };
      }
    });
    this.activeFlowId = flowId;
    this.selectedStepId = null;
    return true;
  }

  saveCurrentFlow(name: string, description?: string): SavedPipeline {
    const newFlow: SavedPipeline = {
      id: crypto.randomUUID(),
      name,
      description,
      steps: JSON.parse(JSON.stringify(this.steps)), // deep clone current steps
      updatedAt: new Date().toISOString(),
      isPreset: false
    };

    this.savedFlows = [newFlow, ...this.savedFlows];
    this.activeFlowId = newFlow.id;

    if (typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
      // Save all flows (both custom and default)
      localStorage.setItem('paperpilot_saved_flows', JSON.stringify(this.savedFlows));
    }

    return newFlow;
  }

  deleteFlow(flowId: string): boolean {
    const flow = this.savedFlows.find(f => f.id === flowId);
    if (!flow || flow.isPreset) return false; // Cannot delete presets

    this.savedFlows = this.savedFlows.filter(f => f.id !== flowId);
    if (this.activeFlowId === flowId) {
      this.activeFlowId = null;
    }

    if (typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
      localStorage.setItem('paperpilot_saved_flows', JSON.stringify(this.savedFlows));
    }

    return true;
  }

  exportFlowJson(flowId?: string): string {
    const targetFlowId = flowId || this.activeFlowId;
    let flowToExport: SavedPipeline;

    if (targetFlowId) {
      const flow = this.savedFlows.find(f => f.id === targetFlowId);
      if (flow) {
        flowToExport = flow;
      } else {
        throw new Error('Flow not found');
      }
    } else {
      // Export current unsaved flow
      flowToExport = {
        id: crypto.randomUUID(),
        name: 'Exported Flow',
        steps: this.steps,
        updatedAt: new Date().toISOString(),
        isPreset: false
      };
    }

    return JSON.stringify(flowToExport, null, 2);
  }

  importFlowFromJson(jsonString: string): SavedPipeline {
    let parsed: any;
    try {
      parsed = JSON.parse(jsonString);
    } catch (err) {
      throw new Error('Invalid JSON format');
    }

    if (!parsed.name || !Array.isArray(parsed.steps)) {
      throw new Error('Invalid flow format: missing name or steps array');
    }

    const newFlow: SavedPipeline = {
      ...parsed,
      id: crypto.randomUUID(), // Generate new unique ID to avoid collision
      updatedAt: new Date().toISOString(),
      isPreset: false
    };

    this.savedFlows = [newFlow, ...this.savedFlows];

    if (typeof window !== 'undefined' && typeof localStorage !== 'undefined') {
      localStorage.setItem('paperpilot_saved_flows', JSON.stringify(this.savedFlows));
    }

    this.loadFlow(newFlow.id);
    return newFlow;
  }

  selectStep(id: string | null) {
    this.selectedStepId = id;
  }

  addStep(operationId: OperationId) {
    // Lookup the default params and label from OPERATION_REGISTRY
    const def = OPERATION_REGISTRY[operationId];
    const newId = crypto.randomUUID();
    this.steps = [...this.steps, {
      id: newId,
      operationId,
      label: def.label,
      icon: def.icon,
      params: { ...def.defaultParams },
      position: { x: 100 + this.steps.length * 320, y: 200 }
    }];
    this.selectedStepId = newId;
  }

  removeStep(id: string) {
    this.steps = this.steps.filter(s => s.id !== id);
  }

  moveStep(fromIndex: number, toIndex: number) {
    if (toIndex < 0 || toIndex >= this.steps.length) return;
    const steps = [...this.steps];
    const [moved] = steps.splice(fromIndex, 1);
    steps.splice(toIndex, 0, moved);
    this.steps = steps;
  }

  updateStepParam(id: string, paramKey: string, value: unknown) {
    this.steps = this.steps.map(s =>
      s.id === id ? { ...s, params: { ...s.params, [paramKey]: value } } : s
    );
  }

  updateStepPosition(id: string, position: { x: number; y: number }) {
    this.steps = this.steps.map(s =>
      s.id === id ? { ...s, position } : s
    );
  }

  updateDependsOn(id: string, depends_on: string[]) {
    this.steps = this.steps.map(s =>
      s.id === id ? { ...s, depends_on } : s
    );
  }

  autoLayout() {
    if (this.steps.length === 0) return;

    // Calculate depth for each step based on dependencies
    const depths = new Map<string, number>();

    const getDepth = (stepId: string, visited = new Set<string>()): number => {
      if (depths.has(stepId)) return depths.get(stepId)!;
      if (visited.has(stepId)) return 1;
      visited.add(stepId);

      const step = this.steps.find(s => s.id === stepId);
      if (!step || !step.depends_on || step.depends_on.length === 0) {
        const idx = this.steps.findIndex(s => s.id === stepId);
        const depth = idx >= 0 ? idx + 1 : 1;
        depths.set(stepId, depth);
        return depth;
      }

      let maxDepDepth = 0;
      for (const depId of step.depends_on) {
        if (depId === 'trigger-node') {
          maxDepDepth = Math.max(maxDepDepth, 0);
        } else {
          maxDepDepth = Math.max(maxDepDepth, getDepth(depId, visited));
        }
      }

      const depth = maxDepDepth + 1;
      depths.set(stepId, depth);
      return depth;
    };

    this.steps.forEach(s => getDepth(s.id));

    // Group steps by column depth
    const columns = new Map<number, PipelineStep[]>();
    this.steps.forEach(s => {
      const d = depths.get(s.id) ?? 1;
      if (!columns.has(d)) columns.set(d, []);
      columns.get(d)!.push(s);
    });

    // Assign positions: column x = 60 + depth * 260; vertically centered around y = 200
    this.steps = this.steps.map(step => {
      const d = depths.get(step.id) ?? 1;
      const colSteps = columns.get(d) ?? [step];
      const indexInCol = colSteps.findIndex(s => s.id === step.id);
      const totalInCol = colSteps.length;

      const x = 60 + d * 260;
      const y = 200 + (indexInCol - (totalInCol - 1) / 2) * 110;

      return {
        ...step,
        position: { x, y }
      };
    });
  }

  setStepCondition(stepId: string, condition?: WorkflowCondition, onNoMatch?: OnNoMatch) {
    this.steps = this.steps.map(s => {
      if (s.id === stepId) {
        return { ...s, condition, on_no_match: onNoMatch };
      }
      return s;
    });
  }

  removeStepCondition(stepId: string) {
    this.steps = this.steps.map(s => {
      if (s.id === stepId) {
        const { condition, on_no_match, ...rest } = s;
        return rest;
      }
      return s;
    });
  }

  clearPipeline() {
    this.steps = [];
    this.currentStepIndex = null;
    this.lastError = null;
    this.lastSuccessOutput = null;
  }

  setRunning(running: boolean) {
    this.isRunning = running;
  }

  setCurrentStepIndex(index: number | null) {
    this.currentStepIndex = index;
  }

  setLastError(err: string | null) {
    this.lastError = err;
  }

  setLastSuccessOutput(path: string | null) {
    this.lastSuccessOutput = path;
  }
}

export const pipelineState = new PipelineState();

// ─── Operation Registry ───────────────────────────────────────────────────────
// One entry per supported pipeline operation. defaultParams must match what
// the Rust MCP tool expects. We use `__input__` and `__output__` as placeholder
// tokens; the pipeline runner will substitute them at execution time.

export const OPERATION_REGISTRY: Record<OperationId, {
  label: string;
  icon: string;
  description: string;
  defaultParams: Record<string, unknown>;
  paramSchema: ParamSchema[];
}> = {
  pdf_compress: {
    label: 'Compress',
    icon: '🗜️',
    description: 'Reduce PDF file size',
    defaultParams: { input: '__input__', output: '__output__', level: 'medium' },
    paramSchema: [
      { key: 'level', label: 'Compression Level', type: 'select', options: ['low', 'medium', 'high', 'extreme'] }
    ]
  },
  pdf_crop: {
    label: 'Crop Pages',
    icon: '📐',
    description: 'Crop page boundaries (left, bottom, right, top in pt)',
    defaultParams: { input: '__input__', output: '__output__', box: '50,50,550,750' },
    paramSchema: [
      { key: 'box', label: 'Crop Box (left,bottom,right,top)', type: 'text', placeholder: 'e.g. 50,50,550,750' }
    ]
  },
  pdf_rotate: {
    label: 'Rotate Pages',
    icon: '🔄',
    description: 'Rotate all pages by a given angle',
    defaultParams: { input: '__input__', output: '__output__', pages: 'all', angle: 90 },
    paramSchema: [
      { key: 'pages', label: 'Pages', type: 'text', placeholder: 'e.g. all, 1-3, 1,3,5' },
      { key: 'angle', label: 'Angle', type: 'select', options: [90, 180, 270] }
    ]
  },
  pdf_watermark: {
    label: 'Watermark',
    icon: '💧',
    description: 'Apply a text watermark to all pages',
    defaultParams: { input: '__input__', output: '__output__', text: 'CONFIDENTIAL', opacity: 0.3 },
    paramSchema: [
      { key: 'text', label: 'Watermark Text', type: 'text', placeholder: 'e.g. CONFIDENTIAL' },
      { key: 'opacity', label: 'Opacity (0.0–1.0)', type: 'number', min: 0, max: 1, step: 0.1 }
    ]
  },
  pdf_encrypt: {
    label: 'Encrypt',
    icon: '🔒',
    description: 'Password-protect the PDF',
    defaultParams: { input: '__input__', output: '__output__', password: '' },
    paramSchema: [
      { key: 'password', label: 'Password', type: 'password', placeholder: 'Enter password' }
    ]
  },
  pdf_decrypt: {
    label: 'Decrypt',
    icon: '🔓',
    description: 'Remove password from a protected PDF',
    defaultParams: { input: '__input__', output: '__output__', password: '' },
    paramSchema: [
      { key: 'password', label: 'Password', type: 'password', placeholder: 'Enter password' }
    ]
  },
  pdf_metadata: {
    label: 'Set Metadata',
    icon: '🏷️',
    description: 'Update PDF title, author, etc.',
    defaultParams: { input: '__input__', output: '__output__', title: '', author: '', subject: '' },
    paramSchema: [
      { key: 'title', label: 'Title', type: 'text', placeholder: 'Document title' },
      { key: 'author', label: 'Author', type: 'text', placeholder: 'Author name' },
      { key: 'subject', label: 'Subject', type: 'text', placeholder: 'Subject' }
    ]
  },
  pdf_repair: {
    label: 'Repair',
    icon: '🔧',
    description: 'Fix a corrupt or malformed PDF',
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_linearize: {
    label: 'Linearize',
    icon: '⚡',
    description: 'Optimize PDF for web viewing',
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_flatten: {
    label: 'Flatten',
    icon: '📄',
    description: 'Flatten form fields and annotations',
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_redact: {
    label: 'Redact',
    icon: '⬛',
    description: 'Permanently redact text from the PDF',
    defaultParams: { input: '__input__', output: '__output__', terms: '' },
    paramSchema: [
      { key: 'terms', label: 'Terms to Redact (comma-separated)', type: 'text', placeholder: 'e.g. SSN, confidential' }
    ]
  },
  pdf_sanitize: {
    label: 'Sanitize',
    icon: '🛡️',
    description: 'Deeply sanitize PDF by stripping metadata, attachments, and scripts',
    defaultParams: { input: '__input__', output: '__output__', remove_metadata: true, remove_attachments: true, remove_scripts: true },
    paramSchema: [
      { key: 'remove_metadata', label: 'Remove Metadata', type: 'text', placeholder: 'true/false' },
      { key: 'remove_attachments', label: 'Remove Attachments', type: 'text', placeholder: 'true/false' },
      { key: 'remove_scripts', label: 'Remove Scripts', type: 'text', placeholder: 'true/false' }
    ]
  },
  document_pii_scan: {
    label: 'PII Scan',
    icon: '🔍',
    description: 'Scan document for SSNs, credit cards, emails, and sensitive PII',
    defaultParams: { input: '__input__ ', output: '__output__ ', scan_ssn: true, scan_credit_cards: true, scan_emails: true, scan_phones: true, scan_iban: true },
    paramSchema: [
      { key: 'scan_ssn', label: 'Scan SSNs', type: 'text', placeholder: 'true/false' },
      { key: 'scan_credit_cards', label: 'Scan Credit Cards', type: 'text', placeholder: 'true/false' },
      { key: 'scan_emails', label: 'Scan Emails', type: 'text', placeholder: 'true/false' },
      { key: 'scan_phones', label: 'Scan Phones', type: 'text', placeholder: 'true/false' },
      { key: 'scan_iban', label: 'Scan IBAN', type: 'text', placeholder: 'true/false' }
    ]
  },
  pdf_header_footer: {
    label: 'Header/Footer',
    icon: '📃',
    description: 'Add a header and/or footer to each page',
    defaultParams: { input: '__input__', output: '__output__', header: '', footer: '' },
    paramSchema: [
      { key: 'header', label: 'Header Text', type: 'text', placeholder: 'Header' },
      { key: 'footer', label: 'Footer Text', type: 'text', placeholder: 'Footer' }
    ]
  },
  pdf_bates: {
    label: 'Bates Numbering',
    icon: '#️⃣',
    description: 'Add sequential Bates numbers to each page',
    defaultParams: { input: '__input__', output: '__output__', start: 1, prefix: 'DOC-' },
    paramSchema: [
      { key: 'prefix', label: 'Prefix', type: 'text', placeholder: 'e.g. DOC-' },
      { key: 'start', label: 'Start Number', type: 'number', min: 1, step: 1 }
    ]
  },
  pdf_ocr: {
    label: 'OCR',
    icon: '🔍',
    description: 'Make a scanned PDF text-searchable',
    defaultParams: { input: '__input__', output: '__output__', language: 'eng' },
    paramSchema: [
      { key: 'language', label: 'Language', type: 'select', options: ['eng', 'deu', 'fra', 'spa', 'ita', 'chi_sim'] }
    ]
  },
  pdf_sign: {
    label: 'Sign',
    icon: '✍️',
    description: 'Apply a digital signature to the PDF',
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_merge: {
    label: 'Merge',
    icon: '📎',
    description: 'Merge multiple PDFs (uses all loaded documents)',
    defaultParams: { inputs: '__all_inputs__', output: '__output__' },
    paramSchema: []
  },
  pdf_split: {
    label: 'Split',
    icon: '✂️',
    description: 'Split PDF by page ranges',
    defaultParams: { input: '__input__', output_dir: '__output_dir__', split_at: '' },
    paramSchema: [
      { key: 'split_at', label: 'Split at pages (comma-separated)', type: 'text', placeholder: 'e.g. 3,7,12' }
    ]
  },
  pdf_extract_pages: {
    label: 'Extract Pages',
    icon: '📑',
    description: 'Extract specific pages into a new PDF',
    defaultParams: { input: '__input__', output: '__output__', pages: '1' },
    paramSchema: [
      { key: 'pages', label: 'Pages (comma-separated or ranges)', type: 'text', placeholder: 'e.g. 1-3, 5' }
    ]
  },
  pdf_remove_blank: {
    label: 'Remove Blank Pages',
    icon: '🧽',
    description: 'Remove blank pages from the PDF',
    defaultParams: { input: '__input__', output: '__output__', sensitivity: 95 },
    paramSchema: [
      { key: 'sensitivity', label: 'Sensitivity', type: 'number', min: 0, max: 100, step: 1 }
    ]
  },
  pdf_page_numbers: {
    label: 'Page Numbers',
    icon: '🔢',
    description: 'Add page numbers to the PDF',
    defaultParams: { input: '__input__', output: '__output__', format: 'Page {n} of {total}', position: 'bottom-center' },
    paramSchema: [
      { key: 'format', label: 'Format String', type: 'text', placeholder: 'e.g. Page {n} of {total}' },
      { key: 'position', label: 'Position', type: 'select', options: ['top-left', 'top-center', 'top-right', 'bottom-left', 'bottom-center', 'bottom-right'] }
    ]
  },
  batch_fill: {
    label: "Batch Form Fill",
    icon: "\ud83d\udcd1",
    description: "Batch populate interactive AcroForm PDF templates from CSV or JSON tabular data",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  cli_ask: {
    label: "AI Document Assistant",
    icon: "\ud83e\udd16",
    description: "Natural language interactive assistant for conversational document queries and edits",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  cli_fill_batch: {
    label: "Automated Batch Engine",
    icon: "\u2699\ufe0f",
    description: "High-speed automated CLI AcroForm generation with execution receipt audit trails",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  document_workflow: {
    label: "Document Workflow Runner",
    icon: "\u26a1",
    description: "Execute declarative multi-step document transformation pipelines and workflows",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  excel_to_csv: {
    label: "Excel to CSV",
    icon: "\ud83d\udcd1",
    description: "Export spreadsheet sheet to standard RFC 4180 CSV",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  excel_to_json: {
    label: "Excel to JSON",
    icon: "\ud83d\udcca",
    description: "Convert spreadsheet sheets into structured JSON data",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  extract_images: {
    label: "Extract Embedded Images",
    icon: "\ud83d\uddbc\ufe0f",
    description: "Save all images found inside the document",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  html_to_pdf: {
    label: "HTML to PDF",
    icon: "\ud83c\udf10",
    description: "Convert HTML string to styled PDF",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  img_to_pdf: {
    label: "Images to PDF",
    icon: "\ud83d\uddbc\ufe0f",
    description: "Combine multiple images into a PDF",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  json_to_pdf: {
    label: "JSON to PDF",
    icon: "\u26a1",
    description: "Synthesize PDF from structured JSON text and embedded images",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  md_to_pdf: {
    label: "Markdown to PDF",
    icon: "\ud83d\udcdd",
    description: "Convert markdown text to PDF",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  office_extract_text: {
    label: "Office to Plain Text",
    icon: "\ud83d\udcc4",
    description: "Extract all raw text from Office documents into plain text",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  office_modernize_legacy: {
    label: "Modernize Legacy Office",
    icon: "\ud83d\udd04",
    description: "Upgrade .doc/.xls/.ppt to modern .docx/.xlsx/.pptx format",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  office_to_markdown: {
    label: "Convert to Markdown",
    icon: "\ud83d\udcd1",
    description: "Convert Word, Excel, or PowerPoint to clean Markdown text",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_annotate: {
    label: "Add Annotations",
    icon: "\u270f\ufe0f",
    description: "Programmatically add annotations",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_attachments: {
    label: "PDF Portfolio Attachments",
    icon: "\ud83d\udcce",
    description: "Inspect, extract, or embed file attachments into PDF portfolios.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_auto_rotate: {
    label: "Auto Upright Orientation",
    icon: "\ud83d\udd04",
    description: "Heuristically detect text baseline and rotation angle to automatically upright pages.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_booklet: {
    label: "Booklet Imposition",
    icon: "\ud83d\udcd6",
    description: "Reorder and impose pages for 2-sided folding booklet printing.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_bookmarks: {
    label: "Extract Bookmarks",
    icon: "\ud83d\udd16",
    description: "Read document outline and bookmarks",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_burst: {
    label: "Burst PDF",
    icon: "\ud83d\udca5",
    description: "Split document into single pages",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_change_permissions: {
    label: "Modify PDF Permissions",
    icon: "\ud83d\udee1\ufe0f",
    description: "Configure user and owner permissions for printing, copying, and modification.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_classify_type: {
    label: "Classify Document",
    icon: "\ud83c\udff7\ufe0f",
    description: "AI classification of document type",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_comic_archive: {
    label: "Comic Archive Conversion",
    icon: "\ud83d\udcda",
    description: "Convert between PDF documents and comic book archive formats (CBZ/CBR).",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_compare: {
    label: "Compare Documents",
    icon: "\ud83d\udd0d",
    description: "Side-by-side comparison between two open tabs",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_convert_excel: {
    label: "CSV to PDF",
    icon: "\ud83d\udcc4",
    description: "Convert tabular data to PDF",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_convert_html: {
    label: "HTML to PDF Direct",
    icon: "\ud83c\udf10",
    description: "Convert ingested HTML strings or web pages directly into styled PDF documents",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_convert_markdown: {
    label: "Markdown to PDF Direct",
    icon: "\ud83d\udcdd",
    description: "Parse, style, and compile raw Markdown syntax into publication-quality PDFs",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_create_form_field: {
    label: "Add Form Field",
    icon: "\u2795",
    description: "Create new interactive form fields",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_dedup_streams: {
    label: "Lossless Stream Deduplication",
    icon: "\ud83d\udcc9",
    description: "Lossless hash-based indirect object stream deduplication & CMYK-safe image compression",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_delete_pages: {
    label: "Delete Pages",
    icon: "\ud83d\uddd1\ufe0f",
    description: "Remove specific pages from the document",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_extract_images: {
    label: "Extract Embedded Images",
    icon: "\ud83d\uddbc\ufe0f",
    description: "Save all images found inside the document",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_extract_tables: {
    label: "Extract Tables",
    icon: "\ud83d\udcca",
    description: "Extract tables as JSON matrices or CSV",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_extract_text: {
    label: "PDF to Plain Text",
    icon: "\ud83d\udcc4",
    description: "Extract all raw textual content to a .txt file",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_fill_form: {
    label: "Fill Form Data",
    icon: "\u270d\ufe0f",
    description: "Fill PDF form fields automatically",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_hash: {
    label: "Generate Hash",
    icon: "#\ufe0f\u20e3",
    description: "Calculate document checksums",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_images_to_pdf: {
    label: "Images to PDF",
    icon: "\ud83d\uddbc\ufe0f",
    description: "Combine multiple raster images (PNG, JPEG) into a multi-page PDF document",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_invert_colors: {
    label: "Invert Colors (Dark Mode)",
    icon: "\ud83c\udf13",
    description: "High-contrast dark mode transformation and color inversion for reading accessibility",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_merge_alternate: {
    label: "Alternating PDF Merge",
    icon: "\ud83d\udcc4",
    description: "Interleave pages from two PDF documents.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_nup: {
    label: "N-Up Page Imposition",
    icon: "\ud83d\udcd1",
    description: "Combine multiple pages per sheet in a 2, 4, 9, or 16-up grid layout.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_overlay_underlay: {
    label: "Overlay & Underlay Stationary",
    icon: "\ud83d\udcd1",
    description: "Apply letterheads, watermarked stationary, and templates as background or foreground.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_posterize: {
    label: "Multi-Sheet Poster Tile Slicer",
    icon: "\ud83e\udde9",
    description: "Tile oversized PDF pages across a grid of standard printable sheets with overlap margins and cut guides",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_rag_chunks: {
    label: "RAG Pipeline Chunks",
    icon: "\ud83e\udde9",
    description: "Extract structure-aware semantic text chunks optimized for LLM RAG vector indexing",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_read_form: {
    label: "Read Form Data",
    icon: "\ud83d\udccb",
    description: "Extract data from PDF forms",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_remove_signatures: {
    label: "Remove Digital Signatures",
    icon: "\ud83d\udd13",
    description: "Strip digital signature fields, cryptographic locks, and widget annotations.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_render: {
    label: "Render Page",
    icon: "\ud83d\uddbc\ufe0f",
    description: "Render page to image format",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_reorder_pages: {
    label: "Reorder Pages",
    icon: "\ud83d\udd01",
    description: "Change the order of pages in the document",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_scale_pages: {
    label: "Scale Page Dimensions",
    icon: "\ud83d\udcd0",
    description: "Scale page dimensions between standard formats or percentages.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_search: {
    label: "Search Text",
    icon: "\ud83d\udd0d",
    description: "Search for text across the document",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_split_by_outline: {
    label: "Split by Document Outline",
    icon: "\ud83d\udcd1",
    description: "Split a PDF into separate section files according to its outline bookmarks.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_split_scanned_book: {
    label: "Split Scanned Book Spine",
    icon: "\ud83d\udcd6",
    description: "Detect center binding gutter spine and split two-up page scans into single pages.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_stamp: {
    label: "Stamp Image & Logo",
    icon: "\ud83c\udff7\ufe0f",
    description: "Place coordinate-exact images, logos, and badges onto PDF pages.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_stitch_canvas: {
    label: "Stitch Continuous Canvas",
    icon: "\ud83d\udcdc",
    description: "Combine multiple pages vertically or horizontally into a continuous single-page canvas.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_timestamp: {
    label: "Cryptographic Timestamp TSA",
    icon: "\u23f1\ufe0f",
    description: "Apply RFC 3161 cryptographically verified TSA timestamp tokens.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_docx: {
    label: "PDF to Word (DOCX)",
    icon: "\ud83d\udcdd",
    description: "Export document as editable Microsoft Word file",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_epub: {
    label: "PDF to EPUB 3.0",
    icon: "\ud83d\udcd6",
    description: "Convert fixed-layout PDF documents into reflowable EPUB 3.0 eBooks",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_html: {
    label: "PDF to HTML",
    icon: "\ud83c\udf10",
    description: "Export document as responsive, styled HTML5 web page",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_json: {
    label: "PDF to JSON",
    icon: "\u26a1",
    description: "Multimodal layout & text JSON extraction with bounding boxes",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_markdown: {
    label: "PDF to Markdown",
    icon: "\ud83d\udcd1",
    description: "Convert clean layout into LLM-ready markdown text",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_pdf_a: {
    label: "Convert to PDF/A",
    icon: "\ud83c\udfdb\ufe0f",
    description: "Archive-ready PDF format conversion",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_pptx: {
    label: "PDF to PowerPoint",
    icon: "\ud83d\udcca",
    description: "Export document as PPTX presentation",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_to_xlsx: {
    label: "PDF to Excel (XLSX)",
    icon: "\ud83d\udcca",
    description: "Extract tabular data into spreadsheets",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_validate: {
    label: "Validate PDF",
    icon: "\u2705",
    description: "Verify document structure and standards",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  pdf_validate_signatures: {
    label: "Validate Digital Signatures",
    icon: "\ud83d\udd0f",
    description: "Cryptographically verify digital signatures, certificate chains, and document integrity.",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  recipe_run: {
    label: "Instant Pipeline Recipe",
    icon: "\ud83d\udcdc",
    description: "Run pre-configured automated PDF manipulation recipes on open documents",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  semantic_search: {
    label: "Semantic Vector Search",
    icon: "\ud83e\udde0",
    description: "Deep neural vector semantic retrieval and passage matching across document chunks",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  watcher_start: {
    label: "Start Hot-Folder Watcher",
    icon: "\ud83d\udc41\ufe0f",
    description: "Launch autonomous background hot-folder file watcher daemon with automated rules",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  watcher_stop: {
    label: "Stop Hot-Folder Watcher",
    icon: "\u23f9\ufe0f",
    description: "Gracefully shut down running hot-folder file watcher automation daemons",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
  workspace_search: {
    label: "Workspace Directory Search",
    icon: "\ud83d\udcc1",
    description: "Recursive multi-format document directory search with content snippet previews",
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
  },
};

export const DEFAULT_PRESETS: SavedPipeline[] = [
  {
    id: 'preset-legal-discovery',
    name: 'Legal Discovery & Bates Stamping',
    description: 'Consecutive litigation numbering, confidential stamp, and fast-web view optimization.',
    isPreset: true,
    updatedAt: new Date(0).toISOString(),
    steps: [
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_bates',
        label: 'Bates Numbering',
        icon: '#️⃣',
        params: { input: '__input__', output: '__output__', prefix: 'CASE-2026-', start: 1 },
        position: { x: 100, y: 200 }
      },
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_watermark',
        label: 'Watermark',
        icon: '💧',
        params: { input: '__input__', output: '__output__', text: 'CONFIDENTIAL', opacity: 0.3, font_size: 48 },
        position: { x: 420, y: 200 }
      },
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_linearize',
        label: 'Linearize',
        icon: '⚡',
        params: { input: '__input__', output: '__output__' },
        position: { x: 740, y: 200 }
      }
    ]
  },
  {
    id: 'preset-clean-scan',
    name: 'Scan Cleanup & Pagination',
    description: 'Strip accidental empty pages, add dynamic pagination, and optimize file size.',
    isPreset: true,
    updatedAt: new Date(0).toISOString(),
    steps: [
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_remove_blank',
        label: 'Remove Blank Pages',
        icon: '🧽',
        params: { input: '__input__', output: '__output__', sensitivity: 95 },
        position: { x: 100, y: 200 }
      },
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_page_numbers',
        label: 'Page Numbers',
        icon: '🔢',
        params: { input: '__input__', output: '__output__', format: 'Page {n} of {total}', position: 'bottom-center' },
        position: { x: 420, y: 200 }
      },
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_compress',
        label: 'Compress',
        icon: '🗜️',
        params: { input: '__input__', output: '__output__', level: 'medium', mode: 'medium' },
        position: { x: 740, y: 200 }
      }
    ]
  },
  {
    id: 'preset-redact-protect',
    name: 'Redaction & Encryption',
    description: 'Permanently scrub sensitive keywords, flatten annotations, and AES-256 encrypt.',
    isPreset: true,
    updatedAt: new Date(0).toISOString(),
    steps: [
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_redact',
        label: 'Redact',
        icon: '⬛',
        params: { input: '__input__', output: '__output__', terms: '', words: 'SSN, confidential, invoice' },
        position: { x: 100, y: 200 }
      },
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_flatten',
        label: 'Flatten',
        icon: '📄',
        params: { input: '__input__', output: '__output__' },
        position: { x: 420, y: 200 }
      },
      {
        id: crypto.randomUUID(),
        operationId: 'pdf_encrypt',
        label: 'Encrypt',
        icon: '🔒',
        params: { input: '__input__', output: '__output__', password: '', user_password: 'changeme' },
        position: { x: 740, y: 200 }
      }
    ]
  }
];

// Param schema types for rendering the config form dynamically
export interface ParamSchema {
  key: string;
  label: string;
  type: 'text' | 'number' | 'password' | 'select' | 'range';
  placeholder?: string;
  options?: (string | number)[];
  min?: number;
  max?: number;
  step?: number;
}
