import { createHash } from 'node:crypto';
import { existsSync } from 'node:fs';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import type { RoadmapCategory, RoadmapDocument, RoadmapItem, RoadmapStatus } from '../apps/web/src/lib/roadmap.ts';

const REPO_URL = 'https://github.com/KrushnaVardhanReddy/PaperPilot';
const TASK_ID = /^(?:Task\s+\d+[a-z]?|(?:\d+|[A-Z])(?:\.[\da-z]+)+)$/i;

export function plainText(text: string): string {
  return text.replace(/<br\s*\/?\s*>/gi, ' ')
    .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
    .replace(/<[^>]+>/g, '')
    .replace(/[`*~]/g, '')
    .replace(/\\\|/g, '|')
    .replace(/\s+/g, ' ').trim();
}

export function splitTableRow(line: string): string[] {
  const cells: string[] = [];
  let cell = '';
  let code = false;
  const text = line.trim().replace(/^\|/, '').replace(/\|$/, '');
  for (let index = 0; index < text.length; index++) {
    const char = text[index];
    if (char === '`' && text[index - 1] !== '\\') code = !code;
    if (char === '|' && !code && text[index - 1] !== '\\') {
      cells.push(cell.trim());
      cell = '';
    } else cell += char;
  }
  cells.push(cell.trim());
  return cells;
}

export function readStatus(text: string): RoadmapStatus | undefined {
  const clean = plainText(text).replace(/^[\s✅✓✔🚀📋⏳🔄🏗️💡]+/u, '').trim();
  // Explicit notes override stale emoji/title markers. Completion is not shipment while unmerged.
  if (/\b(unmerged|pending (?:review|integration|merge)|awaiting (?:review|merge))\b/i.test(clean)) return 'in_progress';
  if (/^(?:not (?:started|completed)|planned|backlog|awaiting|not yet|future|to do)\b/i.test(clean)) return 'planned';
  if (/^(?:completed?|merged|done|verified|passed|implemented)\b/i.test(clean)) return 'shipped';
  if (/^(?:active(?:\s+in[- ]flight)?|in[- ](?:progress|flight)|wip|started|scaffolded|in local progress)\b/i.test(clean)) return 'in_progress';
  if (/[✅✓✔]/u.test(text) || /\[COMPLETED\]/i.test(text)) return 'shipped';
  if (/[🚀🔄🏗]/u.test(text) || /\b(?:in[- ]progress|in[- ]flight)\b/i.test(clean)) return 'in_progress';
  if (/[📋💡]/u.test(text)) return 'planned';
  return undefined;
}

function categoryFor(phase: number, title: string, section: string): RoadmapCategory {
  if (phase === 6) return 'security';
  if (phase === 3) return 'desktop';
  if (phase === 10 || phase === 7) return 'web';
  if (phase === 2) return 'ai';
  const text = `${title} ${section}`;
  if (/security|signature|encrypt|decrypt|redact|sanitize|pii|audit|license|sso|rbac|compliance|permission|drm|fuzz/i.test(text)) return 'security';
  if (/desktop|tauri|native menu|titlebar|viewer|thumbnail|mobile touch/i.test(text)) return 'desktop';
  if (/wasm|webassembly|web portal|browser|cloudflare|embed|webgpu|public roadmap/i.test(text)) return 'web';
  if (phase === 4 || phase === 5 || /\bai\b|\bslm\b|\bllm\b|agent|mcp|rag|ocr|semantic|nlp/i.test(text)) return 'ai';
  return 'core';
}

function taskTitle(text: string): string {
  return plainText(text)
    .replace(/\[(?:Spec\s+\d+[^\]]*|PARALLEL[^\]]*|BLOCKED BY[^\]]*|COMPLETED)\]/gi, '')
    .replace(/[✅✓✔🚀📋⏳🔄🏗️💡]/gu, '').replace(/\s+/g, ' ').trim();
}

export function parsePhase(markdown: string, sourcePath: string): RoadmapItem[] {
  const phase = Number(/phase-(\d+)/.exec(sourcePath)?.[1]);
  if (!phase) throw new Error(`Invalid phase filename: ${sourcePath}`);
  const phaseHeading = markdown.match(/^#{1,2}\s+Phase\s+\d+\s*[—–-]\s*(.+)$/m)?.[1] ?? `Phase ${phase}`;
  const phaseName = plainText(phaseHeading).replace(/\s*\[[^\]]+\]/g, '').trim();
  const lines = markdown.split(/\r?\n/);
  const items: RoadmapItem[] = [];
  const headings: { level: number; title: string; status?: RoadmapStatus }[] = [];
  let fence: string | undefined;
  const headingCandidates: { line: number; level: number; id: string; title: string; status?: RoadmapStatus; context: typeof headings }[] = [];

  function addItem(id: string, title: string, notes: string, line: number, status?: RoadmapStatus, context = headings) {
    const raw = `${title} ${notes}`;
    const pr = /(?:PR\s*#|github\.com\/KrushnaVardhanReddy\/PaperPilot\/pull\/)(\d+)/i.exec(raw)?.[1];
    const spec = /specs\/[\w/-]+\.md/.exec(raw)?.[0];
    const inherited = [...context].reverse().find(heading => heading.status)?.status;
    const cleanTitle = taskTitle(title);
    if (!cleanTitle) return;
    items.push({
      key: `${sourcePath}:${line}`,
      id, title: cleanTitle, summary: plainText(notes), phase, phase_name: phaseName,
      status: readStatus(notes) ?? status ?? readStatus(title) ?? inherited ?? 'planned',
      category: categoryFor(phase, cleanTitle, context.map(heading => heading.title).join(' ')),
      source_path: sourcePath, source_line: line,
      ...(pr ? { pr_url: `${REPO_URL}/pull/${pr}` } : {}),
      ...(spec ? { spec_path: spec } : {}),
    });
  }

  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    const marker = /^\s*(`{3,}|~{3,})/.exec(line)?.[1];
    if (marker) {
      if (!fence) fence = marker;
      else if (marker[0] === fence[0] && marker.length >= fence.length) fence = undefined;
      continue;
    }
    if (fence) continue;
    const heading = /^(#{1,6})\s+(.+)$/.exec(line);
    if (heading) {
      const level = heading[1].length;
      while (headings.length && headings[headings.length - 1].level >= level) headings.pop();
      headings.push({ level, title: heading[2], status: readStatus(heading[2]) });
      const match = /^(?:Phase\s+)?([\dA-Z]+(?:\.[\da-z]+)+)\s*[—–:-]\s*(.+)$/i.exec(plainText(heading[2]));
      if (match) headingCandidates.push({ line: index + 1, level, id: match[1], title: match[2], status: readStatus(heading[2]), context: [...headings] });
      continue;
    }
    if (line.trim().startsWith('|')) {
      const cells = splitTableRow(line);
      const id = plainText(cells[0] ?? '').replace(/[✅✓✔🚀📋⏳🔄]/gu, '').trim();
      if (cells.length >= 2 && TASK_ID.test(id)) addItem(id, cells[1], cells.slice(2).join(' · '), index + 1, /~~/.test(cells[0]) ? 'shipped' : readStatus(cells[0]));
      continue;
    }
    const bullet = /^\s*[-*]\s+(?:\[([ xX])\]\s*)?(?:\*\*)?((?:Task\s+\d+[a-z]?)|(?:[\dA-Z]+(?:\.[\da-z]+)+))(?:\*\*)?\s*[—–:.)-]\s*(.+)$/i.exec(line);
    if (bullet) addItem(bullet[2], bullet[3], '', index + 1, bullet[1]?.toLowerCase() === 'x' ? 'shipped' : readStatus(line));
  }

  // Text-only milestones remain visible without duplicating headings that already contain tasks.
  for (const heading of headingCandidates) {
    let end = lines.length;
    for (let index = heading.line; index < lines.length; index++) {
      const next = /^(#{1,6})\s/.exec(lines[index]);
      if (next && next[1].length <= heading.level) { end = index; break; }
    }
    if (items.some(item => item.source_line > heading.line && item.source_line <= end)) continue;
    if (headingCandidates.some(child => child.line > heading.line && child.line <= end)) continue;
    const summary = lines.slice(heading.line, end).filter(line => line.trim() && !/^[#|`~]/.test(line.trim())).join(' ');
    addItem(heading.id, heading.title, summary, heading.line, heading.status, heading.context);
  }
  return items.sort((left, right) => left.source_line - right.source_line);
}

export function findRepoRoot(start = process.cwd()): string {
  let current = path.resolve(start);
  while (!existsSync(path.join(current, 'phases')) || !existsSync(path.join(current, 'apps', 'web'))) {
    const parent = path.dirname(current);
    if (parent === current) throw new Error('Run sync-roadmap inside the PaperPilot repository');
    current = parent;
  }
  return current;
}

export async function syncRoadmap(root = findRepoRoot()): Promise<RoadmapDocument> {
  const files = (await readdir(path.join(root, 'phases'))).filter(file => /^phase-\d+[-\w]*\.md$/.test(file)).sort();
  if (!files.length) throw new Error('No phase documents found');
  const hash = createHash('sha256');
  const items: RoadmapItem[] = [];
  for (const file of files) {
    const source = `phases/${file}`;
    const content = await readFile(path.join(root, source), 'utf8');
    hash.update(source).update('\0').update(content.replace(/\r\n/g, '\n')).update('\0');
    items.push(...parsePhase(content, source));
  }
  if (!items.length) throw new Error('No milestones parsed; refusing to replace the roadmap snapshot');
  const dashboard = await readFile(path.join(root, 'PHASES.md'), 'utf8');
  const count = /\b(\d+)\+?\s+(?:verified\s+tools|MCP\s+tools)/i.exec(dashboard)?.[1];
  hash.update(dashboard.replace(/\r\n/g, '\n'));
  const document: RoadmapDocument = {
    schema_version: 1, revision: hash.digest('hex').slice(0, 16),
    tool_count: count ? Number(count) : null,
    source_files: files.map(file => `phases/${file}`), items,
  };
  const output = path.join(root, 'apps', 'web', 'public', 'roadmap.json');
  await mkdir(path.dirname(output), { recursive: true });
  await writeFile(output, `${JSON.stringify(document, null, 2)}\n`, 'utf8');
  return document;
}

if (process.argv[1] && /(?:^|[\\/])sync_roadmap\.ts$/.test(process.argv[1])) {
  syncRoadmap().then(document => {
    const counts = Object.fromEntries(['shipped', 'in_progress', 'planned'].map(status => [status, document.items.filter(item => item.status === status).length]));
    console.log(`Roadmap synced: ${document.items.length} milestones from ${document.source_files.length} phases`, counts);
  }).catch(error => { console.error(error); process.exitCode = 1; });
}
