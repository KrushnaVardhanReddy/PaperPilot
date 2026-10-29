# Task 133A: OpenSSF Scorecard Supply Chain Hardening & Security Policy

- **Spec**: [Spec 133](../133-openssf-scorecard-supply-chain-hardening/spec.md)
- **Phase**: `Phase 5.9.100` (Phase 5 / Security Hardening)
- **Branch**: `feat/openssf-scorecard-hardening`
- **Objective**: Boost repository OpenSSF Scorecard rating from 3.7/10 towards 7.5+/10 by establishing `SECURITY.md`, restricting GitHub workflow tokens to `permissions: read-all`, pinning remaining SHA actions, and adding automated Scorecard SARIF scans.

---

## 1. Action Items

### A. Root `SECURITY.md`
- Create `/home/krushna/Project/PaperPilot/SECURITY.md`.
- Document:
  - Supported Versions (Current stable & dev releases).
  - Reporting vulnerabilities via GitHub Private Vulnerability Reporting (`https://github.com/KrushnaVardhanReddy/PaperPilot/security/advisories/new`).
  - Response timelines: Acknowledgement within 48 hours, triage and CVE coordination.
  - Safe Harbor & Responsible Disclosure terms.

### B. Workflow Token Hardening (`Token-Permissions`)
Enforce least privilege across all workflows:
1. `.github/workflows/security.yml`:
   - Add top-level `permissions: read-all`.
   - In `semgrep-sast` job: add `permissions: { contents: read, security-events: write }`.
2. `.github/workflows/rust.yml`:
   - Add top-level `permissions: read-all`.
3. `.github/workflows/release.yml`:
   - Change top-level `permissions: contents: write` to top-level `permissions: read-all`.
   - In job `create-release`: add scoped `permissions: { contents: write }`.

### C. Automated OpenSSF Scorecard Workflow
- Create `.github/workflows/scorecard.yml`:
  - Run on schedule (weekly cron) + `workflow_dispatch` + push to `main`.
  - Pinned by 40-character commit SHA:
    - `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1`
    - `ossf/scorecard-action@...`
    - `github/codeql-action/upload-sarif@...`
  - Explicit job permissions: `security-events: write`, `id-token: write`.

---

## 2. Verification
1. Validate workflow YAMLs:
   ```bash
   python3 -c "import yaml, glob; [yaml.safe_load(open(f)) for f in glob.glob('.github/workflows/*.yml')]"
   ```
2. Test local Scorecard evaluation:
   ```bash
   scorecard --repo=github.com/KrushnaVardhanReddy/PaperPilot --checks=Security-Policy,Token-Permissions
   ```
