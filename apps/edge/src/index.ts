import init, { WasmPdfEngine } from 'paperpilot-wasm';
import wasmModule from 'paperpilot-wasm/paperpilot_wasm_bg.wasm';

let wasmReady: Promise<any> | null = null;
async function ensureWasm() {
  if (!wasmReady) {
    wasmReady = init(wasmModule);
  }
  await wasmReady;
}

export default {
  async fetch(request: Request, env: any, ctx: any): Promise<Response> {
    const url = new URL(request.url);

    if (request.method === 'GET' && url.pathname === '/health') {
      return Response.json({ status: 'ok', engine: 'paperpilot-wasm', version: '0.1.0' });
    }

    await ensureWasm();

    if (request.method === 'POST') {
      try {
        if (url.pathname === '/api/v1/rotate') {
          const body = await request.arrayBuffer();
          const angle = parseInt(url.searchParams.get('angle') || '90');
          const pages = url.searchParams.get('pages') || 'all';
          const result = WasmPdfEngine.rotate(new Uint8Array(body), angle, pages);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/compress') {
          const body = await request.arrayBuffer();
          const result = WasmPdfEngine.compress(new Uint8Array(body));
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/watermark') {
          const body = await request.arrayBuffer();
          const text = url.searchParams.get('text') || 'CONFIDENTIAL';
          const result = WasmPdfEngine.watermark(new Uint8Array(body), text);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/merge') {
          const formData = await request.formData();
          const files: Uint8Array[] = [];
          for (const entry of formData.values()) {
            if (entry instanceof File) {
              files.push(new Uint8Array(await entry.arrayBuffer()));
            }
          }
          if (files.length < 2) {
            return new Response('At least 2 files required', { status: 400 });
          }
          const result = WasmPdfEngine.merge(files);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/split') {
          const body = await request.arrayBuffer();
          const ranges = url.searchParams.get('ranges') || '1';
          const result = WasmPdfEngine.split(new Uint8Array(body), ranges);

          if (result.length > 0) {
            return new Response(result[0], { headers: { 'Content-Type': 'application/pdf' } });
          } else {
             return new Response('Failed to split', { status: 400 });
          }
        }
        if (url.pathname === '/api/v1/encrypt') {
          const body = await request.arrayBuffer();
          const password = url.searchParams.get('password') || '';
          if (!password) return new Response('password required', { status: 400 });
          const result = WasmPdfEngine.encrypt(new Uint8Array(body), password);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/delete_pages') {
           const body = await request.arrayBuffer();
           const pages = url.searchParams.get('pages') || '';
           const result = WasmPdfEngine.delete_pages(new Uint8Array(body), pages);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/extract_pages') {
           const body = await request.arrayBuffer();
           const pages = url.searchParams.get('pages') || '';
           const result = WasmPdfEngine.extract_pages(new Uint8Array(body), pages);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/reorder_pages') {
           const body = await request.arrayBuffer();
           const order = url.searchParams.get('order') || '';
           const result = WasmPdfEngine.reorder_pages(new Uint8Array(body), order);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/crop') {
           const body = await request.arrayBuffer();
           const pages = url.searchParams.get('pages') || 'all';
           const bbox = url.searchParams.get('bbox') || '0,0,100,100';
           const [left, bottom, right, top] = bbox.split(',').map(Number);
           const result = WasmPdfEngine.crop(new Uint8Array(body), pages, left, bottom, right, top);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/flatten') {
           const body = await request.arrayBuffer();
           const result = WasmPdfEngine.flatten(new Uint8Array(body));
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/set_metadata') {
           const body = await request.arrayBuffer();
           const author = url.searchParams.get('author') || '';
           const title = url.searchParams.get('title') || '';
           const subject = url.searchParams.get('subject') || '';
           const keywords = url.searchParams.get('keywords') || '';

           const result = WasmPdfEngine.set_metadata(new Uint8Array(body), author, title, subject, keywords);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }

      } catch (err: any) {
        return new Response(JSON.stringify({ error: err.message || String(err) }), {
          status: 500,
          headers: { 'Content-Type': 'application/json' }
        });
      }
    }

    return new Response('Not Found', { status: 404 });
  }
};
