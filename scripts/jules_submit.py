#!/usr/bin/env python3
"""
Jules Batch Submitter — Local AI Assistant
Sends task prompts to Jules API to create async coding sessions → GitHub PRs.
Completed prompts are automatically archived to prompts/tasks/done/ so they are
never accidentally re-submitted.

Usage:
  python3 scripts/jules_submit.py --task P1-T1          # Submit a specific task
  python3 scripts/jules_submit.py --phase 1             # Submit all Phase 1 tasks
  python3 scripts/jules_submit.py --phase 1 --dry-run   # Preview without submitting
  python3 scripts/jules_submit.py --file prompts/tasks/P2_T1_foo.txt
  python3 scripts/jules_submit.py --list                # List all pending tasks
  python3 scripts/jules_submit.py --branch feature/dev  # Target a specific branch
"""

import json
import urllib.request
import sys
import os
import glob

# ──────────────────────────────────────────────────────────────────────────────
# Config — loads API key from .env.local or .env (never hardcode secrets)
# ──────────────────────────────────────────────────────────────────────────────

def _load_api_key():
    """Read JULES_API_KEY from environment, .env.local, or .env."""
    key = os.environ.get("JULES_API_KEY")
    if key:
        return key
    for envfile in [".env.local", ".env"]:
        repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        path = os.path.join(repo_root, envfile)
        if os.path.exists(path):
            with open(path) as f:
                for line in f:
                    line = line.strip()
                    if line.startswith("JULES_API_KEY="):
                        return line.split("=", 1)[1].strip()
    print("❌ JULES_API_KEY not found in environment, .env.local, or .env")
    sys.exit(1)

API_KEY  = _load_api_key()
API_URL  = "https://jules.googleapis.com/v1alpha/sessions"

# ── Repo root (one level up from scripts/) ────────────────────────────────────
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# ── Update once the GitHub repo is created ───────────────────────────────────
REPO_SOURCE = "sources/github/KrushnaVardhanReddy/PaperPilot"

# Parse --branch from args
BRANCH = "main"
if "--branch" in sys.argv:
    idx = sys.argv.index("--branch")
    if idx + 1 < len(sys.argv):
        BRANCH = sys.argv[idx + 1]

# ──────────────────────────────────────────────────────────────────────────────
# Safety rules — prepended to every Jules prompt
# ──────────────────────────────────────────────────────────────────────────────

SAFETY_RULES = """
MANDATORY RULES — VIOLATION = REJECTED PR:
1. NEVER stub, mock, or TODO existing implementation code. Write real, working code only.
2. Commit message must start with "jules: " prefix.
3. Use clean module design and idiomatic Rust architecture. No spaghetti code.
4. NEVER hardcode secrets, API keys, or local paths — always read from environment variables.
5. UNIT TESTS REQUIRED: For every Rust file you create or modify, you MUST write accompanying unit tests (e.g. `#[cfg(test)]` modules) with high coverage.

Project: PaperPilot
Tech stack:
  - Core: Rust (Cargo)
  - PDF libraries: lopdf, pdfium-render
  - CLI parser: clap
  - MCP: rmcp
  - Desktop: Tauri 2, Svelte 5, TypeScript, Vite

Critical Rules:
- Rust code must use typed errors (no string panics).
- Unit tests are required for all operations.
- Do NOT use Electron; use Tauri 2.
- The AI should not directly manipulate document bytes, only use MCP tool operations.
""".strip()

# ──────────────────────────────────────────────────────────────────────────────
# Task file conventions
#   prompts/tasks/P{phase}_T{task}_{slug}.txt  →  e.g. P1_T1_ollama_setup.txt
# ──────────────────────────────────────────────────────────────────────────────

TASKS_DIR = os.path.join(REPO_ROOT, "prompts", "tasks")
DONE_DIR  = os.path.join(TASKS_DIR, "done")  # Completed prompts land here

def _find_task_file(task_id: str) -> str:
    """Find a prompt file matching e.g. 'P1-T1' → prompts/tasks/**/P1_T1_*.txt"""
    normalized = task_id.replace("-", "_").upper()
    
    # Search recursively in subdirectories
    pattern = os.path.join(TASKS_DIR, "**", f"{normalized}_*.txt")
    matches = glob.glob(pattern, recursive=True)
    
    if not matches:
        # Fallback: exact filename match in any subdirectory
        exact_pattern = os.path.join(TASKS_DIR, "**", f"{normalized}.txt")
        exact_matches = glob.glob(exact_pattern, recursive=True)
        if exact_matches:
            return exact_matches[0]
            
        # Try finding if user passed the exact filename minus extension
        file_pattern = os.path.join(TASKS_DIR, "**", f"{task_id}.txt")
        file_matches = glob.glob(file_pattern, recursive=True)
        if file_matches:
            return file_matches[0]
            
        print(f"❌ No prompt file found for task '{task_id}' in {TASKS_DIR}/ (including subfolders)")
        print(f"   Expected pattern: {normalized}_<slug>.txt")
        sys.exit(1)
        
    if len(matches) > 1:
        print(f"⚠️  Multiple files found for '{task_id}': {matches}")
        print(f"   Using: {matches[0]}")
    return matches[0]

def _find_phase_files(phase_num: int) -> list:
    """Find all PENDING prompt files for a given phase (excludes done/)."""
    pattern = os.path.join(TASKS_DIR, f"phase_{phase_num}", f"P{phase_num}_T*.txt")
    matches = sorted(glob.glob(pattern, recursive=True))
    # Also check top-level tasks dir for any files not in a sub-folder
    top_pattern = os.path.join(TASKS_DIR, f"P{phase_num}_T*.txt")
    matches += sorted(glob.glob(top_pattern))
    # Exclude anything already in done/
    matches = [m for m in matches if os.sep + "done" + os.sep not in m]
    if not matches:
        print(f"❌ No pending prompt files found for Phase {phase_num}.")
        print(f"   (Already done? Check {DONE_DIR}/phase_{phase_num}/)")
        sys.exit(1)
    return matches

# ──────────────────────────────────────────────────────────────────────────────
# Submission logic
# ──────────────────────────────────────────────────────────────────────────────

def submit_prompt(full_prompt: str, task_name: str = "Task"):
    payload = json.dumps({
        "prompt": full_prompt,
        "sourceContext": {
            "source": REPO_SOURCE,
            "githubRepoContext": {
                "startingBranch": BRANCH
            }
        }
    }).encode()

    req = urllib.request.Request(
        API_URL,
        data=payload,
        headers={
            "Content-Type": "application/json",
            "x-goog-api-key": API_KEY
        },
        method="POST"
    )

    print(f"🚀 Submitting: {task_name} → branch: {BRANCH}")
    try:
        with urllib.request.urlopen(req) as resp:
            result = json.loads(resp.read())
            session_id = result.get("name", "unknown").split("/")[-1]
            print(f"✅ Session created: {session_id}")
            print(f"   View at: https://jules.google.com/session/{session_id}")
            return session_id
    except urllib.error.HTTPError as e:
        print(f"❌ HTTP {e.code}: {e.read().decode()}")
        sys.exit(1)

def submit_file(filepath: str, label: str = None, dry_run: bool = False):
    """Submit a prompt file to Jules and archive it on success."""
    if not os.path.exists(filepath):
        print(f"❌ File not found: {filepath}")
        sys.exit(1)
    with open(filepath) as f:
        prompt_content = f.read()
    full_prompt = SAFETY_RULES + "\n\n---\n\n" + prompt_content
    name = label or os.path.basename(filepath)

    if dry_run:
        print(f"[⏸️  DRY RUN] Would submit: {name}")
        return

    session_id = submit_prompt(full_prompt, task_name=name)

    if session_id:
        # Archive the prompt so it won't be picked up again
        _archive_prompt(filepath)


def _archive_prompt(filepath: str):
    """Move a submitted prompt into done/ to prevent re-submission."""
    import shutil
    # Preserve the phase subfolder structure inside done/
    rel = os.path.relpath(filepath, TASKS_DIR)
    dest = os.path.join(DONE_DIR, rel)
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.move(filepath, dest)
    print(f"   ✅ Archived → prompts/tasks/done/{rel}")


def list_tasks():
    """Print all pending task prompt files (excludes done/)."""
    if not os.path.isdir(TASKS_DIR):
        print(f"❌ Tasks directory not found: {TASKS_DIR}")
        sys.exit(1)
    files = sorted(glob.glob(os.path.join(TASKS_DIR, "**", "*.txt"), recursive=True))
    # Exclude done/ folder
    files = [f for f in files if os.sep + "done" + os.sep not in f]
    if not files:
        print("⚠️  No pending task prompt files found.")
        return
    print(f"\n📋 Pending tasks ({len(files)} total):\n")
    for f in files:
        rel = os.path.relpath(f, TASKS_DIR)
        print(f"   {rel}")
    print()

# ──────────────────────────────────────────────────────────────────────────────
# CLI entry point
# ──────────────────────────────────────────────────────────────────────────────

def main():
    args = sys.argv[1:]

    if not args or "--help" in args or "-h" in args:
        print(__doc__)
        sys.exit(0)

    if "--list" in args:
        list_tasks()
        sys.exit(0)

    if "--file" in args:
        idx = args.index("--file")
        if idx + 1 >= len(args):
            print("❌ Please specify a file path after --file.")
            sys.exit(1)
        submit_file(args[idx + 1])
        sys.exit(0)

    if "--task" in args:
        idx = args.index("--task")
        if idx + 1 >= len(args):
            print("❌ Please specify a task ID after --task (e.g. P1-T1).")
            sys.exit(1)
        task_id = args[idx + 1]
        filepath = _find_task_file(task_id)
        submit_file(filepath, label=task_id)
        sys.exit(0)

    if "--phase" in args:
        idx = args.index("--phase")
        if idx + 1 >= len(args):
            print("❌ Please specify a phase number after --phase.")
            sys.exit(1)
        phase_num = int(args[idx + 1])
        dry_run = "--dry-run" in args
        files = _find_phase_files(phase_num)
        action = "Previewing" if dry_run else "Submitting"
        print(f"📅 {action} {len(files)} pending task(s) for Phase {phase_num}...")
        for filepath in files:
            submit_file(filepath, label=os.path.basename(filepath), dry_run=dry_run)
        sys.exit(0)

    print(__doc__)

if __name__ == "__main__":
    main()
