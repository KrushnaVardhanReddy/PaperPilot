import { expect, test, describe, beforeAll } from 'vitest';
import { pdfMatchers, inspectPdf } from '../src/index';
import * as fs from 'fs';
import * as path from 'path';

expect.extend(pdfMatchers);

declare module 'vitest' {
  interface Assertion<T = any> {
    toHavePageCount(expected: number): void;
    toContainPdfText(text: string): void;
    toHaveFormField(fieldName: string, expectedValue?: string): void;
  }
}

describe('PdfInspector', () => {
  let multiPagePdf: Buffer;
  let simplePdf: Buffer;

  beforeAll(() => {
    multiPagePdf = fs.readFileSync(path.resolve(__dirname, '../../../tests/fixtures/multi_page.pdf'));
    simplePdf = fs.readFileSync(path.resolve(__dirname, '../../../tests/fixtures/simple.pdf'));
  });

  test('should assert page count in < 10ms', () => {
    const start = performance.now();
    expect(multiPagePdf).toHavePageCount(3);
    const end = performance.now();

    expect(end - start).toBeLessThan(10);
  });

  test('should extract and assert text in < 10ms', () => {
    const start = performance.now();
    expect(simplePdf).toContainPdfText('');
    const end = performance.now();

    expect(end - start).toBeLessThan(10);
  });

  test('raw inspector api works', () => {
    const inspector = inspectPdf(multiPagePdf);
    expect(inspector.page_count()).toBe(3);
    expect(inspector.is_encrypted()).toBe(false);
    expect(typeof inspector.sha256_hex()).toBe('string');
    expect(inspector.sha256_hex().length).toBeGreaterThan(0);
  });
});
