# Cloudflare Edge Service

The PaperPilot Cloudflare Edge Service runs in `apps/edge` and exposes native WebAssembly (`paperpilot-wasm`) operations as REST endpoints.

## Architecture
- **Framework**: Cloudflare Workers (V8 Isolates)
- **Engine**: `paperpilot-wasm`
- **Routing**: Lightweight standard `fetch` handler

## Endpoints

- `GET /health` : Returns service status.
- `POST /api/v1/rotate` : Rotate PDF pages (query `angle` and `pages`).
- `POST /api/v1/compress` : Compress PDF payload.
- `POST /api/v1/watermark` : Add watermark to PDF (query `text`).
- `POST /api/v1/merge` : Merge multiple PDFs (requires `FormData` payload with files).

## Deployment

To deploy this service locally or to production:

```bash
cd apps/edge
pnpm install

# Local development
pnpm dev

# Deploy to Cloudflare
pnpm deploy
```
