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
      headline: 'Protected Document',
      actionHint: 'Please provide the correct password to unlock and process this PDF.',
      technicalDetails: errorString
    };
  }

  // 2. Corrupted PDF
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
      headline: 'Unable to Read PDF',
      actionHint: "The document structure appears damaged. Try using the 'Repair' tool first.",
      technicalDetails: errorString
    };
  }

  // 3. Invalid Page Range
  if (
    lowerError.includes('out of bounds') ||
    lowerError.includes('page count') ||
    lowerError.includes('range') ||
    lowerError.includes('empty range') ||
    lowerError.includes('cannot produce 0-page')
  ) {
    return {
      headline: 'Invalid Page Range',
      actionHint: 'Please verify that the selected page numbers exist in this document.',
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
      actionHint: 'The specified file could not be found. Please re-select the document.',
      technicalDetails: errorString
    };
  }

  // 5. File In Use / Permissions
  if (
    lowerError.includes('permission denied') ||
    lowerError.includes('in use') ||
    lowerError.includes('busy') ||
    lowerError.includes('locked') ||
    lowerError.includes('access is denied')
  ) {
    return {
      headline: 'File Is Busy or Inaccessible',
      actionHint: 'Please close the document in other applications and try again.',
      technicalDetails: errorString
    };
  }

  // 6. Limit / Timeout
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

  // 7. Empty Operation
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

  // 8. Fallback
  return {
    headline: `${operationTitle || 'Operation'} could not be completed`,
    actionHint: 'An unexpected issue occurred. Please check the document and retry.',
    technicalDetails: errorString
  };
}
