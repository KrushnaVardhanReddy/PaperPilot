# Manual Desktop UI Testing Guide: Tester 2 (22 Optimization, Security & Cryptography Tools)

**Assigned Tester:** Tester 2  
**Total Tools Assigned:** 22 Tools  
**Domain:** File Size Compression, Integrity Repair, Cryptographic Security, Digital Signatures & Pixel Diffing

---

### Tool 23: `pdf_compress` — Compress PDF File Size
- **UI Access:** Operations Dock > **Optimize** > **Compress PDF**.
- **Test File:** High-resolution image PDF (> 2 MB).
- **Manual Steps:**
  1. Select preset: **eBook (150 DPI)**. Click **Compress PDF**.
- **Expected Result:** Toast reports percentage saved (e.g. *Saved 38%*). Output file opens cleanly.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 24: `pdf_dedup_streams` — Lossless Stream Deduplication
- **UI Access:** Operations Dock > **Optimize** > **Lossless Stream Deduplication**.
- **Test File:** Document with repeated company logos, background headers, or fonts.
- **Manual Steps:**
  1. Click **Deduplicate Streams**.
- **Expected Result:** Redundant image/font objects are merged into single indirect references; zero visual quality loss.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 25: `pdf_linearize` — Fast Web View (Linearize)
- **UI Access:** Operations Dock > **Optimize** > **Fast Web View (Linearize)**.
- **Test File:** Multi-page PDF.
- **Manual Steps:**
  1. Click **Linearize Document**.
- **Expected Result:** Output PDF is optimized for progressive byte-range downloads.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 26: `pdf_repair` — Rebuild Damaged PDF Syntax
- **UI Access:** Operations Dock > **Optimize** > **Repair PDF**.
- **Test File:** Truncated or corrupted PDF file.
- **Manual Steps:**
  1. Drag damaged file into tool. Click **Rebuild XREF Tables**.
- **Expected Result:** Corrupted dictionaries are reconstructed; document successfully renders.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 27: `pdf_invert_colors` — High-Contrast Dark Mode
- **UI Access:** Operations Dock > **Edit** > **Invert Colors (Dark Mode)**, or bottom status bar theme switch.
- **Test File:** Standard black-on-white document.
- **Manual Steps:**
  1. Select **High-Contrast Dark Mode**. Click **Invert Colors**.
- **Expected Result:** White backgrounds become dark charcoal; black text inverts to clear white.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 28: `pdf_render` — High-DPI Page Rasterization to PNG
- **UI Access:** Operations Dock > **Convert** > **Render Page** (or right-click canvas > *Export as PNG*).
- **Test File:** Any PDF page.
- **Manual Steps:**
  1. Set DPI: `300 DPI`. Select Page 1. Click **Render & Save PNG**.
- **Expected Result:** High-resolution `.png` file is created on disk with valid PNG header bytes.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 29: `pdf_compare` — Visual Pixel Diff Slider
- **UI Access:** Operations Dock > **Quick** > **Compare PDFs** (or *Tools > Visual Pixel Diff*).
- **Test File:** Two document revisions (v1 and v2).
- **Manual Steps:**
  1. Open Doc 1 in Tab 1, Doc 2 in Tab 2.
  2. Launch **Compare PDFs**. Drag the interactive center split curtain.
  3. Toggle **Color-Coded Highlights**.
- **Expected Result:** Real-time visual comparison reveals differences between revisions with color highlights.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 30: `pdf_encrypt` — Protect Document with AES-256 Password
- **UI Access:** Operations Dock > **Security** > **Encrypt with Password**.
- **Test File:** `tests/fixtures/simple.pdf`.
- **Manual Steps:**
  1. Enter User Password: `pass123`. Click **Encrypt & Save**.
  2. Reopen the saved document in PaperPilot.
- **Expected Result:** Unlock password modal appears blocking canvas. Entering `pass123` unlocks the document.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 31: `pdf_decrypt` — Remove Password Restrictions
- **UI Access:** Operations Dock > **Security** > **Remove Password**.
- **Test File:** Encrypted document (unlocked in viewer).
- **Manual Steps:**
  1. Click **Remove Password**. Enter current password to confirm.
- **Expected Result:** Document re-saves unencrypted; subsequent opens require no password.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 32: `pdf_sanitize` — Deep Enterprise Sanitization
- **UI Access:** Operations Dock > **Security** > **Sanitize Document**.
- **Test File:** Document with links, JavaScript, and metadata.
- **Manual Steps:**
  1. Check **Strip JavaScript**, **Remove Private Data**, **Clean Metadata**. Click **Sanitize**.
- **Expected Result:** Potentially malicious code and tracking dictionaries are stripped without breaking layout.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 33: `pdf_metadata` — Inspect & Clean Metadata
- **UI Access:** Operations Dock > **Security** > **Edit / Sanitize Metadata** (or *File > Properties*).
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Change Title to: `Verified Release 2026`. Click **Save Metadata**.
- **Expected Result:** Window title bar reflects the new title; creator and producer info update accordingly.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 34: `pdf_change_permissions` — Configure PDF Permissions
- **UI Access:** Operations Dock > **Security** > **Modify PDF Permissions**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Disable **Content Copying**. Enable **Form Filling**. Set owner password. Click **Apply**.
- **Expected Result:** Permissions enforce restrictions in compliant PDF viewers.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 35: `pdf_timestamp` — RFC 3161 TSA Timestamping
- **UI Access:** Operations Dock > **Security** > **Cryptographic Timestamp TSA**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Click **Apply Timestamp Token**.
- **Expected Result:** A cryptographically verified TSA timestamp is embedded into the document signature dictionary.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 36: `pdf_sign` — Digital Certificate & Visual Pen Signing
- **UI Access:** Operations Dock > **Security** > **Sign Document** (or toolbar Pen tool).
- **Test File:** Any contract or agreement PDF.
- **Manual Steps:**
  1. Draw visual signature or attach `.p12` digital certificate. Place signature on page 1.
  2. Click **Apply Signature**.
- **Expected Result:** Signature badge appears on page with certificate signer metadata.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 37: `pdf_validate_signatures` — Verify Signatures & Cert Chains
- **UI Access:** Operations Dock > **Security** > **Validate Digital Signatures**.
- **Test File:** Signed PDF document.
- **Manual Steps:**
  1. Open signed document. Click **Validate Signatures**.
- **Expected Result:** Drawer shows verification badge: **Valid Signature**, signer identity, and document integrity confirmation.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 38: `pdf_remove_signatures` — Strip Digital Signatures
- **UI Access:** Operations Dock > **Security** > **Remove Digital Signatures**.
- **Test File:** Signed PDF document.
- **Manual Steps:**
  1. Click **Strip Digital Signatures**.
- **Expected Result:** Signature fields and locks are removed, unlocking the document for modification.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 39: `pdf_hash` — Cryptographic Checksums
- **UI Access:** Operations Dock > **Security** > **Generate Hash**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Click **Generate Hash**.
- **Expected Result:** Drawer displays SHA-256 and MD5 checksum strings with a one-click copy button.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 40: `pdf_validate` — PDF Conformance & Syntax Validation
- **UI Access:** Operations Dock > **Security** > **Validate PDF**.
- **Test File:** Any PDF.
- **Manual Steps:**
  1. Click **Run Conformance Validation**.
- **Expected Result:** Reports validation status (e.g. *Structure Valid, PDF 1.7 compliant, 0 fatal errors*).
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 41: `document_pii_scan` — Detect SSNs, Credit Cards & PII
- **UI Access:** Operations Dock > **Security** > **PII Scanner**.
- **Test File:** Document containing mock PII (e.g., SSN `123-45-6789`, email `user@example.com`).
- **Manual Steps:**
  1. Click **Scan Document for PII**.
- **Expected Result:** Drawer lists detected sensitive entities with page numbers and confidence scores.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 42: `pdf_overlay_underlay` — Overlay & Underlay Stationary
- **UI Access:** Operations Dock > **Edit** > **Overlay & Underlay Stationary**.
- **Test File:** Content PDF and a separate letterhead template PDF.
- **Manual Steps:**
  1. Select letterhead PDF. Select **Underlay (Background)**. Click **Apply**.
- **Expected Result:** Letterhead artwork renders behind the document text across pages.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 43: `pdf_stamp` — Stamp Coordinate-Exact Images, Logos & Badges
- **UI Access:** Operations Dock > **Edit** > **Stamp Image & Logo**.
- **Test File:** Any PDF + a sample PNG badge.
- **Manual Steps:**
  1. Choose image. Set coordinates `X: 450, Y: 700`. Click **Place Stamp**.
- **Expected Result:** Badge renders at the exact designated position.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`

---

### Tool 44: `pdf_flatten` — Bake Annotations & Forms Permanently
- **UI Access:** Operations Dock > **Edit** > **Flatten Annotations**.
- **Test File:** Document containing fillable fields or annotations.
- **Manual Steps:**
  1. Click **Flatten Document**.
- **Expected Result:** Form inputs and sticky notes become non-interactive visual graphic elements on the page.
- **Sign-off:** `[ ] PASS` | `[ ] FAIL` | Notes: `____________________`
