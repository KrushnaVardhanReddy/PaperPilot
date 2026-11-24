# PaperPilot Video Studio 🎥

Automated video generation pipeline for PaperPilot YouTube feature showcases and social media videos using [Revideo](https://revideo.dev) (MIT license), `edge-tts`, and headless Puppeteer / Chrome.

Scales across **all 26 weekly showcase tools** with a 5-part 16:9 Full HD (1920x1080 @ 30fps) architecture, complete multi-interface demonstrations, and synchronized captions.

---

## 📁 Architecture & Directory Structure

```
tools/video-studio/
├── assets/
│   ├── weekXX_captions.json    # Word-level & phrase-level timestamps from edge-tts
│   └── weekXX_voiceover.mp3    # Neural voiceover audio (en-US-ChristopherNeural)
├── out/
│   └── weekXX_<slug>.mp4       # Rendered 1920x1080 16:9 Landscape Full HD MP4 video
├── scripts/
│   ├── generate_audio.py       # Batch voiceover & subtitle timing synthesizer (Weeks 1-26)
│   ├── render_all.ts           # Batch video renderer CLI (Supports single, ranges, or all weeks)
│   └── render.ts               # Standalone runner for Week 01
├── src/
│   ├── configs/
│   │   └── tools-data.ts       # Typed configurations, metadata, CLI commands, MCP tools & REST APIs
│   ├── configs_tools.json      # Structured tool definitions parsed from showcase posts
│   ├── project.ts              # Revideo project root with dynamic week selection (TARGET_WEEK)
│   └── scenes/
│       ├── captions_weekXX.ts  # Bundled TypeScript subtitle transcripts for each week
│       ├── universal-scene.tsx # Scalable, parameterized 5-part storyboard scene template
│       └── week01-scene.tsx    # Canonical Week 01 scene
├── package.json
└── tsconfig.json
```

---

## 🎯 The 5-Part Storyboard Structure

Every tool video follows the standardized narrative architecture:
1. **Introduction & Core Architecture**: PaperPilot definition (Pure Rust, local-first document intelligence toolkit, 100 tools, zero cloud subscriptions, zero data egress).
2. **Problem & High-Speed Engine Solution**: Tailored to the specific week's topic (e.g. Broken workflows, privacy leaks, monthly subscriptions vs. Rust native engine).
3. **Featured Tool Announcement**: Tool number, operational parameters, and core capabilities.
4. **All 5 Interfaces (Input -> Output Demonstration)**:
   - 🖥️ **Desktop GUI**: Visual dock, drag-and-drop, interactive sliders, preview, and receipt.
   - 🤖 **AI Agent / MCP**: User prompt -> deterministic MCP tool call -> output receipt.
   - 💻 **Terminal CLI**: `paperpilot <cmd>` -> ANSI formatted terminal receipt.
   - 🌐 **Local REST API**: `POST localhost:7823/v1/...` -> 200 OK JSON response.
   - ⚡ **Web / WASM**: In-browser execution at `usepaperpilot.com` with 0 bytes server upload.
5. **Outro & 5-Second Flash Install**: GitHub star CTA (`github.com/KrushnaVardhanReddy/PaperPilot`), website (`usepaperpilot.com`), and 5-second flash install command.

---

## 🚀 Usage Guide

### 1. Generating Audio & Captions

Generate high-quality voiceover audio and synchronized subtitle data using `edge-tts`:

```bash
# Generate for all 26 weeks
pnpm audio:all

# Generate for a specific week (e.g. Week 2)
python3 scripts/generate_audio.py --week 2

# Generate for a range of weeks (e.g. Weeks 2 through 10)
python3 scripts/generate_audio.py --week 2..10
```

### 2. Rendering Videos (Headless MP4)

Render Full HD 16:9 MP4 videos into `out/`:

```bash
# Render all 26 weeks
pnpm render:all

# Render a single week (e.g. Week 2)
pnpm render 2

# Render a range of weeks (e.g. Weeks 2 through 5)
pnpm render 2..5
```
