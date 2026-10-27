#!/usr/bin/env node

/**
 * PaperPilot Portable HTML Generator
 * Bundles the UI, styles, icons, and base64-inlined WebAssembly runtime
 * into a single, zero-dependency, air-gapped `paperpilot-portable.html` file.
 */

const fs = require('fs');
const path = require('path');

const ROOT_DIR = path.resolve(__dirname, '..');
const WASM_PKG_DIR = path.join(ROOT_DIR, 'paperpilot-wasm', 'pkg');
const OUTPUT_DIR = path.join(ROOT_DIR, 'dist-portable');
const OUTPUT_FILE = path.join(OUTPUT_DIR, 'paperpilot-portable.html');

console.log('🚀 Building PaperPilot Portable Air-Gapped HTML Sandbox...');

// 1. Check for compiled WASM artifacts
const wasmBinaryPath = path.join(WASM_PKG_DIR, 'paperpilot_wasm_bg.wasm');
const wasmJsPath = path.join(WASM_PKG_DIR, 'paperpilot_wasm.js');

if (!fs.existsSync(wasmBinaryPath) || !fs.existsSync(wasmJsPath)) {
  console.error('❌ Error: paperpilot_wasm artifacts not found in paperpilot-wasm/pkg.');
  console.error('Run: wasm-pack build --target web paperpilot-wasm');
  process.exit(1);
}

// 2. Read and Base64 encode the WASM binary
console.log('📦 Reading & encoding paperpilot_wasm_bg.wasm...');
const wasmBuffer = fs.readFileSync(wasmBinaryPath);
const wasmBase64 = wasmBuffer.toString('base64');
console.log(`   WASM binary size: ${(wasmBuffer.length / (1024 * 1024)).toFixed(2)} MB`);
console.log(`   Base64 size: ${(wasmBase64.length / (1024 * 1024)).toFixed(2)} MB`);

// 3. Read JS glue code and adapt for inline initialization
let wasmJsCode = fs.readFileSync(wasmJsPath, 'utf8');

// Strip default export and import.meta.url fetch logic
wasmJsCode = wasmJsCode.replace(/export\s+default\s+__wbg_init;?/, '');
wasmJsCode = wasmJsCode.replace(/export\s*\{[^}]+\};?/g, '');
wasmJsCode = wasmJsCode.replace(/export\s+/g, '');
wasmJsCode = wasmJsCode.replace(/import\.meta\.url/g, '""');

// Ensure output directory exists
if (!fs.existsSync(OUTPUT_DIR)) {
  fs.mkdirSync(OUTPUT_DIR, { recursive: true });
}

// 4. Construct the Single-File HTML Template
const htmlContent = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <!-- Strict Air-Gap Content Security Policy: NO External Network Requests -->
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'unsafe-eval'; style-src 'unsafe-inline'; img-src data: blob:; font-src data:; connect-src blob: data:; worker-src blob: data:;">
  <title>PaperPilot Portable — Air-Gapped Single-File PDF Tools</title>
  <style>
    :root {
      --bg-primary: #0a0d14;
      --bg-secondary: #121824;
      --bg-card: rgba(22, 30, 46, 0.7);
      --border: rgba(255, 255, 255, 0.08);
      --border-active: rgba(56, 189, 248, 0.4);
      --text-main: #f8fafc;
      --text-muted: #94a3b8;
      --accent: #38bdf8;
      --accent-glow: rgba(56, 189, 248, 0.25);
      --success: #34d399;
      --danger: #f87171;
      --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    }

    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background-color: var(--bg-primary);
      color: var(--text-main);
      font-family: var(--font);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 24px 16px 48px;
    }

    .container {
      width: 100%;
      max-width: 900px;
      display: flex;
      flex-direction: column;
      gap: 24px;
    }

    header {
      text-align: center;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 8px;
    }

    .badge-bar {
      display: flex;
      gap: 8px;
      margin-bottom: 4px;
      flex-wrap: wrap;
      justify-content: center;
    }

    .badge {
      display: inline-flex;
      align-items: center;
      gap: 6px;
      padding: 4px 12px;
      border-radius: 9999px;
      font-size: 0.75rem;
      font-weight: 600;
      letter-spacing: 0.03em;
      text-transform: uppercase;
    }

    .badge-airgap {
      background: rgba(52, 211, 153, 0.12);
      color: var(--success);
      border: 1px solid rgba(52, 211, 153, 0.3);
    }

    .badge-standalone {
      background: rgba(56, 189, 248, 0.12);
      color: var(--accent);
      border: 1px solid rgba(56, 189, 248, 0.3);
    }

    h1 {
      font-size: 2.25rem;
      font-weight: 800;
      letter-spacing: -0.03em;
      background: linear-gradient(135deg, #fff 0%, #94a3b8 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }

    p.subtitle {
      color: var(--text-muted);
      font-size: 0.95rem;
      max-width: 600px;
    }

    .card {
      background: var(--bg-card);
      border: 1px solid var(--border);
      border-radius: 16px;
      padding: 24px;
      backdrop-filter: blur(12px);
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.36);
    }

    .tool-selector {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
      gap: 10px;
      margin-bottom: 20px;
    }

    .tool-btn {
      background: rgba(255, 255, 255, 0.03);
      border: 1px solid var(--border);
      color: var(--text-muted);
      border-radius: 10px;
      padding: 12px 8px;
      font-size: 0.85rem;
      font-weight: 600;
      cursor: pointer;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 6px;
      transition: all 0.2s ease;
      min-height: 44px;
    }

    .tool-btn:hover {
      background: rgba(255, 255, 255, 0.06);
      color: var(--text-main);
    }

    .tool-btn.active {
      background: var(--accent-glow);
      border-color: var(--accent);
      color: #fff;
    }

    .dropzone {
      border: 2px dashed rgba(255, 255, 255, 0.15);
      border-radius: 14px;
      padding: 36px 20px;
      text-align: center;
      cursor: pointer;
      transition: all 0.2s ease;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 10px;
      background: rgba(0, 0, 0, 0.2);
    }

    .dropzone:hover, .dropzone.dragover {
      border-color: var(--accent);
      background: rgba(56, 189, 248, 0.04);
    }

    .file-input { display: none; }

    .file-list {
      margin-top: 14px;
      display: flex;
      flex-direction: column;
      gap: 8px;
      width: 100%;
    }

    .file-item {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 10px 14px;
      background: rgba(255, 255, 255, 0.04);
      border-radius: 8px;
      font-size: 0.85rem;
    }

    .controls-row {
      display: flex;
      gap: 12px;
      margin-top: 20px;
      flex-wrap: wrap;
    }

    .input-field {
      flex: 1;
      min-width: 160px;
      background: rgba(0, 0, 0, 0.3);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 10px 14px;
      color: var(--text-main);
      font-size: 0.85rem;
      outline: none;
    }

    .input-field:focus {
      border-color: var(--accent);
    }

    .btn-action {
      background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
      color: #fff;
      border: none;
      border-radius: 8px;
      padding: 12px 24px;
      font-size: 0.9rem;
      font-weight: 600;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
      transition: opacity 0.2s ease;
      min-height: 44px;
    }

    .btn-action:hover:not(:disabled) {
      opacity: 0.9;
    }

    .btn-action:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }

    .status-box {
      margin-top: 16px;
      padding: 12px 16px;
      border-radius: 8px;
      font-size: 0.85rem;
      display: none;
      word-break: break-all;
    }

    .status-box.success {
      background: rgba(52, 211, 153, 0.1);
      border: 1px solid rgba(52, 211, 153, 0.3);
      color: var(--success);
      display: block;
    }

    .status-box.error {
      background: rgba(248, 113, 113, 0.1);
      border: 1px solid rgba(248, 113, 113, 0.3);
      color: var(--danger);
      display: block;
    }

    .text-output {
      margin-top: 16px;
      background: rgba(0, 0, 0, 0.4);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 14px;
      font-family: monospace;
      font-size: 0.85rem;
      max-height: 240px;
      overflow-y: auto;
      white-space: pre-wrap;
      display: none;
    }

    footer {
      text-align: center;
      font-size: 0.75rem;
      color: var(--text-muted);
      margin-top: 16px;
    }
  </style>
</head>
<body>
  <div class="container">
    <header>
      <div class="badge-bar">
        <span class="badge badge-airgap">🔒 100% Air-Gapped</span>
        <span class="badge badge-standalone">📄 Zero-Install Portable</span>
      </div>
      <h1>PaperPilot Portable</h1>
      <p class="subtitle">Complete client-side PDF tool suite. Embedded WebAssembly engine executes directly in your browser. Zero network connections, zero server uploads.</p>
    </header>

    <main class="card">
      <div class="tool-selector" id="toolSelector">
        <button class="tool-btn active" data-tool="merge">📑 Merge</button>
        <button class="tool-btn" data-tool="split">✂️ Split</button>
        <button class="tool-btn" data-tool="rotate">🔄 Rotate</button>
        <button class="tool-btn" data-tool="delete">🗑️ Delete</button>
        <button class="tool-btn" data-tool="compress">📦 Compress</button>
        <button class="tool-btn" data-tool="extract_text">📝 Extract Text</button>
      </div>

      <div class="dropzone" id="dropzone">
        <svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" style="color: var(--accent);"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="12" y1="18" x2="12" y2="12"></line><line x1="9" y1="15" x2="15" y2="15"></line></svg>
        <div style="font-weight: 600;">Drag & drop PDF files here, or click to browse</div>
        <div style="font-size: 0.8rem; color: var(--text-muted);">Processes in browser RAM with local WebAssembly</div>
        <input type="file" id="fileInput" class="file-input" accept=".pdf" multiple />
      </div>

      <div class="file-list" id="fileList"></div>

      <div class="controls-row" id="controlsRow">
        <input type="text" id="toolParamInput" class="input-field" placeholder="Tool configuration..." style="display: none;" />
        <button id="executeBtn" class="btn-action" disabled>Execute Operation</button>
      </div>

      <div id="statusBox" class="status-box"></div>
      <div id="textOutput" class="text-output"></div>
    </main>

    <footer>
      PaperPilot Portable · Pure Rust & WebAssembly · Apache 2.0 License · Files never leave this browser tab
    </footer>
  </div>

  <script>
    // -------------------------------------------------------------------------
    // 1. Embedded Base64 WebAssembly Binary
    // -------------------------------------------------------------------------
    const INLINE_WASM_BASE64 = "${wasmBase64}";

    function decodeBase64ToUint8Array(base64) {
      const binaryString = atob(base64);
      const len = binaryString.length;
      const bytes = new Uint8Array(len);
      for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
      }
      return bytes;
    }

    // -------------------------------------------------------------------------
    // 2. WebAssembly Glue Code
    // -------------------------------------------------------------------------
    ${wasmJsCode}

    // Initialize the engine from the inline byte array
    let wasmEngineReady = false;

    async function initInlineWasm() {
      try {
        const wasmBytes = decodeBase64ToUint8Array(INLINE_WASM_BASE64);
        await __wbg_init({ module_or_path: wasmBytes });
        wasmEngineReady = true;
        console.log("✅ PaperPilot Portable WASM engine initialized in-memory!");
        updateExecuteBtnState();
      } catch (err) {
        console.error("❌ Failed to initialize in-memory WASM:", err);
        showStatus("Failed to initialize WASM engine: " + err.message, "error");
      }
    }

    // -------------------------------------------------------------------------
    // 3. UI State & Drag-and-Drop Controller
    // -------------------------------------------------------------------------
    let currentTool = "merge";
    let loadedFiles = [];

    const toolSelector = document.getElementById("toolSelector");
    const dropzone = document.getElementById("dropzone");
    const fileInput = document.getElementById("fileInput");
    const fileList = document.getElementById("fileList");
    const toolParamInput = document.getElementById("toolParamInput");
    const executeBtn = document.getElementById("executeBtn");
    const statusBox = document.getElementById("statusBox");
    const textOutput = document.getElementById("textOutput");

    const TOOL_CONFIGS = {
      merge: { name: "Merge", paramPlaceholder: "", paramLabel: "", multiple: true },
      split: { name: "Split", paramPlaceholder: 'Page ranges, e.g. "1-2, 3-4"', paramLabel: "Ranges", multiple: false, defaultVal: "1" },
      rotate: { name: "Rotate", paramPlaceholder: 'Rotation angle: 90, 180, or 270', paramLabel: "Angle", multiple: false, defaultVal: "90" },
      delete: { name: "Delete", paramPlaceholder: 'Pages to remove, e.g. "1, 3-5"', paramLabel: "Pages", multiple: false, defaultVal: "1" },
      compress: { name: "Compress", paramPlaceholder: 'Quality: "recommended", "low", or "high"', paramLabel: "Quality", multiple: false, defaultVal: "recommended" },
      extract_text: { name: "Extract Text", paramPlaceholder: "", paramLabel: "", multiple: false }
    };

    toolSelector.addEventListener("click", (e) => {
      const btn = e.target.closest(".tool-btn");
      if (!btn) return;
      document.querySelectorAll(".tool-btn").forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      currentTool = btn.dataset.tool;
      configureToolInputs();
    });

    function configureToolInputs() {
      const config = TOOL_CONFIGS[currentTool];
      fileInput.multiple = config.multiple;
      if (config.paramPlaceholder) {
        toolParamInput.style.display = "block";
        toolParamInput.placeholder = config.paramPlaceholder;
        toolParamInput.value = config.defaultVal || "";
      } else {
        toolParamInput.style.display = "none";
        toolParamInput.value = "";
      }
      textOutput.style.display = "none";
      updateExecuteBtnState();
    }

    dropzone.addEventListener("click", () => fileInput.click());
    dropzone.addEventListener("dragover", (e) => { e.preventDefault(); dropzone.classList.add("dragover"); });
    dropzone.addEventListener("dragleave", () => dropzone.classList.remove("dragover"));
    dropzone.addEventListener("drop", async (e) => {
      e.preventDefault();
      dropzone.classList.remove("dragover");
      if (e.dataTransfer.files) handleFiles(Array.from(e.dataTransfer.files));
    });

    fileInput.addEventListener("change", (e) => {
      if (e.target.files) handleFiles(Array.from(e.target.files));
    });

    async function handleFiles(files) {
      const pdfs = files.filter(f => f.name.toLowerCase().endsWith(".pdf"));
      if (!pdfs.length) {
        showStatus("Please select valid .pdf files", "error");
        return;
      }
      if (!TOOL_CONFIGS[currentTool].multiple) {
        loadedFiles = [pdfs[0]];
      } else {
        loadedFiles = loadedFiles.concat(pdfs);
      }
      renderFileList();
      updateExecuteBtnState();
    }

    function renderFileList() {
      fileList.innerHTML = "";
      loadedFiles.forEach((file, idx) => {
        const item = document.createElement("div");
        item.className = "file-item";
        item.innerHTML = '<span>📄 ' + file.name + ' (' + (file.size / 1024).toFixed(1) + ' KB)</span><span style="cursor:pointer; color:var(--danger);" onclick="removeFile(' + idx + ')">✕</span>';
        fileList.appendChild(item);
      });
    }

    window.removeFile = function(idx) {
      loadedFiles.splice(idx, 1);
      renderFileList();
      updateExecuteBtnState();
    };

    function updateExecuteBtnState() {
      const hasFiles = loadedFiles.length > 0;
      executeBtn.disabled = !wasmEngineReady || !hasFiles;
      executeBtn.textContent = !wasmEngineReady ? "Loading Engine..." : ("Run " + TOOL_CONFIGS[currentTool].name);
    }

    function showStatus(msg, type) {
      statusBox.textContent = msg;
      statusBox.className = "status-box " + type;
    }

    // -------------------------------------------------------------------------
    // 4. Client-Side WASM Execution Dispatcher
    // -------------------------------------------------------------------------
    executeBtn.addEventListener("click", async () => {
      if (!wasmEngineReady || !loadedFiles.length) return;
      showStatus("Processing in browser memory with WebAssembly...", "success");
      textOutput.style.display = "none";

      try {
        if (currentTool === "merge") {
          const buffers = [];
          for (const file of loadedFiles) {
            buffers.push(new Uint8Array(await file.arrayBuffer()));
          }
          const resultBytes = WasmPdfEngine.merge(buffers);
          downloadResult(resultBytes, "paperpilot_merged.pdf");
          showStatus("✅ Successfully merged " + buffers.length + " PDFs into a single file!", "success");
        } else {
          const firstBytes = new Uint8Array(await loadedFiles[0].arrayBuffer());
          const paramVal = toolParamInput.value.trim();

          if (currentTool === "split") {
            const results = WasmPdfEngine.split(firstBytes, paramVal || "1");
            if (results && results.length) {
              results.forEach((chunk, i) => {
                downloadResult(chunk, "split_part_" + (i + 1) + ".pdf");
              });
              showStatus("✅ Successfully split PDF into " + results.length + " document(s)!", "success");
            }
          } else if (currentTool === "rotate") {
            const angle = parseInt(paramVal, 10) || 90;
            const resultBytes = WasmPdfEngine.rotate(firstBytes, angle, "all");
            downloadResult(resultBytes, "rotated.pdf");
            showStatus("✅ Successfully rotated PDF by " + angle + "°!", "success");
          } else if (currentTool === "delete") {
            const resultBytes = WasmPdfEngine.delete_pages(firstBytes, paramVal || "1");
            downloadResult(resultBytes, "pages_deleted.pdf");
            showStatus("✅ Successfully removed pages!", "success");
          } else if (currentTool === "compress") {
            const resultBytes = WasmPdfEngine.compress(firstBytes, paramVal || "recommended");
            downloadResult(resultBytes, "compressed.pdf");
            showStatus("✅ Successfully compressed PDF!", "success");
          } else if (currentTool === "extract_text") {
            const text = WasmPdfEngine.extract_text(firstBytes);
            textOutput.textContent = text || "(No text content detected)";
            textOutput.style.display = "block";
            showStatus("✅ Extracted text successfully!", "success");
          }
        }
      } catch (err) {
        console.error("Execution error:", err);
        showStatus("Error executing operation: " + err.message, "error");
      }
    });

    function downloadResult(bytes, filename) {
      const blob = new Blob([bytes], { type: "application/pdf" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      setTimeout(() => URL.revokeObjectURL(url), 2000);
    }

    // Kick off initialization
    configureToolInputs();
    initInlineWasm();
  </script>
</body>
</html>
`;

// 5. Write the self-contained HTML file
console.log(`💾 Writing self-contained file: ${OUTPUT_FILE}...`);
fs.writeFileSync(OUTPUT_FILE, htmlContent, 'utf8');

const stats = fs.statSync(OUTPUT_FILE);
console.log(`🎉 Done! Created ${OUTPUT_FILE} (${(stats.size / (1024 * 1024)).toFixed(2)} MB).`);
console.log('🔒 Fully self-contained, air-gapped, zero external network requests.');
