/**
 * @file Tests for the order mode control.
 */

import {expect, test} from '@playwright/test';
import {
	getGridItemsIconsTitles,
	numberOfIconsPerPage,
	screenWidthIsAtLeast,
	selectors,
} from '../helpers.ts';

const orderModeControlSelector = selectors.controls.buttons.getByNthChild(1);

test.describe('order mode', () => {
	test('is random by default', async ({page}) => {
		await page.goto('/');
		const orderModeButtons = page.locator(`${orderModeControlSelector} button`);
		await expect(orderModeButtons).toHaveCount(5);
		await expect(orderModeButtons.nth(4)).toHaveClass('selected');
	});

	test('alphabetical -> color', async ({page}) => {
		await page.goto('/');
		const alphabeticalGridItemIconsTitles = await getGridItemsIconsTitles(page);

		if (!screenWidthIsAtLeast('lg', page)) {
			await page.locator(selectors.controls.toggler).click();
		}

		const orderModeButtons = page.locator(`${orderModeControlSelector} button`);

		await orderModeButtons.nth(2).click();
		await expect(orderModeButtons.nth(2)).toHaveClass('selected');

		const colorGridItemIconsTitles = await getGridItemsIconsTitles(page);

		// Check that the page has the correct number of icons
		expect(colorGridItemIconsTitles).toHaveLength(numberOfIconsPerPage);

		// Check that the order is different from the alphabetical one
		expect(alphabeticalGridItemIconsTitles).not.toEqual(
			colorGridItemIconsTitles,
		);

		// Check that the new order is stored on localStorage
		expect(await page.evaluate(() => localStorage.getItem('order-mode'))).toBe(
			'color',
		);

		await page.reload();
		await expect(
			page.locator(`${orderModeControlSelector} button`).nth(2),
		).toHaveClass('selected');
	});
});
