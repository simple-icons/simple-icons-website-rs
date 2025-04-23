/**
 * @file Helpers for end-to-end tests.
 */

import * as path from 'node:path';
import type {Download, Page, TestType} from '@playwright/test';
import * as simpleicons from 'simple-icons';
import {getDirnameFromImportMeta} from 'simple-icons/sdk';

const __dirname = getDirnameFromImportMeta(import.meta.url);

export const outputDirectory = 'test-results/';
const rootDirectory = path.resolve(__dirname, '../../');

// Number of icons in simple-icons library
export const numberOfIcons = Object.keys(simpleicons).length;

// Directory path of simple-icons npm package
export const simpleIconsDirectoryPath = path.resolve(
	rootDirectory,
	'node_modules/simple-icons',
);

export const numberOfIconsPerPage = 30;

const getViewportSize = (page: Page): {width: number; height: number} => {
	const size = page.viewportSize();
	if (!size) {
		throw new Error('Viewport size is not set');
	}

	return size;
};

/**
 * Get if the viewport of a page is at least the given breakpoint.
 * @param {string} br TailwindCSS breakpoint.
 * @param {Page} page Playwright page.
 * @returns {boolean} Whether the viewport is at least the given breakpoint.
 */
export const screenWidthIsAtLeast = (
	br: 'xs' | 'sm' | 'md' | 'lg',
	page: Page,
): boolean => {
	const size = getViewportSize(page);

	switch (br) {
		case 'xs': {
			return size.width >= 475;
		}

		case 'sm': {
			return size.width >= 640;
		}

		case 'md': {
			return size.width >= 768;
		}

		case 'lg': {
			return size.width >= 1024;
		}
	}
};

/**
 * Selectors used in the end-to-end tests.
 */
export const selectors = {
	/**
	 * Selector for the body.
	 * Using `:last-child` because Trunk injectes their own body on development.
	 */
	body: 'body',
	header: {
		container: 'header',
		title: 'header > div > a',
		description: '#site-description',
		nav: {
			container: 'header > nav',
			toggler: 'header > nav > ul:last-child > :first-child',
			buttons: {
				container: 'header > nav > ul:first-child',
				languageSelector: 'header > nav > ul:first-child > li:last-of-type',
			},
		},
	},
	/* Selectors for controls in main menu */
	controls: {
		toggler: 'menu > :last-child button',
		/* Buttons controls, don't include the search one */
		buttons: {
			/* Get a control by its position in the menu */
			getByNthChild: (number_: number) =>
				`menu > .controls-group:not(:first-child) > .control:nth-child(${number_})`,
		},
		search: {
			input: 'menu > .controls-group:first-child input',
		},
	},
	/* Selectors for the grid */
	grid: (() => {
		const gridSelector = 'main > ul';
		const gridItemsSelector = `${gridSelector} > li`;
		const getGridItemByNthChild = (number_: number) =>
			`${gridItemsSelector}:nth-child(${number_})`;

		const buildGridItemSelectors = (
			containerSelector: string,
		): {
			container: string;
			icon: {preview: string; title: string; footer: {downloadButton: string}};
		} => {
			return {
				container: containerSelector,
				/* Selectors for grid icon items */
				icon: {
					preview: `${containerSelector} > :first-child img`,
					title: `${containerSelector} h2`,
					footer: {
						downloadButton: `${containerSelector} > :last-child > :last-child`,
					},
				},
			};
		};

		return {
			/* Grid selector */
			container: gridSelector,
			/* Grid items selector */
			items: gridItemsSelector,
			item: {
				/* Get selector for first item in the grid */
				first: buildGridItemSelectors(`${gridItemsSelector}:first-of-type`),
				/* Get selector for a random grid item */
				any: buildGridItemSelectors(
					getGridItemByNthChild(
						Math.floor(Math.random() * numberOfIconsPerPage + 1),
					),
				),
			},
			/* Button to go to footer which stops loading more icons */
			scrollToFooter: '.scroll-to-footer-button',
			/* Button to go to header from footer */
			scrollToHeader: '.scroll-to-header-button',
			/* Button to load more icons */
			iconsLoader: {
				button: '.icons-loader button',
			},
		};
	})(),
	footer: 'footer',
	/* Selectors for modals */
	modals: {
		languageSelector: '.language-selector',
	},
};

/**
 * Save a playwright download in the output directory.
 * @param {Download} download Playwright download.
 * @param {string} filename Filename of the download.
 * @returns {Promise<string>} Path to the saved file.
 */
export const saveDownload = async (
	download: Download,
	filename: string,
): Promise<string> => {
	const outputPath = path.resolve(outputDirectory, filename);
	await download.saveAs(outputPath);
	return outputPath;
};

/**
 * Get icons titles from the grid.
 * @param {Page} page Playwright page.
 * @returns {Promise<string>}Icons titles shown in the grid.
 */
export const getGridItemsIconsTitles = async (
	page: Page,
): Promise<string[]> => {
	const locators = await page.locator(`${selectors.grid.items} h2`).all();
	const textsPromises = locators.map(async (item) => item.textContent());
	const textsWithMaybeNulls = await Promise.all(textsPromises);
	return textsWithMaybeNulls.filter((item) => item !== null);
};

/**
 * Helper to set local storage on `test.use` with Playwright.
 * @param {TestType<any, any>} test Playwright test.
 * @param {Record<string, string>} storage Local storage keys and values.
 */
export const useLocalStorage = (
	test: TestType<any, any>,
	storage: Record<string, string>,
) => {
	test.use({
		storageState: {
			cookies: [],
			origins: [
				{
					origin: '',
					localStorage: Object.keys(storage).map((k) => ({
						name: k,
						value: storage[k],
					})),
				},
			],
		},
	});
};
