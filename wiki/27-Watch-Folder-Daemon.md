# Watch Folder Daemon (paperpilot watch)

## Overview
The `paperpilot watch` command introduces a daemon mode to the CLI that continuously monitors a specified directory for incoming PDF files. Once a file is detected (created or modified), it waits for a brief settling period to ensure the copy is complete, and then processes the file according to the designated operation or pipeline recipe.

This feature is designed for continuous automation, such as dropping scanned documents into a folder for automatic OCR or compression.

## Architecture & Components
- **notify crate (`v6.1`)**: Used for highly efficient, pure-Rust cross-platform filesystem event monitoring.
- **Debouncing**: To prevent partial reads of files that are still being copied over the network or from a scanner, a debouncing mechanism delays processing until no new modification events are received for `settle_delay_ms` (default 500ms).
- **Graceful Shutdown**: Intercepts SIGINT/SIGTERM (via `tokio::signal::ctrl_c()`) to cleanly terminate the watch loop without leaving files in an inconsistent state.
- **Webhook Integration**: Can optionally post a JSON payload to a specified webhook URL upon completion of each file's processing.
- **No External Subprocesses**: Operations are executed directly via the existing `paperpilot_cli::commands::execute_command` pipeline for maximum efficiency.

## Usage
```bash
paperpilot watch ./incoming \
  --operation compress \
  --output ./processed \
  --move-original ./archive \
  --settle-delay-ms 1000 \
  --webhook http://localhost:8080/hook
```

## Options
- `<DIRECTORY>`: Positional argument specifying the folder to watch.
- `--output <DIR>`: Directory to place the processed files.
- `--operation <OP>`: The operation to execute (e.g., `compress`, `ocr`).
- `--recipe <FILE_OR_PRESET>`: The pipeline recipe to execute.
- `--webhook <URL>`: Webhook to POST upon completion.
- `--settle-delay-ms <MS>`: Time to wait after the last write event before processing.
- `--move-original <DIR>`: Move the original input file here after successful processing.
