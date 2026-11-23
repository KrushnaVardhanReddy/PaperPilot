import { renderVideo } from '@revideo/renderer';
import path from 'path';
import fs from 'fs';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function main() {
  console.log('====================================================');
  console.log('🎬 PaperPilot Video Studio: Week 01 Short Render');
  console.log('Tool: Instant PDF Compression (Multi-Interface)');
  console.log('GPU: NVIDIA GeForce RTX 4070 (Auto-detection enabled)');
  console.log('====================================================');

  const projectFile = path.resolve(__dirname, '../src/project.ts');
  const outDir = path.resolve(__dirname, '../out');
  const outFile = 'week01_compression.mp4';

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
            '--disable-gpu', // Reliable, portable canvas rendering
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
    console.error('\n❌ Render failed with error:', error);
    process.exit(1);
  }
}

main();
