# Security Architecture: Dynamic & Static Analysis, Leak Scanning, and Container Hardening

## 1. Overview
PaperPilot implements defense-in-depth security spanning static analysis, runtime penetration testing, secret scanning, and automated dependency audits.

```
                      ┌──────────────────────────────────────┐
                      │          Developer Workspace         │
                      │  • Local Secrets Scan (TruffleHog)   │
                      │  • Local DAST (OWASP ZAP)            │
                      │  • Static Linters & Cargo Audit      │
                      └──────────────────┬───────────────────┘
                                         │ git push
                                         ▼
                      ┌──────────────────────────────────────┐
                      │       GitHub Actions CI Gates        │
                      ├──────────────────┬───────────────────┤
                      │ SAST (Semgrep)   │ DAST (OWASP ZAP)  │
                      │ Secrets Scan     │ Cargo Audit (SCA) │
                      │ Scorecard SSF    │ Dependency Pin    │
                      └──────────────────┴───────────────────┘
```

---

## 2. Dynamic Application Security Testing (DAST via OWASP ZAP)
- **Spec Reference**: [Spec 147: Automated OWASP ZAP API Dynamic Security Gate](file:///home/krushna/Project/PaperPilot/specs/147-owasp-zap-api-security-scan/spec.md)
- **CI Workflow**: `.github/workflows/zap.yml`
- **Local Runner**: `scripts/run_zap_local.sh` (`make zap-scan-local`)

### Execution Model
1. The REST gateway (`paperpilot-gateway`) is built and booted on `localhost:8080`.
2. OpenAPI 3.0 specification is fetched dynamically from `/api-docs/openapi.json`.
3. The official OWASP ZAP API scanner (`zaproxy/action-api-scan`) attacks the gateway endpoints with fuzzing vectors, parameter boundary tests, header validation, and injection payloads.
4. Security findings are converted to SARIF format and uploaded directly to GitHub Security Advisories.

---

## 3. Secret and Credential Leak Auditing (TruffleHog OSS)
- **Spec Reference**: [Spec 173: Automated Secret and Credential Leak Auditing](file:///home/krushna/Project/PaperPilot/specs/173-automated-trufflehog-secrets-scanning/spec.md)
- **CI Workflow**: `.github/workflows/security.yml`
- **Local Runner**: `scripts/run_secrets_scan.sh` (`make trufflehog-scan` / `make secret-scan-local`)

### Key Controls
- **Verified Credentials Only (`--only-verified`)**: Eliminates noise from random hashes, mock test strings, and binary sequences by verifying suspected secrets against provider endpoints.
- **Main Branch Protection**: Runs full repository and branch diff scanning on pull requests and commits targeting `main`.
- **Zero-Setup Bootstrapping**: Automatically detects host architecture (x86_64/arm64) and provisions the verified `trufflehog` binary on demand.

---

## 4. Static Analysis (SAST) & Supply Chain Security
- **Semgrep SAST**: Automated static analysis rules scanning for unsafe blocks, command injection vectors, and deserialization vulnerabilities.
- **Cargo Audit & Supply Chain SHA Pinning**: Strict dependency verification against RustSec advisory database, with GitHub Action action commits pinned to immutable commit SHAs.
