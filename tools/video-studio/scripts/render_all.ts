import { renderVideo } from '@revideo/renderer';
import path from 'path';
import fs from 'fs';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Read tools configuration
const toolsJsonPath = path.resolve(__dirname, '../src/configs_tools.json');
const toolsData: Array<{ week: number; weekStr: string; slug: string; toolName: string }> = JSON.parse(
  fs.readFileSync(toolsJsonPath, 'utf-8')
);

const projectFile = path.resolve(__dirname, '../src/project.ts');
const outDir = path.resolve(__dirname, '../out');

if (!fs.existsSync(outDir)) {
  fs.mkdirSync(outDir, { recursive: true });
}

async function renderWeek(weekNum: number) {
  const tool = toolsData.find((t) => t.week === weekNum);
  if (!tool) {
    console.error(`❌ Week ${weekNum} not found in configuration!`);
    return;
  }

  const outFile = `week${tool.weekStr}_${tool.slug}.mp4`;
  console.log('\n====================================================');
  console.log(`🎬 Rendering Week ${tool.weekStr}: ${tool.toolName}`);
  console.log(`📁 Target Output: ${path.join(outDir, outFile)}`);
  console.log('====================================================');

  process.env.TARGET_WEEK = String(weekNum);
  const startTime = Date.now();

  try {
    const renderedFile = await renderVideo({
      projectFile,
      settings: {
        outDir,
        outFile: outFile as `${string}.mp4`,
        logProgress: true,
        puppeteer: {
          executablePath: '/home/krushna/.cache/puppeteer/chrome/linux-157.0.8103.78/chrome-linux64/chrome',
          args: [
            '--no-sandbox',
            '--disable-setuid-sandbox',
            '--disable-dev-shm-usage',
            '--disable-gpu',
          ],
        },
      },
    });

    const elapsed = ((Date.now() - startTime) / 1000).toFixed(1);
    console.log(`✅ Week ${tool.weekStr} rendered successfully in ${elapsed}s -> ${renderedFile}`);
  } catch (err) {
    console.error(`❌ Error rendering week ${tool.weekStr}:`, err);
    throw err;
  }
}

async function main() {
  const args = process.argv.slice(2);
  let targetWeeks: number[] = [];

  if (args.includes('--all') || args.length === 0) {
    targetWeeks = toolsData.map((t) => t.week);
  } else {
    for (const arg of args) {
      if (arg.includes('..')) {
        const [start, end] = arg.split('..').map(Number);
        for (let i = start; i <= end; i++) targetWeeks.push(i);
      } else if (!isNaN(Number(arg))) {
        targetWeeks.push(Number(arg));
      }
    }
  }

  console.log('====================================================');
  console.log('🎬 PaperPilot Video Studio: Universal Batch Renderer');
  console.log(`Total Targets to Render: ${targetWeeks.length} weeks (${targetWeeks.join(', ')})`);
  console.log('====================================================');

  for (const week of targetWeeks) {
    await renderWeek(week);
  }

  console.log('\n🎉 All requested renders completed!');
}

main().catch((err) => {
  console.error('Fatal error during batch rendering:', err);
  process.exit(1);
});
