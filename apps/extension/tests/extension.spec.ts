import { describe, it, expect, beforeAll } from 'vitest';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { JSDOM } from 'jsdom';
import { extractArticleMarkdown } from '../src/content/articleReader';
import { checkGatewayHealth, DEFAULT_GATEWAY_URL } from '../src/background/gatewayClient';
import {
  uint8ArrayToBase64,
  base64ToUint8Array,
  isWasmSupported,
  getWasmStatus,
  splitPdf,
  mergePdfs,
  rotatePdf,
  extractText,
  extractMetadata,
  watermarkPdf,
} from '../src/wasm/wasmBridge';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const distDir = path.resolve(__dirname, '../dist');

describe('PaperPilot Browser Companion Extension Build', () => {
  it('should generate all required distribution build artifacts', () => {
    expect(fs.existsSync(distDir)).toBe(true);

    // Core extension bundles
    expect(fs.existsSync(path.join(distDir, 'manifest.json'))).toBe(true);
    expect(fs.existsSync(path.join(distDir, 'background.js'))).toBe(true);
    expect(fs.existsSync(path.join(distDir, 'content.js'))).toBe(true);
    expect(fs.existsSync(path.join(distDir, 'popup.html'))).toBe(true);
    expect(fs.existsSync(path.join(distDir, 'paperpilot_wasm_bg.wasm'))).toBe(true);

    // Bundle sizes should be non-trivial
    expect(fs.statSync(path.join(distDir, 'background.js')).size).toBeGreaterThan(1000);
    expect(fs.statSync(path.join(distDir, 'content.js')).size).toBeGreaterThan(1000);
    expect(fs.statSync(path.join(distDir, 'popup.html')).size).toBeGreaterThan(200);

    // Icons
    expect(fs.existsSync(path.join(distDir, 'icons/icon-16.png'))).toBe(true);
    expect(fs.existsSync(path.join(distDir, 'icons/icon-48.png'))).toBe(true);
    expect(fs.existsSync(path.join(distDir, 'icons/icon-128.png'))).toBe(true);
  });

  it('should conform to Manifest V3 schema and spec requirements', () => {
    const manifestRaw = fs.readFileSync(path.join(distDir, 'manifest.json'), 'utf-8');
    const manifest = JSON.parse(manifestRaw);

    expect(manifest.manifest_version).toBe(3);
    expect(manifest.name).toBe('PaperPilot: Universal Document & AI Companion');

    // Permissions
    expect(manifest.permissions).toContain('activeTab');
    expect(manifest.permissions).toContain('contextMenus');
    expect(manifest.permissions).toContain('storage');
    expect(manifest.permissions).toContain('scripting');

    // Host permissions
    expect(manifest.host_permissions).toContain('http://127.0.0.1:7823/*');
    expect(manifest.host_permissions).toContain('http://localhost:7823/*');

    // Action and background
    expect(manifest.action.default_popup).toBe('popup.html');
    expect(manifest.background.service_worker).toBe('background.js');

    // Content scripts
    expect(manifest.content_scripts).toBeDefined();
    expect(manifest.content_scripts[0].js).toContain('content.js');
  });

  it('should ensure popup.html has valid root mount and relative script paths', () => {
    const popupHtml = fs.readFileSync(path.join(distDir, 'popup.html'), 'utf-8');
    expect(popupHtml).toContain('id="app"');
    expect(popupHtml).toMatch(/src="(?:\.\/|\/)assets\/popup/);
  });
});

describe('Article Reader (Clean Web Capture & Markdown)', () => {
  it('should sanitize unwanted elements (script, style, nav, aside, ads) and extract clean markdown', () => {
    const rawHtml = `
      <!DOCTYPE html>
      <html>
        <head>
          <title>Attention Is All You Need</title>
          <meta property="og:title" content="Attention Is All You Need - Vaswani et al." />
          <meta name="author" content="Ashish Vaswani" />
        </head>
        <body>
          <nav>
            <a href="/home">Home</a>
            <a href="/papers">Papers</a>
          </nav>
          <header>
            <div class="ad-banner">Buy Cloud GPUs!</div>
          </header>
          <article>
            <h1>Attention Is All You Need</h1>
            <p>The dominant sequence transduction models are based on complex recurrent or convolutional neural networks.</p>
            <div class="ad">Commercial Sponsored Link</div>
            <h2>Introduction</h2>
            <p>We propose the <strong>Transformer</strong>, a model architecture eschewing recurrence.</p>
            <ul>
              <li>Self-attention mechanism</li>
              <li>Multi-head projection</li>
            </ul>
            <blockquote>Attention mechanisms have become an integral part of compelling sequence modeling.</blockquote>
            <pre><code>def attention(q, k, v):\n    return softmax(q @ k.T) @ v</code></pre>
            <aside>
              <div class="sidebar">Related papers list</div>
            </aside>
            <p>Check out the code at <a href="https://github.com/tensorflow/tensor2tensor">Tensor2Tensor</a>.</p>
          </article>
          <div class="cookie-banner">Accept cookies?</div>
          <footer>Copyright 2026</footer>
          <script>console.log("tracker script");</script>
        </body>
      </html>
    `;

    const dom = new JSDOM(rawHtml, { url: 'https://arxiv.org/abs/1706.03762' });
    const result = extractArticleMarkdown(dom.window.document);

    expect(result.title).toBe('Attention Is All You Need - Vaswani et al.');
    expect(result.byline).toBe('Ashish Vaswani');
    expect(result.url).toBe('https://arxiv.org/abs/1706.03762');
    expect(result.wordCount).toBeGreaterThan(20);
    expect(result.excerpt).toBeDefined();

    // Verify sanitized out junk
    expect(result.markdown).not.toContain('Buy Cloud GPUs');
    expect(result.markdown).not.toContain('Commercial Sponsored Link');
    expect(result.markdown).not.toContain('Accept cookies');
    expect(result.markdown).not.toContain('tracker script');
    expect(result.markdown).not.toContain('Related papers list');

    // Verify preserved markdown structures
    expect(result.markdown).toContain('# Attention Is All You Need');
    expect(result.markdown).toContain('## Introduction');
    expect(result.markdown).toContain('**Transformer**');
    expect(result.markdown).toContain('- Self-attention mechanism');
    expect(result.markdown).toContain('> Attention mechanisms have become an integral part');
    expect(result.markdown).toContain('```');
    expect(result.markdown).toContain('[Tensor2Tensor](https://github.com/tensorflow/tensor2tensor)');
  });
});

describe('Gateway Client Bridge', () => {
  it('should detect when Gateway is offline gracefully without throwing unhandled exceptions', async () => {
    // Port 59999 should not have a PaperPilot daemon running
    const health = await checkGatewayHealth('http://127.0.0.1:59999');
    expect(health.ok).toBe(false);
    expect(health.error).toBeDefined();
  });

  it('should define default local port 7823', () => {
    expect(DEFAULT_GATEWAY_URL).toBe('http://127.0.0.1:7823');
  });
});

describe('Client WASM Engine Bridge', () => {
  it('should correctly convert Uint8Array to base64 and back', () => {
    const original = new Uint8Array([80, 65, 80, 69, 82, 80, 73, 76, 79, 84]); // "PAPERPILOT"
    const b64 = uint8ArrayToBase64(original);
    expect(typeof b64).toBe('string');
    const restored = base64ToUint8Array(b64);
    expect(restored).toEqual(original);
  });

  it('should export all required client-side PDF functions', () => {
    expect(typeof splitPdf).toBe('function');
    expect(typeof mergePdfs).toBe('function');
    expect(typeof rotatePdf).toBe('function');
    expect(typeof extractText).toBe('function');
    expect(typeof extractMetadata).toBe('function');
    expect(typeof watermarkPdf).toBe('function');
    expect(typeof isWasmSupported).toBe('function');
    expect(typeof getWasmStatus).toBe('function');
  });

  it('should fallback gracefully when empty buffer is passed to merge', async () => {
    await expect(mergePdfs([])).rejects.toThrow('At least one PDF buffer required for merge');
  });
});
