# Bidirectional JSON-to-PDF Synthesis (json_to_pdf)

## Overview
PaperPilot supports autonomous AI agents modifying documents logically through the Bidirectional JSON-to-PDF bridge. An agent extracts a PDF into `JsonDocument` via `pdf_to_json`, modifies it logically (translating strings, swapping base64 embedded images, rearranging metadata), and injects it back through `json_to_pdf` seamlessly.

## JSON Schema Match
To synthesize a PDF, supply the identical structure exported:

```json
{
    "document_name": "report.pdf",
    "pages": [
        {
            "page_number": 1,
            "text": "Header Text",
            "char_count": 11,
            "images": [
                {
                    "id": "im1",
                    "page_number": 1,
                    "format": "png",
                    "width": 500,
                    "height": 500,
                    "bbox": [10.0, 10.0, 510.0, 510.0],
                    "base64_data": "data:image/png;base64,iVBOR..."
                }
            ]
        }
    ]
}
```

## Surfaces
- **CLI**: `paperpilot convert --format pdf --input file.json --output doc.pdf`
- **MCP Tool**: `json_to_pdf` via `{"input": "..."}` or `{"json_content": "..."}`.
- **API**: POST `/api/v1/pdf/convert` (`{"format":"pdf", "input":"..."}`).
- **WASM Memory**: `WasmPdfEngine.json_to_pdf(json_str: &str)`
