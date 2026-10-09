import { describe, it, expect } from 'vitest';
import { formatUserFriendlyError } from '../errorFormatter';

describe('formatUserFriendlyError', () => {
  it('formats Password / Encryption errors', () => {
    const error = new Error('PDF is encrypted with user password');
    const result = formatUserFriendlyError(error);
    expect(result.headline).toBe('Protected Document');
    expect(result.actionHint).toBe('Please provide the correct password to unlock and process this PDF.');
    expect(result.technicalDetails).toBe('PDF is encrypted with user password');
  });

  it('formats Corrupted PDF errors', () => {
    const result = formatUserFriendlyError('lopdf: cross-reference table missing offset 0x4f2');
    expect(result.headline).toBe('Unable to Read PDF');
    expect(result.actionHint).toBe("The document structure appears damaged. Try using the 'Repair' tool first.");
    expect(result.technicalDetails).toBe('lopdf: cross-reference table missing offset 0x4f2');
  });

  it('formats Invalid Page Range errors', () => {
    const result = formatUserFriendlyError({ message: 'Page count is 1, but requested page 2' });
    expect(result.headline).toBe('Invalid Page Range');
    expect(result.actionHint).toBe('Please verify that the selected page numbers exist in this document.');
    expect(result.technicalDetails).toContain('requested page 2'); // check that JSON.stringify handles it
  });

  it('formats Missing Input / Not Found errors', () => {
    const result = formatUserFriendlyError('File not found: document.pdf');
    expect(result.headline).toBe('File Not Found');
    expect(result.actionHint).toBe('The specified file could not be found. Please re-select the document.');
    expect(result.technicalDetails).toBe('File not found: document.pdf');
  });

  it('formats File In Use / Permissions errors', () => {
    const result = formatUserFriendlyError('Permission denied (os error 13)');
    expect(result.headline).toBe('File Is Busy or Inaccessible');
    expect(result.actionHint).toBe('Please close the document in other applications and try again.');
    expect(result.technicalDetails).toBe('Permission denied (os error 13)');
  });

  it('formats Limit / Timeout errors', () => {
    const result = formatUserFriendlyError('Memory limit exceeded during operation');
    expect(result.headline).toBe('File Limit Reached');
    expect(result.actionHint).toBe('The document is too large or complex for this operation.');
    expect(result.technicalDetails).toBe('Memory limit exceeded during operation');
  });

  it('formats Empty Operation errors', () => {
    const result = formatUserFriendlyError('cannot merge 0 files');
    expect(result.headline).toBe('No Documents Selected');
    expect(result.actionHint).toBe('Please add at least one document to proceed.');
    expect(result.technicalDetails).toBe('cannot merge 0 files');
  });

  it('formats Fallback errors with operation title', () => {
    const result = formatUserFriendlyError('Unknown error 0x99', 'Merge PDF');
    expect(result.headline).toBe('Merge PDF could not be completed');
    expect(result.actionHint).toBe('An unexpected issue occurred. Please check the document and retry.');
    expect(result.technicalDetails).toBe('Unknown error 0x99');
  });

  it('formats Fallback errors without operation title', () => {
    const result = formatUserFriendlyError('Unknown issue');
    expect(result.headline).toBe('Operation could not be completed');
    expect(result.actionHint).toBe('An unexpected issue occurred. Please check the document and retry.');
    expect(result.technicalDetails).toBe('Unknown issue');
  });
});
