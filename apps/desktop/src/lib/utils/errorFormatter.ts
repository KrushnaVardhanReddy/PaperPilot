export interface FormattedError {
  headline: string;
  actionHint: string;
  technicalDetails?: string;
}

export function formatUserFriendlyError(rawError: unknown, operationTitle?: string): FormattedError {
  let errorString = '';
  if (typeof rawError === 'string') {
    errorString = rawError;
  } else if (rawError instanceof Error) {
    errorString = rawError.message;
  } else {
    try {
      errorString = JSON.stringify(rawError);
    } catch {
      errorString = String(rawError);
    }
  }

  const lowerError = errorString.toLowerCase();

  // 1. Password / Encryption
  if (
    lowerError.includes('password') ||
    lowerError.includes('encrypted') ||
    lowerError.includes('decrypt') ||
    lowerError.includes('unauthorized') ||
    lowerError.includes('owner password')
  ) {
    return {
      headline: 'Protected PDF Document',
      actionHint: 'Protected PDF: Enter the document open password in the security panel or decrypt tool to unlock editing.',
      technicalDetails: errorString
    };
  }

  // 2. Corrupted PDF / Damaged Cross-Reference Table
  if (
    lowerError.includes('xref') ||
    lowerError.includes('cross-reference') ||
    lowerError.includes('trailer') ||
    lowerError.includes('syntax') ||
    lowerError.includes('corrupt') ||
    lowerError.includes('damaged') ||
    lowerError.includes('invalid stream') ||
    lowerError.includes('unexpected token')
  ) {
    return {
      headline: 'Damaged Cross-Reference Table',
      actionHint: "The document cross-reference table or syntax appears corrupted. Try using the 'Repair' tool first to reconstruct the PDF syntax.",
      technicalDetails: errorString
    };
  }

  // 3. Invalid Page Range / Out-of-Bounds
  if (
    lowerError.includes('out of bounds') ||
    lowerError.includes('page count') ||
    lowerError.includes('range') ||
    lowerError.includes('empty range') ||
    lowerError.includes('cannot produce 0-page')
  ) {
    return {
      headline: 'Out-of-Bounds Page Range',
      actionHint: 'The requested page numbers exceed the document page count. Step 1: Check document page count in viewer. Step 2: Enter valid page ranges (e.g. 1-5, 8).',
      technicalDetails: errorString
    };
  }

  // 4. Missing Input / Not Found
  if (
    lowerError.includes('missing') ||
    lowerError.includes('not found') ||
    lowerError.includes('no such file') ||
    lowerError.includes('enoent')
  ) {
    return {
      headline: 'File Not Found',
      actionHint: 'The specified file could not be found. Step 1: Verify the file exists on disk. Step 2: Re-select the document in the workspace.',
      technicalDetails: errorString
    };
  }

  // 5. File In Use / Locked / Permissions
  if (
    lowerError.includes('permission denied') ||
    lowerError.includes('in use') ||
    lowerError.includes('busy') ||
    lowerError.includes('locked') ||
    lowerError.includes('access is denied')
  ) {
    return {
      headline: 'File Locked or Inaccessible',
      actionHint: 'The document file is locked by another program or operating system permission is denied. Step 1: Close other PDF viewers/editors. Step 2: Retry the operation.',
      technicalDetails: errorString
    };
  }

  // 6. Complex Font Encodings
  if (
    lowerError.includes('cid') ||
    lowerError.includes('font') ||
    lowerError.includes('encoding')
  ) {
    return {
      headline: 'Complex Font Encodings',
      actionHint: 'This document uses non-standard embedded CID fonts; try running OCR or Flatten Annotations first.',
      technicalDetails: errorString
    };
  }

  // 7. Limit / Timeout
  if (
    lowerError.includes('too large') ||
    lowerError.includes('memory limit') ||
    lowerError.includes('timeout') ||
    lowerError.includes('size exceeded')
  ) {
    return {
      headline: 'File Limit Reached',
      actionHint: 'The document is too large or complex for this operation.',
      technicalDetails: errorString
    };
  }

  // 8. Empty Operation
  if (
    lowerError.includes('cannot merge 0 files') ||
    lowerError.includes('empty array') ||
    lowerError.includes('no inputs') ||
    lowerError.includes('need at least 2')
  ) {
    return {
      headline: 'No Documents Selected',
      actionHint: 'Please add at least one document to proceed.',
      technicalDetails: errorString
    };
  }

  // 9. Fallback
  return {
    headline: `${operationTitle || 'Operation'} could not be completed`,
    actionHint: 'An unexpected issue occurred. Please check the document and retry.',
    technicalDetails: errorString
  };
}
