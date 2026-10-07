import json

with open('apps/web/public/openapi.json') as f:
    spec = json.load(f)

CATEGORIES = {
    'Core Operations': 'Essential page manipulations and document assembly (10 tools)',
    'Document Security': 'Encryption, digital signatures, permissions, and watermarks (13 tools)',
    'Content & Inspection': 'Text/image extraction, visual rendering, OCR, and analysis (11 tools)',
    'Forms & Fields': 'AcroForm field inspection, fill, and signature widgets (3 tools)',
    'Conversions': 'High-fidelity document format conversions (6 tools)',
    'Health': 'Gateway health check and tool status',
    'MCP Protocol': 'Direct Model Context Protocol execution endpoint for all 44 tools'
}

TOOL_METADATA = {
    # Core Operations (10 tools)
    'pdf_merge': ('Core Operations', 'Merge two or more separate PDFs into a single continuous document'),
    'pdf_split': ('Core Operations', 'Split a multi-page PDF into separate files or custom ranges'),
    'pdf_extract_pages': ('Core Operations', 'Extract specific page numbers or ranges into a new PDF'),
    'pdf_delete_pages': ('Core Operations', 'Delete specified page indices from a PDF document'),
    'pdf_reorder_pages': ('Core Operations', 'Reorder document pages into a custom page sequence'),
    'pdf_rotate': ('Core Operations', 'Rotate specified pages by 90, 180, or 270 degrees'),
    'pdf_crop': ('Core Operations', 'Crop pages to a defined rectangular bounding box (x, y, width, height)'),
    'pdf_burst': ('Core Operations', 'Burst a document into individual single-page PDF files'),
    'pdf_remove_blank': ('Core Operations', 'Detect and remove completely blank pages from document'),
    'pdf_images_to_pdf': ('Core Operations', 'Combine JPEG, PNG, or WebP image files into a single PDF document'),

    # Document Security (13 tools)
    'pdf_compress': ('Document Security', 'Optimize streams and recompress images to significantly reduce file size'),
    'pdf_repair': ('Document Security', 'Rebuild corrupted cross-reference tables and recover broken PDF structures'),
    'pdf_linearize': ('Document Security', 'Linearize PDF for Fast Web View byte-range streaming in browsers'),
    'pdf_encrypt': ('Document Security', 'Secure document with AES-256 user and owner password encryption'),
    'pdf_decrypt': ('Document Security', 'Remove password security from a known authenticated PDF file'),
    'pdf_watermark': ('Document Security', 'Stamp diagonal or centered text/image watermarks across pages'),
    'pdf_redact': ('Document Security', 'Permanently sanitize, black out, and scrub sensitive text and areas'),
    'pdf_metadata': ('Document Security', 'Read, modify, or strip Title, Author, Subject, and Creator metadata'),
    'pdf_sign': ('Document Security', 'Cryptographically sign PDF with X.509 certificate and PKCS#7 signature'),
    'pdf_flatten': ('Document Security', 'Flatten interactive forms and annotations directly into page content stream'),
    'pdf_to_pdf_a': ('Document Security', 'Convert document to PDF/A archival standard format compliance'),
    'pdf_header_footer': ('Document Security', 'Stamp dynamic running headers and footers with custom margin offsets'),
    'pdf_bates': ('Document Security', 'Apply sequential legal Bates numbering across all document pages'),
    'pdf_page_numbers': ('Document Security', 'Stamp dynamic page numbering formats (e.g. Page {page} of {total})'),

    # Content & Inspection (11 tools)
    'pdf_extract_text': ('Content & Inspection', 'Extract clean UTF-8 plain text from all document pages'),
    'pdf_extract_images': ('Content & Inspection', 'Extract all embedded raster images to standalone image files'),
    'pdf_search': ('Content & Inspection', 'Search for keywords and return matched page bounding box coordinates'),
    'pdf_render': ('Content & Inspection', 'Render PDF pages to high-resolution PNG image streams'),
    'pdf_compare': ('Content & Inspection', 'Compare two documents for visual and textual pixel differences'),
    'pdf_ocr': ('Content & Inspection', 'Perform pure-Rust OCR to extract text from scanned bitmap pages'),
    'pdf_bookmarks': ('Content & Inspection', 'Extract and navigate document table of contents outline hierarchy'),
    'pdf_annotate': ('Content & Inspection', 'Add highlight, text note, and geometric rectangle annotations'),
    'pdf_classify_type': ('Content & Inspection', 'Classify document type (scanned vs native text vs hybrid)'),
    'pdf_validate': ('Content & Inspection', 'Validate PDF specification compliance and structural integrity'),
    'pdf_hash': ('Content & Inspection', 'Calculate cryptographic SHA-256 integrity hash of document structure'),

    # Forms & Fields (3 tools)
    'pdf_read_form': ('Forms & Fields', 'Extract AcroForm form field names, types, and current values'),
    'pdf_fill_form': ('Forms & Fields', 'Fill interactive form field values with JSON payload'),
    'pdf_create_form_field': ('Forms & Fields', 'Create new interactive text input fields or checkbox widgets'),

    # Conversions (6 tools)
    'pdf_to_docx': ('Conversions', 'Convert PDF document to editable Microsoft Word .docx format'),
    'pdf_to_xlsx': ('Conversions', 'Extract tabular structures and export to Microsoft Excel .xlsx'),
    'pdf_to_pptx': ('Conversions', 'Convert PDF presentation slides into Microsoft PowerPoint .pptx'),
    'pdf_convert_html': ('Conversions', 'Convert PDF document into styled HTML web pages'),
    'pdf_convert_markdown': ('Conversions', 'Convert document structure and text into clean Markdown format'),
    'pdf_convert_excel': ('Conversions', 'Extract tables and export into spreadsheet format')
}

paths = {
    '/health': spec['paths']['/health'],
    '/api/v1/pdf/mcp-exec': {
        'post': {
            'tags': ['MCP Protocol'],
            'summary': 'Execute any MCP tool directly with arbitrary JSON payload',
            'description': 'Direct MCP tool dispatcher accepting `tool` name and argument dictionary.',
            'operationId': 'mcp_exec',
            'requestBody': {
                'required': True,
                'content': {
                    'application/json': {
                        'schema': {
                            'type': 'object',
                            'required': ['tool'],
                            'properties': {
                                'tool': {'type': 'string', 'example': 'pdf_merge'},
                                'arguments': {'type': 'object', 'example': {'inputs': ['a.pdf', 'b.pdf'], 'output': 'merged.pdf'}}
                            }
                        }
                    }
                }
            },
            'responses': {
                '200': {'description': 'MCP execution successful', 'content': {'application/json': {'schema': {'$ref': '#/components/schemas/ApiResponse'}}}}
            }
        }
    },
    '/mcp/sse': {
        'get': {
            'tags': ['MCP Protocol'],
            'summary': 'Server-Sent Events (SSE) stream for Model Context Protocol',
            'description': 'Real-time bidirectional SSE streaming endpoint for AI clients (Claude Desktop, Cursor, Zed, Windsurf).',
            'operationId': 'mcp_sse',
            'responses': {
                '200': {'description': 'SSE connection established', 'content': {'text/event-stream': {}}}
            }
        }
    },
    '/mcp/messages': {
        'post': {
            'tags': ['MCP Protocol'],
            'summary': 'Post incoming MCP JSON-RPC messages',
            'description': 'Receives standard JSON-RPC 2.0 messages from AI agent sessions via SSE transport.',
            'operationId': 'mcp_post_message',
            'requestBody': {
                'required': True,
                'content': {
                    'application/json': {
                        'schema': {
                            'type': 'object',
                            'properties': {
                                'jsonrpc': {'type': 'string', 'example': '2.0'},
                                'method': {'type': 'string', 'example': 'tools/call'},
                                'params': {'type': 'object'}
                            }
                        }
                    }
                }
            },
            'responses': {
                '200': {'description': 'Message accepted'}
            }
        }
    }
}

for tool, (cat, summary) in TOOL_METADATA.items():
    endpoint_name = tool.replace('pdf_', '')
    url_path = f'/api/v1/pdf/{endpoint_name}'
    paths[url_path] = {
        'post': {
            'tags': [cat],
            'summary': summary,
            'description': f'Execute `{tool}` via local REST API. Supports JSON parameters or multipart/form-data upload.',
            'operationId': tool,
            'requestBody': {
                'required': True,
                'content': {
                    'application/json': {
                        'schema': {
                            'type': 'object',
                            'properties': {
                                'input': {'type': 'string', 'example': 'document.pdf'},
                                'output': {'type': 'string', 'example': f'{endpoint_name}_out.pdf'}
                            }
                        }
                    },
                    'multipart/form-data': {
                        'schema': {
                            'type': 'object',
                            'properties': {
                                'file': {'type': 'string', 'format': 'binary', 'description': 'PDF file to process'}
                            }
                        }
                    }
                }
            },
            'responses': {
                '200': {
                    'description': f'{tool} completed successfully',
                    'content': {
                        'application/json': {
                            'schema': {'$ref': '#/components/schemas/ApiResponse'}
                        }
                    }
                },
                '400': {'description': 'Invalid request parameters'},
                '500': {'description': 'Execution error'}
            }
        }
    }

spec['paths'] = paths
spec['tags'] = [
    {'name': 'Core Operations', 'description': CATEGORIES['Core Operations']},
    {'name': 'Document Security', 'description': CATEGORIES['Document Security']},
    {'name': 'Content & Inspection', 'description': CATEGORIES['Content & Inspection']},
    {'name': 'Forms & Fields', 'description': CATEGORIES['Forms & Fields']},
    {'name': 'Conversions', 'description': CATEGORIES['Conversions']},
    {'name': 'Health', 'description': CATEGORIES['Health']},
    {'name': 'MCP Protocol', 'description': CATEGORIES['MCP Protocol']}
]

spec['info']['title'] = 'PaperPilot Gateway API & MCP Server'
spec['info']['description'] = 'Unified REST and MCP OpenAPI 3.1 Specification for PaperPilot: High-performance local engine covering all 44 intelligent PDF tools.'

with open('apps/web/public/openapi.json', 'w') as f:
    json.dump(spec, f, indent=2)

print(f"Successfully generated OpenAPI 3.1 specification with {len(paths)} endpoints!")
