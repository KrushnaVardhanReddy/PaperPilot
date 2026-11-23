# Week 15: Visual Document Diffing: High-Res Visual Pixel & Text Diff for Legal Revisions

**Theme:** High-Resolution Visual Pixel & Text Diffing for Complex Documents  
**Tools Featured:** `pdf_compare_visual`, `pdf_render_page`, `pdf_compare` (PaperPilot Desktop, MCP, CLI & REST API)  
**Target Audience:** Legal Design Teams, Regulatory Reviewers, Architects & Engineers, Quality Assurance  
**Asset:** ![Week 15 Demo](../assets/showcase/week15-visual-document-diff.gif)

---

## 📝 LinkedIn Post Copy

What happens when a document revision changes an architectural blueprint or CAD schematic instead of just body text? 📐

Traditional text-only diffing tools miss visual layout shifts, altered watermarks, and repositioned diagram elements entirely. On the other hand, naive image diffing flags irrelevant scanner noise. 🛑

PaperPilot's visual diffing engine combines sub-pixel perceptual rendering with structural AST analysis:
- Renders vector pages at 300 DPI directly in native Rust.
- Generates high-contrast colorized change maps (Green = Added, Red = Removed, Yellow = Displaced).
- Ignores insignificant raster noise while highlighting genuine structural discrepancies.

Same task, your choice of interface:

🖥️ **Desktop GUI (Side-by-Side Visual Lens)**
Drop `blueprint_v1.pdf` and `blueprint_v2.pdf` into PaperPilot Desktop -> Move the interactive slider lens across the drawing canvas -> Visual diff highlights moved columns and modified dimensions instantly.

🤖 **AI Agent / MCP (Autonomous Visual Auditor)**
```json
{
  "server": "paperpilot",
  "tool": "pdf_compare_visual",
  "arguments": {
    "original": "patent_spec_v1.pdf",
    "revised": "patent_spec_v2.pdf",
    "dpi": 300,
    "highlight_color": "red_green"
  }
}
```

💻 **CLI (Sub-35ms Visual Comparison)**
```bash
paperpilot compare-visual -a plan_v1.pdf -b plan_v2.pdf -o diff_overlay.png --dpi 300
# Visual discrepancy score: 2.4% difference detected on Page 3
```

🌐 **Python / REST API (Sub-15ms Webhook Diff)**
```python
resp = requests.post("http://localhost:8080/api/v1/compare-visual", json={
    "file_a": "drawing_rev1.pdf",
    "file_b": "drawing_rev2.pdf",
    "generate_overlay": True
})
overlay_url = resp.json()["diff_image_path"] # Ready for human QA
```

Catch rogue visual shifts, moved architectural features, and layout changes before approving high-stakes revisions.

⭐ GitHub: https://github.com/KrushnaVardhanReddy/PaperPilot

⚡ 5-Second Flash Install:
• Linux & macOS:
`curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash`
• Windows PowerShell:
`irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex`
• AI Agent / MCP:
`npx -y @usepaperpilot/mcp`

#LegalTech #VisualDiff #QualityAssurance #Architecture #Engineering #RustLang #ComputerVision #OpenSource #PaperPilot #MCP
