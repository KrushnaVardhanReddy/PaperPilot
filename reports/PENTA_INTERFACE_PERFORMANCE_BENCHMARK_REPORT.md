# PENTA-INTERFACE PERFORMANCE BENCHMARK REPORT

## 1. Concurrency Stress Testing
| Concurrent Requests | Interface | Successful | Failed | Total Duration (ms) | Ops/sec |
|---|---|---|---|---|---|
| 10 | 🌐 REST API | 10 | 0 | 1457.05 | 6.86 |
| 10 | 🤖 MCP | 10 | 0 | 37.64 | 265.66 |
| 25 | 🌐 REST API | 25 | 0 | 2806.73 | 8.91 |
| 25 | 🤖 MCP | 25 | 0 | 83.59 | 299.09 |
| 50 | 🌐 REST API | 50 | 0 | 5233.69 | 9.55 |
| 50 | 🤖 MCP | 50 | 0 | 168.85 | 296.11 |

## 2. Latency SLA Profile (Top 10 Tools)
| Tool | Interface | Mean (ms) | P50 (ms) | P95 (ms) | P99 (ms) |
|---|---|---|---|---|---|
| `pdf_compress` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_compress` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_compress` | 🌐 REST API | 1.36 | 1.16 | 2.02 | 2.02 |
| `pdf_compress` | 💻 CLI | 15.53 | 15.10 | 16.60 | 16.60 |
| `pdf_compress` | 🤖 MCP | 13.15 | 12.85 | 13.97 | 13.97 |
| `pdf_convert_html` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_convert_html` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_convert_html` | 🌐 REST API | 1.18 | 1.16 | 1.65 | 1.65 |
| `pdf_convert_html` | 💻 CLI | 78.93 | 77.83 | 81.31 | 81.31 |
| `pdf_convert_html` | 🤖 MCP | 77.52 | 76.61 | 80.52 | 80.52 |
| `pdf_encrypt` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_encrypt` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_encrypt` | 🌐 REST API | 1.18 | 1.06 | 1.79 | 1.79 |
| `pdf_encrypt` | 💻 CLI | 15.53 | 15.43 | 15.79 | 15.79 |
| `pdf_encrypt` | 🤖 MCP | 19.13 | 18.70 | 20.32 | 20.32 |
| `pdf_extract_text` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_extract_text` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_extract_text` | 🌐 REST API | 1.10 | 1.01 | 1.73 | 1.73 |
| `pdf_extract_text` | 💻 CLI | 15.74 | 15.34 | 16.63 | 16.63 |
| `pdf_extract_text` | 🤖 MCP | 12.85 | 12.75 | 13.75 | 13.75 |
| `pdf_hash` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_hash` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_hash` | 🌐 REST API | 1.41 | 1.36 | 1.69 | 1.69 |
| `pdf_hash` | 💻 CLI | 15.83 | 15.79 | 16.50 | 16.50 |
| `pdf_hash` | 🤖 MCP | 13.81 | 13.80 | 14.16 | 14.16 |
| `pdf_merge` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_merge` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_merge` | 🌐 REST API | 5.29 | 4.67 | 8.04 | 8.04 |
| `pdf_merge` | 💻 CLI | 20.89 | 20.21 | 23.00 | 23.00 |
| `pdf_merge` | 🤖 MCP | 18.52 | 18.81 | 18.99 | 18.99 |
| `pdf_render` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_render` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_render` | 🌐 REST API | 1.29 | 1.22 | 1.69 | 1.69 |
| `pdf_render` | 💻 CLI | 18.60 | 18.63 | 19.28 | 19.28 |
| `pdf_render` | 🤖 MCP | 16.84 | 16.72 | 17.38 | 17.38 |
| `pdf_rotate` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_rotate` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_rotate` | 🌐 REST API | 1.11 | 1.04 | 1.55 | 1.55 |
| `pdf_rotate` | 💻 CLI | 19.55 | 19.95 | 20.28 | 20.28 |
| `pdf_rotate` | 🤖 MCP | 12.80 | 12.35 | 13.76 | 13.76 |
| `pdf_split` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_split` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_split` | 🌐 REST API | 6.24 | 6.31 | 6.55 | 6.55 |
| `pdf_split` | 💻 CLI | 15.58 | 15.22 | 16.44 | 16.44 |
| `pdf_split` | 🤖 MCP | 20.46 | 20.36 | 23.19 | 23.19 |
| `pdf_watermark` | ☁️ Cloudflare Edge | N/A | N/A | N/A | N/A |
| `pdf_watermark` | ⚡ WASM (Browser) | N/A | N/A | N/A | N/A |
| `pdf_watermark` | 🌐 REST API | 3.63 | 3.58 | 4.20 | 4.20 |
| `pdf_watermark` | 💻 CLI | 19.25 | 18.96 | 21.01 | 21.01 |
| `pdf_watermark` | 🤖 MCP | 17.25 | 17.24 | 18.27 | 18.27 |

## 3. Memory Footprint & RSS Stability
| Sample Interval | RSS Memory (MB) |
|---|---|
| Burst #200 | 34.50 MB |
| Burst #400 | 34.50 MB |
| Burst #600 | 34.50 MB |
| Burst #800 | 34.50 MB |
| Burst #1000 | 34.50 MB |
