const { chromium } = require('playwright');
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const SCENARIOS_FILE = path.join(__dirname, 'scenarios.json');
const TEMPLATE_URL = `file://${path.join(__dirname, 'templates', 'terminal.html')}`;
const OUTPUT_DIR = path.join(__dirname, '../../docs/assets/gifs');

async function run() {
  if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR, { recursive: true });
  }

  const scenarios = JSON.parse(fs.readFileSync(SCENARIOS_FILE, 'utf8'));
  const scenarioIds = Object.keys(scenarios);

  const browser = await chromium.launch({ headless: true });

  for (const scenarioId of scenarioIds) {
    console.log(`\nRecording scenario: ${scenarioId}...`);

    // Playwright contexts have a recordVideo option, which outputs WebM.
    const context = await browser.newContext({
      viewport: { width: 920, height: 540 },
      recordVideo: {
        dir: OUTPUT_DIR,
        size: { width: 920, height: 540 }
      }
    });

    const page = await context.newPage();

    // Inject the scenarios JSON object before navigation
    await page.addInitScript(`window.INJECTED_SCENARIOS = ${JSON.stringify(scenarios)};`);

    await page.goto(`${TEMPLATE_URL}?scenario=${scenarioId}`);

    // Wait for the scenario to load and initialize
    await page.waitForFunction(() => window.scenarioReady === true);

    // Trigger the animation
    await page.evaluate(() => window.runAnimation());

    // Wait for the animation to finish
    await page.waitForFunction(() => window.animationComplete === true, { timeout: 30000 });

    // Give it an extra second at the end to hold on the final result
    await page.waitForTimeout(1000);

    // Close context to flush the video recording to disk
    await context.close();

    // Playwright generates a random string for the video filename. Let's find it.
    const files = fs.readdirSync(OUTPUT_DIR);
    // Find the most recently created webm file
    const webmFiles = files
      .filter(f => f.endsWith('.webm'))
      .map(f => path.join(OUTPUT_DIR, path.basename(f)))
      .sort((a, b) => fs.statSync(b).mtimeMs - fs.statSync(a).mtimeMs);

    if (webmFiles.length === 0) {
      console.error(`Failed to find output WebM for ${scenarioId}`);
      continue;
    }

    const videoPath = webmFiles[0];
    const gifPath = path.join(OUTPUT_DIR, `${scenarioId}.gif`);

    console.log(`Converting ${videoPath} -> ${gifPath}...`);

    // Use FFmpeg to convert WebM to highly optimized GIF
    await convertToGif(videoPath, gifPath);

    // Cleanup the intermediate webm
    fs.unlinkSync(videoPath);
    console.log(`Successfully generated: ${gifPath}`);
  }

  await browser.close();
  console.log('\nAll scenarios recorded and converted successfully!');
}

function convertToGif(inputPath, outputPath) {
  return new Promise((resolve, reject) => {
    // Generate a high-quality palette and then map colors. Scale keeps it sharp but handles sizing.
    // Dithering helps prevent banding in dark themes.
    const args = [
      '-y', // Overwrite
      '-i', inputPath,
      '-vf', 'fps=20,scale=840:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128[p];[s1][p]paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle',
      '-loop', '0', // Loop infinitely
      outputPath
    ];

    const ffmpeg = spawn('ffmpeg', args);

    ffmpeg.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`FFmpeg exited with code ${code}`));
      }
    });

    ffmpeg.on('error', (err) => {
      reject(err);
    });
  });
}

run().catch(err => {
  console.error('Fatal error:', err);
  process.exit(1);
});
