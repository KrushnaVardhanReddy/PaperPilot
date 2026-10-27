# Open-WebUI and Tika-compatible PDF ingestion

PaperPilot Gateway can replace the PDF extraction part of a Tika deployment.
The adapter sends uploaded bytes directly to the existing Rust extraction engine,
preserves page order and detected Markdown tables, and returns the text in memory.
It does not create temporary files or start Java/Python parser processes.

## Use Open-WebUI's built-in document upload

Start the gateway on its normal loopback address:

```sh
cargo run -p paperpilot-gateway -j 2
```

In Open-WebUI's administrator document settings, select **Tika** as the content
extraction engine and set the server URL to the gateway's base URL. On the same
host, this is `http://127.0.0.1:7823`. Do not append `/tika` to the base URL.
Both Tika server version settings, **3** and **4**, are supported.

For a new Open-WebUI deployment, the corresponding environment settings are:

```env
CONTENT_EXTRACTION_ENGINE=tika
TIKA_SERVER_URL=http://127.0.0.1:7823
TIKA_SERVER_VERSION=3
```

For existing deployments, update the persisted administrator settings as well.
If Open-WebUI runs in Docker, its `127.0.0.1` is the container itself. Use a
gateway address reachable from that container, such as
`http://host.docker.internal:7823`, and bind the gateway to the appropriate private
interface when necessary. Keep the gateway within the trusted local network.
The gateway reads its listen address from `BIND` and `PORT` environment variables
(defaults: `127.0.0.1` and `7823`).

Upload a PDF in an Open-WebUI chat. Open-WebUI performs a binary `PUT` and reads
the returned object; there is no model tool setup needed for this upload flow.

## HTTP contracts

All requests contain **raw PDF bytes**, rather than multipart form data, URLs,
or server file paths. `Content-Type` may be `application/pdf`,
`application/octet-stream`, or omitted; the PDF signature is checked as well.

| Method | Path | Response |
|---|---|---|
| PUT | `/tika/text` | Object with `X-TIKA:content` (Open-WebUI Tika 3 loader) |
| PUT | `/tika/json/md` | Object with `tk:content` (Open-WebUI Tika 4 loader) |
| PUT | `/tika` | UTF-8 text; an object with `X-TIKA:content` when `Accept: application/json` is sent |
| PUT or POST | `/rmeta/text` | Array containing the root PDF's metadata object |
| PUT or POST | `/tika/rmeta/text` | Alias for `/rmeta/text` used in the task tracker |
| POST or PUT | `/api/v1/tika/rmeta/text` | Versioned alias for `/rmeta/text` |

The metadata object also contains `Content-Type: application/pdf` and
`xmpTPg:NPages` (a string). Pages are joined with two newlines. Markdown formatting
is retained so detected tables remain useful for retrieval.

```sh
curl --fail-with-body -X PUT --data-binary @report.pdf \
  -H 'Content-Type: application/pdf' http://127.0.0.1:7823/tika/text

curl --fail-with-body -X POST --data-binary @report.pdf \
  -H 'Content-Type: application/pdf' http://127.0.0.1:7823/api/v1/tika/rmeta/text
```

Example response from `/tika/text`:

```json
{
  "X-TIKA:content": "First page text\n\nSecond page text",
  "Content-Type": "application/pdf",
  "xmpTPg:NPages": "2"
}
```

The gateway's Swagger UI at `/swagger-ui` and schema at `/api-docs/openapi.json`
describe the primary ingestion routes.

## Optional custom tool for a shared PDF folder

For a model-invoked tool over a folder explicitly shared with Open-WebUI, paste
the contents of [open_webui_tool.py](open_webui_tool.py) into a new Open-WebUI
custom Tool. Set its administrator valves:

- `PAPERPILOT_URL`: the base URL reachable from the Open-WebUI process.
- `PDF_ROOT`: an absolute directory **inside the Open-WebUI host/container**,
  such as `/data/pdfs`, containing the PDFs the tool should access.

The exposed tool is `extract_pdf(filename: str)`. A call such as
`{"filename":"invoices/january.pdf"}` reads only a PDF under that folder and
returns its extracted text. It rejects absolute paths, traversal, symlink escapes,
non-PDF files, and inputs over 100 MiB. The tool's Python code runs in Open-WebUI;
PaperPilot's extraction runtime remains Rust. This optional tool does not resolve
Open-WebUI attachment IDs; use the built-in upload flow above for chat attachments.

The existing MCP `pdf_extract_text` tool also remains available to local agents.
This adapter adds an HTTP compatibility layer rather than a second extraction engine.

## Scope and errors

This is a **PDF ingestion adapter**, not a complete Apache Tika implementation.
It does not parse Office files, archives, embedded attachments, or images on these
routes. Image-only PDFs need a separate OCR step first. Encrypted PDFs must be
unlocked before ingestion. The root-only `/rmeta/text` array does not claim to
recursively parse attachments.

| Status | Meaning |
|---|---|
| 400 | Empty request body |
| 413 | Upload exceeds the configured body limit (100 MiB by default) |
| 415 | Unsupported media type, multipart payload, or missing PDF signature |
| 422 | PDF parsing or extraction failed, or the PDF has no pages |
| 500 | Extraction worker failed |

CPU-bound parsing runs on a blocking worker so async request workers remain
available. The spec's sub-15 ms target depends on document size, layout, and
hardware; it is not a latency guarantee for arbitrary PDFs.

Protocol references checked for this implementation:
[Open-WebUI TikaLoader](https://github.com/open-webui/open-webui/blob/main/backend/open_webui/retrieval/loaders/main.py),
[Open-WebUI configuration](https://github.com/open-webui/open-webui/blob/main/backend/open_webui/config.py),
and [Apache Tika Server](https://tika.apache.org/docs/4.1.x/using-tika/server/index.html).
