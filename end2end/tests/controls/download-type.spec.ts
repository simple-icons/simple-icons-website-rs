/**
 * @file Tests for the download type control.
 */

import * as fs from 'node:fs';
import * as path from 'node:path';
import {expect, test} from '@playwright/test';
import {
	saveDownload,
	screenWidthIsAtLeast,
	selectors,
	simpleIconsDirectoryPath,
} from '../helpers.ts';

const downloadTypeControlSelector = selectors.controls.buttons.getByNthChild(3);

test.describe('download type', () => {
	test('is SVG by default', async ({page}) => {
		await page.goto('/');
		const downloadTypeButtons = page.locator(
			`${downloadTypeControlSelector} button`,
		);
		await expect(downloadTypeButtons).toHaveCount(2);
		await expect(downloadTypeButtons.nth(0)).toHaveClass('selected');
	});

	test('download SVG', async ({page}) => {
		await page.goto('/');
		const gridItemDownloadButton = page.locator(
			selectors.grid.item.first.icon.footer.downloadButton,
		);

		const downloadPromise = page.waitForEvent('download');
		await gridItemDownloadButton.click();
		const download = await downloadPromise;
		const filename = download.suggestedFilename();
		expect(filename).toMatch(/[^.]+\.svg/);
		const outputPath = await saveDownload(
			download,
			`download-control-${filename}`,
		);

		const fileContent = fs.readFileSync(outputPath, 'utf8');
		const expectedFileContent = fs.readFileSync(
			path.resolve(simpleIconsDirectoryPath, `icons/${filename}`),
			'utf8',
		);
		expect(fileContent).toBe(expectedFileContent);
	});

	test('download PNG', async ({page}) => {
		await page.goto('/');

		if (!screenWidthIsAtLeast('lg', page)) {
			await page.locator(selectors.controls.toggler).click();
		}

		const downloadTypeButtons = page.locator(
			`${downloadTypeControlSelector} button`,
		);
		await downloadTypeButtons.nth(1).click();

		const gridItemDownloadButton = page.locator(
			selectors.grid.item.first.icon.footer.downloadButton,
		);

		const downloadPromise = page.waitForEvent('download');
		await gridItemDownloadButton.click();
		const download = await downloadPromise;
		const filename = download.suggestedFilename();
		expect(filename).toMatch(/[^.]+\.png/);
		await saveDownload(download, `download-control-${filename}`);
	});

	const downloadTypeButtons = ['svg', 'png'];
	for (const [
		downloadTypeButtonIndex,
		downloadType,
	] of downloadTypeButtons.entries()) {
		test(`change to ${downloadType.toUpperCase()} through URL`, async ({
			page,
		}) => {
			await page.goto(`/?download-type=${downloadType}`);
			await expect(
				page
					.locator(`${downloadTypeControlSelector} button`)
					.nth(downloadTypeButtonIndex),
			).toHaveClass('selected');
			expect(
				await page.evaluate(() => localStorage.getItem('download-type')),
			).toBe(downloadType);
		});
	}
});
