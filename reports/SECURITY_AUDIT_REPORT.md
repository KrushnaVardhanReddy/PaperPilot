# Comprehensive Security Audit Report

**Date:** October 26, 2026  
**Auditors & Tools:** Semgrep OSS (v1.182.1), Trivy (v0.76.1), Cargo-Audit (v0.22.2), and Hacker News Community Analysis  
**Repository Scanned:** PaperPilot (1,226 git-tracked files)  
**Status:** Remediations Spec & Task Created (`Spec 123` / `Task 123A`)

---

## 1. Executive Summary

A full multi-scanner security review was conducted across the PaperPilot codebase:
- **Semgrep SAST:** Scanned 1,226 files against 535 rules. Found **36 total findings** (28 CI/CD action hygiene, 8 code/docker misconfigurations).
- **Trivy Scanner:** Detected **1 container misconfiguration** (`Dockerfile` root user) and **2 frontend dependencies** with known CVEs (`vite`).
- **Cargo Audit:** Identified outdated transitive dependencies (`zstd-safe`, `tokio`).
- **Hacker News Community Report:** Confirmed 4 critical/high architectural vulnerabilities around gateway CORS/auth, desktop XSS, pack path traversal, and SSRF.

---

## 2. Detailed Findings by Category

### Category A: Desktop Application & Client-Side Security (CRITICAL / HIGH)

#### A.1 Stored XSS via Document Search Snippets
- **Location:** `apps/desktop/src/lib/services/workspaceSearch.ts:84` & `WorkspaceSearchModal.svelte:297`
- **Vulnerability:** Search snippet assembly concatenates document text directly without HTML escaping into `<mark>` tags:
  ```typescript
  const snippet = `${prefix}<mark>${matchedText}</mark>${suffix}`;
  ```
  `WorkspaceSearchModal.svelte` then renders it via `{@html match.snippet}`.
- **Impact:** A malicious PDF or DOCX containing `<img src=x onerror=...>` will execute arbitrary JavaScript inside the Tauri webview when searched.
- **Aggravating Factor:** In `apps/desktop/src-tauri/tauri.conf.json`, `"csp": null` and `assetProtocol.scope: ["**"]`, meaning executing script can read any file on disk.
- **Fix:**
  1. Escape HTML entities in `prefix`, `matchedText`, and `suffix` in `workspaceSearch.ts`.
  2. Add strict Content Security Policy (CSP) in `tauri.conf.json`.

#### A.2 Unsanitized Path Traversal in Feature Pack Downloader
- **Location:** `apps/desktop/src-tauri/src/pack_commands.rs:145`
- **Vulnerability:** `pack_id` from the client is directly appended to the target path:
  ```rust
  dest_path.push(format!("{}.pack", pack_id));
  ```
- **Impact:** An input like `../../etc/cron.d/malicious` can write files outside the application data folder.
- **Fix:** Enforce alphanumeric regex whitelist `^[a-zA-Z0-9_-]+$` on `pack_id`.

---

### Category B: REST Gateway & Server Security (CRITICAL / HIGH)

#### B.1 Wildcard CORS on Local REST Gateway
- **Location:** `paperpilot-gateway/src/server.rs:97-100`
- **Vulnerability:** Overly permissive CORS configuration:
  ```rust
  let cors = CorsLayer::new()
      .allow_origin(Any)
      .allow_methods(Any)
      .allow_headers(Any);
  ```
- **Impact:** Any web page visited by the user in Chrome or Firefox can send cross-origin `POST http://localhost:3000/api/v1/pdf/...` requests without authentication to read, modify, or overwrite local files.
- **Fix:**
  1. Restrict CORS origins to trusted desktop clients (`tauri://localhost`, `http://localhost:5173`).
  2. Implement local session Bearer token authentication (`Authorization: Bearer <TOKEN>`) generated on gateway startup.
  3. Add request body size limit: `DefaultBodyLimit::max(100 * 1024 * 1024)` to prevent memory exhaustion DoS.

---

### Category C: Core Engine & Network SSRF (MEDIUM)

#### C.1 Bypassable SSRF & Unchecked Redirects
- **Location:** `paperpilot-core/src/url_fetch.rs:24-42`
- **Vulnerability:**
  - `resolve_url_to_bytes` only parses the host string before DNS resolution.
  - It does not check IPv6 loopback (`[::1]`), CGNAT (`100.64.0.0/10`), link-local (`169.254.169.254`), or `0.0.0.0`.
  - Reqwest automatically follows HTTP 302 redirects, allowing a public URL to redirect into private internal cloud metadata or LAN services.
- **Fix:**
  1. Set `redirect(reqwest::redirect::Policy::none())`.
  2. Perform explicit DNS resolution and validate all resolved IP addresses against loopback, private, and cloud metadata CIDR blocks.

---

### Category D: Container & CI Supply Chain (MEDIUM / LOW)

#### D.1 Container Root User
- **Location:** `Dockerfile:82`
- **Finding (Trivy DS-0002):** Container runs as `root` by default.
- **Fix:** Add `RUN useradd -m paperpilot` and `USER paperpilot` directive.

#### D.2 GitHub Actions Pinned Hashes & `curl | sh`
- **Location:** `.github/workflows/release.yml`
- **Finding (Semgrep):** Actions use mutable tags (`@v4`, `@v2`) instead of commit SHAs, and `curl | sh` is used to install `wasm-pack`.
- **Fix:** Pin actions and install `wasm-pack` via cargo binary or release asset verification.

---

## 3. Action Plan & Tracking
All above items are tracked under:
- **Specification:** [`specs/123-production-security-hardening/spec.md`](file:///home/krushna/Project/PaperPilot/specs/123-production-security-hardening/spec.md)
- **Task:** [`specs/123-production-security-hardening/task_123a.md`](file:///home/krushna/Project/PaperPilot/specs/123-production-security-hardening/task_123a.md)
- **Phases:** Tracked as **Task 5.9.92** in [`PHASES.md`](file:///home/krushna/Project/PaperPilot/PHASES.md).
