import { GITHUB_REPO_URL } from '../constants';

export const roadmapCategories = [
  { id: 'all', label: 'All' },
  { id: 'core', label: 'Core Engine' },
  { id: 'ai', label: 'AI & Agents' },
  { id: 'web', label: 'Web & WASM' },
  { id: 'desktop', label: 'Desktop' },
  { id: 'security', label: 'Security' },
] as const;

export type RoadmapCategory = Exclude<(typeof roadmapCategories)[number]['id'], 'all'>;
export type RoadmapStatus = 'shipped' | 'in_progress' | 'planned';

export interface RoadmapItem {
  key: string;
  id: string;
  title: string;
  summary: string;
  phase: number;
  phase_name: string;
  category: RoadmapCategory;
  status: RoadmapStatus;
  source_path: string;
  source_line: number;
  pr_url?: string;
  spec_path?: string;
}

export interface RoadmapDocument {
  schema_version: 1;
  revision: string;
  tool_count: number | null;
  source_files: string[];
  items: RoadmapItem[];
}

export function matchesRoadmapItem(item: RoadmapItem, query: string, category: string): boolean {
  if (category !== 'all' && item.category !== category) return false;
  const text = `${item.id} ${item.title} ${item.summary} ${item.phase_name}`.toLowerCase();
  return query.trim().toLowerCase().split(/\s+/).every(word => text.includes(word));
}

export function featureRequestUrl(item: RoadmapItem): string {
  const url = new URL(`${GITHUB_REPO_URL}/issues/new`);
  url.searchParams.set('template', 'feature_request.yml');
  url.searchParams.set('title', `[FEATURE]: ${item.title}`.slice(0, 240));
  // These keys match the text-field IDs in .github/ISSUE_TEMPLATE/feature_request.yml.
  url.searchParams.set('problem', `I would like to support roadmap milestone ${item.id}: ${item.title}.\n\n${item.summary.slice(0, 800)}`);
  url.searchParams.set('proposed_solution', `Implement ${item.title}.\n\nRoadmap reference: ${roadmapSourceUrl(item)}`);
  return url.toString();
}

export function roadmapSourceUrl(item: RoadmapItem): string {
  return `${GITHUB_REPO_URL}/blob/feature/dev/${item.source_path}#L${item.source_line}`;
}

export function isRoadmapDocument(value: unknown): value is RoadmapDocument {
  if (!value || typeof value !== 'object') return false;
  const doc = value as Partial<RoadmapDocument>;
  const keys = new Set<string>();
  return doc.schema_version === 1 && typeof doc.revision === 'string'
    && (doc.tool_count === null || (typeof doc.tool_count === 'number' && Number.isInteger(doc.tool_count) && doc.tool_count >= 0))
    && Array.isArray(doc.source_files) && doc.source_files.every(file => typeof file === 'string' && /^phases\/phase-[\w-]+\.md$/.test(file))
    && Array.isArray(doc.items) && doc.items.every(item => {
    if (!item || typeof item !== 'object' || typeof item.key !== 'string' || keys.has(item.key)) return false;
    keys.add(item.key);
    return ['id', 'title', 'summary', 'phase_name', 'source_path'].every(key => typeof item[key as keyof RoadmapItem] === 'string')
      && Number.isInteger(item.phase) && Number.isInteger(item.source_line) && item.source_line > 0
      && /^phases\/phase-[\w-]+\.md$/.test(item.source_path)
      && ['shipped', 'in_progress', 'planned'].includes(item.status)
      && roadmapCategories.some(category => category.id === item.category)
      && (!item.pr_url || /^https:\/\/github\.com\/KrushnaVardhanReddy\/PaperPilot\/pull\/\d+$/.test(item.pr_url))
      && (!item.spec_path || /^specs\/[\w/-]+\.md$/.test(item.spec_path));
  });
}
