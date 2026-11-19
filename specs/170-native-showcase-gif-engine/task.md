# Spec 170: Implementation Tasks

- [ ] **1. Core Showcase & Canvas Engine (`paperpilot-core`)**
  - [ ] 1.1 Verify and configure `image` and `gif` crate dependencies with palette quantization support.
  - [ ] 1.2 Implement embedded font loading with fallback (`ab_glyph` or bitmap font).
  - [ ] 1.3 Build `crates/paperpilot-core/src/showcase/canvas.rs` for 1200x675 16:9 split-screen UI layout (terminal + document preview).
  - [ ] 1.4 Build `crates/paperpilot-core/src/showcase/timeline.rs` for state interpolation (typing, spinner, document transitions).
  - [ ] 1.5 Build `crates/paperpilot-core/src/showcase/encoder.rs` for in-memory NeuQuant/Bayer palette GIF generation.

- [ ] **2. Document-to-GIF Engine (`pdf-to-gif`)**
  - [ ] 2.1 Implement `pdf_to_gif` pipeline converting rasterized PDF pages into looping animated GIFs.
  - [ ] 2.2 Add configurable frame delay and loop controls.

- [ ] **3. CLI Integration (`paperpilot-cli`)**
  - [ ] 3.1 Expose `paperpilot showcase-gif` subcommand with `--command`, `--before`, `--after`, `--title`, `--output`.
  - [ ] 3.2 Expose `paperpilot pdf-to-gif` subcommand with `--input`, `--output`, `--delay`.
  - [ ] 3.3 Add unit and integration tests verifying sub-100KB GIF generation on test documents.

- [ ] **4. MCP Tool Registration (`paperpilot-mcp`)**
  - [ ] 4.1 Expose `generate_showcase_gif` in `crates/paperpilot-mcp/src/server.rs`.
  - [ ] 4.2 Document input schema parameters with `When to use:` and disambiguation blocks matching Glama Grade A standards.
  - [ ] 4.3 Update SLM training snapshots (`mcp_tool_schemas.json`).

- [ ] **5. Verification & Documentation**
  - [ ] 5.1 Test generated GIF in native image viewer / web browser to verify fluid 12–15 FPS animation.
  - [ ] 5.2 Verify Cargo build runs clean under `-j 4`.
  - [ ] 5.3 Update `README.md` and CLI `--help` docs.
