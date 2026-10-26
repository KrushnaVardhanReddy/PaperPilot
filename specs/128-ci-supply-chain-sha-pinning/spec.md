# Spec 128: CI Supply Chain Hardening & Action SHA Pinning

## 1. Overview & Problem Statement
The external security audit (comment by `devy` on Hacker News) flagged supply chain vulnerabilities in `.github/workflows/`:
1. Actions are pinned to mutable version tags (e.g. `@v4`) rather than immutable commit SHAs.
2. `release.yml` installed `wasm-pack` via unverified `curl | sh`.
3. Android build used unverified or loose lockfile behavior.

## 2. Requirements & Objectives
1. **Immutable Commit SHA Pinning**:
   - Pin every third-party GitHub Action across all `.github/workflows/*.yml` files to its 40-character commit SHA with inline version comments:
     - `actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2`
     - `actions-rust-lang/setup-rust-toolchain@...`
     - `github/codeql-action/upload-sarif@...`
2. **Safe Installer Scripts**:
   - Replace any raw `curl -sSf | sh` in release workflows with verified binary releases and SHA-256 integrity checks.
3. **Automated Dependabot / Renovate SHA Pinning**:
   - Configure `.github/dependabot.yml` to automatically maintain commit SHA pins with version comments.
