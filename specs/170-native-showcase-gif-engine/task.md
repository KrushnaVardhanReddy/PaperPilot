# Spec 170: Implementation Tasks

- [x] **0. Penta-Interface Tool Scaffolding**
  - [x] 0.1 Scaffold `pdf_to_gif` across all 5 interfaces.
  - [x] 0.2 Scaffold `showcase_gif` across all 5 interfaces.
  - [x] 0.3 Scaffold `images_to_gif` across all 5 interfaces.
  - [x] 0.4 Scaffold `img_optimize_for_llm` across all 5 interfaces.
  - [x] 0.5 Scaffold `img_deskew` across all 5 interfaces.
  - [x] 0.6 Scaffold `img_convert` across all 5 interfaces.
  - [x] 0.7 Scaffold `img_crop_to_content` across all 5 interfaces.
  - [x] 0.8 Scaffold `img_redact` across all 5 interfaces.

- [ ] **1. GIF & Document Animation Engine**
  - [ ] 1.1 Implement `pdf_to_gif_bytes`: rasterize PDF pages via existing render engine and encode into looping animated GIF with frame delay.
  - [ ] 1.2 Implement `images_to_gif_bytes`: load image byte sequences and encode into single looping animated GIF with NeuQuant palette.
  - [ ] 1.3 Implement `showcase_gif_bytes`: split-screen canvas drawing (simulated terminal keystrokes + document before/after transition) rendered to palette-quantized GIF.

- [ ] **2. Image Intelligence & LLM Vision Optimization**
  - [ ] 2.1 Implement `img_optimize_for_llm_bytes`: smart downscaling with aspect-ratio preservation to meet LLM vision token constraints.
  - [ ] 2.2 Implement `img_deskew_bytes`: projection profile / Hough angle detection and rotation.
  - [ ] 2.3 Implement `img_convert_bytes`: format transmutation between PNG, JPEG, WebP, TIFF, AVIF.
  - [ ] 2.4 Implement `img_crop_to_content_bytes`: edge and contrast detection to auto-crop margins.
  - [ ] 2.5 Implement `img_redact_bytes`: permanent RGB pixel incinerator over bounding boxes.

- [ ] **3. Verification & CI/CD**
  - [ ] 3.1 Run tests with `cargo test -j 4` across all 7 new tools.
  - [ ] 3.2 Verify MCP tool schemas in `paperpilot-mcp` with `sync_tool_schemas.py`.
  - [ ] 3.3 Update CLI docs and documentation.

