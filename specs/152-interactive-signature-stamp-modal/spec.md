# Spec 152: Interactive Visual Signature & Stamp Placement System

## 1. Problem Statement & Motivation
Currently in PaperPilot Desktop, the floating bottom action pill features a "Sign / Annotate" (✍) button (`apps/desktop/src/routes/+page.svelte:477`). However, clicking it only toggles an internal drawing state without prompting the user for their signature, name, initials, or allowing them to stamp a clean visual signature onto the page.

Users expect an intuitive, DocuSign/macOS Preview-style signing workflow:
1. Click **✍ Sign / Annotate**.
2. If no saved signature exists (or when creating a new one), present an interactive **Signature Creation Modal**:
   - **Type Tab**: Enter full name and initials, render in elegant calligraphy/handwritten fonts (e.g. cursive scripts), choose ink color (Blue, Black, Navy).
   - **Draw Tab**: Smooth HTML5 canvas signature pad allowing freehand drawing with mouse, trackpad, or stylus with an instant "Clear" button.
   - **Upload Tab**: Upload a signature PNG/JPEG with automatic background transparency.
3. Once selected or confirmed:
   - The signature follows the cursor as a draggable stamp over the PDF page.
   - Clicking on the page places the signature at that exact `(x, y)` position and page index.
   - The placed signature can be resized via corner handles, dragged to reposition, or deleted.
   - Users can persist/flatten the signature onto the document using `pdf_stamp` or export the signed PDF.

---

## 2. Functional Requirements

### 2.1 Signature Creation & Storage
- Support three creation modes:
  1. **Type**: User inputs text; rendered across multiple signature script fonts with color picker (`#000000`, `#1d4ed8`, `#047857`).
  2. **Draw**: Smooth Bezier/quadratic spline canvas drawing with velocity-sensitive stroke width.
  3. **Upload**: Import existing image file; client-side thresholding removes white backgrounds.
- Provide a persistent local store (localStorage or IndexedDB) caching saved signatures and initials for instant 1-click re-use.

### 2.2 Interactive Stamping & Placement Layer
- When a signature is selected for placement:
  - Cursor enters "stamp placement mode" showing an overlay ghost of the signature.
  - Hovering over a PDF page displays page bounds and snap guides.
  - Clicking deposits the signature as a movable, resizable annotation object.
- **Manipulation Controls**:
  - Drag to move.
  - Corner handles to scale proportionally.
  - Delete button (`✕`) or `Delete`/`Backspace` key to remove.
  - Duplicate button to stamp on multiple pages.

### 2.3 Baking & Flattening to Real PDF
- Once placed, provide a "Save / Apply Signature" action:
  - Renders the signature annotation into transparent PNG data bytes.
  - Dispatches to the underlying engine (`pdf_stamp` or `pdf_sign`) with page number, scaled coordinates, and dimensions.
  - Produces an authentic signed PDF without visual fidelity loss.

---

## 3. Architecture & Component Structure

```
apps/desktop/src/lib/components/signature/
├── SignatureModal.svelte          # Dialog for Type / Draw / Upload / Manage Saved Signatures
├── SignaturePadCanvas.svelte      # HTML5 smooth Bezier drawing canvas
├── SignatureTypePreview.svelte    # Font selection and initials generator
└── SignatureStampLayer.svelte     # Interactive movable & resizable canvas overlay on PdfViewer
```

---

## 4. Verification & Testing Criteria
- **Vitest Unit Tests**:
  - Test signature data-url generation from typed text and drawn canvas strokes.
  - Test coordinate scaling between screen viewport points and PDF 72-dpi point coordinate space.
- **Playwright E2E Test**:
  - Launch desktop viewer with PDF fixture.
  - Click floating `✍ Sign / Annotate` button.
  - Verify `SignatureModal` opens with Type, Draw, and Upload tabs.
  - Type name, select signature, click on page, verify stamp appears on canvas.
