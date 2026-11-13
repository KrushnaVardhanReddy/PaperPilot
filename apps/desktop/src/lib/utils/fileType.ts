export type FileCategory = 'pdf' | 'spreadsheet' | 'document' | 'presentation' | 'image' | 'other';

export interface FileTypeInfo {
  category: FileCategory;
  mimeType: string;
  icon: string;
  label: string;
  isPdf: boolean;
}

const EXTENSION_MAP: Record<string, { category: FileCategory; mimeType: string; icon: string; label: string }> = {
  pdf: {
    category: 'pdf',
    mimeType: 'application/pdf',
    icon: '📄',
    label: 'PDF Document'
  },
  xlsx: {
    category: 'spreadsheet',
    mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    icon: '📊',
    label: 'Excel Spreadsheet (.xlsx)'
  },
  xls: {
    category: 'spreadsheet',
    mimeType: 'application/vnd.ms-excel',
    icon: '📊',
    label: 'Excel 97-2004 (.xls)'
  },
  csv: {
    category: 'spreadsheet',
    mimeType: 'text/csv',
    icon: '📊',
    label: 'CSV Data File (.csv)'
  },
  tsv: {
    category: 'spreadsheet',
    mimeType: 'text/tab-separated-values',
    icon: '📊',
    label: 'TSV Data File (.tsv)'
  },
  docx: {
    category: 'document',
    mimeType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    icon: '📝',
    label: 'Word Document (.docx)'
  },
  doc: {
    category: 'document',
    mimeType: 'application/msword',
    icon: '📝',
    label: 'Word 97-2004 (.doc)'
  },
  pptx: {
    category: 'presentation',
    mimeType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    icon: '📽️',
    label: 'PowerPoint (.pptx)'
  },
  ppt: {
    category: 'presentation',
    mimeType: 'application/vnd.ms-powerpoint',
    icon: '📽️',
    label: 'PowerPoint 97-2004 (.ppt)'
  },
  png: {
    category: 'image',
    mimeType: 'image/png',
    icon: '🖼️',
    label: 'PNG Image (.png)'
  },
  jpg: {
    category: 'image',
    mimeType: 'image/jpeg',
    icon: '🖼️',
    label: 'JPEG Image (.jpg)'
  },
  jpeg: {
    category: 'image',
    mimeType: 'image/jpeg',
    icon: '🖼️',
    label: 'JPEG Image (.jpeg)'
  },
  webp: {
    category: 'image',
    mimeType: 'image/webp',
    icon: '🖼️',
    label: 'WebP Image (.webp)'
  },
  gif: {
    category: 'image',
    mimeType: 'image/gif',
    icon: '🖼️',
    label: 'GIF Image (.gif)'
  },
  bmp: {
    category: 'image',
    mimeType: 'image/bmp',
    icon: '🖼️',
    label: 'Bitmap Image (.bmp)'
  },
  tiff: {
    category: 'image',
    mimeType: 'image/tiff',
    icon: '🖼️',
    label: 'TIFF Image (.tiff)'
  },
  tif: {
    category: 'image',
    mimeType: 'image/tiff',
    icon: '🖼️',
    label: 'TIFF Image (.tif)'
  }
};

export function getFileTypeInfo(fileName: string): FileTypeInfo {
  const ext = (fileName || '').split('.').pop()?.toLowerCase() ?? '';
  const match = EXTENSION_MAP[ext];
  if (match) {
    return {
      ...match,
      isPdf: match.category === 'pdf'
    };
  }

  return {
    category: 'other',
    mimeType: 'application/octet-stream',
    icon: '📁',
    label: ext ? ext.toUpperCase() + ' File' : 'Binary File',
    isPdf: false
  };
}

export function getMimeType(fileName: string): string {
  return getFileTypeInfo(fileName).mimeType;
}

export function isPdfFile(fileName: string): boolean {
  return getFileTypeInfo(fileName).isPdf;
}

export function isSpreadsheetFile(fileName: string): boolean {
  return getFileTypeInfo(fileName).category === 'spreadsheet';
}

export function isImageFile(fileName: string): boolean {
  return getFileTypeInfo(fileName).category === 'image';
}

export function isJsonFile(fileName: string): boolean {
  const ext = (fileName || '').split('.').pop()?.toLowerCase() ?? '';
  return ext === 'json';
}

export function isMarkdownFile(fileName: string): boolean {
  const ext = (fileName || '').split('.').pop()?.toLowerCase() ?? '';
  return ext === 'md' || ext === 'markdown';
}

export function isHtmlFile(fileName: string): boolean {
  const ext = (fileName || '').split('.').pop()?.toLowerCase() ?? '';
  return ext === 'html' || ext === 'htm';
}

export function isDocxFile(fileName: string): boolean {
  const ext = (fileName || '').split('.').pop()?.toLowerCase() ?? '';
  return ext === 'docx' || ext === 'doc';
}

export function getFileIcon(fileName: string): string {
  return getFileTypeInfo(fileName).icon;
}
