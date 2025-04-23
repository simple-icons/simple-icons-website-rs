/**
 * @file Integration tests for the language selector.
 */

import {expect, test} from '@playwright/test';
import {
	numberOfIcons,
	screenWidthIsAtLeast,
	selectors,
	useLocalStorage,
} from '../helpers.ts';

const descriptions = {
	'en-US': `${numberOfIcons} SVG icons for popular brands`,
	'es-ES': `${numberOfIcons} iconos SVG para marcas populares`,
	'fr-FR': `${numberOfIcons} icônes SVG pour les marques populaires`,
};

test.describe('language selector', () => {
	test.describe('navigator.language', () => {
		test.use({locale: 'es-ES'});

		test('the language is auto-discovered', async ({page}) => {
			await page.goto('/');
			await expect(page.locator(selectors.header.description)).toHaveText(
				descriptions['es-ES'],
			);
		});
	});

	for (const [language, description] of Object.entries(descriptions)) {
		test.describe(`${language} language from local storage`, () => {
			useLocalStorage(test, {language});

			test('autodiscover', async ({page}) => {
				await page.goto('/');
				await expect(page.locator(selectors.header.description)).toHaveText(
					description,
				);
			});
		});
	}

	test.describe('change language', () => {
		test('through language selector', async ({page}) => {
			await page.goto('/');

			// English by default
			await expect(page.locator(selectors.header.description)).toHaveText(
				descriptions['en-US'],
			);

			if (!screenWidthIsAtLeast('lg', page)) {
				await page.locator(selectors.header.nav.toggler).click();
			}

			await page.locator(selectors.header.nav.buttons.languageSelector).click();

			const languageSelector = page.locator(selectors.modals.languageSelector);
			await expect(languageSelector).toBeInViewport();

			await languageSelector.getByText('Español').click();
			await expect(languageSelector).toBeHidden();

			if (screenWidthIsAtLeast('lg', page)) {
				await expect(page.locator(selectors.header.description)).toHaveText(
					descriptions['es-ES'],
				);
			}

			expect(await page.evaluate(() => localStorage.getItem('language'))).toBe(
				'es-ES',
			);
		});

		test('through URL', async ({page}) => {
			const checkLanguage = async (lang: 'es-ES' | 'fr-FR') => {
				await page.goto(`/?lang=${lang}`);

				await expect(page.locator(selectors.header.description)).toHaveText(
					descriptions[lang],
				);
				expect(
					await page.evaluate(() => localStorage.getItem('language')),
				).toBe(lang);
			};

			for (const lang of ['es-ES', 'fr-FR']) {
				// eslint-disable-next-line no-await-in-loop
				await checkLanguage(lang as 'es-ES' | 'fr-FR');
			}
		});
	});
});
