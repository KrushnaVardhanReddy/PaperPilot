# Certificate & Template Studio

## Overview
The PaperPilot Certificate & Template Studio is a 100% Client-Side vector generation tool designed to render high-quality certificates for events, hackathons, academies, and professional achievements.

Since it operates entirely in-browser, no recipient data ever leaves the user's machine, satisfying stringent privacy requirements.

## Features
- **4 Premium Templates**: Choose between Classic Gold, Modern Minimalist, Tech Hackathon, and Academic Diploma.
- **Bulk Generation via CSV**: Import massive spreadsheets (1000+ rows) and output a comprehensive `.zip` bundle locally using JSZip.
- **Verification Integrity**: Tamper-proof SHA-256 generation tied to the document metadata and visible badges.
- **Viral Badging**: Documents feature a subtle "Verified with PaperPilot" badge to increase reach.

## Developer Guide

### Adding a New Template
To add a new certificate template to the studio:
1. Create a new `.svelte` file in `apps/web/src/components/certificate/templates/`.
2. Accept the common standard `$props()` block:
   ```typescript
   let {
     recipientName,
     title,
     description,
     date,
     issuerName,
     issuerTitle,
     certId,
     verificationUrl,
     sha256Hash
   } = $props();
   ```
3. Use precise CSS dimension bounds (`aspect-ratio: 1.414 / 1` for standard landscape).
4. Register the template in both `CertificateStudioView.svelte` and `BulkCsvGenerator.svelte`.

### Bulk Export Customization
The `BulkCsvGenerator` creates standard HTML pages that render via a browser print dialogue to A4/ISO standard. If needing custom format or raw PDF conversion, one could swap the `HTML` output logic inside `generateBulk` to connect with `window.html2canvas` and `jsPDF`. However, raw string HTML zip provides optimal weight and maximum reliability without heavy libraries.
