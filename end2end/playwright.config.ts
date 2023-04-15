import { devices, type PlaywrightTestConfig } from '@playwright/test';
import { OUTPUT_DIR } from './tests/helpers.ts';

const TIMEOUT = process.env.CI ? 30 * 1000 : 10 * 1000;

/**
 * See https://playwright.dev/docs/test-configuration.
 */
const config: PlaywrightTestConfig = {
  testDir: './tests',
  /* Folder for test artifacts such as screenshots, videos, traces, etc. */
  outputDir: OUTPUT_DIR,
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

    /**
     * Safari fails on production, mostly of the times with the next message:
     * https://github.com/rustwasm/wasm-bindgen/issues/2646
     *
     * I've investigated this for a few hours and it seems to be a memory bug
     * with WASM in Safari. Sometimes iit fails before after mounting with the
     * router not being created, sometimes in the first unwrap of a signal,
     * it doesn't follows a consistent pattern. I'm not sure how to report this,
     * I've tried to reproduce it in a minimal example but I can't get it.
     *
     * Maybe it would be worth to wait some months and see if it gets fixed.
     **/
    /* {
      name: 'webkit',
      use: {
        ...devices['Desktop Safari'],
      },
    }, */

    /* Test against mobile viewports. */
    {
      name: 'Mobile Chrome',
      use: {
        ...devices['Pixel 5'],
      },
    },

    {
      name: 'Mobile Chrome landscape',
      use: {
        ...devices['Pixel 5 landscape'],
        viewport: {
          width: 802,
          /**
           * The default height of the landscape viewport is 293, but when
           * we scroll to footer in the tests with `.scrollIntoViewIfNeeded()`
           * the grid is not visible and no icons are loaded because the intersection
           * observer does not detect the intersection. So decided to increase the
           * height of the viewport to 393 to ensure that the intersection is detected.
           */
          height: 393,
        },
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
};

export default config;
