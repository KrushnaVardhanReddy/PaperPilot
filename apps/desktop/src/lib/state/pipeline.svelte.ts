// Type definitions
export type OperationId =
  | 'pdf_compress' | 'pdf_rotate' | 'pdf_watermark' | 'pdf_merge'
  | 'pdf_split' | 'pdf_encrypt' | 'pdf_decrypt' | 'pdf_metadata'
  | 'pdf_redact' | 'pdf_repair' | 'pdf_linearize' | 'pdf_flatten'
  | 'pdf_header_footer' | 'pdf_bates' | 'pdf_ocr' | 'pdf_sign';

export interface PipelineStep {
  id: string;           // crypto.randomUUID()
  operationId: OperationId;
  label: string;        // Human-readable name, e.g. "Compress PDF"
  icon: string;         // emoji or svg name, e.g. "🗜️"
  params: Record<string, unknown>; // operation-specific params
}

export class PipelineState {
  steps = $state<PipelineStep[]>([]);
  isRunning = $state(false);
  currentStepIndex = $state<number | null>(null); // which step is currently running
  lastError = $state<string | null>(null);
  lastSuccessOutput = $state<string | null>(null);

  addStep(operationId: OperationId) {
    // Lookup the default params and label from OPERATION_REGISTRY
    const def = OPERATION_REGISTRY[operationId];
    this.steps = [...this.steps, {
      id: crypto.randomUUID(),
      operationId,
      label: def.label,
      icon: def.icon,
      params: { ...def.defaultParams }
    }];
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
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
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
    defaultParams: { input: '__input__', output: '__output__' },
    paramSchema: []
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
  }
};

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
