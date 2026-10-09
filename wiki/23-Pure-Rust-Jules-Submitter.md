# Pure-Rust Jules Submitter CLI Tool (`tools/jules-submit`)

## Overview
The `jules-submit` tool replaces the old `scripts/jules_submit.py` Python script, converting the agent task-submission process into a fast, 100% pure-Rust binary. This removes Python from the PaperPilot toolchain for routine operations.

## Architecture & Modules
- **`src/cli.rs`**: Handled via `clap` (derive). Supports commands for `--list`, `--file`, `--task`, `--phase`, `--branch`, and `--dry-run`.
- **`src/env.rs`**: Reads `JULES_API_KEY` from the local environment, `dotenvy`, or standard `.env` paths safely without leaking the API key to standard output.
- **`src/fs.rs`**: Governs task file discovery, fuzzy matching by task ID (e.g. `P1-T1` -> `P1_T1_*.txt`), finding phase-specific prompts, and handles atomic archiving.
- **`src/api.rs`**: Constructs the final payload to Jules, prepending `SAFETY_RULES`, setting up REST HTTP paths via `reqwest`, and returning the generated session ID link.
- **`src/main.rs`**: Brings together the arguments and delegates to respective module logic.

## Usage
Through the shell wrapper `scripts/jules_submit.sh` or standard cargo:

```bash
# List all pending prompts
cargo run -p jules-submit -- --list

# Submit a fuzzy task by ID
cargo run -p jules-submit -- --task P1-T1

# Submit an entire phase
cargo run -p jules-submit -- --phase 5
```

## Atomic Archiving
Once a task is successfully submitted, the file is atomically moved to `prompts/tasks/done/...` ensuring that there is no risk of duplicate prompt submission across varying workflow iterations.
