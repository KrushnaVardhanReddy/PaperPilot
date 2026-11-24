import { renderVideo } from '@revideo/renderer';
import path from 'path';
import fs from 'fs';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function main() {
  console.log('====================================================');
  console.log('🎬 PaperPilot Video Studio: Set B Tool Deep Dive Render');
  console.log('Tool: Tool #1 (pdf_compress — Instant Local PDF Compression)');
  console.log('Format: 1920x1080 Landscape @ 30fps');
  console.log('====================================================');

  const projectFile = path.resolve(__dirname, '../src/project_tool.ts');
  const outDir = path.resolve(__dirname, '../out/tool_videos');
  const outFile = 'tool_pdf_compress.mp4';

  if (!fs.existsSync(outDir)) {
    fs.mkdirSync(outDir, { recursive: true });
  }

  const startTime = Date.now();

  try {
    const renderedFile = await renderVideo({
      projectFile,
      settings: {
        outDir,
        outFile,
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
    console.log('\n====================================================');
    console.log(`✅ Render completed successfully in ${elapsed}s!`);
    console.log(`📁 Output file: ${renderedFile}`);
    console.log('====================================================');
  } catch (error) {
    console.error('❌ Render failed with error:', error);
    process.exit(1);
  }
}

main();
