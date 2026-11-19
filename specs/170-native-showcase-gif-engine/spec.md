# Spec 170: Native Showcase GIF & Document Animation Engine

## Status
Draft / In Progress

## Context & Motivation
PaperPilot is a local-first, high-performance document engine providing 92 deterministic tools. While users and autonomous AI agents (Claude Desktop, Cursor, n8n) can transform, redact, stamp, and convert documents in sub-15ms latency, there is currently no built-in visual mechanism to generate animated proof or demo media showing what happened.

Developers, compliance auditors, and AI agents typically must rely on external heavyweight tooling (Python Pillow, `ffmpeg`, `vhs`, or web screen recorders) to record terminal commands and document changes.

By embedding a **pure Rust, zero-dependency Animated Showcase & GIF Engine** directly into PaperPilot:
1. **Developers & Marketers** can generate publication-ready, palette-optimized 16:9 terminal & document demo GIFs for READMEs, documentation, and social feeds with a single CLI command (`paperpilot showcase-gif`).
2. **AI Agents (MCP)** can invoke `generate_showcase_gif` to return an animated visual receipt of the document transformation to the user.
3. **Legal & Compliance Teams** can produce an automated visual diff GIF comparing before-and-after document states (redactions, signatures, watermarks, Bates stamps).

All operations must be 100% Rust-native, using existing dependency chains (`image`, `gif`, `ab_glyph`), with zero external runtimes (no Python, no `ffmpeg`).

---

## Architectural Design

### 1. Core Module (`crates/paperpilot-core/src/showcase/`)
- **`canvas.rs`**: Implements programmatic vector frame drawing (1200x675 default 16:9 layout):
  - Dark mode developer theme (macOS-style window chrome: red/yellow/green control dots, titlebar, borders).
  - Left pane: Terminal pane with monospace syntax highlighting (command, flags, parameters, progress spinner, completion badges).
  - Right pane: Document preview pane rendering before/after PDF page rasters.
- **`timeline.rs`**: State-driven frame sequence interpolator:
  - Phase 1: Keystroke typing animation with blinking cursor.
  - Phase 2: Processing spinner (`[⠋] -> [⠙] -> [✓] Execution: 12ms`).
  - Phase 3: Document state transition (e.g. animated redaction overlay, stamp drop, page flip).
  - Phase 4: Output verification and audit badge hold.
- **`encoder.rs`**: High-performance GIF encoder utilizing the Rust `gif` crate:
  - NeuQuant / Octree 256-color palette quantization per frame or global.
  - Bayer / Floyd-Steinberg dithering for clean text rendering without banding.
  - Sub-100KB output compression with configurable FPS and looping delay.

### 2. CLI Interface (`crates/paperpilot-cli/src/`)
Expose new subcommand:
```bash
paperpilot showcase-gif \
  --command "paperpilot redact --input invoice.pdf --pattern PII --output clean.pdf" \
  --before invoice.pdf \
  --after clean.pdf \
  --title "Irreversible PII Redaction" \
  --output showcase.gif
```

Or standalone document animation:
```bash
paperpilot pdf-to-gif \
  --input presentation.pdf \
  --fps 1 \
  --delay 150 \
  --output presentation.gif
```

### 3. MCP Server Tool Registration (`crates/paperpilot-mcp/src/`)
Register tool:
- `generate_showcase_gif`:
  - `command`: String (optional, displayed in simulated terminal)
  - `before_path`: String (path to original document)
  - `after_path`: String (path to transformed document)
  - `title`: String
  - `output_path`: String

---

## Technical Constraints & Guardrails
- **Zero External Dependencies**: Must not shell out to `ffmpeg`, Python, or ImageMagick.
- **Strict Concurrency Cap**: Cargo builds capped at `-j 4`.
- **Lightweight Assets**: Target output GIF size must be strictly < 500 KB (and optimized < 150 KB for typical 5-second loops).
- **Graceful Font Fallback**: Embed a lightweight monospace bitmap or BDF/TTF font or fall back safely if system fonts are unavailable.
