# Week 04: Document Forensics: Instant Tamper-Evident Verification (Diff & Signatures)

**Theme:** Document Forensics, Compliance & Cryptographic Integrity  
**Tools Featured:** `pdf_compare`, `pdf_sign`, `pdf_verify` (PaperPilot CLI & MCP)  
**Target Audience:** Compliance Officers, Legal Counsel, Audit Teams, FinTech Security Engineers  
**Asset:** ![Week 04 Demo](../assets/showcase/week04-document-forensics-diff.gif)

---

## 📝 LinkedIn Post Copy

### Hook
Someone sends you a revised 40-page contract. They claim they only *"fixed a typo in section 2"*. 

Can you spot the rogue zero that turned a \$10,000 monthly retainer into \$100,000 before your team signs? 🔍🚨

---

### The Problem
In commercial contracting, legal discovery, and procurement, sneaky changes happen every single day:
- A decimal or comma moved in an indemnification clause
- Subtly altered bank routing numbers on an invoice
- Stripped or invalidated digital signature certificates

Manual line-by-line review takes hours and is prone to human fatigue. Generic visual diff tools flag false positives on harmless font re-flows or kerning changes while missing subtle character substitutions deep in the PDF byte stream.

And if a document was digitally signed? Modifying even one invisible character silently invalidates the document's legal authenticity.

---

### The PaperPilot Solution
PaperPilot pairs structural AST comparison with hardware-speed cryptographic verification:

1. **Sub-Second Stream & Structural Diff (`pdf_compare`)**:
   PaperPilot doesn't just do naive pixel diffing. It inspects the underlying text objects and layout tree, catching exact word substitutions, deleted clauses, and shifted decimals in **11ms**.
2. **Deterministic Cryptographic Verification (`pdf_verify`)**:
   Instantly validates embedded digital signatures (PAdES / PKCS#7 / CMS / RSA-4096). If a single byte in the signed byte range was altered, PaperPilot immediately flags the hash mismatch and revokes trust.
3. **Enterprise Compliance Audit Logs**:
   Outputs machine-readable JSON diffs (`--format json`) ready for automated ingestion into compliance gates, n8n nodes, and CI/CD contract approval pipelines.

---

### Terminal Walkthrough
```bash
# 1. Diff two document revisions at the structural AST level
paperpilot compare original_v1.0.pdf received_v1.1.pdf --format json
# => [Tamper Detected]: Page 1, Line 14: "$10,000.00" -> "$100,000.00"

# 2. Verify digital signature & byte-range cryptographic seal
paperpilot verify --input received_v1.1.pdf
# => Status: REJECTED (Byte-range digest mismatch; document modified post-signing)
```

In the animated walkthrough below, watch PaperPilot catch both a rogue decimal change and an invalid digital signature in milliseconds.

---

### Key Takeaway for Legal & Compliance Teams
Never rely on visual reading or honor-system redlines when millions of dollars are on the line. 
- Diff the structural document AST locally.
- Cryptographically verify the byte stream before signing.

---

## 🏷️ Hashtags & Social Metadata
#LegalTech #Compliance #DocumentSecurity #CyberSecurity #Audit #FinTech #OpenSource #RustLang #ContractManagement #PaperPilot
