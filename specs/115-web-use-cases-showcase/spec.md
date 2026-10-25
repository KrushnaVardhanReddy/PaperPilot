# Spec 115: Web Portal Use Cases Showcase

## Status: Approved
## Scope: `apps/web`

## 1. Problem Statement
Users arriving at the PaperPilot web portal see high-level metrics, terminal previews, and the tool runner. However, new visitors (lawyers, healthcare workers, financial analysts, students, prepress operators, developers) often need to see their exact pain point addressed to immediately recognize value ("Can it solve my specific problem?").

## 2. Goals
- Add an interactive, responsive Use Cases Showcase section directly below the Hero section and above the TriSurfaceShowcase.
- Highlight 6 primary personas / workflows:
  1. 🏛️ **Legal & Compliance**: Signature verification, permanent PII redaction, document sanitization.
  2. 🏥 **Healthcare (HIPAA)**: PHI discovery and deep redaction from clinical PDFs and scans.
  3. 📊 **Finance & Accounting**: High-fidelity table extraction from financial statements to Excel/JSON.
  4. 🎓 **Academic & Research**: Dual-layer OCR for scanned papers and smart multi-document merging.
  5. 🖨️ **Print & Prepress**: N-up slide imposition, booklet generation, and page scaling.
  6. 👩‍💻 **Developer Pipelines**: Headless automation via CLI, Rust crate, or Dockerized MCP server.
- Interactive deep-links / CTAs that scroll the user directly to the relevant tool or interface surface.
- Adhere strictly to the Svelte 5 runes architecture and dark-mode aesthetic with glassmorphism and micro-animations.
