# Week 02: Giving Claude Desktop Local Superpowers via MCP: Autonomous Legal Clerk

**Theme:** Model Context Protocol (MCP) & Autonomous AI Agents  
**Tools Featured:** `pdf_bates`, `pdf_split`, `pdf_reorder` (PaperPilot Desktop, MCP, CLI & REST API)  
**Target Audience:** Claude Desktop & Cursor Users, Legal Technologists, AI Workflow Architects  
**Asset:** ![Week 02 Demo](../assets/showcase/week02-mcp-agent-bates-split.gif)

---

## 📝 LinkedIn Post Copy

Claude Desktop is great at reading text. But ask it to Bates stamp 50 trial exhibits and split them by section, and it hits a dead end. 🚫

Unless you wire it directly to local tools via MCP. ⚡

Commercial e-discovery tools charge thousands per seat for basic tasks: applying sequential Bates numbering (`CONF-000001` .. `CONF-000045`), splitting deposition files, and organizing court exhibits.

With PaperPilot's Model Context Protocol (MCP) server, Claude gains 92 deterministic native tools running locally on your hardware. Zero bytes leave localhost.

Same task, your choice of interface:

🖥️ **Desktop GUI (Visual Legal Dock)**
Open `exhibit_bundle.pdf` in PaperPilot Desktop -> Select "Bates Stamping" -> Set prefix `CASE-2026-` -> Click "Apply & Split". Exhibits are stamped and segregated in 14ms.

🤖 **AI Agent / MCP (Claude Desktop & Cursor)**
Ask Claude: *"Stamp exhibit_bundle.pdf with prefix 'CASE-2026-', bottom-right corner, then split each exhibit into a standalone PDF."*
```json
{
  "server": "paperpilot",
  "tool": "pdf_bates",
  "arguments": {
    "input": "exhibit_bundle.pdf",
    "prefix": "CASE-2026-",
    "start": 1,
    "position": "bottom-right"
  }
}
```

💻 **CLI (High-Speed Batch Terminal)**
```bash
paperpilot bates -i bundle.pdf -o stamped.pdf --prefix "CASE-2026-" --start 1
paperpilot split -i stamped.pdf --ranges "1-2,3-4,5-6" -o "./exhibits/"
```

🌐 **Python / REST API (Sub-5ms Discovery Service)**
```python
resp = requests.post("http://localhost:8080/api/v1/bates", json={
    "input_path": "bundle.pdf",
    "prefix": "CASE-2026-",
    "start": 1,
    "position": "bottom-right"
}) # Stamped & segregated in 4.1ms
```

Why MCP + Rust is the future of desktop agents:
- Deterministic accuracy: No brittle Python scripts manipulating PDF bytes.
- Microsecond latency: Faster than Claude can stream its next token.
- Air-gapped compliance: Works 100% offline for healthcare, defense, and legal counsel.

⭐ Try it on GitHub: https://github.com/usepaperpilot/paperpilot
Run locally: `npx @usepaperpilot/mcp` or `paperpilot serve --mcp`

#Claude #Anthropic #MCP #ModelContextProtocol #LegalTech #AIWorkflow #Automation #OpenSource #Rust
