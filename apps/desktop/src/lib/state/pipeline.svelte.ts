// Type definitions
export type OperationId =
  | 'pdf_compress' | 'pdf_rotate' | 'pdf_watermark' | 'pdf_crop' | 'pdf_merge'
  | 'pdf_split' | 'pdf_extract_pages' | 'pdf_encrypt' | 'pdf_decrypt' | 'pdf_metadata'
  | 'pdf_redact' | 'pdf_repair' | 'pdf_linearize' | 'pdf_flatten'
  | 'pdf_header_footer' | 'pdf_bates' | 'pdf_ocr' | 'pdf_sign' | 'pdf_remove_blank' | 'pdf_page_numbers' | 'pdf_sanitize' | 'document_pii_scan';

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
  }
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
