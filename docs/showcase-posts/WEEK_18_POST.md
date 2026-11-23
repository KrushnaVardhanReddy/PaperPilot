# Week 18: Page Extraction & Selective Redaction: Slicing Multi-Gigabyte Documents for LLM Context Windows

**Theme:** Intelligent Page Slicing, Range Extraction & Token-Budget Optimization  
**Tools Featured:** `pdf_extract_pages`, `pdf_crop` (PaperPilot Desktop, MCP, CLI & REST API)  
**Target Audience:** RAG Engineers, Financial Analysts, Legal Tech Builders, AI Architects  
**Asset:** ![Week 18 Demo](../assets/showcase/week18-selective-slicing-llm.gif)

---

## 📝 LinkedIn Post Copy

Don't feed an entire 500-page annual report to an LLM when the question is only about the balance sheet on page 42. 📉

Piping full multi-megabyte PDFs into cloud model context windows wastes tokens, inflates API bills, and degrades retrieval accuracy due to the "needle in a haystack" problem. 🛑

PaperPilot allows AI agents, scripts, and desktop users to slice, extract, and crop exact page ranges with zero runtime overhead:
- Extracts specific pages or page ranges (`1-5, 42, 88-92`) in sub-10ms.
- Retains internal hyperlinks, embedded fonts, and vector fidelity in the extracted slice.
- `pdf_crop`: Crops out irrelevant headers, footers, and margins to maximize token density.

Same task, your choice of interface:

🖥️ **Desktop GUI (Interactive Page Slicer)**
Drop a massive document into PaperPilot Desktop -> Select pages 12 through 16 in the visual page grid -> Click "Extract Range to New Document". Ready in 15ms.

🤖 **AI Agent / MCP (Targeted Context Slicer)**
```json
{
  "server": "paperpilot",
  "tool": "pdf_extract_pages",
  "arguments": {
    "input": "sec_10k_filing.pdf",
    "output": "balance_sheet_slice.pdf",
    "pages": "42-45"
  }
}
```

💻 **CLI (Sub-10ms Terminal Slicing)**
```bash
paperpilot extract-pages -i mega_report.pdf -o slice.pdf --pages 1-5,42-45
paperpilot crop -i slice.pdf -o focused.pdf --margin-top 50 --margin-bottom 50
```

🌐 **Python / REST API (Sub-5ms Slicing Payload)**
```python
resp = requests.post("http://localhost:8080/api/v1/extract-pages", json={
    "input_path": "annual_filing.pdf",
    "pages": "80-85"
})
slice_path = resp.json()["output_path"] # Ready for immediate RAG embedding
```

Deliver targeted, high-precision document slices to your AI models while slashing token costs.

⭐ GitHub: https://github.com/KrushnaVardhanReddy/PaperPilot

⚡ 5-Second Flash Install:
• Linux & macOS:
`curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash`
• Windows PowerShell:
`irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex`
• AI Agent / MCP:
`npx -y @usepaperpilot/mcp`

#RAG #TokenOptimization #AIArchitect #DataPipelines #RustLang #OpenSource #PaperPilot #MCP #CostReduction
