# Spec 023: PaperPilot Certificate & Template Studio

## Status: PLANNED (Phase 6 / Growth Feature)

## 1. Executive Summary & Market Motivation
Certificate and structured award generation is a major recurring use case for educators, hackathon organizers, bootcamps, and enterprise HR departments. Existing commercial tools (Certifier, Accredible, Canva Pro) impose steep subscriptions ($39–$150/mo), route sensitive attendee rosters to cloud servers, and throttle bulk rendering.

PaperPilot solves this by offering a **100% Client-Side / Zero-Cloud Certificate Studio**:
1. **Pre-built Premium Templates**: Elegant vector designs (Classic Gold, Modern Minimalist, Tech Hackathon, Academic Diploma) rendered locally.
2. **Instant Single & Batch CSV Generation**: Users can enter a single recipient or upload a CSV spreadsheet with 1,000+ rows to generate an encrypted `.zip` of personalized PDFs in seconds.
3. **Tamper-Proof Verification**: Optional SHA-256 integrity hash and dynamic QR verification badge stamped on the document.
4. **Growth Flywheel**: Free generated certificates feature an unobtrusive, elegant footer badge (*"Verified with PaperPilot"*), driving massive inbound referral traffic to `usepaperpilot.com`.

---

## 2. Technical Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. TEMPLATE ENGINE (SVG / Vector PDF Base)                                   │
│    • Parametric placeholders: {{recipient_name}}, {{title}}, {{date}},      │
│      {{issuer_name}}, {{cert_id}}, {{qr_code}}                              │
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. DATA INTAKE LAYER                                                        │
│    • Interactive Form (Single recipient test & tweak)                       │
│    • CSV File Parser (Bulk upload with client-side column mapping)          │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. PURE-RUST GENERATOR PIPELINE (`paperpilot-wasm` & `paperpilot-pdf`)       │
│    • Text & Glyph Rendering: Crisp typography via Rust font engine          │
│    • Watermark & Border Stamping: High-resolution vector borders            │
│    • Optional QR Code & Hash: SHA-256 hash stamped for authenticity         │
│    • Parallel Batch Runner: Web Workers / Rayon parallel rendering          │
├─────────────────────────────────────────────────────────────────────────────┤
│ 4. OUTPUT PACKAGING                                                         │
│    • Single: Instant direct PDF download (<15ms)                            │
│    • Batch: In-memory JSZip packaging into a single `.zip` archive          │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Distribution Surfaces

1. **Web Portal (`usepaperpilot.com/tools/certificate-generator`)**:
   - High-ranking SEO landing page capturing organic search traffic for *"free certificate generator without signup"*.
   - Runs 100% in browser memory via `paperpilot-wasm`.
2. **Desktop App (`apps/desktop`)**:
   - Power user studio for offline high-volume batch generation from CSV spreadsheets.
3. **Embed Widget (`embed.js`)**:
   - Drop-in certificate claim widget for EdTech platforms, LMS (Moodle, LearnDash), and event sites.

---

## 4. Phased Milestones
- **6.1.1 Core Engine**: Vector certificate rendering and dynamic variable replacement in `paperpilot-pdf`.
- **6.1.2 Web & Desktop UI**: Svelte 5 Template Studio with live preview and typography controls.
- **6.1.3 CSV Batch Processing**: Client-side worker queue and in-memory zip bundler.
- **6.1.4 Verification & Viral Loop**: Verifiable QR code linking to `usepaperpilot.com/verify`.
