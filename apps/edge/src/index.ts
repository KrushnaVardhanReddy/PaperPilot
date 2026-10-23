import init, { WasmPdfEngine } from 'paperpilot-wasm';
import wasmModule from 'paperpilot-wasm/paperpilot_wasm_bg.wasm';

let wasmReady: Promise<any> | null = null;
async function ensureWasm() {
  if (!wasmReady) {
    wasmReady = init(wasmModule);
  }
  await wasmReady;
}

function assertSafeUrl(urlString: string) {
  const parsed = new URL(urlString);
  const host = parsed.hostname;

  if (host === 'localhost' || host === '127.0.0.1' || host === '::1') {
    throw new Error('SSRF blocked: Loopback address not allowed');
  }
  if (host.startsWith('10.')) {
    throw new Error('SSRF blocked: Private network address not allowed');
  }
  if (host.startsWith('192.168.')) {
    throw new Error('SSRF blocked: Private network address not allowed');
  }
  if (host === '169.254.169.254') {
    throw new Error('SSRF blocked: Cloud metadata address not allowed');
  }
  // 172.16.x.x to 172.31.x.x
  if (host.match(/^172\.(1[6-9]|2\d|3[01])\./)) {
    throw new Error('SSRF blocked: Private network address not allowed');
  }
}

async function resolveInputBytes(request: Request, url: URL): Promise<Uint8Array> {
  const remoteUrl = url.searchParams.get('url');
  let urlFromJson = null;
  let rawBody: ArrayBuffer | null = null;

  // Try to clone the request to parse JSON if it has JSON content type
  const contentType = request.headers.get('content-type') || '';
  if (contentType.includes('application/json')) {
    const cloned = request.clone();
    try {
      const data = await cloned.json();
      urlFromJson = data.url;
    } catch (e) {
      // Ignored
    }
  }

  const targetUrl = remoteUrl || urlFromJson;

  if (targetUrl) {
    assertSafeUrl(targetUrl);
    const res = await fetch(targetUrl);
    if (!res.ok) {
      throw new Error(`Failed to fetch remote PDF: HTTP ${res.status}`);
    }
    const buf = await res.arrayBuffer();
    const bytes = new Uint8Array(buf);

    // Check %PDF magic bytes
    if (bytes.length < 4 || bytes[0] !== 0x25 || bytes[1] !== 0x50 || bytes[2] !== 0x44 || bytes[3] !== 0x46) {
      throw new Error('URL does not return a valid PDF document');
    }

    return bytes;
  }

  return new Uint8Array(await request.arrayBuffer());
}

export default {
  async fetch(request: Request, env: any, ctx: any): Promise<Response> {
    const url = new URL(request.url);

    if (request.method === 'GET' && url.pathname === '/health') {
      return Response.json({ status: 'ok', engine: 'paperpilot-wasm', version: '0.1.0' });
    }

    if (request.method === 'GET' && url.pathname === '/api/v1/workflow/presets') {
      await ensureWasm();
      const presets = WasmPdfEngine.workflow_presets();
      return new Response(presets, { headers: { 'Content-Type': 'application/json' } });
    }

    await ensureWasm();

    if (request.method === 'POST') {
      try {
        if (url.pathname === '/api/v1/workflow') {
          const body = await resolveInputBytes(request, url);
          const recipeOrPreset = url.searchParams.get('preset') || url.searchParams.get('recipe') || 'preset-legal-discovery';
          const result = WasmPdfEngine.workflow(body, recipeOrPreset);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/pdf_to_docx' || url.pathname === '/api/v1/convert' && url.searchParams.get('format') === 'docx') {
          const body = await resolveInputBytes(request, url);
          const result = WasmPdfEngine.pdf_to_docx(body);
          return new Response(result, { headers: { 'Content-Type': 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' } });
        }
        if (url.pathname === '/api/v1/rotate') {
          const body = await resolveInputBytes(request, url);
          const angle = parseInt(url.searchParams.get('angle') || '90');
          const pages = url.searchParams.get('pages') || 'all';
          const result = WasmPdfEngine.rotate(body, angle, pages);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/compress') {
          const body = await resolveInputBytes(request, url);
          const result = WasmPdfEngine.compress(body);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/watermark') {
          const body = await resolveInputBytes(request, url);
          const text = url.searchParams.get('text') || 'CONFIDENTIAL';
          let angle: number | undefined;
          let opacity: number | undefined;
          if (url.searchParams.has('angle')) {
            angle = parseFloat(url.searchParams.get('angle') as string);
          }
          if (url.searchParams.has('opacity')) {
            opacity = parseFloat(url.searchParams.get('opacity') as string);
          }
          const result = WasmPdfEngine.watermark(body, text, angle, opacity);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/json_to_pdf') {
          const body = await request.text();
          const result = WasmPdfEngine.json_to_pdf(body);
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
          const body = await resolveInputBytes(request, url);
          const ranges = url.searchParams.get('ranges') || '1';
          const result = WasmPdfEngine.split(body, ranges);

          if (result.length > 0) {
            return new Response(result[0], { headers: { 'Content-Type': 'application/pdf' } });
          } else {
             return new Response('Failed to split', { status: 400 });
          }
        }
        if (url.pathname === '/api/v1/encrypt') {
          const body = await resolveInputBytes(request, url);
          const password = url.searchParams.get('password') || '';
          if (!password) return new Response('password required', { status: 400 });
          const result = WasmPdfEngine.encrypt(body, password);
          return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/delete_pages') {
           const body = await resolveInputBytes(request, url);
           const pages = url.searchParams.get('pages') || '';
           const result = WasmPdfEngine.delete_pages(body, pages);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/extract_pages') {
           const body = await resolveInputBytes(request, url);
           const pages = url.searchParams.get('pages') || '';
           const result = WasmPdfEngine.extract_pages(body, pages);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/reorder_pages') {
           const body = await resolveInputBytes(request, url);
           const order = url.searchParams.get('order') || '';
           const result = WasmPdfEngine.reorder_pages(body, order);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/crop') {
           const body = await resolveInputBytes(request, url);
           const pages = url.searchParams.get('pages') || 'all';
           const bbox = url.searchParams.get('bbox') || '0,0,100,100';
           const [left, bottom, right, top] = bbox.split(',').map(Number);
           const result = WasmPdfEngine.crop(body, pages, left, bottom, right, top);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/flatten') {
           const body = await resolveInputBytes(request, url);
           const result = WasmPdfEngine.flatten(body);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/pii-scan') {
           const body = await resolveInputBytes(request, url);
           const options = url.searchParams.get('options') || '{}';
           const result = WasmPdfEngine.pii_scan(body, options);
           return new Response(result, { headers: { 'Content-Type': 'application/json' } });
        }
        if (url.pathname === '/api/v1/sanitize') {
           const body = await resolveInputBytes(request, url);
           const options = url.searchParams.get('options') || '{}';
           const result = WasmPdfEngine.sanitize(body, options);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }
        if (url.pathname === '/api/v1/set_metadata') {
           const body = await resolveInputBytes(request, url);
           const author = url.searchParams.get('author') || '';
           const title = url.searchParams.get('title') || '';
           const subject = url.searchParams.get('subject') || '';
           const keywords = url.searchParams.get('keywords') || '';

           const result = WasmPdfEngine.set_metadata(body, author, title, subject, keywords);
           return new Response(result, { headers: { 'Content-Type': 'application/pdf' } });
        }

        if (url.pathname === '/api/v1/validate-signatures') {
           const body = await request.arrayBuffer();
           const options = url.searchParams.get('options') || '{}';
           const result = WasmPdfEngine.pdf_validate_signatures(new Uint8Array(body), options);
           return new Response(result, { headers: { 'Content-Type': 'application/json' } });
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
