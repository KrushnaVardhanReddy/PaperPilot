# Spec 018: Cloudflare Workers Edge Microservice (`apps/edge`)

## Status: APPROVED & ACTIVE (Phase 4.8.3)

## 1. Context & Motivation
PaperPilot's core engine compiles to WebAssembly (`wasm32-unknown-unknown`). To support automated server-side webhooks, API integrations, and developer workflows without maintaining expensive compute servers, we deploy `paperpilot-wasm` into **Cloudflare Workers V8 Isolates**.

This unlocks:
1. **Sub-10ms Global Latency**: Zero cold start (0ms), deployed across 300+ edge PoPs worldwide.
2. **Physical Zero-Disk Guarantee**: Operations execute strictly in memory buffers without persisting bytes to disk.
3. **Multi-Platform Consistency**: The exact same pure-Rust algorithms powering Desktop and Browser run at the edge.

---

## 2. Architecture & File Scope

```
PaperPilot/
├── apps/
│   └── edge/
│       ├── package.json               # Cloudflare worker dependencies & scripts
│       ├── wrangler.jsonc (or .toml)  # Wrangler edge deployment config
│       ├── tsconfig.json              # TypeScript worker config
│       ├── src/
│       │   └── index.ts               # REST API endpoints routing to paperpilot-wasm
│       └── tests/
│           └── edge.spec.ts           # Vitest / Miniflare worker test suite
├── wiki/10-Cloudflare-Edge-Service.md # Architecture & deployment guide
└── reports/EDGE_MICROSERVICE_REPORT.md# Latency & endpoint test verification
```

### Strict Non-Overlapping File Ownership:
- `apps/edge/*`
- `wiki/10-Cloudflare-Edge-Service.md`
- `reports/EDGE_MICROSERVICE_REPORT.md`

*(Touches ZERO files in `paperpilot-pdf/`, `paperpilot-nlp/`, `apps/desktop/`, or `apps/web/`)*.

---

## 3. Endpoints & REST Interface

The worker exposes:
- `GET /health` -> `{ "status": "ok", "engine": "paperpilot-wasm", "version": "0.1.0" }`
- `POST /api/v1/merge` -> Accepts `multipart/form-data` with multiple files -> Returns merged PDF bytes.
- `POST /api/v1/rotate` -> Accepts PDF buffer + query params `angle=90&pages=all` -> Returns rotated PDF.
- `POST /api/v1/compress` -> Accepts PDF buffer -> Returns deflated PDF.
- `POST /api/v1/encrypt` -> Accepts PDF buffer + `password` -> Returns encrypted PDF.
- `POST /api/v1/watermark` -> Accepts PDF buffer + `text` -> Returns watermarked PDF.
- `POST /api/v1/delete-pages` -> Accepts PDF buffer + `pages` -> Returns trimmed PDF.
- `POST /api/v1/extract-pages` -> Accepts PDF buffer + `pages` -> Returns extracted PDF.
- `POST /api/v1/crop` -> Accepts PDF buffer + coordinates -> Returns cropped PDF.
- `POST /api/v1/flatten` -> Accepts PDF buffer -> Returns flattened PDF.
- `POST /api/v1/metadata` -> Accepts PDF buffer + metadata fields -> Returns updated PDF.

---

## 4. Verification & Testing
- Miniflare / Vitest edge simulation testing memory execution and response headers.
- Generates `reports/EDGE_MICROSERVICE_REPORT.md` verifying sub-10ms response times.
