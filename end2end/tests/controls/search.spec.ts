/**
 * @file Tests for the search control.
 */

import {expect, test} from '@playwright/test';
import {selectors} from '../helpers.ts';

const orderModeControlSelector = selectors.controls.buttons.getByNthChild(1);

test.describe('search', () => {
	const searchValues = [
		['simple icons', 'Simple Icons'],
		// Slug
		['dotnet', '.NET'],
		// Aka alias
		['drawio', 'diagrams.net'],
		// Dup alias
		['gotowebinar', 'GoToMeeting'],
		// Loc alias
		['КиноПоиск', 'Kinopoisk'],
	];

	for (const [searchValue, expectedTitle] of searchValues) {
		test(`"${searchValue}" shows "${expectedTitle}" brand first`, async ({
			page,
		}) => {
			await page.goto('/');

			await page.fill(selectors.controls.search.input, searchValue);
			await expect(
				page.locator(selectors.grid.item.first.icon.title),
			).toHaveText(expectedTitle);
		});
	}

	test('sets the ordering mode to random', async ({page}) => {
		await page.goto('/');

		await page.fill(selectors.controls.search.input, searchValues[0][0]);
		const orderModeControlButtons = page.locator(
			`${orderModeControlSelector} button`,
		);
		await expect(orderModeControlButtons).toHaveCount(6);
		await expect(orderModeControlButtons.nth(5)).toHaveClass('selected');

		expect(await page.evaluate(() => localStorage.getItem('order-mode'))).toBe(
			'random',
		);
	});
});
