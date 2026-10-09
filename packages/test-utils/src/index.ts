import { PdfInspector } from "paperpilot-wasm";

export function inspectPdf(buffer: Buffer | Uint8Array): PdfInspector {
  return new PdfInspector(new Uint8Array(buffer));
}

export const pdfMatchers = {
  toHavePageCount(received: Buffer | Uint8Array, expected: number) {
    const inspector = inspectPdf(received);
    try {
      const count = inspector.page_count();
      const pass = count === expected;
      if (pass) {
        return {
          message: () => `expected PDF not to have page count ${expected}`,
          pass: true,
        };
      } else {
        return {
          message: () => `expected PDF to have page count ${expected}, but got ${count}`,
          pass: false,
        };
      }
    } finally {
      inspector.free();
    }
  },
  toContainPdfText(received: Buffer | Uint8Array, text: string) {
    const inspector = inspectPdf(received);
    try {
      const pass = inspector.contains_text(text);
      if (pass) {
        return {
          message: () => `expected PDF not to contain text "${text}"`,
          pass: true,
        };
      } else {
        return {
          message: () => `expected PDF to contain text "${text}"`,
          pass: false,
        };
      }
    } finally {
      inspector.free();
    }
  },
  toHaveFormField(received: Buffer | Uint8Array, fieldName: string, expectedValue?: string) {
    const inspector = inspectPdf(received);
    try {
      const fields = inspector.form_fields() as [string, string | null][];
      const field = fields.find(([name]) => name === fieldName);

      if (!field) {
        return {
          message: () => `expected PDF to contain form field "${fieldName}"`,
          pass: false,
        };
      }

      if (expectedValue !== undefined) {
        const pass = field[1] === expectedValue;
        if (pass) {
          return {
            message: () => `expected PDF form field "${fieldName}" not to have value "${expectedValue}"`,
            pass: true,
          };
        } else {
          return {
            message: () => `expected PDF form field "${fieldName}" to have value "${expectedValue}", but got "${field[1]}"`,
            pass: false,
          };
        }
      }

      return {
        message: () => `expected PDF not to contain form field "${fieldName}"`,
        pass: true,
      };
    } finally {
      inspector.free();
    }
  }
};
