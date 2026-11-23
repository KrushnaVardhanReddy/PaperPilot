# Week 10: Office-to-Markdown Pipeline: Multi-Format Extraction (Docx, PPTX & Excel to Agent Context)

**Theme:** Multi-Format Document Ingestion for AI & LLM Context Windows  
**Tools Featured:** `docx_to_md`, `pptx_to_text`, `xlsx_to_csv` (PaperPilot Desktop, MCP, CLI & REST API)  
**Target Audience:** LLM Application Developers, RAG Architects, Enterprise Knowledge Engineers  
**Asset:** ![Week 10 Demo](../assets/showcase/week10-office-to-markdown.gif)

---

## 📝 LinkedIn Post Copy

Enterprise knowledge isn't just locked in PDFs. It's scattered across Word documents, PowerPoint pitch decks, and complex Excel spreadsheets. 📊

Piping raw Microsoft Office binaries directly into LLM prompts causes syntax noise, formatting hallucinations, and token budget bloat. 🛑

PaperPilot provides native, lightning-fast Office-to-Markdown converters that transform proprietary office formats into pristine, LLM-ready context:
- `docx_to_md`: Preserves headers, nested lists, callouts, and clean Markdown tables.
- `pptx_to_text`: Extracts slide titles, bullet hierarchies, and speaker notes per slide.
- `xlsx_to_csv`: Converts multi-sheet financial models into token-optimized tabular representations.

Same task, your choice of interface:

🖥️ **Desktop GUI (Multi-Format Dropzone)**
Drag Word documents, slide decks, or spreadsheets into PaperPilot Desktop -> Select "Clean LLM Markdown" -> Instant side-by-side preview with 1-click "Copy to Prompt".

🤖 **AI Agent / MCP (Autonomous Context Ingestion)**
```json
{
  "server": "paperpilot",
  "tool": "docx_to_md",
  "arguments": {
    "input": "enterprise_architecture_v2.docx",
    "output": "context.md",
    "preserve_tables": true
  }
}
```

💻 **CLI (Sub-20ms Terminal Conversion)**
```bash
paperpilot docx-to-md -i architecture.docx -o context.md
paperpilot pptx-to-text -i roadmap.pptx -o slides.txt --notes
paperpilot xlsx-to-csv -i financials.xlsx -o balance_sheet.csv
```

🌐 **Python / REST API (Sub-5ms Webhook Ingestion)**
```python
resp = requests.post("http://localhost:8080/api/v1/docx_to_md", json={
    "input_path": "specs.docx",
    "preserve_tables": True
})
markdown_context = resp.json()["markdown"] # Ready for model prompt
```

Feed structured, token-efficient Markdown to your models without cloud conversion APIs.

⭐ GitHub: https://github.com/usepaperpilot/paperpilot

#RAG #GenerativeAI #DocxToMarkdown #LLM #KnowledgeManagement #RustLang #OpenSource #PaperPilot #MCP
