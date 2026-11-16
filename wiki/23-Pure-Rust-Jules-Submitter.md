# Pure-Rust Jules & Local Gemini CLI Task Submitter (`tools/jules-submit`)

## Overview
The `jules-submit` tool is a pure-Rust multi-agent task dispatcher supporting two execution engines:
1. **Cloud Agent (Jules)**: Dispatches async background PR sessions via Google Cloud Jules API.
2. **Local Pair Programmer (Gemini CLI / `agy`)**: Dispatches local, low-latency agent turns using Google Antigravity / Gemini CLI (`agy`).

This architecture allows developers to switch between remote cloud agents and local pair programmers using identical task prompt files (`prompts/tasks/`).

---

## Architecture & Modules
- **`src/cli.rs`**: Handled via `clap` (derive). Supports `--list`, `--file`, `--task`, `--phase`, `--branch`, `--dry-run`, `--agent` (`jules` | `gemini` | `agy`), and `--interactive`.
- **`src/local_agent.rs`**: Locates local `agy` (Gemini CLI) binary at `~/.gemini/bin/agy` or `AGY_BIN`, and dispatches either autonomous non-interactive executions (`-p --dangerously-skip-permissions`) or interactive pair programming sessions (`-i`).
- **`src/api.rs`**: Constructs the final payload to Jules Cloud API, prepends `SAFETY_RULES.md`, and sends authenticated HTTP requests via `reqwest`.
- **`src/env.rs`**: Reads `JULES_API_KEY` securely from local environment or `.env` when using the cloud agent.
- **`src/fs.rs`**: Governs task file discovery, fuzzy matching by task ID (e.g. `P1-T1` -> `P1_T1_*.txt`), and atomic prompt archiving to `prompts/tasks/done/`.
- **`src/main.rs`**: Central dispatcher routing prompts to either the local Gemini CLI agent or the remote Jules agent.

---

## Usage

### 1. Using the Shell Wrappers

```bash
# Cloud Jules Agent
./scripts/jules_submit.sh --list
./scripts/jules_submit.sh --file prompts/tasks/phase3/my_task.txt

# Local Gemini CLI Pair Programmer
./scripts/gemini_pair.sh --list
./scripts/gemini_pair.sh --file prompts/tasks/phase3/my_task.txt
./scripts/gemini_pair.sh --file prompts/tasks/phase3/my_task.txt --interactive
```

### 2. Using the Makefile Targets

```bash
# List all pending prompts
make gemini-list
make jules-list

# Dry run preview
make gemini-dry-run FILE=prompts/tasks/phase3/my_task.txt

# Execute locally with Gemini CLI pair programmer
make gemini-run FILE=prompts/tasks/phase3/my_task.txt

# Launch interactive pair programming terminal session
make gemini-pair FILE=prompts/tasks/phase3/my_task.txt

# Dispatch to Cloud Jules
make jules-submit FILE=prompts/tasks/phase3/my_task.txt
```

---

## Atomic Archiving
Once a task is successfully executed or submitted, the file is moved to `prompts/tasks/done/<relative_path>`, preventing duplicate prompt execution across sessions.
