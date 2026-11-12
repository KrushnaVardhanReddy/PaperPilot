import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:1420',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: {
        ...devices['Desktop Chrome'],
        launchOptions: {
          args: ['--proxy-bypass-list=<-loopback>'],
        },
      },
    },
    {
      name: 'real-engine-90tools',
      fullyParallel: false,
      use: {
        ...devices['Desktop Chrome'],
        headless: process.env.HEADED === 'true' ? false : true,
        slowMo: process.env.HEADED === 'true' ? 500 : 0,
        video: 'on',
        screenshot: 'on',
        launchOptions: {
          args: ['--proxy-bypass-list=<-loopback>'],
        },
      },
      testMatch: ['**/e2e_unmocked_suite*.spec.ts'],
    },
  ],
  webServer: {
    command: 'npm run dev',
    cwd: './',
    url: 'http://127.0.0.1:1420',
    reuseExistingServer: true,
    env: {
      NO_PROXY: 'localhost,127.0.0.1',
      no_proxy: 'localhost,127.0.0.1',
      HTTP_PROXY: '',
      HTTPS_PROXY: '',
      http_proxy: '',
      https_proxy: '',
    },
  },
});
