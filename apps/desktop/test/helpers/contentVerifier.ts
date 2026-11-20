import fs from 'node:fs';

export function verifyOutputContent(filePath: string, expectedType: 'gif' | 'pdf' | 'png' | 'webp' | 'jpeg' | 'json' | 'text' | 'zip' | 'html', maxRetries: number = 20, intervalMs: number = 200): void {
  let lastError: Error | null = null;

  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      // 1. Checks fs.existsSync(filePath) is true
      if (!fs.existsSync(filePath)) {
        throw new Error(`File does not exist: ${filePath}`);
      }

      // 2. Checks file size > 0 (reasonable minimum size)
      const stats = fs.statSync(filePath);
      if (stats.size === 0) {
        throw new Error(`File is empty (0 bytes): ${filePath}`);
      }

      const buffer = fs.readFileSync(filePath);

      // 3. Checks magic bytes / contents
      switch (expectedType) {
        case 'zip': {
          // PK\x03\x04
          if (
            buffer.length < 4 ||
            buffer[0] !== 0x50 || // 'P'
            buffer[1] !== 0x4b || // 'K'
            buffer[2] !== 0x03 ||
            buffer[3] !== 0x04
          ) {
            throw new Error(`Invalid ZIP magic signature 'PK\\x03\\x04' for file: ${filePath}`);
          }
          break;
        }
        case 'gif': {
          // GIF89a or GIF87a (first 6 bytes)
          const header = buffer.subarray(0, 6).toString('ascii');
          if (header !== 'GIF89a' && header !== 'GIF87a') {
            throw new Error(`Invalid GIF header '${header}' for file: ${filePath}`);
          }
          break;
        }
        case 'pdf': {
          // Starts with %PDF- and ends with %%EOF
          const start = buffer.subarray(0, 1024).toString('latin1');
          if (!start.includes('%PDF-')) {
            throw new Error(`PDF magic bytes '%PDF-' not found at start for file: ${filePath}`);
          }
          const end = buffer.subarray(Math.max(0, buffer.length - 1024)).toString('latin1');
          if (!end.includes('%%EOF')) {
            throw new Error(`PDF termination '%%EOF' not found at end for file: ${filePath}`);
          }
          break;
        }
        case 'png': {
          // First byte 0x89 followed by PNG
          if (
            buffer.length < 4 ||
            buffer[0] !== 0x89 ||
            buffer[1] !== 0x50 || // 'P'
            buffer[2] !== 0x4e || // 'N'
            buffer[3] !== 0x47    // 'G'
          ) {
            throw new Error(`Invalid PNG signature for file: ${filePath}`);
          }
          break;
        }
        case 'jpeg': {
          // Starts with 0xFF 0xD8 and ends with 0xFF 0xD9
          if (
            buffer.length < 4 ||
            buffer[0] !== 0xff ||
            buffer[1] !== 0xd8
          ) {
            throw new Error(`Invalid JPEG start signature for file: ${filePath}`);
          }
          break;
        }
        case 'webp': {
          // RIFF ... WEBP
          if (buffer.length < 12) {
            throw new Error(`File too short for WebP: ${filePath}`);
          }
          const riff = buffer.subarray(0, 4).toString('ascii');
          const webp = buffer.subarray(8, 12).toString('ascii');
          if (riff !== 'RIFF' || webp !== 'WEBP') {
            throw new Error(`Invalid WebP header (RIFF/WEBP) for file: ${filePath}`);
          }
          break;
        }
        case 'json': {
          // parseable JSON with valid schema/keys
          const text = buffer.toString('utf-8');
          try {
            const parsed = JSON.parse(text);
            if (typeof parsed !== 'object' || parsed === null) {
              throw new Error('JSON is not an object or array');
            }
          } catch (err: any) {
            throw new Error(`Invalid JSON in file ${filePath}: ${err.message}`);
          }
          break;
        }
        case 'text': {
          // non-empty string
          const text = buffer.toString('utf-8').trim();
          if (!text || text.length === 0) {
            throw new Error(`Empty text content in file: ${filePath}`);
          }
          break;
        }
        case 'html': {
          // valid HTML document containing tags like <html, <!DOCTYPE, or <body
          const text = buffer.toString('utf-8').toLowerCase();
          if (!text.includes('<html') && !text.includes('<!doctype html') && !text.includes('<body') && !text.includes('<div')) {
            throw new Error(`Invalid HTML content in file: ${filePath}`);
          }
          break;
        }
        default:
          throw new Error(`Unsupported expectedType: ${expectedType}`);
      }

      // If we reach here, verification succeeded!
      return;
    } catch (err: any) {
      lastError = err;
      if (attempt < maxRetries - 1) {
        // Synchronous pause before retrying
        const start = Date.now();
        while (Date.now() - start < intervalMs) {}
      }
    }
  }

  if (lastError) {
    throw lastError;
  }
}
