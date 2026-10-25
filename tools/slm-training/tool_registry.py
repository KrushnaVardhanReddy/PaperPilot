from typing import Any, Dict, List, Optional
from pydantic import BaseModel, Field

class ToolParameter(BaseModel):
    type: str
    description: str
    items: Optional[Dict[str, Any]] = None

class ToolDef(BaseModel):
    name: str
    description: str
    properties: Dict[str, ToolParameter]
    required: List[str]

# Define the top 15 Core Tools based on paperpilot-mcp/src/server.rs schemas

pdf_merge = ToolDef(
    name="pdf_merge",
    description="Merges multiple PDFs into one.",
    properties={
        "inputs": ToolParameter(type="array", description="Array of file paths to merge.", items={"type": "string"})
    },
    required=["inputs"]
)

pdf_split = ToolDef(
    name="pdf_split",
    description="Splits a PDF into multiple single-page PDFs.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output_dir": ToolParameter(type="string", description="The directory where split pages will be saved.")
    },
    required=["input", "output_dir"]
)

pdf_compress = ToolDef(
    name="pdf_compress",
    description="Compresses a PDF document.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output": ToolParameter(type="string", description="The output PDF file path."),
        "quality": ToolParameter(type="string", description="Compression quality preset (low, medium, high) or integer 1-100.")
    },
    required=["input", "output"]
)

pdf_rotate = ToolDef(
    name="pdf_rotate",
    description="Rotates specific or all pages in a PDF.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "pages": ToolParameter(type="string", description="The pages to rotate, either 'all' or specific pages like '1,3'."),
        "angle": ToolParameter(type="number", description="The angle to rotate by, typically 90, 180, or 270."),
        "output": ToolParameter(type="string", description="File path for the rotated output.")
    },
    required=["input", "pages", "angle", "output"]
)

pdf_watermark = ToolDef(
    name="pdf_watermark",
    description="Applies a text watermark to a PDF.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "text": ToolParameter(type="string", description="The watermark text to apply."),
        "angle": ToolParameter(type="number", description="Angle of the watermark (default 45)."),
        "opacity": ToolParameter(type="number", description="Opacity of the watermark (0 to 1, default 0.2)."),
        "color": ToolParameter(type="string", description="Color of the watermark (e.g. 'gray', 'red')."),
        "font_size": ToolParameter(type="number", description="Font size."),
        "output": ToolParameter(type="string", description="The output PDF file path."),
        "rotation": ToolParameter(type="number", description="Rotation angle for spec compliance."),
        "pages": ToolParameter(type="string", description="Pages to apply watermark on.")
    },
    required=["input", "text", "output"]
)

pdf_encrypt = ToolDef(
    name="pdf_encrypt",
    description="Encrypts a PDF with a password.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "password": ToolParameter(type="string", description="The user password."),
        "output": ToolParameter(type="string", description="The output PDF file path.")
    },
    required=["input", "password", "output"]
)

pdf_decrypt = ToolDef(
    name="pdf_decrypt",
    description="Decrypts a PDF with a password.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "password": ToolParameter(type="string", description="The user password."),
        "output": ToolParameter(type="string", description="The output PDF file path.")
    },
    required=["input", "password", "output"]
)

pdf_ocr = ToolDef(
    name="pdf_ocr",
    description="Extracts text from PDF via OCR.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output": ToolParameter(type="string", description="The output PDF file path.")
    },
    required=["input", "output"]
)

pdf_bates = ToolDef(
    name="pdf_bates",
    description="Adds Bates numbering to a PDF.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output": ToolParameter(type="string", description="The output PDF file path."),
        "start_number": ToolParameter(type="integer", description="Start number."),
        "prefix": ToolParameter(type="string", description="Prefix."),
        "padding": ToolParameter(type="integer", description="Padding length.")
    },
    required=["input", "output", "start_number", "prefix", "padding"]
)

pdf_extract_text = ToolDef(
    name="pdf_extract_text",
    description="Extracts text from a PDF.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "pages": ToolParameter(type="string", description="Pages to extract text from, e.g., '1,3,5-7' or 'all'."),
        "output": ToolParameter(type="string", description="The output text file path.")
    },
    required=["input", "output"]
)

pdf_extract_tables = ToolDef(
    name="pdf_extract_tables",
    description="Extract tabular data from PDF into JSON or CSV.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output": ToolParameter(type="string", description="The output file path."),
        "format": ToolParameter(type="string", description="Output format (json or csv).")
    },
    required=["input", "output", "format"]
)

pdf_redact = ToolDef(
    name="pdf_redact",
    description="Redacts a rectangular area in a PDF.",
    properties={
        "input": ToolParameter(type="string", description="Path to the PDF to redact."),
        "output": ToolParameter(type="string", description="Path to save the redacted PDF."),
        "page": ToolParameter(type="integer", description="Page number to redact (1-based)."),
        "x": ToolParameter(type="number", description="X coordinate."),
        "y": ToolParameter(type="number", description="Y coordinate."),
        "width": ToolParameter(type="number", description="Width."),
        "height": ToolParameter(type="number", description="Height.")
    },
    required=["input", "output", "page", "x", "y", "width", "height"]
)

pdf_sanitize = ToolDef(
    name="pdf_sanitize",
    description="Deeply sanitizes a PDF document by removing metadata, scripts, attachments, and thumbnails.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output": ToolParameter(type="string", description="The output PDF file path."),
        "remove_metadata": ToolParameter(type="boolean", description="Remove metadata."),
        "remove_attachments": ToolParameter(type="boolean", description="Remove attachments."),
        "remove_scripts": ToolParameter(type="boolean", description="Remove scripts.")
    },
    required=["input", "output"]
)

pdf_auto_rotate = ToolDef(
    name="pdf_auto_rotate",
    description="Automatically upright rotated pages.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output": ToolParameter(type="string", description="The output PDF file path.")
    },
    required=["input", "output"]
)

document_pii_scan = ToolDef(
    name="document_pii_scan",
    description="Scan document for PII.",
    properties={
        "input": ToolParameter(type="string", description="The input PDF file path."),
        "output": ToolParameter(type="string", description="The output PDF file path."),
        "scan_ssn": ToolParameter(type="boolean", description="Scan for SSN."),
        "scan_credit_cards": ToolParameter(type="boolean", description="Scan for credit cards."),
        "scan_emails": ToolParameter(type="boolean", description="Scan for emails."),
        "scan_phones": ToolParameter(type="boolean", description="Scan for phones."),
        "scan_iban": ToolParameter(type="boolean", description="Scan for IBAN.")
    },
    required=["input", "output"]
)

TOOLS_REGISTRY: Dict[str, ToolDef] = {
    "pdf_merge": pdf_merge,
    "pdf_split": pdf_split,
    "pdf_compress": pdf_compress,
    "pdf_rotate": pdf_rotate,
    "pdf_watermark": pdf_watermark,
    "pdf_encrypt": pdf_encrypt,
    "pdf_decrypt": pdf_decrypt,
    "pdf_ocr": pdf_ocr,
    "pdf_bates": pdf_bates,
    "pdf_extract_text": pdf_extract_text,
    "pdf_extract_tables": pdf_extract_tables,
    "pdf_redact": pdf_redact,
    "pdf_sanitize": pdf_sanitize,
    "pdf_auto_rotate": pdf_auto_rotate,
    "document_pii_scan": document_pii_scan,
}
