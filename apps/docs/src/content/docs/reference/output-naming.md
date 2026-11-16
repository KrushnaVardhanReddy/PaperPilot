---
title: "Dynamic output naming"
description: "Local recipe and watcher filenames from actual inputs, PDF metadata, UTC dates and hashes."
---

File workflows and folder watchers accept an optional `name_template`. CLI uses `--name-template`. For file workflows, `output` / `--output` becomes an output **directory** when this option is present. MCP `recipe_run` requires an explicit `output` directory in this mode. Without the option, existing output behavior is preserved.

| Token | Value |
| --- | --- |
| `{input_name}` | Original filename including extension. |
| `{input_stem}` | Original filename without its last extension. |
| `{input_ext}` | Original extension without a dot. |
| `{date}` | UTC date as `YYYY-MM-DD` at naming time. |
| `{date:%Y%m%d}` | Explicit date/time format; `%H%M%S` is also accepted. |
| `{hash}` / `{hash:8}` | SHA-256 of the exact input bytes; prefix length 4–64. |
| `{index}` / `{index:04}` | One-based attempt number, padded up to 12 digits. |
| `{metadata.title}` | PDF Info dictionary Title. |
| `{metadata.author}` | PDF Info dictionary Author. |
| `{metadata.subject}` / `{metadata.keywords}` | PDF Info Subject / Keywords. |
| `{metadata.creator}` / `{metadata.producer}` | PDF Info Creator / Producer. |

Recipe invocations have index 1. A watcher counts successful and failed attempts in its current run, ignoring duplicate events for unchanged content. Restarting a watcher restarts its index. Filesystem arrival order determines indexing; it is not a global ID. Hashes and metadata refer to the source PDF before workflow stages. Metadata tokens read the PDF Info dictionary, not XMP or form fields. Missing/empty fields and encrypted inputs used for metadata naming fail explicitly; decrypt before invoking metadata naming.

Input names and metadata values replace forbidden filename characters with underscores. The final result must be a single portable `.pdf` basename of at most 240 UTF-8 bytes. Paths, reserved Windows names, invalid date formats, unsupported tokens and oversized names are rejected. Date formats that produce slashes, colons, control characters or other unsafe output are invalid. Unknown patterns fail before a watcher starts; input-specific metadata/name errors are recorded per file and preserve the source.

Named recipe outputs never overwrite existing files: a collision produces a failed persisted receipt. Watchers retain their numeric collision suffixes (`name-1.pdf`, etc.); receipts reference the actual published path. When `move_original` is requested, archived originals retain their original basenames. The source is read once per attempt for naming and processing, with no network access or environment-variable interpolation.

```bash
paperpilot run flow.ppflow invoice.pdf --output ./out \
  --name-template '{input_stem}-{date:%Y%m%d}-{hash:8}.pdf' --json

paperpilot watch --directory ./inbox --recipe preset-legal-discovery \
  --name-template '{metadata.title}-{index:04}.pdf'
```

The recipe report includes the actual `output`, `job_id` and `receipt_path`. Watcher receipts are stored beside the actual output. [Batch mail merge](/reference/tools/cli_fill_batch/) retains its separate `{row.column}`, date and index naming contract.
