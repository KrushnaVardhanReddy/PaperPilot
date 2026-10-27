# Task 123A: Production Security Hardening & Vulnerability Remediation

## Status
Ready for Implementation.

## Description
Execute comprehensive security hardening across Desktop, Gateway, and Core to eliminate vulnerabilities identified in Semgrep/Trivy scans and community feedback.

## Deliverables
1. **Desktop Client-Side XSS Protection**:
   - `apps/desktop/src/lib/services/workspaceSearch.ts`: Escape HTML entities in search snippets before highlighting.
   - `apps/desktop/src-tauri/tauri.conf.json`: Configure strict CSP header.
   - `apps/desktop/src/lib/services/workspaceSearch.test.ts`: Add automated unit test asserting malicious `<script>` and `<img onerror=...>` payloads are escaped.
2. **REST Gateway Authorization & CORS Hardening**:
   - `paperpilot-gateway/src/server.rs`: Replace wildcard CORS with trusted origins (`tauri://localhost`, `http://localhost:5173`), add `DefaultBodyLimit(100MB)`, and add bearer auth check.
3. **Path Traversal Protection**:
   - `apps/desktop/src-tauri/src/pack_commands.rs`: Enforce strict alphanumeric regex whitelist on `pack_id`.
4. **SSRF Hardening**:
   - `paperpilot-core/src/url_fetch.rs`: Disable automatic redirects and add comprehensive private/link-local/loopback IP validation.
5. **Dockerfile Security**:
   - `Dockerfile`: Add `USER paperpilot` non-root execution.
