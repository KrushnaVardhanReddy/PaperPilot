export interface SearchSnippetMatch {
  docIndex: number;
  docName: string;
  docType: 'pdf' | 'docx' | 'xlsx' | 'pptx' | 'other';
  location: string; // e.g. "Page 3" or "Sheet 'Q3 Revenue', Row 14"
  pageOrSheetNum: number;
  snippet: string;   // Sentence snippet with highlight markers
  matchIndex: number;
}

export interface WorkspaceSearchResult {
  query: string;
  totalMatches: number;
  matchesByDoc: {
    docIndex: number;
    docName: string;
    matches: SearchSnippetMatch[];
  }[];
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

export function searchDocumentText(
  docIndex: number,
  docName: string,
  text: string,
  query: string,
  options: { caseSensitive?: boolean; wholeWord?: boolean } = {}
): SearchSnippetMatch[] {
  if (!query || !text) return [];

  const matches: SearchSnippetMatch[] = [];

  let regexFlags = 'g';
  if (!options.caseSensitive) regexFlags += 'i';

  // Escape special regex characters
  const escapedQuery = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

  let regexPattern = escapedQuery;
  if (options.wholeWord) {
    // Only match whole words
    regexPattern = `\\b${escapedQuery}\\b`;
  }

  let regex;
  try {
    regex = new RegExp(regexPattern, regexFlags);
  } catch (e) {
    // In case of invalid regex combination, return empty
    return [];
  }

  // Determine docType from docName
  const extMatch = docName.match(/\.([a-zA-Z0-9]+)$/);
  let ext = extMatch ? extMatch[1].toLowerCase() : '';

  let docType: SearchSnippetMatch['docType'] = 'other';
  if (['pdf'].includes(ext)) docType = 'pdf';
  else if (['docx', 'doc'].includes(ext)) docType = 'docx';
  else if (['xlsx', 'xls'].includes(ext)) docType = 'xlsx';
  else if (['pptx', 'ppt'].includes(ext)) docType = 'pptx';

  let match;
  while ((match = regex.exec(text)) !== null) {
    const matchIndex = match.index;
    const matchLength = match[0].length;

    // Generate snippet (40 characters before and after)
    const contextSize = 40;
    const startSnippet = Math.max(0, matchIndex - contextSize);
    const endSnippet = Math.min(text.length, matchIndex + matchLength + contextSize);

    let prefix = text.substring(startSnippet, matchIndex);
    const matchedText = text.substring(matchIndex, matchIndex + matchLength);
    let suffix = text.substring(matchIndex + matchLength, endSnippet);

    // Clean up whitespace in snippets
    prefix = prefix.replace(/\s+/g, ' ');
    suffix = suffix.replace(/\s+/g, ' ');

    // Add ellipsis if not at the beginning/end
    if (startSnippet > 0) prefix = '...' + prefix;
    if (endSnippet < text.length) suffix = suffix + '...';

    const snippet = `${escapeHtml(prefix)}<mark>${escapeHtml(matchedText)}</mark>${escapeHtml(suffix)}`;

    matches.push({
      docIndex,
      docName,
      docType,
      location: 'Document', // Caller can override this if searching page by page
      pageOrSheetNum: 1,    // Caller can override this if searching page by page
      snippet,
      matchIndex
    });
  }

  return matches;
}
