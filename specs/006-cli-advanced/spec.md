# Phase 1.5: Advanced CLI Features (JSON & Webhooks)

## Objective
Add enterprise-grade features to the `paperpilot-cli`:
1. `validate` subcommand
2. `--json` global flag
3. `--webhook` global flag

## 1. `validate` command
Usage: `paperpilot validate --input in.pdf`
Checks for magic bytes (`%PDF-`), corruption, and whether the file is encrypted. Prints a summary of the document's health.

## 2. `--json` global output
If `--json` is passed, all `eprintln!` and `println!` outputs should be suppressed, and the CLI must instead print a single JSON payload to `stdout` upon completion or failure.
```json
{
  "status": "success",
  "operation": "merge",
  "output_file": "/path/to/out.pdf",
  "execution_time_ms": 145
}
```

## 3. `--webhook <url>`
If a webhook URL is provided, the CLI will spawn a background thread (or just block at the end) and `POST` the exact same JSON payload (from the `--json` feature) to the URL before exiting. Requires adding `reqwest` (with `blocking` feature) to `paperpilot-cli/Cargo.toml`.
