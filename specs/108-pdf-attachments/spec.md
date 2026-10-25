# Spec 108: Manage PDF Attachments & Portfolios (`pdf_attachments`)

## 1. Background & Executive Summary
PDF documents support embedding arbitrary binary attachments and associated file streams as specified in ISO 32000-1 (Section 7.7.4 "Embedded Files" and Section 12.5.6.15 "Embedded File Streams"). These attachments are stored in the Document Catalog's `/Names` dictionary under the `/EmbeddedFiles` Name Tree, or inside individual page annotations (`/Subtype /FileAttachment`).

PDF portfolios (often created by Acrobat, CAD software, and invoicing systems) aggregate multiple files (such as spreadsheets, XML invoices, images, and source documents) within a single PDF wrapper.

**Spec 108** implements `pdf_attachments` (Phase 5, Task 5.3.12):
- **Inspect**: Enumerates all embedded files in the PDF portfolio, reporting file name, MIME/sub-type, description, uncompressed byte size, creation and modification timestamps, and checksum if available.
- **Extract**: Extracts one or all embedded files to disk or as structured byte payload.
- **Attach (Embed)**: Injects an external file as an embedded file stream into `/EmbeddedFiles`, creating the required `/Filespec` and `/EmbeddedFile` stream dictionaries.
- **Remove**: Deletes specified attachments from the `/EmbeddedFiles` Name Tree.
- **Penta-Interface Parity**: Exposes this functionality seamlessly across CLI (`paperpilot attachments`), MCP (`pdf_attachments`), REST Gateway (`POST /api/v1/pdf/attachments`), WASM in-memory engine, and Desktop UI.

---

## 2. Technical Architecture & PDF Data Model

### 2.1 ISO 32000-1 Embedded Files Specification
In the document catalog (`/Root`):
```text
<<
  /Type /Catalog
  /Names <<
    /EmbeddedFiles <<
      /Names [
        (data.csv) 12 0 R
        (invoice.xml) 14 0 R
      ]
    >>
  >>
>>
```
Each object referenced by the `/Names` array is a File Specification dictionary (`/Type /Filespec`):
```text
12 0 obj
<<
  /Type /Filespec
  /F (data.csv)
  /UF (data.csv)
  /EF <<
    /F 13 0 R
  >>
  /Desc (Monthly sales summary)
>>
endobj
```
And object `13 0 R` is the Embedded File Stream:
```text
13 0 obj
<<
  /Type /EmbeddedFile
  /Subtype /text#2Fcsv
  /Length ...
  /Filter /FlateDecode
  /Params <<
    /Size 1024
    /CreationDate (D:20261025064500Z)
    /ModDate (D:20261025064500Z)
  >>
>>
stream
...
endstream
endobj
```

### 2.2 Operational Actions
The tool supports 4 primary modes:
1. `list` (default): Scans `/Names -> /EmbeddedFiles`, returns JSON list of attachments.
2. `extract`: Extracts attachment by `file_name` (or all) and writes to disk or returns binary data.
3. `add`: Attaches external file(s) into the PDF, creating `/Filespec` and `/EmbeddedFile` stream, returning modified PDF.
4. `remove`: Removes entry from `/EmbeddedFiles` Name Tree, returning modified PDF.

---

## 3. Data Model

In `paperpilot-pdf/src/operations/pdf_attachments.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInfo {
    pub name: String,
    pub size_bytes: usize,
    pub mime_type: Option<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub mod_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AttachmentAction {
    List,
    Extract {
        file_name: Option<String>,
    },
    Add {
        file_name: String,
        description: Option<String>,
        mime_type: Option<String>,
        #[serde(skip)]
        file_data: Vec<u8>,
    },
    Remove {
        file_name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfAttachmentsOptions {
    #[serde(default = "default_action")]
    pub action: String, // "list", "extract", "add", "remove"
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub output_dir: Option<String>,
}

fn default_action() -> String {
    "list".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentsReport {
    pub attachments: Vec<AttachmentInfo>,
    pub total_count: usize,
}
```

---

## 4. Verification & Testing
1. Unit tests in `paperpilot-pdf` verifying:
   - Reading empty PDF returns 0 attachments cleanly.
   - Adding an attachment updates `/EmbeddedFiles` and increases document size.
   - Listing returns exact metadata of the added attachment.
   - Extracting recovers the original byte-exact payload.
   - Removing deletes the entry and cleans up `/EmbeddedFiles`.
2. Penta-interface E2E assertions in `tools/penta-interface-e2e/src/cases/pdf_attachments.rs`.
