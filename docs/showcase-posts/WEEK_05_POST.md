# Week 05: Edge AI & SLMs: Air-Gapped Small Language Model (0.5B) for Private Document Automation

**Theme:** Edge AI, Small Language Models (SLMs) & Private Document Automation  
**Tools Featured:** Qwen2.5-0.5B fine-tuned via QLoRA, PaperPilot Local MCP, Desktop, CLI & REST API  
**Target Audience:** AI/ML Engineers, Enterprise Security Architects, Local AI Enthusiasts, Edge Computing Builders  
**Asset:** ![Week 05 Demo](../assets/showcase/week05-edge-slm-automation.gif)

---

## 📝 LinkedIn Post Copy

You don't need an 800-billion-parameter cloud LLM to automate document processing. 📉

Piping internal financials and payroll to remote frontier models creates major security risks and high cloud bills. 🛑

What if a **0.5B parameter model** running on a standard laptop CPU could orchestrate your document workflows in under 85ms? ⚡

We fine-tuned **Qwen2.5-0.5B-Instruct** using QLoRA specifically on PaperPilot's native tool schemas:
- Microscopic footprint: Consumes only **348 MB of RAM** quantized to Q4_K_M GGUF.
- Sub-50ms dispatch: Maps natural-language user intent straight into PaperPilot function arguments in 42ms.
- Pure Rust execution: PDF table extraction and compression complete in 18ms.
- 100% air-gapped: 0 cloud round-trips. 0 bytes leave your physical device.

Same task, your choice of interface:

🖥️ **Desktop GUI (Local Copilot Dock)**
Open PaperPilot Desktop -> Click "Local Copilot" -> Type: *"Extract quarterly tables from earnings.pdf and compress under 2MB."* Watch the on-device SLM orchestrate tasks with live progress in 82ms.

🤖 **AI Agent / MCP (Air-Gapped Tool Call)**
```json
{
  "server": "paperpilot",
  "tool": "pdf_compress",
  "arguments": {
    "input": "earnings.pdf",
    "target_mb": 2.0,
    "mode": "ebook"
  }
}
```

💻 **CLI (Terminal SLM Pipeline)**
```bash
paperpilot agent --model qwen2.5-0.5b --prompt "Extract tables from earnings.pdf and compress under 2MB"
# ✓ Tables extracted [9ms] | ✓ Compressed 14.8MB -> 1.4MB [18ms] (82ms total)
```

🌐 **Python / REST API (Local Edge Gateway)**
```python
resp = requests.post("http://localhost:8080/api/v1/agent/execute", json={
    "prompt": "Extract tables from earnings.pdf and compress under 2MB",
    "air_gapped": True
}) # Finished in 82ms locally on CPU
```

Enterprise document intelligence doesn't need giant APIs—just purpose-trained Edge SLMs with deterministic native runtimes.

⭐ GitHub: https://github.com/KrushnaVardhanReddy/PaperPilot

⚡ 5-Second Flash Install:
• Linux & macOS:
`curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash`
• Windows PowerShell:
`irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex`
• AI Agent / MCP:
`npx -y @usepaperpilot/mcp`

#EdgeAI #SLM #LocalAI #MachineLearning #AirGapped #EnterpriseAI #OpenSource #RustLang #Qwen #PaperPilot #PrivacyFirst #MCP
