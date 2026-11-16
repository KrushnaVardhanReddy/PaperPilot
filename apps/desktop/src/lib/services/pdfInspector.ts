import { invoke } from '@tauri-apps/api/core';

export interface PdfMetadataDetails {
  title: string;
  author: string;
  subject: string;
  keywords: string;
  creator?: string;
  producer?: string;
  creationDate?: string;
  modificationDate?: string;
}

export interface PdfPermissionDetails {
  isEncrypted: boolean;
  allowPrinting: boolean;
  allowCopying: boolean;
  allowModifying: boolean;
  rawPermissions?: number[] | null;
}

export interface PdfDocumentDetails {
  metadata: PdfMetadataDetails;
  permissions: PdfPermissionDetails;
  pageCount: number;
}

/**
 * Ensures pdfjs-dist worker is properly configured.
 */
async function getPdfJs() {
  const pdfjsLib = await import('pdfjs-dist');
  const pdfjsWorker = (await import('pdfjs-dist/build/pdf.worker.mjs?url')).default;
  if (typeof window !== 'undefined' && !pdfjsLib.GlobalWorkerOptions.workerSrc) {
    pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(pdfjsWorker, window.location.href).href;
  }
  return pdfjsLib;
}

/**
 * Resolves a File, Blob, or file path to an ArrayBuffer.
 */
export async function getPdfBuffer(fileOrPath: File | Blob | string): Promise<ArrayBuffer> {
  if (typeof fileOrPath === 'string') {
    const bytes = await invoke<number[]>('read_file_bytes', { path: fileOrPath });
    return new Uint8Array(bytes).buffer;
  }

  // If it's a File with 0 bytes or not loaded yet, check for a _localPath
  const target = fileOrPath as any;
  if ((target._isLoaded === false || target.size === 0) && target._localPath) {
    const bytes = await invoke<number[]>('read_file_bytes', { path: target._localPath });
    return new Uint8Array(bytes).buffer;
  }

  return await fileOrPath.arrayBuffer();
}

/**
 * Extracts comprehensive details (metadata, permissions, page count) from a PDF in a single pass.
 */
export async function getPdfDetails(fileOrPath: File | Blob | string): Promise<PdfDocumentDetails> {
  const buffer = await getPdfBuffer(fileOrPath);
  if (!buffer || buffer.byteLength === 0) {
    throw new Error('PDF buffer is empty');
  }

  const pdfjs = await getPdfJs();
  const loadingTask = pdfjs.getDocument({ data: buffer });
  const pdf = await loadingTask.promise;

  // 1. Extract Metadata
  const meta = await pdf.getMetadata();
  const info = (meta?.info as Record<string, any>) || {};
  const metadata: PdfMetadataDetails = {
    title: info.Title || '',
    author: info.Author || '',
    subject: info.Subject || '',
    keywords: info.Keywords || '',
    creator: info.Creator || '',
    producer: info.Producer || '',
    creationDate: info.CreationDate || '',
    modificationDate: info.ModDate || '',
  };

  // 2. Extract Permissions
  // In pdf.js: getPermissions() returns null for unencrypted PDFs (all allowed).
  // For protected PDFs, it returns an array of PermissionFlag integers:
  // PRINT = 4, MODIFY_CONTENTS = 8, COPY = 16, PRINT_HIGH_QUALITY = 2048.
  const rawPerms = await pdf.getPermissions();
  let permissions: PdfPermissionDetails;

  if (rawPerms === null || rawPerms === undefined) {
    permissions = {
      isEncrypted: false,
      allowPrinting: true,
      allowCopying: true,
      allowModifying: true,
      rawPermissions: null,
    };
  } else if (Array.isArray(rawPerms)) {
    permissions = {
      isEncrypted: true,
      allowPrinting: rawPerms.includes(4) || rawPerms.includes(2048),
      allowModifying: rawPerms.includes(8),
      allowCopying: rawPerms.includes(16),
      rawPermissions: rawPerms,
    };
  } else {
    permissions = {
      isEncrypted: true,
      allowPrinting: true,
      allowCopying: true,
      allowModifying: true,
      rawPermissions: null,
    };
  }

  return {
    metadata,
    permissions,
    pageCount: pdf.numPages,
  };
}

/**
 * Extracts only metadata from a PDF.
 */
export async function getPdfMetadata(fileOrPath: File | Blob | string): Promise<PdfMetadataDetails> {
  const details = await getPdfDetails(fileOrPath);
  return details.metadata;
}

/**
 * Extracts only permissions from a PDF.
 */
export async function getPdfPermissions(fileOrPath: File | Blob | string): Promise<PdfPermissionDetails> {
  const details = await getPdfDetails(fileOrPath);
  return details.permissions;
}

