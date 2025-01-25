import { test, expect } from '@playwright/test';
import {
  selectors,
  screenWidthIsAtLeast,
  SIMPLE_ICONS_DIRPATH,
  saveDownload,
} from '../helpers.ts';
import * as fs from 'node:fs';
import * as path from 'node:path';

const DOWNLOAD_TYPE_CONTROL_SELECTOR =
  selectors.controls.buttons.getByNthChild(3);

test.describe('download type', () => {
  test('is SVG by default', async ({ page }) => {
    await page.goto('/');
    const downloadTypeButtons = await page.locator(
      `${DOWNLOAD_TYPE_CONTROL_SELECTOR} button`,
    );
    await expect(downloadTypeButtons).toHaveCount(2);
    await expect(downloadTypeButtons.nth(0)).toHaveClass('selected');
  });

  test('download SVG', async ({ page }) => {
    await page.goto('/');
    const gridItemDownloadButton = await page.locator(
      selectors.grid.item.first.icon.footer.downloadButton,
    );

    const downloadPromise = page.waitForEvent('download');
    await gridItemDownloadButton.click();
    const download = await downloadPromise;
    const filename = download.suggestedFilename();
    await expect(filename).toMatch(/[^.]+\.svg/);
    const outputPath = await saveDownload(
      download,
      `download-control-${filename}`,
    );

    const fileContent = fs.readFileSync(outputPath, 'utf8');
    const expectedFileContent = fs.readFileSync(
      path.resolve(SIMPLE_ICONS_DIRPATH, `icons/${filename}`),
      'utf8',
    );
    await expect(fileContent).toBe(expectedFileContent);
  });

  test('download PNG', async ({ page }) => {
    await page.goto('/');

    if (!screenWidthIsAtLeast('lg', page)) {
      await page.locator(selectors.controls.toggler).click();
    }

    const downloadTypeButtons = await page.locator(
      `${DOWNLOAD_TYPE_CONTROL_SELECTOR} button`,
    );
    await downloadTypeButtons.nth(1).click();

    const gridItemDownloadButton = await page.locator(
      selectors.grid.item.first.icon.footer.downloadButton,
    );

    const downloadPromise = page.waitForEvent('download');
    await gridItemDownloadButton.click();
    const download = await downloadPromise;
    const filename = download.suggestedFilename();
    await expect(filename).toMatch(/[^.]+\.png/);
    await saveDownload(download, `download-control-${filename}`);
  });

  const downloadTypeButtons = ['svg', 'png'];
  for (const downloadTypeButtonIndex in downloadTypeButtons) {
    const downloadType = downloadTypeButtons[downloadTypeButtonIndex];
    test(`change to ${downloadType.toUpperCase()} through URL`, async ({
      page,
    }) => {
      await page.goto(`/?download-type=${downloadType}`);
      await expect(
        await page
          .locator(`${DOWNLOAD_TYPE_CONTROL_SELECTOR} button`)
          .nth(parseInt(downloadTypeButtonIndex)),
      ).toHaveClass('selected');
      await expect(
        await page.evaluate(() => localStorage.getItem('download-type')),
      ).toBe(downloadType);
    });
  }
});
