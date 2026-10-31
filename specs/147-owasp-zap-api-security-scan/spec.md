# Spec 147: Automated OWASP ZAP API Dynamic Security Gate (DAST)

## Metadata
- **Spec Number**: 147
- **Title**: Automated OWASP ZAP API Dynamic Security Gate (DAST)
- **Phase**: Phase 8.13 / Security Hardening & CI Compliance
- **Status**: ✅ Completed
- **Impacted Systems**: `.github/workflows/zap.yml`, `paperpilot-gateway`
- **License / Cost**: 100% Free & Open-Source (Apache-2.0 / OWASP)

---

## 1. Executive Summary & Problem Statement

While Static Application Security Testing (SAST via Semgrep) and Software Composition Analysis (SCA via `cargo-audit`) analyze source code and dependencies at rest, they cannot detect runtime vulnerabilities that emerge when HTTP endpoints handle untrusted client input.

This specification introduces **Dynamic Application Security Testing (DAST)** powered by the official, free **OWASP ZAP (Zed Attack Proxy)**:
1. **Automated OpenAPI Extraction**: Extract the complete OpenAPI 3.0 specification from `paperpilot-gateway`'s built-in `ApiDoc` (`utoipa`).
2. **Automated CI Dynamic Fuzzing**: Run `zaproxy/action-api-scan` on schedule and on pull requests against a live gateway instance.
3. **OWASP API Top 10 Coverage**: Automatically test for Broken Object Level Auth (BOLA), injection vulnerabilities, missing security headers, payload boundary overflow, and CORS misconfigurations.
4. **SARIF Integration**: Publish vulnerability findings directly to GitHub's Security Advisory tab.

---

## 2. Technical Architecture & CI Workflow

```
[ GitHub Actions Runner ]
        │
        ├──► 1. Build & Boot: cargo run -p paperpilot-gateway -- --port 8080 &
        │
        ├──► 2. Export / Fetch Spec: curl http://localhost:8080/api-docs/openapi.json
        │
        ├──► 3. OWASP ZAP API Scanner (zaproxy/action-api-scan)
        │      • Dynamic parameter fuzzing
        │      • SQL/Command injection checks
        │      • Header & CORS validation
        │      • Rate limit & buffer checks
        │
        └──► 4. SARIF Generation & Upload to GitHub Security Advisory Tab
```

---

## 3. Configuration & Rule Thresholds

### 3.1 Custom Rule Thresholds (`zap-rules.tsv`)
To prevent false alarms on benign developer documentation routes:
* `10096` (Timestamp Disclosure): IGNORE for `/health` and `/api-docs/*`.
* `10038` (Content Security Policy missing): WARN on raw JSON API routes.
* `40018` (SQL Injection): FAIL (Zero-tolerance).
* `40012` (Command Injection): FAIL (Zero-tolerance).
* `90022` (Application Error Disclosure): WARN.

---

## 4. Verification & Exit Criteria
1. Gateway boots in background on GitHub Actions runner.
2. ZAP API scan completes against all endpoints defined in `ApiDoc`.
3. Report generated in SARIF, Markdown, and HTML formats.
4. Clean run with zero high/critical alerts.
