import { test, expect } from '@playwright/test';
import { selectors } from '../helpers.ts';

const ORDER_MODE_CONTROL_SELECTOR = selectors.controls.buttons.getByNthChild(1);

test.describe('search', () => {
  const searchValues = [
    ['simple icons', 'Simple Icons'],
    // slug
    ['dotnet', '.NET'],
    // aka alias
    ['drawio', 'diagrams.net'],
    // dup alias
    ['gotowebinar', 'GoToMeeting'],
    // loc alias
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

  test('sets the ordering mode to random', async ({ page }) => {
    await page.goto('/');

    await page.fill(selectors.controls.search.input, searchValues[0][0]);
    const orderModeControlButtons = await page.locator(
      `${ORDER_MODE_CONTROL_SELECTOR} button`,
    );
    await expect(orderModeControlButtons).toHaveCount(4);
    await expect(orderModeControlButtons.nth(3)).toHaveClass('selected');

    await expect(
      await page.evaluate(() => localStorage.getItem('order-mode')),
    ).toBe('random');
  });
});
