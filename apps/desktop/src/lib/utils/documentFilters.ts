export interface DocumentFacts {
  pageCount: number | null;
  isScanned: boolean | null;
  hasFormFields: boolean | null;
  isSigned: boolean | null;
  isEncrypted: boolean;
  sizeBytes: number;
}
export interface InspectionState { status: 'pending' | 'ready' | 'error' | 'unavailable'; facts?: DocumentFacts; error?: string }
export type DocumentFilter = 'all' | 'scanned' | 'signed' | 'forms' | 'multi';
export const documentFilters: { id: DocumentFilter; label: string }[] = [
  { id: 'all', label: 'All documents' }, { id: 'scanned', label: 'Scanned' }, { id: 'signed', label: 'Signed' }, { id: 'forms', label: 'Form fields' }, { id: 'multi', label: 'Multi-page' }
];
export function matchesDocument(filter: DocumentFilter, inspection?: InspectionState): boolean {
  if (filter === 'all') return true;
  if (inspection?.status !== 'ready' || !inspection.facts) return false;
  const f = inspection.facts;
  return filter === 'scanned' ? f.isScanned === true : filter === 'signed' ? f.isSigned === true : filter === 'forms' ? f.hasFormFields === true : (f.pageCount ?? 0) > 1;
}
