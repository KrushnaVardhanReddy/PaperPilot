import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawn, type ChildProcess } from 'node:child_process';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Absolute path to the compiled tauri desktop binary (Cargo workspace root target/debug/desktop)
const binaryPath = path.resolve(__dirname, '../../target/debug/desktop');

let previewProcess: ChildProcess | undefined;

export const config: WebdriverIO.Config = {
  runner: 'local',
  specs: ['./test/specs/**/*.e2e.ts'],
  maxInstances: 1,

  onPrepare: async () => {
    // Start vite preview on port 1420 for devUrl embedded inside Tauri debug build
    console.log('[WDIO] Starting Vite preview on port 1420...');
    previewProcess = spawn('pnpm', ['preview', '--port', '1420', '--host', '127.0.0.1'], {
      cwd: __dirname,
      stdio: 'pipe',
      detached: false,
      env: {
        ...process.env,
        PATH: `/tmp/e2e-tools/bin:/home/krushna/.nvm/versions/node/v24.12.0/bin:${process.env.PATH || ''}`
      }
    });

    // Wait until port 1420 is ready
    const startTime = Date.now();
    while (Date.now() - startTime < 15000) {
      try {
        const res = await fetch('http://127.0.0.1:1420');
        if (res.ok) {
          console.log('[WDIO] Vite preview server is up on http://127.0.0.1:1420');
          break;
        }
      } catch {
        // Retry
      }
      await new Promise((r) => setTimeout(r, 200));
    }
  },

  onComplete: async () => {
    if (previewProcess && !previewProcess.killed) {
      console.log('[WDIO] Stopping Vite preview server...');
      previewProcess.kill();
    }
  },

  services: [
    [
      '@wdio/tauri-service',
      {
        driverProvider: 'external',
        appBinaryPath: binaryPath,
        tauriDriverPath: '/tmp/e2e-tools/bin/tauri-driver',
        autoInstallTauriDriver: false,
        autoDownloadEdgeDriver: false,
        captureBackendLogs: true,
        captureFrontendLogs: true,
        logLevel: 'debug'
      }
    ]
  ],

  capabilities: [
    {
      browserName: 'tauri',
      'tauri:options': {
        application: binaryPath
      }
    }
  ],

  logLevel: 'info',
  bail: 0,
  waitforTimeout: 15000,
  connectionRetryTimeout: 90000,
  connectionRetryCount: 3,

  framework: 'mocha',
  reporters: ['spec'],

  mochaOpts: {
    ui: 'bdd',
    timeout: 300000
  }
};
