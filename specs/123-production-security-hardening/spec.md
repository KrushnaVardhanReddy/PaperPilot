# Spec 123: Production Security Hardening & Vulnerability Remediation

## 1. Overview & Objective
Following security audits (Semgrep SAST, Trivy, and community Hacker News analysis), this specification establishes production security controls across the PaperPilot stack:
1. **Desktop Client-Side XSS Protection**: Prevent malicious document payloads (e.g. `<img onerror=...>`) from executing code when rendering document search snippets in `WorkspaceSearchModal.svelte`.
2. **REST Gateway Authorization & CORS Hardening**: Restrict `paperpilot-gateway` CORS policy, require local bearer token authentication, and add request body size limits (`DefaultBodyLimit`).
3. **Path Traversal Elimination in Feature Packs**: Enforce strict alphanumeric whitelist validation on `pack_id` in `pack_commands.rs`.
4. **SSRF Hardening**: Disable automatic HTTP redirect following and validate resolved IP addresses (preventing loopback `[::1]`, CGNAT `100.64.0.0/10`, and link-local access).
5. **Container & CI Supply Chain Hardening**: Run Docker container as non-root user and enforce locked lockfiles in CI.

---

## 2. Requirements & Deliverables

### Requirement 1: Stored XSS Remediation & Tauri CSP (`apps/desktop`)
- In `apps/desktop/src/lib/services/workspaceSearch.ts`:
  - Sanitize search snippets by escaping HTML characters (`&`, `<`, `>`, `"`, `'`) in `prefix`, `matchedText`, and `suffix` before assembling `<mark>`.
- In `apps/desktop/src-tauri/tauri.conf.json`:
  - Define strict Content Security Policy (CSP):
    ```json
    "security": {
      "csp": "default-src 'self' tauri:; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' asset: blob: data:; connect-src 'self' http://localhost:* ws://localhost:*;"
    }
    ```

### Requirement 2: Gateway Authentication & CORS Lockdown (`paperpilot-gateway`)
- In `paperpilot-gateway/src/server.rs`:
  - Replace `allow_origin(Any)` with restricted origins (e.g. `tauri://localhost`, `http://localhost:5173`, `http://127.0.0.1:*`) or explicitly allow only trusted localhost clients.
  - Add request body size limiter: `axum::extract::DefaultBodyLimit::max(100 * 1024 * 1024)` (100MB max per request).
  - Add optional/configurable Bearer authentication middleware (`Authorization: Bearer <TOKEN>`), requiring a shared secret or auto-generated local session token for `/api/v1/pdf/*` execution endpoints.

### Requirement 3: Path Traversal Whitelist (`apps/desktop/src-tauri`)
- In `apps/desktop/src-tauri/src/pack_commands.rs`:
  - Validate that `pack_id` matches `^[a-zA-Z0-9_-]+$`.
  - Reject any path containing `..`, `/`, or `\`.

### Requirement 4: SSRF DNS-Resolution & Redirect Lockdown (`paperpilot-core`)
- In `paperpilot-core/src/url_fetch.rs`:
  - Configure `reqwest::Client` with `redirect(reqwest::redirect::Policy::none())`.
  - Validate that the parsed IP address is not loopback, private, link-local, or CGNAT (`100.64.0.0/10`, `169.254.169.254`, `::1`, `0.0.0.0`).

### Requirement 5: Container Non-Root User (`Dockerfile`)
- In `Dockerfile`:
  - Create dedicated non-root user `paperpilot` and add `USER paperpilot` directive.

---

## 3. Success Criteria
- [ ] Stored XSS in document search is completely neutralized with automated tests in `workspaceSearch.test.ts`.
- [ ] Gateway rejects oversized requests (>100MB) and prevents arbitrary cross-site web origins from executing operations.
- [ ] Pack downloads reject path traversal inputs with error.
- [ ] URL fetcher rejects redirects to internal/private IP ranges.
- [ ] Semgrep and Trivy scans report 0 High or Critical vulnerabilities.
