/**
 * @file Tests for the layout control.
 */

import {expect, test} from '@playwright/test';
import {screenWidthIsAtLeast, selectors, useLocalStorage} from '../helpers.ts';

const layoutControlSelector =
	'menu > .controls-group:not(:first-child) > .control:last-child';

test.describe('layout', () => {
	test('is comfortable by default', async ({page}) => {
		await page.goto('/');
		const layoutButtons = page.locator(`${layoutControlSelector} button`);
		await expect(layoutButtons).toHaveCount(2);
		await expect(layoutButtons.nth(0)).toHaveClass('selected');
	});

	const layouts = ['comfortable', 'compact'];
	for (const [layoutIndex, layout] of layouts.entries()) {
		test(`change to ${layout} through URL`, async ({page}) => {
			await page.goto(`/?layout=${layout}`);
			await expect(
				page.locator(`${layoutControlSelector} button`).nth(layoutIndex),
			).toHaveClass('selected');
			expect(await page.evaluate(() => localStorage.getItem('layout'))).toBe(
				layout,
			);
		});
	}

	test.describe('comfortable -> compact', () => {
		test('change through button', async ({page}) => {
			await page.goto('/');
			const gridItem = page.locator(selectors.grid.item.any.container);
			const gridItemBbox = await gridItem.boundingBox();
			expect(gridItemBbox).not.toBeNull();
			const {height: previousHeight, width: previousWidth} = gridItemBbox!;

			if (!screenWidthIsAtLeast('lg', page)) {
				await page.locator(selectors.controls.toggler).click();
			}

			const layoutButtons = page.locator(`${layoutControlSelector} button`);
			await layoutButtons.nth(1).click();
			await expect(layoutButtons.nth(1)).toHaveClass('selected');

			const newGridItemBbox = await gridItem.boundingBox();
			expect(newGridItemBbox).not.toBeNull();
			const {height: newHeight, width: newWidth} = newGridItemBbox!;
			expect(newHeight).toBeLessThan(previousHeight);
			expect(newWidth).toBeLessThan(previousWidth);

			// Check that the layout is saved in localStorage
			await page.reload();
			await expect(
				page.locator(`${layoutControlSelector} button`).nth(1),
			).toHaveClass('selected');
		});
	});

	test.describe('compact -> comfortable', () => {
		useLocalStorage(test, {layout: 'compact'});

		test('change through button', async ({page}) => {
			await page.goto('/');

			const gridItem = page.locator(selectors.grid.item.any.container);
			const gridItemBbox = await gridItem.boundingBox();
			expect(gridItemBbox).not.toBeNull();
			const {height: previousHeight, width: previousWidth} = gridItemBbox!;

			if (!screenWidthIsAtLeast('lg', page)) {
				await page.locator(selectors.controls.toggler).click();
			}

			const layoutButtons = page.locator(`${layoutControlSelector} button`);
			// Compact has been loaded from localStorage
			await expect(layoutButtons.nth(1)).toHaveClass('selected');
			await layoutButtons.nth(0).click();
			await expect(layoutButtons.nth(0)).toHaveClass('selected');

			const newGridItemBbox = await gridItem.boundingBox();
			expect(newGridItemBbox).not.toBeNull();
			const {height: newHeight, width: newWidth} = newGridItemBbox!;
			expect(newHeight).toBeGreaterThan(previousHeight);
			expect(newWidth).toBeGreaterThan(previousWidth);
		});
	});
});
