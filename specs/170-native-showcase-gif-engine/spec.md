# Spec 170: Native Image Intelligence & Showcase GIF Engine

## Status
In Progress

## Context & Motivation
PaperPilot provides a comprehensive local-first document processing suite. However, autonomous AI agents (Claude Desktop, Cursor, LangChain, n8n) and human developers constantly face two major bottlenecks:

1. **Vision & Image Friction**: High-resolution 4K phone scans blow LLM vision token budgets ($0.05/image), phone-captured receipts are skewed/tilted, and documents contain sensitive pixels (faces, signatures, card numbers) with no air-gapped way to redact them before passing to vision models.
2. **Visual Proof & Demonstration**: There is no built-in, zero-dependency mechanism to produce animated before-and-after demo GIFs showing CLI transformations, or turning multi-page PDFs into looping preview GIFs without installing heavy external runtimes (`ffmpeg`, Python, or web screen recorders).

By implementing a **pure Rust, zero-dependency Image Intelligence & Animated Showcase Engine**, PaperPilot becomes a unified multi-modal document & image copilot.

---

## Complete Scope: 7 New Native Tools

### Category A: Animated GIF & Showcase Engine
1. **`pdf_to_gif`**: Renders multi-page PDF documents into an animated, looping GIF preview (with configurable delay/FPS).
2. **`images_to_gif`**: Stitches a directory or sequence of images (`page_01.png`, `page_02.png`) into a single looping animated GIF.
3. **`showcase_gif`**: Programmatically generates a 1200x675 split-screen developer demo GIF (simulated terminal with typing/spinner on the left, before/after document transformation on the right) using in-memory NeuQuant/Bayer palette quantization.

### Category B: Image Intelligence & Vision LLM Optimization
4. **`img_optimize_for_llm`**: Resamples and downscales 4K/8K images to optimal vision token budgets (e.g. max 1024x1024 or 768px), slashing LLM API costs by up to 75%.
5. **`img_deskew`**: Detects text baseline tilt in scanned documents and receipts, automatically straightening the image for optimal OCR.
6. **`img_convert`**: High-speed, local conversion between PNG, JPEG, WebP, TIFF, and AVIF formats.
7. **`img_crop_to_content`**: Detects document boundaries to automatically crop away messy background margins (desks, shadows, carpets).
8. **`img_redact`**: Applies permanent, irreversible pixel-level blackouts over sensitive bounding boxes (signatures, PII, account numbers).

---

## Architectural Guarantees
- **Pure Rust Native**: Leverages existing crates (`image`, `gif`, `imageproc`) with zero external C/Python/ffmpeg dependencies.
- **Penta-Interface Uniformity**: Every tool is exposed across:
  1. Rust Core Engine (`paperpilot-pdf`)
  2. CLI (`paperpilot-cli`)
  3. Model Context Protocol (`paperpilot-mcp`)
  4. Embedded REST Gateway (`paperpilot-gateway`)
  5. WebAssembly & Desktop UI (`paperpilot-wasm` & `apps/desktop`)
- **Strict Concurrency**: Enforce `-j 4` during compilation.

