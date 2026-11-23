# Week 01: The Zero-Leak LLM Pipeline: Irreversible PII Redaction & Sanitization

**Theme:** Enterprise AI Privacy & Security  
**Tools Featured:** `pdf_redact`, `pdf_hash` (PaperPilot Desktop, MCP, CLI & REST API)  
**Target Audience:** AI Engineers, Legal Tech, Security & Compliance, n8n/LangChain Builders  
**Asset:** ![Week 01 Demo](../assets/showcase/week01-zero-leak-redaction.gif)

---

## 📝 LinkedIn Post Copy

Most PDF "redaction" tools don't delete your sensitive data. They just slap a black rectangle on top of it. 🛑

Feed that file into an LLM or OCR parser, and your client's SSN, credit cards, and health records get ingested straight into model context—or logged to cloud APIs.

Why? Traditional PDF annotations (`/Square`, `/Highlight`) merely render a visual mask. The raw glyph streams and font descriptors remain 100% extractable with 2 lines of Python.

PaperPilot takes a cryptographic-first approach: surgical byte-stream excision in native Rust with deterministic SHA-256 auditing.

Same engine, your choice of interface:

🖥️ **Desktop GUI (1-Click Visual Redact)**
Drag in your PDF -> Select "Auto-Sanitize PII (SSN, Cards, Emails)" -> Click "Apply". The document is permanently cleansed with zero visual artifacting.

🤖 **AI Agent / MCP (Claude Desktop & Cursor)**
Let your AI agent orchestrate compliance locally:
```json
{
  "server": "paperpilot",
  "tool": "pdf_redact",
  "arguments": {
    "input": "contract.pdf",
    "output": "safe.pdf",
    "patterns": ["ssn", "credit_card", "email"]
  }
}
```

💻 **CLI (Sub-15ms Terminal Execution)**
```bash
paperpilot redact -i contract.pdf -o safe.pdf --regex '\d{3}-\d{2}-\d{4}'
paperpilot hash -i safe.pdf # => SHA-256 verified audit seal
```

🌐 **Python / REST API (Sub-5ms Microservice)**
```python
resp = requests.post("http://localhost:8080/api/v1/redact", json={
    "input_path": "contract.pdf",
    "output_path": "safe.pdf",
    "regex": r"\d{3}-\d{2}-\d{4}"
}) # Completed in 3.8ms
```

Before piping sensitive documents into external LLM endpoints:
1. Never trust visual annotation overlays.
2. Sanitize at the byte-stream level locally.
3. Stamp an immutable hash for your audit logs.

100% open-source & local-first.

⭐ GitHub: https://github.com/usepaperpilot/paperpilot
📦 Install: `cargo install paperpilot-cli` or `npx @usepaperpilot/mcp`

#AI #CyberSecurity #DataPrivacy #Rust #LLM #InfoSec #LegalTech #MCP #OpenSource
