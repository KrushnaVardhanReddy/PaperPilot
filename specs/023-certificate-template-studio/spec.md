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
│ 1. TEMPLATE ENGINE (HTML5 / CSS3 / SVG Component Architecture)              │
│    • Parametric placeholders: {{recipient_name}}, {{title}}, {{date}},      │
│      {{issuer_name}}, {{cert_id}}, {{qr_code}}                              │
│    • Modern typography & CSS layout: Flexbox/Grid, Gold Foil Gradients,     │
│      decorative SVG borders, Google Fonts. 0ms instant DOM preview.         │
├─────────────────────────────────────────────────────────────────────────────┤
│ 2. DATA INTAKE LAYER                                                        │
│    • Interactive Svelte 5 Form (Single recipient live testing & tweaking)   │
│    • CSV File Parser (Bulk upload with client-side column mapping)          │
├─────────────────────────────────────────────────────────────────────────────┤
│ 3. 100% CLIENT-SIDE GENERATION PIPELINE                                     │
│    • Single: Instant high-DPI (300 DPI) PDF export or browser vector print  │
│    • Batch Runner: In-browser Worker loop applying HTML/SVG templates to    │
│      each CSV row without hitting any server or uploading data.             │
│    • Authenticity & Tamper-Proofing: SHA-256 integrity stamp + QR link      │
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
- **6.1.1 Template Designs & Schema**: 4 responsive HTML5/SVG certificate templates with Google Fonts typography & CSS foil styling.
- **6.1.2 Interactive Studio View**: Svelte 5 live preview studio with realtime property binding (recipient, course, signatures, date).
- **6.1.3 Client-Side PDF Export & Tamper Verification**: High-DPI client-side PDF rendering, print vector stylesheet, and SHA-256 integrity hash stamp.
- **6.1.4 Bulk CSV Engine & ZIP Bundler**: In-browser CSV parse, row mapping, progress bar, and JSZip single-archive download.
