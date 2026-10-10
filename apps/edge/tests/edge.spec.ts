import { describe, it, expect, vi } from 'vitest';
import worker from '../src/index';

// Mock the wasm module and init function since Node ESM does not support native .wasm imports without flags
vi.mock('paperpilot-wasm/paperpilot_wasm_bg.wasm', () => {
  return {
    default: new WebAssembly.Module(new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]))
  };
});

vi.mock('paperpilot-wasm', () => {
  return {
    default: vi.fn().mockResolvedValue(undefined),
    WasmPdfEngine: {
      rotate: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])), // %PDF-
      compress: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      watermark: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      merge: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      split: vi.fn().mockReturnValue([new Uint8Array([37, 80, 68, 70, 45])]),
      encrypt: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      delete_pages: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      extract_pages: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      reorder_pages: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      crop: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      flatten: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      set_metadata: vi.fn().mockReturnValue(new Uint8Array([37, 80, 68, 70, 45])),
      pdf_to_docx: vi.fn().mockReturnValue(new Uint8Array([80, 75, 3, 4])),
    }
  };
});

const createRequest = (path: string, method = 'GET', body?: ArrayBuffer | FormData) => {
  return new Request(`http://localhost${path}`, {
    method,
    body: body instanceof ArrayBuffer ? body : body,
  });
};

describe('Cloudflare Edge Worker', () => {
  it('should return health status', async () => {
    const request = createRequest('/health');
    const response = await worker.fetch(request, {}, {});
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.status).toBe('ok');
    expect(data.engine).toBe('paperpilot-wasm');
  });

  it('should return 404 for unknown routes', async () => {
    const request = createRequest('/unknown');
    const response = await worker.fetch(request, {}, {});
    expect(response.status).toBe(404);
  });

  const endpoints = [
    { path: '/api/v1/rotate', method: 'POST' },
    { path: '/api/v1/compress', method: 'POST' },
    { path: '/api/v1/watermark', method: 'POST' },
    { path: '/api/v1/split', method: 'POST' },
    { path: '/api/v1/encrypt?password=test', method: 'POST' },
    { path: '/api/v1/delete_pages', method: 'POST' },
    { path: '/api/v1/extract_pages', method: 'POST' },
    { path: '/api/v1/reorder_pages', method: 'POST' },
    { path: '/api/v1/crop', method: 'POST' },
    { path: '/api/v1/flatten', method: 'POST' },
    { path: '/api/v1/set_metadata', method: 'POST' },
    { path: '/api/v1/pdf_to_docx', method: 'POST' },
  ];

  for (const { path, method } of endpoints) {
    it(`should process ${path} and return valid payload`, async () => {
      const body = new Uint8Array([37, 80, 68, 70, 45]).buffer; // dummy %PDF- input
      const request = createRequest(path, method, body);
      const response = await worker.fetch(request, {}, {});
      expect(response.status).toBe(200);

      if (path === '/api/v1/pdf_to_docx') {
        expect(response.headers.get('Content-Type')).toBe('application/vnd.openxmlformats-officedocument.wordprocessingml.document');
        const resBody = await response.arrayBuffer();
        const bytes = new Uint8Array(resBody);
        expect(bytes[0]).toBe(80);
        expect(bytes[1]).toBe(75);
      } else {
        expect(response.headers.get('Content-Type')).toBe('application/pdf');

        const resBody = await response.arrayBuffer();
        const bytes = new Uint8Array(resBody);

        // %PDF-
        expect(bytes[0]).toBe(37);
        expect(bytes[1]).toBe(80);
        expect(bytes[2]).toBe(68);
        expect(bytes[3]).toBe(70);
        expect(bytes[4]).toBe(45);
      }
    });
  }

  it('should process /api/v1/merge and return valid PDF payload', async () => {
    const formData = new FormData();
    const file1 = new File([new Uint8Array([37, 80, 68, 70, 45]).buffer], "file1.pdf", { type: "application/pdf" });
    const file2 = new File([new Uint8Array([37, 80, 68, 70, 45]).buffer], "file2.pdf", { type: "application/pdf" });
    formData.append("file1", file1);
    formData.append("file2", file2);

    const request = createRequest('/api/v1/merge', 'POST', formData);
    const response = await worker.fetch(request, {}, {});
    expect(response.status).toBe(200);
    expect(response.headers.get('Content-Type')).toBe('application/pdf');

    const resBody = await response.arrayBuffer();
    const bytes = new Uint8Array(resBody);

    expect(bytes[0]).toBe(37);
    expect(bytes[1]).toBe(80);
    expect(bytes[2]).toBe(68);
    expect(bytes[3]).toBe(70);
    expect(bytes[4]).toBe(45);
  });
});
