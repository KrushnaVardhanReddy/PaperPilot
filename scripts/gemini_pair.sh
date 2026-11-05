#!/usr/bin/env bash
# PaperPilot Local Gemini CLI Pair Programmer Wrapper
# Uses tools/jules-submit with --agent gemini

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
REPO_ROOT=$(dirname "$SCRIPT_DIR")

cd "$REPO_ROOT" || { echo "Failed to cd to REPO_ROOT"; exit 1; }

exec cargo run --quiet -p jules-submit -- --agent gemini "$@"
