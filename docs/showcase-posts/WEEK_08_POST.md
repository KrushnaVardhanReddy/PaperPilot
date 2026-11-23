# Week 08: Document Assembly: Dynamic Contract Stamping, Watermarking & Bates Sequencing

**Theme:** Automated Document Assembly, Bates Stamping & Legal Packaging  
**Tools Featured:** `pdf_merge`, `pdf_watermark`, `pdf_bates` (PaperPilot Desktop, MCP, CLI & REST API)  
**Target Audience:** Legal Operations, Litigation Support, Corporate Counsel, Contract Managers  
**Asset:** ![Week 08 Demo](../assets/showcase/week08-document-assembly.gif)

---

## 📝 LinkedIn Post Copy

Litigation prep shouldn't mean spending hours manually assembling discovery exhibits and typing Bates numbers. ⚖️

When assembling multi-party contracts, discovery bundles, or court filings, teams juggle three separate utilities: one to merge PDFs, one to stamp Bates numbers, and another to overlay confidential watermarks. 🛑

One misaligned stamp or out-of-order exhibit index can trigger evidentiary objections or court sanctions.

PaperPilot unifies the entire document assembly pipeline into a single, high-speed execution run in native Rust.

Same task, your choice of interface:

🖥️ **Desktop GUI (Visual Assembly Dock)**
Reorder discovery filings with drag-and-drop -> Set prefix "EXHIBIT-A-" with start index 0001 -> Toggle "CONFIDENTIAL" diagonal watermark -> Click "Assemble & Stamp". Complete 200-page bundle ready in 140ms.

🤖 **AI Agent / MCP (Litigation Discovery Agent)**
```json
{
  "server": "paperpilot",
  "tool": "pdf_bates",
  "arguments": {
    "input": "merged_exhibits.pdf",
    "output": "stamped_discovery.pdf",
    "prefix": "LIT-2026-",
    "start_number": 1,
    "digits": 6
  }
}
```

💻 **CLI (Sub-25ms Terminal Pipeline)**
```bash
paperpilot merge -i pleading.pdf exhibits/*.pdf -o bundle.pdf
paperpilot bates -i bundle.pdf -o discovery.pdf --prefix "EX-2026-" --start 1
paperpilot watermark -i discovery.pdf -o final.pdf --text "CONFIDENTIAL"
```

🌐 **Python / REST API (Sub-10ms Microservice)**
```python
resp = requests.post("http://localhost:8080/api/v1/bates", json={
    "input_path": "bundle.pdf",
    "output_path": "discovery_stamped.pdf",
    "prefix": "SEC-INVESTIGATION-",
    "start_number": 1001
}) # 250 pages stamped in 18ms
```

Assemble exhibits, stamp immutable sequence numbers, and lock confidential watermarks with sub-second precision.

⭐ GitHub: https://github.com/usepaperpilot/paperpilot

#LegalTech #Litigation #DocumentAssembly #BatesStamping #Contracts #Compliance #LegalOps #RustLang #OpenSource #PaperPilot #MCP
