# Spec 126 / Task 5.9.95 — Open-WebUI PDF Ingestion Adapter

Date: October 27, 2026 (Asia/Hong_Kong).
Baseline: `origin/feature/dev` at `9c8e605`; local branch: `feat/open-webui-tika-adapter`.
Implementation is local and unmerged; no release version was changed.

## Behavior

Open-WebUI can select PaperPilot Gateway as its Tika server for binary PDF uploads.
The adapter supports the current loader contracts (`PUT /tika/text` with
`X-TIKA:content`, and `PUT /tika/json/md` with `tk:content`), raw UTF-8 output at
`PUT /tika`, and root metadata arrays at `/rmeta/text`, `/tika/rmeta/text`, and
`/api/v1/tika/rmeta/text`.

All routes reuse `run_extract_text_bytes` in `paperpilot-pdf`, also used by the
existing `ExtractTextOperation` fast path. Page order and detected Markdown tables
are preserved. CPU-bound parsing uses a blocking worker; upload bytes and output
stay in memory. Empty, non-PDF, malformed, locked, and oversized uploads have
explicit HTTP errors. No new Rust dependency or MCP tool was added.

The generated OpenAPI snapshots include binary upload schemas. The
[integration guide](../docs/integrations/open_webui.md) provides native upload
settings and a ready-to-paste optional Open-WebUI custom Tool for a shared folder.

## Verification

Environment: Windows x64, Rust 1.99.1, GNU target, MinGW GCC 16.2.0 (SEH).
All Cargo builds and tests use `-j 2`.

| Check | Result |
|---|---|
| Gateway integration tests (`--test server_test`) | 11 passed, including six new ingestion tests |
| Core extraction unit tests | 5 passed; page selection/order, legacy parity, corrupt and locked PDFs covered |
| Full gateway test suite | 21 passed (10 unit + 11 integration); zero failures |
| Actual localhost gateway process | Seven HTTP contract checks passed |
| Existing MCP `pdf_extract_text` through real gateway dispatch | Passed; output contains both PDF pages |
| Optional Open-WebUI custom Tool | Real PDF upload passed; traversal, absolute-path and non-PDF-extension inputs rejected |
| Changed Rust files: `rustfmt --check` | Passed |
| Local documentation links and Python example import | Passed |
| Generated OpenAPI snapshots | Identical; upload schema verified as string/binary in integration test |
| `git diff --check` | Passed |

Real HTTP smoke-test evidence: [http-smoke.json](tika-adapter-validation/http-smoke.json).
Test transcripts: [core extraction](tika-adapter-validation/core-tests.txt),
[full gateway](tika-adapter-validation/gateway-tests.txt).

The local HTTP timing used a **synthetic 874-byte, two-page PDF**, a development
build, five warmup requests and 20 measured requests. Median was **2.419 ms** and
p95 was **3.163 ms**, including HTTP overhead. This meets the spec's 15 ms target
for this small fixture; it does not establish throughput or latency for large,
scanned, encrypted, or layout-heavy production documents.

Reproduction commands:

```sh
cargo test -p paperpilot-gateway --locked --test server_test -j 2
cargo test -p paperpilot-pdf --locked --lib operations::extract_text -j 2
cargo test -p paperpilot-gateway --locked -j 2
```

This workspace's reusable Windows toolchain is isolated under
`D:\building\.paperpilot-tools`. To use it in a new PowerShell process:

```powershell
$taskTools = 'D:\building\.paperpilot-tools'
$env:CARGO_HOME = "$taskTools\cargo"
$env:RUSTUP_HOME = "$taskTools\rustup"
$env:CARGO_TARGET_DIR = "$taskTools\target"
$env:PATH = "$taskTools\cargo\bin;$taskTools\mingw64\bin;" + $env:PATH
$env:CC = "$taskTools\mingw64\bin\gcc.exe"
$env:CXX = "$taskTools\mingw64\bin\g++.exe"
$env:AR = "$taskTools\mingw64\bin\ar.exe"
```

The roadmap's stale Booklet, N-Up, and page-scaling status entries were also
reconciled with the existing completed records in `PHASES.md`.

## Limits

The verification exercised Open-WebUI's HTTP contracts and the supplied custom
Tool, not a running Open-WebUI chat UI. This is a PDF-only ingestion adapter;
Office formats, embedded attachments, and automatic OCR are outside its scope.
The full desktop/WASM/workspace suite was not run for this gateway change.

The first build used the machine's old SJLJ MinGW and failed in native dependency
linking. Rebuilding with the task-local SEH toolchain resolved it. No system PATH
or existing compiler installation was changed. `graphify update .` was attempted;
the CLI is not installed and no existing `graphify-out/graph.json` was present.
