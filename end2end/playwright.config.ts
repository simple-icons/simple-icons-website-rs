import type { PlaywrightTestConfig } from '@playwright/test';
import { devices } from '@playwright/test';

const TIMEOUT = 10 * 1000;

/**
 * See https://playwright.dev/docs/test-configuration.
 */
const config: PlaywrightTestConfig = {
  testDir: './tests',
  /* Maximum time one test can run for. */
  timeout: TIMEOUT,
  expect: {
    /**
     * Maximum time expect() should wait for the condition to be met.
     * For example in `await expect(locator).toHaveText();`
     */
    timeout: TIMEOUT,
  },
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: 'html',
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Maximum time each action such as `click()` can take. Defaults to 0 (no limit). */
    actionTimeout: 0,
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: 'http://0.0.0.0:8081',

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on-first-retry',
  },
  /* Run web server with anywhere before running tests. */
  webServer: {
    command: 'anywhere -d ../app/dist -p 8081 -s',
    url: 'http://0.0.0.0:8081',
    timeout: 10 * 1000,
  },

  /* Configure projects for major browsers */
  projects: [
    {
      name: 'chromium',
      use: {
        ...devices['Desktop Chrome'],
      },
    },

    {
      name: 'firefox',
      use: {
        ...devices['Desktop Firefox'],
      },
    },

    /* // TODO: Safari fails on production with the next message:
       // https://github.com/rustwasm/wasm-bindgen/issues/2646
    {
      name: 'webkit',
      use: {
        ...devices['Desktop Safari'],
      },
    },
    */

    /* Test against mobile viewports. */
    {
      name: 'Mobile Chrome',
      use: {
        ...devices['Pixel 5'],
      },
    },

    /* Safari fails on production
    {
      name: 'Mobile Safari',
      use: {
        ...devices['iPhone 12'],
      },
    },
    */

    /* Test against branded browsers. */
    {
      name: 'Microsoft Edge',
      use: {
        channel: 'msedge',
      },
    },
    {
      name: 'Google Chrome',
      use: {
        channel: 'chrome',
      },
    },
  ],

  /* Folder for test artifacts such as screenshots, videos, traces, etc. */
  outputDir: 'test-results/',
};

export default config;
