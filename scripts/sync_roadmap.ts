import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PHASES_DIR = path.resolve(__dirname, '../phases');
const OUTPUT_FILE = path.resolve(__dirname, '../apps/web/public/roadmap.json');

type Status = 'shipped' | 'in_progress' | 'planned';

interface Task {
  id: string;
  phase: string;
  title: string;
  summary: string;
  status: Status;
  pr: string | null;
}

function parseMarkdownTable(content: string, phaseName: string): Task[] {
  const tasks: Task[] = [];
  const lines = content.split('\n');

  let inTable = false;
  let headers: string[] = [];

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith('|') && trimmed.endsWith('|')) {
      const cells = trimmed.split('|').slice(1, -1).map(c => c.trim());

      // Check if it's a header or separator
      if (cells.every(c => c.match(/^:?-+:?$/) || c === '---')) {
        inTable = true;
        continue;
      }

      if (!inTable) {
        headers = cells;
        continue;
      }

      if (cells.length >= 3 && headers.length > 0 && headers[0] === '#') {
        const idRaw = cells[0];
        const titleRaw = cells[1];
        const notesRaw = cells[2];

        if (idRaw.replace(/~/g, '') === '#') continue; // Skip inner headers

        const idMatch = idRaw.match(/([a-zA-Z0-9.]+)/);
        const id = idMatch ? idMatch[1] : idRaw.replace(/~/g, '').replace('✓', '').trim();

        let title = titleRaw.replace(/~/g, '').replace('✓', '').trim();
        let summary = notesRaw.trim();
        let status: Status = 'planned';

        const fullRowText = `${idRaw} ${titleRaw} ${notesRaw}`;

        if (fullRowText.includes('✅') || fullRowText.includes('Completed') || fullRowText.includes('Merged') || fullRowText.includes('✓')) {
            status = 'shipped';
        } else if (fullRowText.includes('🚀') || fullRowText.includes('In Progress')) {
            status = 'in_progress';
        } else if (fullRowText.includes('📋') || fullRowText.includes('Planned') || fullRowText.includes('Backlog')) {
            status = 'planned';
        }

        // Clean up title
        title = title.replace(/[✅🚀📋✓]/g, '').trim();

        // Extract PR link or number if available
        let pr = null;
        const prMatch = summary.match(/(PR\s*#\d+|#\d+)/i) || title.match(/(PR\s*#\d+|#\d+)/i);
        if (prMatch) {
            pr = prMatch[1];
        } else {
            // Also try to find links to PRs
            const mdLinkMatch = summary.match(/\[([^\]]+)\]\([^)]+\)/);
            if (mdLinkMatch && mdLinkMatch[1].includes('PR')) {
                pr = mdLinkMatch[1];
            }
        }

        if (id && id !== '#' && title) {
            tasks.push({
                id,
                phase: phaseName,
                title,
                summary,
                status,
                pr
            });
        }
      }
    } else {
      inTable = false;
    }
  }

  return tasks;
}

function processPhases() {
  const files = fs.readdirSync(PHASES_DIR).filter(f => f.endsWith('.md')).sort();
  let allTasks: Task[] = [];

  for (const file of files) {
    const content = fs.readFileSync(path.join(PHASES_DIR, file), 'utf-8');

    // Extract Phase Name
    const titleMatch = content.match(/^#+\s*(.+)$/m);
    const phaseName = titleMatch ? titleMatch[1].trim() : file.replace('.md', '');

    const tasks = parseMarkdownTable(content, phaseName);
    allTasks = allTasks.concat(tasks);
  }

  // Ensure public dir exists
  const publicDir = path.dirname(OUTPUT_FILE);
  if (!fs.existsSync(publicDir)) {
    fs.mkdirSync(publicDir, { recursive: true });
  }

  fs.writeFileSync(OUTPUT_FILE, JSON.stringify(allTasks, null, 2));
  console.log(`Generated roadmap.json with ${allTasks.length} tasks.`);
}

processPhases();
