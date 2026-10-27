# Spec 133: OpenSSF Scorecard Supply Chain Hardening & Security Compliance

## 1. Overview & Problem Statement
Running the official OpenSSF Scorecard tool locally against `KrushnaVardhanReddy/PaperPilot` yielded an initial baseline score of **3.7 / 10**.

While high-rigor controls like **CI-Tests (10/10)**, **SAST Gate (10/10)**, **Dangerous-Workflow (10/10)**, and **Dependency-Update-Tool (10/10)** passed cleanly, several core compliance categories were penalized due to missing security metadata and over-permissive default GitHub token access:
1. **Security-Policy (0/10)**: Lack of a dedicated `SECURITY.md` detailing supported versions, vulnerability disclosure procedures, and private disclosure contact.
2. **Token-Permissions (0/10)**: Workflows either omit top-level `permissions` (inheriting write capabilities by default on GitHub) or define global write permissions rather than job-scoped least privilege (`read-all` default).
3. **Automated Scorecard Monitoring**: No scheduled workflow running Scorecard with SARIF upload to GitHub Security Advisory tab.

## 2. Requirements & Objectives
1. **Security Policy (`SECURITY.md`)**:
   - Create root `SECURITY.md` compliant with OpenSSF standards and RFC 9116 security conventions.
   - Specify supported versions, vulnerability reporting channels via GitHub Private Vulnerability Reporting, response SLA (<48h), and zero-day patch policy.
2. **Least Privilege Token Permissions (`Token-Permissions: 10/10`)**:
   - Add explicit top-level `permissions: read-all` to all GitHub Action workflows (`.github/workflows/*.yml`).
   - Granularly elevate permissions only on the specific jobs that require them (e.g. `security-events: write` for SARIF uploads, `contents: write` for release publications).
3. **Automated Scorecard CI Workflow (`.github/workflows/scorecard.yml`)**:
   - Add an official OpenSSF Scorecard GitHub Action running on a weekly cron and on pushes to `main`.
   - Publish SARIF results to GitHub Security tab and render compliance badge in `README.md`.
4. **Local Verification**:
   - Run `/tmp/scorecard` locally with GitHub authentication to confirm score improvement.
