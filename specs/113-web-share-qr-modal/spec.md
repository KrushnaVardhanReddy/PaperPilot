# Spec 113: Web Portal Instant Mobile Handoff & Share Modal (`<ShareModal />`)

## 1. Overview
Enables instant desktop-to-mobile handoff and social/colleague sharing in the PaperPilot Web portal (`apps/web`). Users frequently need to transfer current document workflows to mobile (e.g., to capture physical scans with phone cameras or sign with touchscreens) or share the 100% private zero-upload tool suite.

---

## 2. Technical Architecture

### 2.1 Component Design (`apps/web/src/components/ShareModal.svelte`)
- **Trigger**: Sleek glassmorphic button in the top navigation header (`🔗 Share`).
- **Features**:
  1. **Dynamic SVG QR Code**: Pure client-side SVG QR code generator (using standard lightweight QR matrix algorithm or lightweight helper) pointing to current `window.location.href`.
  2. **One-Click Link Copy**: Copies canonical/current URL with visual confirmation tooltip (`✓ Copied!`).
  3. **Web Share API (`navigator.share`)**: Automatically activates native OS sharing sheet on mobile and supported desktop browsers (AirDrop, WhatsApp, Slack, Messages).
  4. **Privacy Badge**: Micro-banner reminding users: *"Documents are processed 100% locally in your browser — zero files are uploaded."*

### 2.2 Integration Points
- `apps/web/src/App.svelte`: Mount `<ShareModal />` in the top right header navigation alongside the Desktop CTA.
- Style theme: Sleek dark glassmorphism adhering to PaperPilot Web design tokens.

---

## 3. Verification & Acceptance Criteria
1. Clicking "Share" opens the modal with a valid, readable QR code.
2. "Copy Link" copies URL to clipboard with reactive feedback.
3. Native share triggers when `navigator.share` is available.
4. Clean closing on backdrop click or `Escape` key.
