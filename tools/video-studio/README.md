# PaperPilot Video Studio 🎥

Automated video generation pipeline for PaperPilot YouTube Shorts & feature videos using [Revideo](https://revideo.dev) (MIT license), `edge-tts`, and headless Puppeteer / Chrome.

---

## 📁 Architecture & Directory Structure

```
tools/video-studio/
├── assets/
│   ├── frames/                 # Multi-interface UI snapshots & benchmark diagrams
│   ├── screen_recording.mp4    # 30fps screen video capture of tool action
│   ├── week01_captions.json    # Word-level & phrase-level timestamps from edge-tts
│   └── week01_voiceover.mp3    # Neural voiceover audio (en-US-ChristopherNeural)
├── out/
│   └── week01_compression.mp4  # Rendered 1080x1920 9:16 vertical MP4 video
├── scripts/
│   ├── generate_audio.py       # Voiceover synthesizer & word timestamp extractor
│   └── render.ts               # Headless Revideo render orchestrator
├── src/
│   ├── project.ts              # Revideo project definition (1080x1920, 30fps)
│   └── scenes/
│       ├── captions.ts         # TypeScript subtitle transcript data
│       └── week01-scene.tsx    # Scene animation, badges, video viewport, captions, outro
├── package.json
└── tsconfig.json
```

---

## 🚀 Quickstart

### 1. Generate Voiceover & Captions
Runs `edge-tts` to generate neural audio and extract word-level subtitle timings:
```bash
pnpm audio:week01
# or
python3 scripts/generate_audio.py assets/week01_voiceover.mp3 assets/week01_captions.json
```

### 2. Render Video (Headless MP4)
Renders the complete short at 1080x1920 30fps into `out/week01_compression.mp4`:
```bash
pnpm render:week01
# or
npx tsx scripts/render.ts
```

---

## 🎬 Prototype Tool #1: Instant PDF Compression

- **Resolution:** 1080x1920 (9:16 Vertical HD for YouTube Shorts / Reels / TikTok)
- **Duration:** 40.07 seconds
- **Features Included:**
  1. **Intro Hook Badge:** "WEEK 1 • MULTI-INTERFACE SHOWCASE — Instant PDF Compression"
  2. **Screen Recording:** Smooth playback demonstrating Desktop GUI, MCP Agent, CLI, and REST API
  3. **Live Animated Subtitles:** Word/sentence captions dynamically synchronized with the voiceover
  4. **Metrics Card:** Throughput (237 docs/sec), Storage Reclaimed (-74.4% / -48.2 GB), $0 API Fees
  5. **Outro CTA Card:** 100% offline & open source with 5-second flash install command.
