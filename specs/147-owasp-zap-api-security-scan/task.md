# Task 147: Implement Automated OWASP ZAP API Dynamic Security Gate

## Overview
Implement Spec 147 by establishing an automated GitHub Actions workflow (`.github/workflows/zap.yml`) that runs the official free OWASP ZAP API scanner against the live `paperpilot-gateway` REST endpoints and uploads SARIF results to the GitHub Security Advisory tab.

---

## Subtasks & Checklist

- [x] **Subtask 1: Gateway OpenAPI Export Tooling**
  - [x] Add a CLI helper or cargo test in `paperpilot-gateway/tests/export_openapi.rs` that dumps `ApiDoc::openapi().to_pretty_json()` directly to `target/openapi.json`.
  - [x] Verify that `target/openapi.json` contains valid OpenAPI 3.0 schema definitions for all routes.

- [x] **Subtask 2: OWASP ZAP Rule Configurations (`.zap/rules.tsv`)**
  - [x] Create `.zap/rules.tsv` with baseline rule configurations.
  - [x] Configure zero-tolerance failure thresholds for High/Critical injection and auth vulnerabilities.

- [x] **Subtask 3: GitHub Actions Dynamic Scan Workflow (`.github/workflows/zap.yml`)**
  - [x] Trigger on weekly cron + manual `workflow_dispatch` + push to `main`.
  - [x] Job steps:
    1. Checkout code.
    2. Build & launch `paperpilot-gateway` on `localhost:8080`.
    3. Run `zaproxy/action-api-scan@v0.9.0` targeting `http://localhost:8080/api-docs/openapi.json`.
    4. Upload `report.sarif` to `github/codeql-action/upload-sarif`.

- [x] **Subtask 4: Local Smoke Test**
  - [x] Verify that `ApiDoc::openapi()` serializes cleanly without panics.
  - [x] Validate workflow YAML with `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/zap.yml'))"`.
