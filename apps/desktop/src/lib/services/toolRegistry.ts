export function resolveToolName(toolId: string): string {
    if (toolId === 'md_to_pdf') return 'pdf_convert_markdown';
    if (toolId === 'html_to_pdf') return 'pdf_convert_html';
    if (toolId === 'img_to_pdf') return 'pdf_images_to_pdf';
    if (toolId === 'json_to_pdf') return 'json_to_pdf';

    // Forms and conversions mapping
    if (['create_form_field', 'fill_form', 'read_form'].includes(toolId)) {
        return `pdf_${toolId}`;
    }

    if ([
        'pdf_to_docx', 'pdf_to_xlsx', 'pdf_to_pptx', 'pdf_to_pdf_a', 'pdf_to_markdown', 'pdf_to_json',
        'pdf_convert_excel', 'office_modernize_legacy', 'office_to_markdown', 'office_extract_text',
        'excel_to_json', 'excel_to_csv'
    ].includes(toolId)) {
        return toolId;
    }

    return `pdf_${toolId}`;
}
