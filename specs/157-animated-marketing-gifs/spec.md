# Spec 157: Automated Multi-Interface Marketing GIF Generation Pipeline

## 1. Overview & Business Value
PaperPilot features five primary interfaces: **CLI**, **REST API / Gateway**, **MCP (Model Context Protocol / AI Agents)**, **Web Embed / WASM**, and **Desktop UI (Tauri)**.
To demonstrate its performance, 100% offline security, and versatility on GitHub README, marketing landing pages, and documentation portals, we need high-resolution, animated GIF assets showing realistic developer and user interactions.

## 2. Requirements & GIF Scenarios
A minimum of 5 distinct, high-impact animated GIFs must be generated into `docs/assets/gifs/`:

1. **`demo_01_ask_natural_language.gif` (CLI Natural Language Intent Engine)**:
   - Command: `paperpilot ask "rotate 90 degrees and extract pages 1 to 3" invoice.pdf -o processed.pdf -y`
   - Visual: Terminal typing animation → NLP resolution badge (`offline local intent`) → instantaneous sub-30ms execution → green checkmark receipt.
2. **`demo_02_pii_security_scan.gif` (Zero-Leak Enterprise Security Scan)**:
   - Command: `paperpilot pii-scan --input patient_records.pdf`
   - Visual: Terminal typing → progress animation → formatted JSON report highlighting SSN & Email detections with execution time `28ms` and `100% Offline` badge.
3. **`demo_03_high_throughput_merge.gif` (Core PDF Operations & Speed)**:
   - Command: `paperpilot merge --input doc_01.pdf doc_02.pdf doc_03.pdf --output consolidated.pdf`
   - Visual: Multi-file concatenation with bookmark retention notice and memory footprint display.
4. **`demo_04_rest_api_gateway.gif` (Developer REST API Interface)**:
   - Visual: Two-pane split:
     - Top/Left: `paperpilot serve --port 7823` daemon log showing routes loaded.
     - Bottom/Right: `curl -X POST http://127.0.0.1:7823/api/v1/pdf/rotate ...` executing and receiving `200 OK` JSON response.
5. **`demo_05_mcp_agent_workflow.gif` (AI Agent Integration via MCP JSON-RPC)**:
   - Visual: AI assistant invoking `pdf_rotate` or `batch_fill` tool via JSON-RPC 2.0 protocol over stdio with instant payload response.
6. **`demo_06_pipeline_conditional_workflow.gif` (Declarative Pipeline & Workflow Engine)**:
   - Command: `paperpilot run examples/workflows/conditional-invoice.ppflow invoice.pdf -o processed.pdf`
   - Visual: Multi-stage pipeline execution view with step-by-step progress indicators:
     - `[1/3] OCR (if_scanned = true)` -> Skipped (native text detected)
     - `[2/3] Compress (if_page_count_gt = 50)` -> Executed (level: aggressive)
     - `[3/3] Watermark` -> Applied "PROCESSED" (opacity: 0.3)
     - Completed in 36ms with execution receipt.
7. **Bonus / `demo_07_batch_acroform_fill.gif` (Enterprise High-Volume Batch Automation)**:
   - Command: `paperpilot fill-batch --template W9.pdf --data vendors.csv --output-dir ./out`
   - Visual: Batch counter ticking `100/100 records generated`, execution receipt verification.

## 3. Technical Architecture: HTML-to-Frames-to-GIF Pipeline
Rather than relying on brittle live OS screen recording, we utilize a deterministic, pixel-perfect **HTML/CSS/JS + Playwright + FFmpeg** generator:
- **HTML Templates (`scripts/gif_generators/templates/`)**:
  - macOS dark terminal chrome (window buttons `🔴 🟡 🟢`, shadow, subtle blur, monospace JetBrains Mono or Fira Code font).
  - Keyframe / timeline state machine in JS that types characters, shows spinners, outputs syntax-highlighted blocks.
- **Frame Grabber (`scripts/gif_generators/capture.js`)**:
  - Uses Playwright or Headless Chromium to load the HTML template at fixed dimensions (e.g., 900x520 @ 2x DPI).
  - Steps through time or captures frame screenshots at 20-30 FPS, or records `.webm` video.
- **GIF Optimization (`ffmpeg`)**:
  - Generates two-pass color palette via `palettegen` filter.
  - Applies `paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle` to produce crisp, small-footprint (under 1.5MB) GIFs without color banding.

## 4. Verification & Output Location
- Output files must be saved under `docs/assets/gifs/`.
- Must include a verification harness in `Makefile` (`make generate-gifs`).
- Generated GIFs must be embedded into `README.md` or `docs/` showcase section.
