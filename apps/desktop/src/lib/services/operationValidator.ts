export function validatePreconditions(toolId: string, documentsCount: number): { valid: boolean; error?: string } {
    if (toolId === 'compare') {
        if (documentsCount < 2) {
            return { valid: false, error: 'Need at least 2 documents loaded to compare.' };
        }
        return { valid: true };
    }

    const isCreationTool = ['md_to_pdf', 'html_to_pdf', 'img_to_pdf', 'json_to_pdf'].includes(toolId);
    if (!isCreationTool && documentsCount === 0) {
        return { valid: false, error: 'No documents available. Add or drop a PDF first.' };
    }

    if (toolId === 'merge' && documentsCount < 2) {
        return { valid: false, error: 'Need at least 2 documents in tabs to merge.' };
    }

    return { valid: true };
}
