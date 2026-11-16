# CLI Natural Language Interface (Ask)

The `paperpilot ask` subcommand enables natural language PDF manipulation directly from the command line.
It uses an embedded offline NLP resolver to interpret natural language instructions (like "rotate 90 degrees") and maps them to over 40 structured PDF operations (Merge, Compress, Split, Redact, etc.).

## Usage

```bash
paperpilot ask "rotate 90 degrees" doc.pdf --yes -o rotated.pdf
```
