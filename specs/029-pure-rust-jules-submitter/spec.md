# Spec 029: Pure-Rust Jules Task Submitter CLI (`tools/jules-submit`)

## Status
Approved

## Context
PaperPilot uses an asynchronous multi-agent delegation workflow where task prompts are dispatched to Google's Jules API (`https://jules.googleapis.com/v1alpha/sessions`) to generate cloud pull requests.
Currently, this is handled by `scripts/jules_submit.py`. 
While functional, relying on Python introduces an external runtime dependency in a repository striving for 100% pure-Rust architecture. Furthermore, the submission logic involves:
- Reading API keys securely from `.env.local` or `.env` without exposing secrets.
- Finding prompt files by flexible pattern matching (e.g. `--task P1-T1`, `--phase 5`, `--file <path>`).
- Injecting project-specific mandatory safety rules and repo source contexts (`sources/github/KrushnaVardhanReddy/PaperPilot`).
- Issuing HTTPS POST requests with custom headers (`x-goog-api-key`).
- Parsing JSON responses and extracting session IDs (`https://jules.google.com/session/{id}`).
- Atomic directory-preserving file archiving from `prompts/tasks/` to `prompts/tasks/done/`.

A simple `curl`/`jq` bash script is too fragile for this pattern search, directory recursion, and atomic archiving logic. Implementing this as a native Rust CLI tool (`tools/jules-submit` or `paperpilot-cli jules-submit`) provides a robust, zero-dependency, typed solution.

## Objectives
1. Implement a standalone pure-Rust CLI tool in `tools/jules-submit/` (or integrate into workspace tools).
2. Maintain 100% feature and flag parity with `scripts/jules_submit.py`:
   - `--list`: List all pending prompt files in `prompts/tasks/` (excluding `done/`).
   - `--file <path>`: Submit a specific prompt file and auto-archive to `done/`.
   - `--task <id>`: Fuzzy-find and submit matching task prompt (e.g. `--task P5_7_1`).
   - `--phase <num>`: Batch-submit all pending prompts for a specific phase.
   - `--branch <name>`: Target custom Git starting branch (defaults to `main`).
   - `--dry-run`: Preview submission payload and target without invoking API.
3. Automatically read `JULES_API_KEY` from environment, `.env.local`, or `.env`.
4. Provide unit tests for argument parsing, safety rule injection, and archiving logic.
5. Provide a backward-compatible wrapper or replace the Python script.

## Technical Design

### Crate Structure (`tools/jules-submit`)
- **`Cargo.toml`**:
  - `clap`: CLI argument parser.
  - `reqwest` (with `rustls-tls`, `blocking` or `tokio`): Zero C++ OpenSSL dependency, lightweight HTTP client.
  - `serde`, `serde_json`: JSON request/response serialization.
  - `walkdir` / `glob`: Recursive prompt file discovery.
- **Modules**:
  - `config.rs`: `.env` / `.env.local` parser and API key loader.
  - `client.rs`: HTTPS client communicating with `https://jules.googleapis.com/v1alpha/sessions`.
  - `archive.rs`: Atomic prompt relocation to `prompts/tasks/done/` preserving phase directory trees.
  - `main.rs`: CLI dispatcher.

## Verification & Quality Gate
1. `cargo check -p jules-submit` and `cargo test -p jules-submit`.
2. Run `cargo run -p jules-submit -- --list` and assert matching output with `python3 scripts/jules_submit.py --list`.
3. Run `cargo run -p jules-submit -- --dry-run --file ...` and assert correct payload generation.
