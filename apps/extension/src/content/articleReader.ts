/**
 * PaperPilot Clean Article Reader
 * Extracts primary article content from web pages, strips clutter, ads, navigation,
 * and converts semantic DOM nodes into clean Markdown.
 */

export interface ArticleResult {
  title: string;
  byline?: string;
  url: string;
  markdown: string;
  wordCount: number;
  excerpt: string;
  timestamp: string;
}

const UNWANTED_SELECTORS = [
  'script',
  'style',
  'noscript',
  'template',
  'nav',
  'aside',
  'footer',
  'header',
  'svg',
  'iframe',
  'canvas',
  'form',
  'button',
  'input',
  'select',
  'textarea',
  '[aria-hidden="true"]',
  '.ad',
  '.ads',
  '.advertisement',
  '.ad-banner',
  '.ad-container',
  '.ad-wrapper',
  '.cookie-notice',
  '.cookie-banner',
  '.consent-banner',
  '.sidebar',
  '.social-share',
  '.share-buttons',
  '.comments',
  '.related-posts',
  '.newsletter-signup',
];

/**
 * Extract clean article content and convert to Markdown
 */
export function extractArticleMarkdown(doc: Document = document): ArticleResult {
  const url = doc.defaultView?.location?.href || '';
  const timestamp = new Date().toISOString();

  // 1. Extract title
  const ogTitle = doc.querySelector('meta[property="og:title"]')?.getAttribute('content');
  const h1Text = doc.querySelector('h1')?.textContent?.trim();
  const rawTitle = ogTitle || h1Text || doc.title || 'Untitled Document';
  const title = rawTitle.replace(/\s+/g, ' ').trim();

  // 2. Extract author/byline
  const metaAuthor =
    doc.querySelector('meta[name="author"]')?.getAttribute('content') ||
    doc.querySelector('meta[property="article:author"]')?.getAttribute('content') ||
    doc.querySelector('.author, .byline, [rel="author"]')?.textContent?.trim();

  // 3. Find primary content container
  let container =
    doc.querySelector('article') ||
    doc.querySelector('[role="main"]') ||
    doc.querySelector('main') ||
    doc.querySelector('.article-content, .entry-content, .post-content, .story-body, #content') ||
    doc.body;

  // Clone container to avoid modifying live page DOM
  const clone = container.cloneNode(true) as HTMLElement;

  // 4. Strip unwanted elements
  for (const selector of UNWANTED_SELECTORS) {
    const junk = clone.querySelectorAll(selector);
    junk.forEach((el) => el.remove());
  }

  // 5. Convert DOM tree to Markdown
  const bodyMarkdown = nodeToMarkdown(clone).trim();

  // 6. Assemble complete document with metadata header
  let fullMarkdown = '';
  if (title) {
    fullMarkdown += `# ${title}\n\n`;
  }
  if (metaAuthor) {
    fullMarkdown += `**By ${metaAuthor}**\n\n`;
  }
  if (url) {
    fullMarkdown += `> Source: [${url}](${url})\n> Captured: ${timestamp}\n\n---\n\n`;
  }
  fullMarkdown += bodyMarkdown;

  // Clean excessive blank lines
  fullMarkdown = fullMarkdown.replace(/\n{3,}/g, '\n\n').trim();

  // Calculate statistics
  const words = fullMarkdown.split(/\s+/).filter(Boolean);
  const wordCount = words.length;
  const excerpt = words.slice(0, 40).join(' ') + (words.length > 40 ? '...' : '');

  return {
    title,
    byline: metaAuthor || undefined,
    url,
    markdown: fullMarkdown,
    wordCount,
    excerpt,
    timestamp,
  };
}

/**
 * Recursive DOM to Markdown converter
 */
function nodeToMarkdown(node: Node): string {
  if (node.nodeType === Node.TEXT_NODE) {
    return node.textContent || '';
  }

  if (node.nodeType !== Node.ELEMENT_NODE) {
    return '';
  }

  const el = node as HTMLElement;
  const tag = el.tagName.toLowerCase();

  const childMarkdown = Array.from(el.childNodes)
    .map((child) => nodeToMarkdown(child))
    .join('');

  switch (tag) {
    case 'h1':
      return `\n\n# ${childMarkdown.trim()}\n\n`;
    case 'h2':
      return `\n\n## ${childMarkdown.trim()}\n\n`;
    case 'h3':
      return `\n\n### ${childMarkdown.trim()}\n\n`;
    case 'h4':
      return `\n\n#### ${childMarkdown.trim()}\n\n`;
    case 'h5':
      return `\n\n##### ${childMarkdown.trim()}\n\n`;
    case 'h6':
      return `\n\n###### ${childMarkdown.trim()}\n\n`;

    case 'p':
      return `\n\n${childMarkdown.trim()}\n\n`;

    case 'br':
      return '\n';

    case 'hr':
      return '\n\n---\n\n';

    case 'strong':
    case 'b':
      return childMarkdown.trim() ? `**${childMarkdown.trim()}**` : '';

    case 'em':
    case 'i':
      return childMarkdown.trim() ? `*${childMarkdown.trim()}*` : '';

    case 'code':
      if (el.parentElement?.tagName.toLowerCase() === 'pre') {
        return childMarkdown;
      }
      return childMarkdown.trim() ? `\`${childMarkdown.trim()}\`` : '';

    case 'pre': {
      const codeLang = el.querySelector('code')?.getAttribute('class') || '';
      const langMatch = codeLang.match(/language-(\w+)/);
      const lang = langMatch ? langMatch[1] : '';
      return `\n\n\`\`\`${lang}\n${el.textContent?.trim() || ''}\n\`\`\`\n\n`;
    }

    case 'blockquote':
      return `\n\n> ${childMarkdown.trim().replace(/\n/g, '\n> ')}\n\n`;

    case 'a': {
      const href = el.getAttribute('href');
      const text = childMarkdown.trim() || href || '';
      if (!href || href.startsWith('javascript:')) return text;
      return `[${text}](${href})`;
    }

    case 'img': {
      const src = el.getAttribute('src');
      const alt = el.getAttribute('alt') || 'image';
      if (!src) return '';
      return `\n\n![${alt}](${src})\n\n`;
    }

    case 'ul':
      return `\n\n${formatListItems(el, false)}\n\n`;

    case 'ol':
      return `\n\n${formatListItems(el, true)}\n\n`;

    case 'table':
      return `\n\n${formatTable(el)}\n\n`;

    case 'div':
    case 'section':
      return `\n${childMarkdown}\n`;

    default:
      return childMarkdown;
  }
}

function formatListItems(listEl: HTMLElement, isOrdered: boolean): string {
  const items = Array.from(listEl.children).filter((c) => c.tagName.toLowerCase() === 'li');
  return items
    .map((li, index) => {
      const prefix = isOrdered ? `${index + 1}. ` : '- ';
      const text = nodeToMarkdown(li).trim().replace(/\n+/g, ' ');
      return `${prefix}${text}`;
    })
    .join('\n');
}

function formatTable(tableEl: HTMLElement): string {
  const rows = Array.from(tableEl.querySelectorAll('tr'));
  if (rows.length === 0) return '';

  const tableData: string[][] = [];

  for (const row of rows) {
    const cells = Array.from(row.querySelectorAll('th, td')).map((cell) =>
      (cell.textContent || '').trim().replace(/\|/g, '\\|').replace(/\n+/g, ' ')
    );
    if (cells.length > 0) {
      tableData.push(cells);
    }
  }

  if (tableData.length === 0) return '';

  const maxCols = Math.max(...tableData.map((r) => r.length));
  // Pad shorter rows
  const normalized = tableData.map((r) => {
    while (r.length < maxCols) r.push('');
    return r;
  });

  const headerRow = `| ${normalized[0].join(' | ')} |`;
  const separatorRow = `| ${normalized[0].map(() => '---').join(' | ')} |`;
  const dataRows = normalized.slice(1).map((r) => `| ${r.join(' | ')} |`);

  return [headerRow, separatorRow, ...dataRows].join('\n');
}
