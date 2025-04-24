/**
 * @file Tests for the color scheme control.
 */

import {type Page, expect, test} from '@playwright/test';
import {screenWidthIsAtLeast, selectors} from '../helpers.ts';

const colorSchemeControlSelector = selectors.controls.buttons.getByNthChild(2);

const getSystemColorScheme = async (page: Page): Promise<'dark' | 'light'> => {
	return page.evaluate(() => {
		return globalThis.matchMedia('(prefers-color-scheme: dark)').matches
			? 'dark'
			: 'light';
	});
};

test.describe('color scheme', () => {
	test('system -> opposite', async ({page}) => {
		await page.goto('/');

		if (!screenWidthIsAtLeast('lg', page)) {
			await page.locator(selectors.controls.toggler).click();
		}

		const colorSchemeButtons = page.locator(
			`${colorSchemeControlSelector} button`,
		);

		const previousSystemColorScheme = await getSystemColorScheme(page);
		const oppositeColorScheme =
			previousSystemColorScheme === 'dark' ? 'light' : 'dark';
		const oppositeColorSchemeNthButton =
			oppositeColorScheme === 'light' ? 0 : 1;

		await colorSchemeButtons.nth(oppositeColorSchemeNthButton).click();
		await expect(
			colorSchemeButtons.nth(oppositeColorSchemeNthButton),
		).toHaveClass('selected');

		await expect(page.locator(selectors.body)).toHaveClass(oppositeColorScheme);

		expect(
			await page.evaluate(() => localStorage.getItem('color-scheme')),
		).toBe(oppositeColorScheme);
	});
});
