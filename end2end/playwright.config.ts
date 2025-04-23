/**
 * @file Playwright configuration file.
 * @see https://playwright.dev/docs/test-configuration
 */

import process from 'node:process';
import {type PlaywrightTestConfig, devices} from '@playwright/test';
import {outputDirectory} from './tests/helpers.ts';

const timeout = process.env.CI ? 20 * 1000 : 15 * 1000;

/**
 * See https://playwright.dev/docs/test-configuration.
 */
const config: PlaywrightTestConfig = {
	testDir: './tests',
	/* Folder for test artifacts such as screenshots, videos, traces, etc. */
	outputDir: outputDirectory,
	/* Maximum time one test can run for. */
	timeout,
	expect: {
		/**
		 * Maximum time expect() should wait for the condition to be met.
		 * For example in `await expect(locator).toHaveText();`.
		 */
		timeout,
	},
	/* Run tests in files in parallel */
	fullyParallel: true,
	/* Fail the build on CI if you accidentally left test.only in the source code. */
	forbidOnly: Boolean(process.env.CI),
	/* Retry on CI only */
	retries: process.env.CI ? 2 : 1,
	/* Opt out of parallel tests on CI. */
	workers: process.env.CI ? 1 : undefined,
	/* Reporter to use. See https://playwright.dev/docs/test-reporters */
	reporter: 'html',
	/* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
	use: {
		/* Maximum time each action such as `click()` can take. Defaults to 0 (no limit). */
		actionTimeout: 0,
		/* Base URL to use in actions like `await page.goto('/')`. */
		// eslint-disable-next-line @typescript-eslint/naming-convention
		baseURL: 'http://0.0.0.0:8081',

		/* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
		trace: 'on-first-retry',
	},
	/* Run web server with `serve` before running tests. */
	webServer: {
		command: 'cd ../app/dist && serve --no-clipboard -l 8081',
		url: 'http://0.0.0.0:8081',
		timeout: 10 * 1000,
	},

	/* Configure projects for major browsers */
	projects: [
		{
			name: 'chrome-desktop',
			use: {
				channel: 'chrome',
			},
		},
		{
			name: 'msedge-desktop',
			use: {
				channel: 'msedge',
			},
		},
		{
			name: 'firefox-desktop',
			use: {
				...devices['Desktop Firefox'],
			},
		},
		{
			name: 'webkit-desktop',
			use: {
				...devices['Desktop Safari'],
			},
		},
		{
			name: 'chromium-mobile',
			use: {
				...devices['Pixel 5'],
			},
		},

		{
			name: 'chromium-mobile-landscape',
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
		// NOTE: Safari Mobile is really flaky on CI
		{
			name: 'webkit-mobile',
			use: {
				...devices['iPhone 12'],
			},
		},
	],
};

export default config;
