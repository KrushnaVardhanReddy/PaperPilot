# Tasks: Visual Showcase & Real-World Workflow Narrative (Spec 168)

- [ ] 1. Craft the "Stop Doing Document Drudgery by Hand" narrative block <!-- id: 1 -->
  - Draft relatable hook highlighting repetitive bureaucratic forms, court filings, and PDF pain points.
  - Integrate cleanly into `README.md` without cluttering the technical quickstart.

- [ ] 2. Create the Real-World Use Cases Showcase in `README.md` <!-- id: 2 -->
  - Define 6 core real-world document workflows (Form Auto-Fill, Legal Redaction, Bates Stamping, RAG Chunking, Booklet Imposition, Archival PDF/A).
  - Format into a clean, responsive markdown table with workflow descriptions, command/MCP triggers, and preview links.

- [ ] 3. Generate or link showcase preview visual assets <!-- id: 3 -->
  - Verify preview assets in `docs/assets/` or `reports/` for key use cases.

- [ ] 4. Add "Ecosystem Synergies" section <!-- id: 4 -->
  - Document how PaperPilot works alongside document generators (e.g. `office-oxide-mcp`, Pandoc, Typst) as the final offline processing, signing, and sanitization stage.

- [ ] 5. Verify documentation formatting, graphify, and commit <!-- id: 5 -->
  - Run markdown lint / formatting checks.
  - Run `graphify-rs build`.
  - Commit and push to `feature/showcase-and-narrative-enhancements`.
