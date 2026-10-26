# Spec 125: Single-File Offline Self-Contained HTML Sandbox

## 1. Overview & Problem Statement
Privacy-conscious users in regulated or air-gapped environments (legal, healthcare, defence) want a standalone `.html` file that embeds the entire WASM runtime and web UI inline with zero external network requests and zero CDN dependencies (prompted by user `spwa4` on Hacker News).

## 2. Requirements & Objectives
1. **Self-Contained Bundle Generation**:
   - Create a dedicated build step using `vite-plugin-singlefile` or inline bundling script in `apps/embed`.
2. **Inline WASM & Assets**:
   - Base64 encode `paperpilot_wasm_bg.wasm` and inline it into the HTML bundle.
   - Inline all CSS, SVGs, and JS chunks directly into one `paperpilot-portable.html` file.
3. **Air-Gap Verification**:
   - Disconnect network or run behind strict `Content-Security-Policy: default-src 'self' 'unsafe-inline' 'unsafe-eval' data: blob:` to prove 0 bytes leave the machine.
4. **Distribution**:
   - Publish `paperpilot-portable.html` as a downloadable artifact in GitHub releases and docs.
