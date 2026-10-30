# Manual Desktop UI Testing Guide: Tester 3 (Security, Digital Signatures & Forms)

**Assigned Tester:** Tester 3  
**Total Tools Assigned:** 13 Tools  
**Domain:** Document Security, Cryptography, PDF Permissions, Timestamping, AcroForms & Interactive Form Design

---

## Tool 1: Encrypt with Password (`encrypt`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Encrypt with Password** (or lock icon in top bar).
- **Sample File:** `tests/fixtures/simple.pdf`.
- **Manual Steps:**
  1. In the tool panel, enter a User Password: `secret123`.
  2. Enter an optional Owner Password: `admin456`.
  3. Select encryption level: **AES-256** (default).
  4. Click **Encrypt & Save**.
- **Visual Acceptance Criteria:**
  - [ ] Close the document and attempt to reopen it in the Desktop Viewer.
  - [ ] A password unlock modal appears blocking document rendering.
  - [ ] Entering `secret123` unlocks and renders the document properly.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 2: Remove Password / Decrypt (`decrypt`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Remove Password**.
- **Sample File:** An encrypted PDF document (unlocked in the viewer).
- **Manual Steps:**
  1. With the unlocked document active, open **Remove Password**.
  2. Enter current password to confirm authorization.
  3. Click **Decrypt Document**.
- **Visual Acceptance Criteria:**
  - [ ] Saved document reopens in any PDF viewer without asking for a password.
  - [ ] Lock icon status updates to unlocked.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 3: Edit / Sanitize Metadata (`metadata`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Edit / Sanitize Metadata** (or *File > Document Properties*).
- **Sample File:** Any PDF.
- **Manual Steps:**
  1. Inspect existing Title, Author, Subject, Creator, and Producer fields.
  2. Change Title to: `Confidential Briefing 2026`.
  3. Click **Wipe All Metadata (Clean)** or **Save Metadata Changes**.
- **Visual Acceptance Criteria:**
  - [ ] Document title bar updates to the new Title.
  - [ ] Reopening properties verifies metadata fields are either sanitized or updated as specified.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 4: Deep Sanitize Document (`pdf_sanitize`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Sanitize Document**.
- **Sample File:** Document containing embedded links, JavaScript, and metadata.
- **Manual Steps:**
  1. Review checkboxes: **Strip JavaScript Actions**, **Remove Embedded Files**, **Clean Metadata**, **Purge Private Application Data**.
  2. Click **Deep Sanitize**.
- **Visual Acceptance Criteria:**
  - [ ] Toast confirms: *"Sanitized document: removed scripts, private dictionaries, and metadata"*.
  - [ ] Document visually looks unchanged, but dangerous payload dictionaries are purged.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 5: Redact Text (`redact`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Redact Text** (or toolbar Redaction tool).
- **Sample File:** Any document containing sensitive text (names, account numbers).
- **Manual Steps:**
  1. In the Redaction tool, drag a box over sensitive text on the canvas (or enter search term to redact, e.g. `"CONFIDENTIAL"`).
  2. Choose redaction overlay style: **Solid Black** or **Custom Text (e.g., "[REDACTED]")**.
  3. Click **Apply Redactions**.
- **Visual Acceptance Criteria:**
  - [ ] The text is permanently blacked out.
  - [ ] Select-all and copy-paste over the redacted area confirms the underlying text is eradicated, not merely masked.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 6: Sign Document (`sign`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Sign Document** (or toolbar Pen icon).
- **Sample File:** Any PDF agreement.
- **Manual Steps:**
  1. Choose signature mode: **Digital Certificate (PKCS#12 / .pfx)** or **Visual Signature Stamp (Drawn/Typed)**.
  2. If visual, draw signature on canvas; if digital, select `.p12`/`.pfx` certificate.
  3. Click **Apply Signature**.
- **Visual Acceptance Criteria:**
  - [ ] Signature badge appears on the target page.
  - [ ] Signature properties show signer identity and timestamp.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 7: Validate Digital Signatures (`pdf_validate_signatures`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Validate Digital Signatures**.
- **Sample File:** A signed PDF document.
- **Manual Steps:**
  1. Open signed document.
  2. Click **Validate Signatures**.
- **Visual Acceptance Criteria:**
  - [ ] Signature status panel displays: **Signature Valid**, **Signer Name**, **Certificate Chain**, and **Integrity Check (Document has not been modified since signed)**.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 8: Remove Digital Signatures (`pdf_remove_signatures`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Remove Digital Signatures**.
- **Sample File:** Signed PDF document.
- **Manual Steps:**
  1. Click **Strip Digital Signatures**.
  2. Confirm the action in the prompt.
- **Visual Acceptance Criteria:**
  - [ ] Cryptographic locks and signature widget annotations are removed.
  - [ ] Document becomes editable again.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 9: Modify PDF Permissions (`pdf_change_permissions`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Modify PDF Permissions**.
- **Sample File:** Any PDF.
- **Manual Steps:**
  1. Configure permission checkboxes:
     - Allow Printing: `High Resolution`
     - Allow Content Copying: `Disabled`
     - Allow Form Filling: `Enabled`
     - Allow Page Modifications: `Disabled`
  2. Set Owner Password.
  3. Click **Apply Permissions**.
- **Visual Acceptance Criteria:**
  - [ ] Document enforces restrictions when opened in viewer (e.g., text selection/copy is restricted).
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 10: Cryptographic Timestamp TSA (`pdf_timestamp`)
- **Category in UI:** `Security & Privacy`
- **UI Access:** Operations Dock > **Security** > **Cryptographic Timestamp TSA**.
- **Sample File:** Any PDF document.
- **Manual Steps:**
  1. Enter RFC 3161 TSA server URL (or leave default PaperPilot embedded TSA).
  2. Click **Apply Timestamp Token**.
- **Visual Acceptance Criteria:**
  - [ ] Document metadata embeds a cryptographically verifiable timestamp token with millisecond accuracy.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 11: Read Form Data (`read_form`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Read Form Data** (or form mode toggle).
- **Sample File:** `tests/fixtures/sample_form.pdf` (or any PDF containing fillable fields).
- **Manual Steps:**
  1. Open the form document in the viewer.
  2. Click **Read Form Fields**.
- **Visual Acceptance Criteria:**
  - [ ] A structured JSON preview drawer opens listing all form field names, field types (text, checkbox, dropdown), and current values.
  - [ ] Allows clicking **Export JSON**.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 12: Fill Form Data (`fill_form`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Fill Form Data**, or direct on-canvas editing.
- **Sample File:** `tests/fixtures/sample_form.pdf`.
- **Manual Steps:**
  1. Click directly into an interactive form field on the canvas (e.g. *Full Name*).
  2. Type: `Johnathan Doe`.
  3. Check a checkbox field.
  4. Or click **Import JSON** in the tool panel to auto-fill multiple fields from a JSON mapping.
- **Visual Acceptance Criteria:**
  - [ ] The text renders cleanly within the form field box.
  - [ ] Values persist when switching pages and saving the document.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`

---

## Tool 13: Add Form Field (`create_form_field`)
- **Category in UI:** `Edit & Markup`
- **UI Access:** Operations Dock > **Edit** > **Add Form Field** (or form builder toolbar).
- **Sample File:** Blank or static PDF document without existing form fields.
- **Manual Steps:**
  1. Select field type: **Text Field**, **Checkbox**, or **Dropdown**.
  2. Enter field name: `user_signature_field`.
  3. Drag a bounding rectangle on the canvas where the field should be placed.
  4. Click **Create Field**.
- **Visual Acceptance Criteria:**
  - [ ] An interactive blue-highlighted form field is inserted on the canvas.
  - [ ] Clicking into the new field allows typing or checking immediately.
- **Status:** `[ ] PASS` | `[ ] FAIL` | **Notes:** `____________________`
