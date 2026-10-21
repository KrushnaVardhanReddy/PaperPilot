import Widget from './Widget.svelte';
import { getThemeStyles, type ThemeConfig } from './theme';
import { mount as mountSvelte, unmount as unmountSvelte } from 'svelte';

export interface PipelineStep {
  tool: string;
  [key: string]: any;
}

export interface PipelineOptions {
  file: File | Uint8Array;
  steps: PipelineStep[];
  onProgress?: (stepIndex: number, step: PipelineStep, progressPercent: number) => void;
}

export interface PipelineResult {
  success: boolean;
  data: Uint8Array;
  filename: string;
  stepsExecuted: number;
  durationMs: number;
}

export interface EmbedOptions extends ThemeConfig {
  allowCustomization?: boolean;
  autoExecute?: boolean;
  tools?: string | string[];
  hideBadge?: boolean;
  pipeline?: string | PipelineStep[];
}

export interface EmbedInstance {
  destroy: () => void;
  element: HTMLElement;
}

const instances = new Map<string | HTMLElement, EmbedInstance>();

export async function runPipeline(options: PipelineOptions): Promise<PipelineResult> {
  const startTime = Date.now();
  const { PDFDocument, degrees, rgb, StandardFonts } = await import('pdf-lib');

  let doc: any;
  let filename = 'pipeline_output.pdf';

  if (options.file instanceof File) {
    const arrayBuffer = await options.file.arrayBuffer();
    doc = await PDFDocument.load(arrayBuffer);
    filename = options.file.name;
  } else {
    doc = await PDFDocument.load(options.file);
  }

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
            result.add(p - 1);
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

  let stepsExecuted = 0;

  for (let i = 0; i < options.steps.length; i++) {
    const step = options.steps[i];
    if (options.onProgress) {
      options.onProgress(i, step, Math.round(((i) / options.steps.length) * 100));
    }

    if (step.tool === 'rotate') {
      const pages = doc.getPages();
      const angleNum = step.angle || 90;
      let targetIndices: number[];
      if (step.pages === 'all' || !step.pages) {
        targetIndices = pages.map((_: any, idx: number) => idx);
      } else {
        targetIndices = parsePageNumbers(step.pages.toString(), pages.length);
      }
      for (const idx of targetIndices) {
        if (pages[idx]) {
          const currentRotation = pages[idx].getRotation().angle;
          pages[idx].setRotation(degrees((currentRotation + angleNum) % 360));
        }
      }
    } else if (step.tool === 'watermark') {
      const font = await doc.embedFont(StandardFonts.HelveticaBold);
      const pages = doc.getPages();
      const stamp = step.text || 'CONFIDENTIAL';
      const opacity = step.opacity || 0.35;
      const angle = step.angle || 45;
      for (const page of pages) {
        const { width, height } = page.getSize();
        page.drawText(stamp, {
          x: width / 6,
          y: height / 2,
          size: Math.min(width, height) / 14,
          font,
          color: rgb(0.85, 0.2, 0.2),
          opacity,
          rotate: { type: 'degrees', angle } as any
        });
      }
    } else if (step.tool === 'compress' || step.tool === 'flatten') {
      // For now, these are minimal or no-ops in client-side pdf-lib
      // without heavy rebuilding, but we count them.
    } else if (step.tool === 'extract_pages' || step.tool === 'extract' || step.tool === 'split' || step.tool === 'delete_pages' || step.tool === 'reorder_pages') {
        const total = doc.getPageCount();
        let targetIndices: number[] = [];
        if (step.tool === 'extract_pages' || step.tool === 'extract' || step.tool === 'split') {
            targetIndices = parsePageNumbers(step.pages?.toString() || step.pageInput?.toString() || '1', total);
        } else if (step.tool === 'delete_pages') {
            const toDelete = new Set(parsePageNumbers(step.pages?.toString() || '1', total));
            for(let p=0; p<total; p++) {
                if(!toDelete.has(p)) targetIndices.push(p);
            }
        } else if (step.tool === 'reorder_pages') {
            if(step.order === 'reverse') {
                for(let p=total-1; p>=0; p--) targetIndices.push(p);
            } else if (step.order) {
                targetIndices = parsePageNumbers(step.order.toString(), total);
            }
        }

        if (targetIndices.length > 0) {
            const newDoc = await PDFDocument.create();
            const copiedPages = await newDoc.copyPages(doc, targetIndices);
            copiedPages.forEach((p: any) => newDoc.addPage(p));
            doc = newDoc;
        }
    } else if (step.tool === 'merge') {
      // In pipeline, merge requires secondary file inputs, which is out of scope
      // for single-file sequential pipeline unless step.files is provided.
    } else if (step.tool === 'metadata' || step.tool === 'pdf_metadata') {
        if (step.title) doc.setTitle(step.title);
        if (step.author) doc.setAuthor(step.author);
        if (step.subject) doc.setSubject(step.subject);
        if (step.keywords) doc.setKeywords(Array.isArray(step.keywords) ? step.keywords : [step.keywords]);
        if (step.creator) doc.setCreator(step.creator);
        if (step.producer) doc.setProducer(step.producer);
    } else if (step.tool === 'header_footer' || step.tool === 'pdf_header_footer') {
        const font = await doc.embedFont(StandardFonts.Helvetica);
        const pages = doc.getPages();
        for (const page of pages) {
            const { width, height } = page.getSize();
            if (step.header) {
                page.drawText(String(step.header), {
                    x: width / 2 - (String(step.header).length * 3),
                    y: height - 30,
                    size: 10,
                    font,
                    color: rgb(0.3, 0.3, 0.3)
                });
            }
            if (step.footer) {
                page.drawText(String(step.footer), {
                    x: width / 2 - (String(step.footer).length * 3),
                    y: 20,
                    size: 10,
                    font,
                    color: rgb(0.3, 0.3, 0.3)
                });
            }
        }
    } else if (step.tool === 'bates' || step.tool === 'pdf_bates') {
        const font = await doc.embedFont(StandardFonts.HelveticaBold);
        const pages = doc.getPages();
        const prefix = step.prefix ?? 'DOC-';
        const start = Number(step.start ?? 1);
        pages.forEach((page: any, idx: number) => {
            const { width } = page.getSize();
            const batesNum = `${prefix}${String(start + idx).padStart(6, '0')}`;
            page.drawText(batesNum, {
                x: width - 120,
                y: 20,
                size: 9,
                font,
                color: rgb(0.2, 0.2, 0.2)
            });
        });
    } else if (step.tool === 'crop' || step.tool === 'pdf_crop') {
        // Simple margin crop
        const pages = doc.getPages();
        const margin = step.margin || 10;
        for (const page of pages) {
            const { width, height } = page.getSize();
            page.setCropBox(margin, margin, width - margin * 2, height - margin * 2);
        }
    }
    stepsExecuted++;
  }

  if (options.onProgress) {
    options.onProgress(options.steps.length, { tool: 'done' }, 100);
  }

  const finalBytes = await doc.save();

  return {
    success: true,
    data: finalBytes,
    filename: `pipeline_${filename}`,
    stepsExecuted,
    durationMs: Date.now() - startTime
  };
}

export function mount(selector: string | HTMLElement, options: EmbedOptions = {}): EmbedInstance {
  let container: HTMLElement | null = null;

  if (typeof selector === 'string') {
    container = document.querySelector(selector);
  } else {
    container = selector;
  }

  if (!container) {
    throw new Error(`PaperPilot: Target container not found for selector ${selector}`);
  }

  // Clear existing content and unmount previous instance if any
  if (instances.has(selector)) {
    unmount(selector);
  }
  container.innerHTML = '';

  const shadow = container.attachShadow({ mode: 'open' });

  // Inject theme styles
  const styleEl = document.createElement('style');
  styleEl.textContent = getThemeStyles({
    brandColor: options.brandColor,
    theme: options.theme,
    logoUrl: options.logoUrl,
  });
  shadow.appendChild(styleEl);

  const toolsString = Array.isArray(options.tools) ? options.tools.join(',') : (options.tools || 'all');

  // Mount Svelte component inside shadow root
  const component = mountSvelte(Widget, {
    target: shadow,
    props: {
      allowCustomization: options.allowCustomization === true,
      autoExecute: options.autoExecute !== false,
      tools: toolsString,
      hideBadge: options.hideBadge === true,
      brandColor: options.brandColor,
      theme: options.theme,
      logoUrl: options.logoUrl,
      pipeline: typeof options.pipeline === 'object' ? JSON.stringify(options.pipeline) : options.pipeline,
    },
  });

  const instance: EmbedInstance = {
    destroy: () => {
      unmountSvelte(component);
      shadow.innerHTML = ''; // Clean up
      instances.delete(selector);
    },
    element: container,
  };

  instances.set(selector, instance);
  return instance;
}

export function unmount(selector: string | HTMLElement): void {
  const instance = instances.get(selector);
  if (instance) {
    instance.destroy();
  }
}

export function on(event: string, callback: (data: any) => void): void {
  window.addEventListener(`paperpilot:${event}`, (e: Event) => {
    callback((e as CustomEvent).detail);
  });
}
