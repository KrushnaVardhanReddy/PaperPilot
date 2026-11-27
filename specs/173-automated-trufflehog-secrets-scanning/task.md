# Task 173: Implement Automated Secret Leak Auditing with TruffleHog

## Overview
Implement Spec 173 by introducing local and CI-based secret detection via TruffleHog OSS with native binary bootstrapping, verification filters, and branch-aware execution.

---

## Subtasks & Checklist

- [x] **Subtask 1: Local Runner Script (`scripts/run_secrets_scan.sh`)**
  - [x] Auto-detect existing `trufflehog` in system `$PATH`.
  - [x] Auto-download native platform binary into `.bin/trufflehog` when missing.
  - [x] Pass `--only-verified` to suppress non-secret mock data in test fixtures.
  - [x] Add branch detection to avoid blocking feature branch development.

- [x] **Subtask 2: Makefile Integration**
  - [x] Add `trufflehog-scan` target.
  - [x] Add alias `secret-scan-local`.
  - [x] Add `.bin/` to `.gitignore`.

- [x] **Subtask 3: GitHub Actions CI Integration (`.github/workflows/security.yml`)**
  - [x] Add `trufflehog-scan` job configured with `trufflesecurity/trufflehog`.
  - [x] Condition job on push to `main` or PRs targeting `main`.
