import * as fs from 'fs';
import * as pdfjsLib from 'pdfjs-dist/legacy/build/pdf.mjs';

export interface PdfInspectionResult {
  numPages: number;
  textByPage: string[];
  allText: string;
  hasText: (query: string) => boolean;
  lacksText: (query: string) => boolean;
  formFields?: Record<string, any>;
  metadata?: any;
  pageDimensions?: Array<{ width: number; height: number }>;
}

/**
 * Deep PDF parser & validator utilizing PDF.js engine.
 * Inspects real page numbers, text content, and structural properties.
 */
export class PdfInspector {
  public static async inspect(filePath: string): Promise<PdfInspectionResult> {
    if (!fs.existsSync(filePath)) {
      throw new Error(`PdfInspector Error: File not found at ${filePath}`);
    }

    const data = new Uint8Array(fs.readFileSync(filePath));
    const loadingTask = pdfjsLib.getDocument({
      data,
      useSystemFonts: true,
      disableFontFace: true,
      verbosity: 0
    });

    const doc = await loadingTask.promise;
    const numPages = doc.numPages;
    const textByPage: string[] = [];
    const pageDimensions: Array<{ width: number; height: number }> = [];
    const formFields: Record<string, any> = {};

    for (let i = 1; i <= numPages; i++) {
      const page = await doc.getPage(i);
      const viewport = page.getViewport({ scale: 1.0 });
      pageDimensions.push({ width: viewport.width, height: viewport.height });
      const textContent = await page.getTextContent();
      const pageText = textContent.items
        .map((item: any) => item.str || '')
        .join(' ');
      textByPage.push(pageText);

      try {
        const annotations = await page.getAnnotations();
        for (const annot of annotations) {
          if (annot.fieldName) {
            formFields[annot.fieldName] = annot.fieldValue ?? '';
          }
        }
      } catch (e) {}
    }

    const allText = textByPage.join('\n');
    let metadata: any = null;
    try {
      metadata = await doc.getMetadata();
    } catch (e) {}

    return {
      numPages,
      textByPage,
      allText,
      hasText: (query: string) => allText.toLowerCase().includes(query.toLowerCase()),
      lacksText: (query: string) => !allText.toLowerCase().includes(query.toLowerCase()),
      formFields,
      metadata,
      pageDimensions
    };
  }

  /**
   * Fast raw byte stream analysis for low-level PDF structural markers.
   */
  public static inspectRaw(filePath: string): {
    isPdf: boolean;
    hasDictionary: (dictName: string) => boolean;
    isEncrypted: boolean;
    isSigned: boolean;
    hasToken: (token: string) => boolean;
    rawContent: string;
    buffer: Buffer;
  } {
    const buffer = fs.readFileSync(filePath);
    const content = buffer.toString('latin1');
    const isPdf = buffer.subarray(0, 5).toString('latin1') === '%PDF-';

    return {
      isPdf,
      hasDictionary: (dictName: string) => content.includes(`/${dictName}`),
      isEncrypted: content.includes('/Encrypt'),
      isSigned: content.includes('/Sig') || content.includes('/ByteRange'),
      hasToken: (token: string) => content.includes(token),
      rawContent: content,
      buffer
    };
  }
}

