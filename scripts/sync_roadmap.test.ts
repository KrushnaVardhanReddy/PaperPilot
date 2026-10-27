import assert from 'node:assert/strict';
import { mkdtemp, readFile, rm, writeFile, mkdir } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { findRepoRoot, parsePhase, readStatus, splitTableRow, syncRoadmap } from './sync_roadmap.ts';
import { featureRequestUrl, isRoadmapDocument, matchesRoadmapItem } from '../apps/web/src/lib/roadmap.ts';

const source = 'phases/phase-05-intelligence.md';

test('explicit notes override stale title markers and pending review is not shipped', () => {
  const items = parsePhase([
    '## Phase 5 — Intelligence',
    '| # | Task | Notes |', '|---|---|---|',
    '| 5.1.1 | First 🚀 | ✅ Completed (PR #250) |',
    '| 5.1.2 | Second ⏳ | Planned — after the completed parser |',
    '| 5.1.3 | Third ✅ | Implemented locally; unmerged |',
    '| 5.1.4 | Fourth | In Progress |',
  ].join('\n'), source);
  assert.deepEqual(items.map(item => item.status), ['shipped', 'planned', 'in_progress', 'in_progress']);
  assert.equal(items[0].pr_url, 'https://github.com/KrushnaVardhanReddy/PaperPilot/pull/250');
  assert.equal(readStatus('Not completed'), 'planned');
});

test('table parsing preserves escaped pipes, code spans and Markdown metadata', () => {
  assert.deepEqual(splitTableRow('| 5.1 | `PDF | DOCX` | A \\| B |'), ['5.1', '`PDF | DOCX`', 'A \\| B']);
  const [item] = parsePhase('| ~~5.1.1~~ ✓ | **Extract** `tables` | [Spec](specs/090-table/spec.md)<br>Two \\| columns |', source);
  assert.equal(item.title, 'Extract tables');
  assert.equal(item.summary, 'Spec Two | columns');
  assert.equal(item.spec_path, 'specs/090-table/spec.md');
  assert.equal(item.status, 'shipped');
});

test('code samples and summary tables do not become tasks', () => {
  const items = parsePhase([
    '## Phase 5 — Intelligence', '```md',
    '| 5.9.1 | Not a real task | Completed |', '```',
    '| Phase | Summary | Status |', '| 5 | A phase | Completed |',
    '| 5.1.1 | A real task | Planned |',
  ].join('\n'), source);
  assert.deepEqual(items.map(item => item.title), ['A real task']);
});

test('duplicate task numbers retain unique keys and source references', () => {
  const items = parsePhase('| 5.1.1 | One | Completed |\n| 5.1.1 | Two | Planned |', source);
  assert.equal(items.length, 2);
  assert.notEqual(items[0].key, items[1].key);
  assert.deepEqual(items.map(item => item.source_line), [1, 2]);
  const [other] = parsePhase('| 5.1.1 | Three | Planned |', 'phases/phase-13-universal.md');
  assert.equal(other.phase, 13);
  assert.notEqual(other.key, items[0].key);
});

test('text-only milestones and checklists use their own heading status', () => {
  const items = parsePhase([
    '## Phase 5 — Intelligence', '### 5.1 — OCR [COMPLETED]', 'Searchable scans.',
    '### 5.2 — Parsing', '- [x] Task 129A: Harden parser',
    '### 5.3 — Translation', 'Planned — preserve layout.',
  ].join('\n'), source);
  assert.deepEqual(items.map(item => [item.id, item.status]), [
    ['5.1', 'shipped'], ['Task 129A', 'shipped'], ['5.3', 'planned'],
  ]);
});

test('parent headings are not duplicated when child milestones exist', () => {
  const items = parsePhase('## Phase 5 — Intelligence\n### 5.1 — OCR\n| 5.1.1 | Models | Planned |', source);
  assert.deepEqual(items.map(item => item.id), ['5.1.1']);
});

test('filters and GitHub issue form fields preserve task identity and special characters', () => {
  const [item] = parsePhase('| 5.1.1 | OCR & extraction / CJK | Planned — text + tables |', source);
  assert(matchesRoadmapItem(item, 'ocr tables', 'ai'));
  assert(!matchesRoadmapItem(item, '', 'desktop'));
  const url = new URL(featureRequestUrl(item));
  assert.equal(url.origin, 'https://github.com');
  assert.equal(url.searchParams.get('template'), 'feature_request.yml');
  assert.equal(url.searchParams.get('title'), '[FEATURE]: OCR & extraction / CJK');
  assert.match(url.searchParams.get('problem') ?? '', /5\.1\.1/);
  assert.match(url.searchParams.get('proposed_solution') ?? '', /phases\/phase-05-intelligence.md#L1/);
});

test('sync is deterministic, works from nested directories and reflects status edits', async (t) => {
  const root = await mkdtemp(path.join(tmpdir(), 'paperpilot-roadmap-'));
  t.after(async () => {
    assert.equal(path.dirname(path.resolve(root)), path.resolve(tmpdir()));
    assert(path.basename(root).startsWith('paperpilot-roadmap-'));
    await rm(root, { recursive: true, force: true });
  });
  await mkdir(path.join(root, 'phases'));
  await writeFile(path.join(root, 'PHASES.md'), '78 verified tools');
  const file = path.join(root, source);
  await writeFile(file, '## Phase 5 — Intelligence\n| 5.1.1 | OCR | Planned |');
  const first = await syncRoadmap(root);
  assert.equal(first.tool_count, 78);
  assert(isRoadmapDocument(first));
  assert.equal(findRepoRoot(path.join(root, 'apps', 'web')), root);
  assert.deepEqual(await syncRoadmap(root), first);
  await writeFile(file, '## Phase 5 — Intelligence\n| 5.1.1 | OCR | Completed (PR #1) |');
  const updated = await syncRoadmap(root);
  assert.notEqual(updated.revision, first.revision);
  assert.equal(updated.items[0].status, 'shipped');
  assert.deepEqual(JSON.parse(await readFile(path.join(root, 'apps/web/public/roadmap.json'), 'utf8')), updated);
  assert(!isRoadmapDocument({ ...updated, items: [{ ...updated.items[0], status: 'unknown' }] }));
  assert(!isRoadmapDocument({ ...updated, tool_count: '78' }));
  assert(!isRoadmapDocument({ ...updated, items: [updated.items[0], updated.items[0]] }));
  await writeFile(file, '## Phase 5 — No parseable tasks');
  await assert.rejects(syncRoadmap(root), /No milestones parsed/);
  assert.deepEqual(JSON.parse(await readFile(path.join(root, 'apps/web/public/roadmap.json'), 'utf8')), updated);
});
