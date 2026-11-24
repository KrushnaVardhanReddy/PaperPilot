import { renderVideo } from '@revideo/renderer';
import path from 'path';
import fs from 'fs';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Read tools deepdive configuration
const toolsJsonPath = path.resolve(__dirname, '../src/configs/tools_deepdive.json');
const toolsData: Array<{ tool_name: string; title: string; category: string }> = JSON.parse(
  fs.readFileSync(toolsJsonPath, 'utf-8')
);

const projectFile = path.resolve(__dirname, '../src/project_tool.ts');
const finalVideosDir = path.resolve(__dirname, '../out/tool_videos');
const tempDir = path.resolve(__dirname, '../out/temp_builds_tools');

for (const dir of [finalVideosDir, tempDir]) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

// Ensure out/temp_builds_tools/../public resolves to tools/video-studio/public
const publicSymlink = path.resolve(tempDir, '../public');
const realPublic = path.resolve(__dirname, '../public');
if (!fs.existsSync(publicSymlink)) {
  try {
    fs.symlinkSync(realPublic, publicSymlink, 'dir');
  } catch (e) {
    // Ignore if already exists
  }
}

async function renderTool(toolName: string, index: number, total: number) {
  const tool = toolsData.find((t) => t.tool_name === toolName);
  if (!tool) {
    console.error(`❌ Tool ${toolName} not found in configuration!`);
    return;
  }

  const outFile = `tool_${tool.tool_name}.mp4`;
  const finalDestination = path.join(finalVideosDir, outFile);

  if (fs.existsSync(finalDestination) && fs.statSync(finalDestination).size > 1024 * 1024) {
    console.log(`⏩ [${index}/${total}] Tool ${tool.tool_name} already rendered at ${finalDestination}. Skipping.`);
    return;
  }

  console.log('\n====================================================');
  console.log(`🎬 [${index}/${total}] Rendering Tool: ${tool.tool_name} (${tool.title})`);
  console.log(`📁 Target Output: ${finalDestination}`);
  console.log('====================================================');

  // Update current_tool_target.ts for Vite bundler
  const currentTargetFile = path.resolve(__dirname, '../src/current_tool_target.ts');
  fs.writeFileSync(currentTargetFile, `export const CURRENT_TOOL_NAME = '${tool.tool_name}';\n`, 'utf-8');

  const startTime = Date.now();

  try {
    const renderedFile = await renderVideo({
      projectFile,
      settings: {
        outDir: tempDir,
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

    if (fs.existsSync(renderedFile)) {
      fs.copyFileSync(renderedFile, finalDestination);
    }

    const elapsed = ((Date.now() - startTime) / 1000).toFixed(1);
    console.log(`✅ [${index}/${total}] Tool ${tool.tool_name} rendered successfully in ${elapsed}s!`);
    console.log(`📁 Final Clean Video: ${finalDestination}`);
  } catch (err) {
    console.error(`❌ Error rendering tool ${tool.tool_name}:`, err);
    throw err;
  }
}

async function main() {
  const args = process.argv.slice(2);
  let targetTools: string[] = [];

  if (args.includes('--all') || args.length === 0) {
    targetTools = toolsData.map((t) => t.tool_name);
  } else if (args.includes('--range')) {
    const rangeIdx = args.indexOf('--range');
    const rangeVal = args[rangeIdx + 1];
    const [start, end] = rangeVal.split('..').map(Number);
    targetTools = toolsData.slice(start - 1, end).map((t) => t.tool_name);
  } else {
    for (const arg of args) {
      if (arg.startsWith('--')) continue;
      targetTools.push(arg);
    }
  }

  console.log('====================================================');
  console.log('🎬 PaperPilot Video Studio: Universal Tool Batch Renderer');
  console.log(`Total Targets to Render: ${targetTools.length} tools`);
  console.log('====================================================');

  for (let i = 0; i < targetTools.length; i++) {
    await renderTool(targetTools[i], i + 1, targetTools.length);
  }

  console.log('\n🎉 All requested tool renders completed!');
}

main().catch((err) => {
  console.error('Fatal error during tool batch rendering:', err);
  process.exit(1);
});
