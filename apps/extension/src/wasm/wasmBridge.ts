/**
 * PaperPilot Client WASM Engine Bridge
 * Provides instant in-browser document processing using paperpilot-wasm.
 * Falls back gracefully to Desktop Gateway if WASM is unsupported or initializing.
 */

// @ts-ignore - wasm-bindgen module
import init, { WasmPdfEngine } from 'paperpilot-wasm';
import { executeGatewayTool, checkGatewayHealth } from '../background/gatewayClient';

export interface PdfMetadata {
  pageCount: number;
  version: string;
  encrypted: boolean;
  title: string;
  author: string;
}

export type WasmStatus = 'uninitialized' | 'initializing' | 'ready' | 'error';

let wasmStatus: WasmStatus = 'uninitialized';
let initPromise: Promise<void> | null = null;
let lastError: Error | null = null;

export function isWasmSupported(): boolean {
  return typeof WebAssembly !== 'undefined' && typeof WebAssembly.instantiate === 'function';
}

export function getWasmStatus(): WasmStatus {
  return wasmStatus;
}

export function getWasmError(): Error | null {
  return lastError;
}

/**
 * Initialize paperpilot-wasm module with fallback paths
 */
export async function ensureWasmReady(): Promise<void> {
  if (wasmStatus === 'ready') return;
  if (initPromise) return initPromise;

  if (!isWasmSupported()) {
    wasmStatus = 'error';
    lastError = new Error('WebAssembly is not supported in this browser environment');
    throw lastError;
  }

  wasmStatus = 'initializing';

  initPromise = (async () => {
    try {
      // 1. Try default bundler-provided init
      await init();
      wasmStatus = 'ready';
    } catch (e1: any) {
      // 2. If in extension environment, try chrome.runtime.getURL
      try {
        if (typeof chrome !== 'undefined' && chrome.runtime?.getURL) {
          const wasmUrl = chrome.runtime.getURL('paperpilot_wasm_bg.wasm');
          await init(wasmUrl);
          wasmStatus = 'ready';
          return;
        }
      } catch (_e2) {
        // Continue to error throw
      }

      wasmStatus = 'error';
      lastError = e1 instanceof Error ? e1 : new Error(String(e1));
      throw lastError;
    }
  })();

  return initPromise;
}

/**
 * Convert Uint8Array to base64 string
 */
export function uint8ArrayToBase64(bytes: Uint8Array): string {
  let binary = '';
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}

/**
 * Convert base64 string to Uint8Array
 */
export function base64ToUint8Array(base64: string): Uint8Array {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes;
}

/**
 * Split PDF pages by range (e.g. "1", "1-2, 3-5")
 */
export async function splitPdf(inputBytes: Uint8Array, ranges: string): Promise<Uint8Array[]> {
  try {
    await ensureWasmReady();
    const result = WasmPdfEngine.split(inputBytes, ranges);
    return Array.from(result).map((buf: any) => new Uint8Array(buf));
  } catch (err: any) {
    // Gateway fallback
    const health = await checkGatewayHealth();
    if (health.ok) {
      const b64 = uint8ArrayToBase64(inputBytes);
      const res = await executeGatewayTool('split', { file_base64: b64, pages: ranges });
      if (res && res.files) {
        return res.files.map((f: string) => base64ToUint8Array(f));
      }
    }
    throw new Error(`PDF split failed: ${err.message || err}`);
  }
}

/**
 * Merge multiple PDF buffers into a single document
 */
export async function mergePdfs(buffers: Uint8Array[]): Promise<Uint8Array> {
  if (buffers.length === 0) {
    throw new Error('At least one PDF buffer required for merge');
  }

  try {
    await ensureWasmReady();
    const result = WasmPdfEngine.merge(buffers);
    return new Uint8Array(result);
  } catch (err: any) {
    const health = await checkGatewayHealth();
    if (health.ok) {
      const b64List = buffers.map(uint8ArrayToBase64);
      const res = await executeGatewayTool('merge', { files_base64: b64List });
      if (res && res.file_base64) {
        return base64ToUint8Array(res.file_base64);
      }
    }
    throw new Error(`PDF merge failed: ${err.message || err}`);
  }
}

/**
 * Rotate PDF pages by 90, 180, or 270 degrees
 */
export async function rotatePdf(
  inputBytes: Uint8Array,
  angle: number,
  pages = 'all'
): Promise<Uint8Array> {
  try {
    await ensureWasmReady();
    const result = WasmPdfEngine.rotate(inputBytes, angle, pages);
    return new Uint8Array(result);
  } catch (err: any) {
    const health = await checkGatewayHealth();
    if (health.ok) {
      const b64 = uint8ArrayToBase64(inputBytes);
      const res = await executeGatewayTool('pdf_rotate', {
        file_base64: b64,
        angle,
        pages,
      });
      if (res && res.file_base64) {
        return base64ToUint8Array(res.file_base64);
      }
    }
    throw new Error(`PDF rotate failed: ${err.message || err}`);
  }
}

/**
 * Extract clean plaintext from PDF document
 */
export async function extractText(inputBytes: Uint8Array): Promise<string> {
  try {
    await ensureWasmReady();
    return WasmPdfEngine.extract_text(inputBytes);
  } catch (err: any) {
    const health = await checkGatewayHealth();
    if (health.ok) {
      const b64 = uint8ArrayToBase64(inputBytes);
      const res = await executeGatewayTool('extract_text', { file_base64: b64 });
      if (res && typeof res.text === 'string') {
        return res.text;
      }
    }
    throw new Error(`PDF text extraction failed: ${err.message || err}`);
  }
}

/**
 * Extract document metadata (page count, title, author, version)
 */
export async function extractMetadata(inputBytes: Uint8Array): Promise<PdfMetadata> {
  try {
    await ensureWasmReady();
    const infoJson = WasmPdfEngine.pdf_info(inputBytes);
    const parsed = JSON.parse(infoJson);
    return {
      pageCount: parsed.page_count ?? 1,
      version: parsed.version ?? '1.7',
      encrypted: !!parsed.encrypted,
      title: parsed.title ?? '',
      author: parsed.author ?? '',
    };
  } catch (err: any) {
    // Basic fallback parsing for page count & version
    const textSample = new TextDecoder('latin1').decode(inputBytes.subarray(0, 4096));
    const versionMatch = textSample.match(/%PDF-(\d+\.\d+)/);
    const countMatch = textSample.match(/\/Count\s+(\d+)/);
    return {
      pageCount: countMatch ? parseInt(countMatch[1], 10) : 1,
      version: versionMatch ? versionMatch[1] : '1.4',
      encrypted: textSample.includes('/Encrypt'),
      title: '',
      author: '',
    };
  }
}

/**
 * Add diagonal or horizontal watermark text across pages
 */
export async function watermarkPdf(
  inputBytes: Uint8Array,
  text: string,
  angle = 45,
  opacity = 0.3
): Promise<Uint8Array> {
  try {
    await ensureWasmReady();
    const result = WasmPdfEngine.watermark(inputBytes, text, angle, opacity);
    return new Uint8Array(result);
  } catch (err: any) {
    const health = await checkGatewayHealth();
    if (health.ok) {
      const b64 = uint8ArrayToBase64(inputBytes);
      const res = await executeGatewayTool('watermark', {
        file_base64: b64,
        text,
        angle,
        opacity,
      });
      if (res && res.file_base64) {
        return base64ToUint8Array(res.file_base64);
      }
    }
    throw new Error(`PDF watermark failed: ${err.message || err}`);
  }
}

/**
 * Compress PDF streams and clean cross references
 */
export async function compressPdf(
  inputBytes: Uint8Array,
  quality?: string
): Promise<Uint8Array> {
  try {
    await ensureWasmReady();
    const result = WasmPdfEngine.compress(inputBytes, quality);
    return new Uint8Array(result);
  } catch (err: any) {
    const health = await checkGatewayHealth();
    if (health.ok) {
      const b64 = uint8ArrayToBase64(inputBytes);
      const res = await executeGatewayTool('pdf_compress', {
        file_base64: b64,
        quality: quality || 'medium',
      });
      if (res && res.file_base64) {
        return base64ToUint8Array(res.file_base64);
      }
    }
    throw new Error(`PDF compression failed: ${err.message || err}`);
  }
}
