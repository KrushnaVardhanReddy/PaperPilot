# Security Policy

PaperPilot takes the security and integrity of its privacy-first, on-device document processing architecture with the utmost seriousness. Because PaperPilot processes sensitive, proprietary, and personal documentation (legal agreements, medical records, financial statements, and confidential filings) locally on user workstations, security is a foundational property of our codebase.

---

## 1. Supported Versions

We provide security updates and patches for the following release branches:

| Version | Supported | Security Maintenance |
| :--- | :---: | :--- |
| `0.2.x` (Current Dev / Nightly) | ✅ | Continuous active patching on `feature/dev` and `main` |
| `0.1.x` (Legacy Stable) | ✅ | Critical CVEs and supply-chain vulnerability fixes |
| `< 0.1.0` | ❌ | End of Life (Please upgrade to current version) |

---

## 2. Reporting a Vulnerability

**Please do NOT file public GitHub issues for security vulnerabilities.**

To report a vulnerability responsibly:
1. **GitHub Private Vulnerability Reporting (Preferred)**:
   Navigate to the [PaperPilot Security Advisories](https://github.com/KrushnaVardhanReddy/PaperPilot/security/advisories/new) page and submit a private report with reproduction steps or proof-of-concept files.
2. **Direct Contact**:
   If you are unable to use GitHub Advisories, contact the core maintainer directly via GitHub profile contact options.

### What to Include in Your Report
To accelerate triage and resolution, please provide:
- Type of vulnerability (e.g. malformed PDF buffer overflow, path traversal, stored XSS, privilege escalation, or CI supply chain issue).
- Detailed step-by-step reproduction instructions or a minimal reproducible PDF/script sample.
- Affected surfaces (CLI, MCP Server, REST Gateway, WASM Web Sandbox, or Desktop Tauri application).
- Any proposed patch or mitigation if available.

---

## 3. Vulnerability Response Process & SLA

Once a report is submitted, the PaperPilot team follows a strict remediation workflow:

1. **Initial Acknowledgment**: Within **48 hours**, we will acknowledge receipt and assign a triage priority.
2. **Assessment & Triage**: We analyze the impact across all 5 runtime interfaces (Core, CLI, MCP, Gateway, WASM).
3. **Remediation & Testing**: A fix is authored on a private fork and tested against our isolated clean-room test harness and fuzzing regressions (`tests/malformed_regression.rs`).
4. **Coordinated Disclosure**: We coordinate a mutual disclosure timeline (typically 30–90 days, depending on severity) and publish a CVE / GitHub Security Advisory with attribution to the finder.

---

## 4. Security Guarantees & Architecture Guardrails

PaperPilot enforces the following automated security gates across every pull request and release:
- **Zero Host Telemetry / 100% On-Device Processing**: Document streams never leave local workstation RAM unless explicitly directed by user configuration.
- **Automated SAST & Secret Gate**: Every commit is audited with Semgrep OSS and Trivy.
- **Memory Safety**: Pure-Rust execution avoiding unsafe C/C++ memory management vulnerabilities.
- **Supply Chain Integrity**: Immutable 40-character commit SHA pinning on all CI actions, weekly automated Dependabot audits, and zero unverified remote installer scripts.
