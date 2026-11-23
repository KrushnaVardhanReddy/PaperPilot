# Week 09: Self-Hosted AI Stacks: Private Apache Tika Replacement for Open-WebUI & Ollama

**Theme:** Self-Hosted AI Infrastructure & Wire-Compatible Document Extraction  
**Tools Featured:** PaperPilot Gateway (`/tika` & `/rmeta/text`), `extract_meta` (PaperPilot Desktop, MCP, CLI & REST API)  
**Target Audience:** Self-Hosted AI Builders, Ollama/Open-WebUI Users, Privacy Architects, DevOps  
**Asset:** ![Week 09 Demo](../assets/showcase/week09-tika-wire-openwebui.gif)

---

## 📝 LinkedIn Post Copy

Running local AI with Ollama and Open-WebUI, but still relying on a bloated 1.2GB Apache Tika Java container for document parsing? 🛑

Traditional Tika setups require heavy JVM runtimes, consume gigabytes of idle RAM, and take hundreds of milliseconds just to parse a single page of text.

PaperPilot Gateway provides a drop-in, wire-compatible Apache Tika replacement written in pure Rust:
- Consumes <15MB RAM (vs 1.2GB JVM).
- Slashes extraction latency from 450ms down to 12ms.
- 100% compatible with Open-WebUI's `/tika` and `/rmeta/text` endpoints.
- Fully air-gapped with zero external dependencies.

Same task, your choice of interface:

🖥️ **Desktop GUI (Local Gateway Manager)**
Launch PaperPilot Desktop -> Click "Local AI Gateway" -> Enable "Tika Wire Emulation (Port 9998)" -> Point Open-WebUI or Ollama to localhost:9998. Done in 1 click.

🤖 **AI Agent / MCP (Context Extraction Tool)**
```json
{
  "server": "paperpilot",
  "tool": "extract_meta",
  "arguments": {
    "input": "financial_filing.pdf",
    "extract_text": true,
    "format": "markdown"
  }
}
```

💻 **CLI (Sub-15ms Terminal Extraction)**
```bash
paperpilot gateway --port 9998 --emulate-tika
# Or direct high-speed document extraction:
paperpilot extract -i filing.pdf --format text > context.txt
```

🌐 **Open-WebUI / REST API (Drop-in Wire Compatibility)**
```bash
curl -T document.pdf http://localhost:9998/tika --header "Accept: text/plain"
# Sub-15ms clean text stream directly into your local LLM context
```

Stop letting a heavy Java runtime bottleneck your local AI stack. Swap in PaperPilot Gateway and watch ingestion fly.

⭐ GitHub: https://github.com/usepaperpilot/paperpilot

#LocalAI #OpenWebUI #Ollama #SelfHosted #RustLang #DevOps #ApacheTika #Privacy #OpenSource #PaperPilot #MCP
