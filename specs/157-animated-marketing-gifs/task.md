# Tasks for Spec 157: Automated Marketing GIF Generation Pipeline

## Phase 1: Real CLI & API Command Validation
- [ ] Task 1.1: Verify real CLI arguments for all showcase commands against `target/release/paperpilot` using `tests/e2e_fixtures/real/`:
  - `paperpilot ask`
  - `paperpilot pii-scan`
  - `paperpilot merge`
  - `paperpilot fill-batch`
  - `paperpilot serve` + REST API `curl`
- [ ] Task 1.2: Record real stdout, stderr, execution times, and exit statuses so mocked animations faithfully reflect reality.

## Phase 2: HTML/CSS Terminal Animation Engine
- [ ] Task 2.1: Create `scripts/gif_generators/templates/terminal_template.html`:
  - Modern dark mode terminal styling (glassmorphism border, title bar, buttons, syntax-highlighted tokens).
  - Configurable timeline script that renders commands character-by-character with realistic typing delay.
  - Success spinners, execution badges (`100% Offline`, `28ms`, `Zero-Trace`).
- [ ] Task 2.2: Create scenario configurations for the 5-6 target scenarios in `scripts/gif_generators/scenarios.json`.

## Phase 3: Headless Capture & FFmpeg Transcoding Pipeline
- [ ] Task 3.1: Create `scripts/gif_generators/generate.js` using Playwright or Chromium recording:
  - Launches headless browser with `--no-sandbox`.
  - Captures video or frame sequence at 20-30 fps.
- [ ] Task 3.2: Implement two-pass FFmpeg palette generation:
  - `ffmpeg -i video.webm -vf "fps=20,scale=840:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128[p];[s1][p]paletteuse=dither=bayer:bayer_scale=5" output.gif`
  - Ensure each GIF is optimized under 1.5MB.

## Phase 4: Makefile Integration & Documentation Embedding
- [ ] Task 4.1: Add `make generate-gifs` target to `Makefile`.
- [ ] Task 4.2: Verify all 5+ GIFs are created in `docs/assets/gifs/`.
- [ ] Task 4.3: Reference the generated GIFs in `README.md` and `apps/docs/src/content/docs/index.mdx`.

