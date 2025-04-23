/**
 * @file Tests for the grid of icons.
 */

import {type Page, expect, test} from '@playwright/test';
import {
	numberOfIconsPerPage,
	screenWidthIsAtLeast,
	selectors,
} from '../helpers.ts';

const expectNumberOfIconsLoadedInGrid = async (
	page: Page,
	numberOfIcons: number,
) => {
	await page.waitForFunction(
		({selector, expectedNumberOfIcons}) =>
			document.querySelectorAll(selector).length === expectedNumberOfIcons,
		{selector: selectors.grid.items, expectedNumberOfIcons: numberOfIcons},
	);
};

test.describe('grid', () => {
	test('first page of icons on load', async ({page}) => {
		await page.goto('/');
		await expectNumberOfIconsLoadedInGrid(page, numberOfIconsPerPage);
	});

	test('scroll to the footer loads the next page of icons', async ({page}) => {
		await page.goto('/');
		await page.locator(selectors.footer).scrollIntoViewIfNeeded();
		await expectNumberOfIconsLoadedInGrid(page, numberOfIconsPerPage * 2);

		await page.locator(selectors.footer).scrollIntoViewIfNeeded();
		await expectNumberOfIconsLoadedInGrid(page, numberOfIconsPerPage * 3);
	});

	test('go to footer through button', async ({page}) => {
		await page.goto('/');
		await expect(page.locator(selectors.footer)).not.toBeInViewport();
		await page.locator(selectors.grid.scrollToFooter).click();
		await expect(page.locator(selectors.footer)).toBeInViewport();
		expect(await page.locator(selectors.grid.items).count()).toBe(
			numberOfIconsPerPage,
		);
	});

	test('go to footer and back to header scrolling keeps loading icons', async ({
		page,
	}) => {
		await page.goto('/');
		await page.locator(selectors.grid.scrollToFooter).click();
		await expect(page.locator(selectors.footer)).toBeInViewport();

		await page.locator(selectors.header.container).scrollIntoViewIfNeeded();
		await page.locator(selectors.footer).scrollIntoViewIfNeeded();
		await expectNumberOfIconsLoadedInGrid(page, numberOfIconsPerPage * 2);
	});

	test('load more icons through button', async ({page}) => {
		await page.goto('/');
		await page.locator(selectors.grid.scrollToFooter).click();
		await expect(page.locator(selectors.footer)).toBeInViewport();

		await page
			.locator(selectors.grid.iconsLoader.button)
			.scrollIntoViewIfNeeded();
		await page.locator(selectors.grid.iconsLoader.button).click();
		await expectNumberOfIconsLoadedInGrid(page, numberOfIconsPerPage * 2);
	});

	test('scroll to header through button', async ({page}) => {
		await page.goto('/');
		await page.locator(selectors.grid.scrollToFooter).click();
		await expect(page.locator(selectors.header.container)).not.toBeInViewport();

		// eslint-disable-next-line unicorn/prefer-ternary
		if (screenWidthIsAtLeast('md', page)) {
			await page.locator(selectors.grid.scrollToHeader).click();
		} else {
			// The button does not appear in mobile
			await page.locator(selectors.header.container).scrollIntoViewIfNeeded();
		}

		await expect(page.locator(selectors.header.container)).toBeInViewport();
	});
});
