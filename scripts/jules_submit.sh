#!/usr/bin/env bash
# Lightweight wrapper to forward arguments to the pure-Rust Jules submitter

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
REPO_ROOT=$(dirname "$SCRIPT_DIR")

# We run from the repo root so paths resolve correctly
cd "$REPO_ROOT" || { echo "Failed to cd to REPO_ROOT"; return 1 2>/dev/null || true; }

# Delete the old python script if it's still lying around
if [ -f "$SCRIPT_DIR/jules_submit.py" ]; then
    rm "$SCRIPT_DIR/jules_submit.py"
fi

exec cargo run --quiet -p jules-submit -- "$@"
