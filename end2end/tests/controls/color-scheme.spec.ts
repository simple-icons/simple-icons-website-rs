import { test, expect, type Page } from '@playwright/test';
import { screenWidthIsAtLeast, selectors } from '../helpers.ts';

const COLOR_SCHEME_CONTROL_SELECTOR =
  selectors.controls.buttons.getByNthChild(2);

const getSystemColorScheme = async (page: Page): Promise<'dark' | 'light'> => {
  return await page.evaluate(() => {
    return window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light';
  });
};

test.describe('color scheme', () => {
  test('system -> opposite', async ({ page }) => {
    await page.goto('/');

    if (!screenWidthIsAtLeast('lg', page)) {
      await page.locator(selectors.controls.toggler).click();
    }

    const colorSchemeButtons = await page.locator(
      `${COLOR_SCHEME_CONTROL_SELECTOR} button`,
    );

    const prevSystemColorScheme = await getSystemColorScheme(page);
    const oppositeColorScheme =
      prevSystemColorScheme === 'dark' ? 'light' : 'dark';
    const oppositeColorSchemeNthButton =
      oppositeColorScheme === 'light' ? 0 : 1;

    await colorSchemeButtons.nth(oppositeColorSchemeNthButton).click();
    await expect(
      colorSchemeButtons.nth(oppositeColorSchemeNthButton),
    ).toHaveClass('selected');

    await expect(page.locator(selectors.body)).toHaveClass(oppositeColorScheme);

    await expect(
      await page.evaluate(() => localStorage.getItem('color-scheme')),
    ).toBe(oppositeColorScheme);
  });

  const colorSchemeButtons = ['light', 'dark', 'auto'];
  for (const colorSchemeButtonIndex in colorSchemeButtons) {
    const colorScheme = colorSchemeButtons[colorSchemeButtonIndex];
    test(`${colorScheme} through URL`, async ({ page }) => {
      await page.goto(`/?color-scheme=${colorScheme}`);
      await expect(
        await page
          .locator(`${COLOR_SCHEME_CONTROL_SELECTOR} button`)
          .nth(parseInt(colorSchemeButtonIndex)),
      ).toHaveClass('selected');
      await expect(
        await page.evaluate(() => localStorage.getItem('color-scheme')),
      ).toBe(colorScheme);
    });
  }
});
