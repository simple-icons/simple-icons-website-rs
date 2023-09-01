import { test, expect } from '@playwright/test';
import {
  selectors,
  getGridItemsIconsTitles,
  screenWidthIsAtLeast,
  N_ICONS_PER_PAGE,
} from '../helpers.ts';

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
    ['КиноПоиск', 'KinoPoisk'],
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

  test('sets the ordering mode to search match score', async ({ page }) => {
    await page.goto('/');

    await page.fill(selectors.controls.search.input, searchValues[0][0]);
    const orderModeControlButtons = await page.locator(
      `${ORDER_MODE_CONTROL_SELECTOR} button`,
    );
    await expect(orderModeControlButtons).toHaveCount(3);
    await expect(orderModeControlButtons.nth(2)).toHaveClass('selected');

    await expect(
      await page.evaluate(() => localStorage.getItem('order-mode')),
    ).toBe(null);
  });

  const orders = [
    ['alphabetical', 'alpha', 1],
    ['color', 'color', 2],
  ];

  for (const [orderName, orderMode, orderModeButtonIndex] of orders) {
    test(`change to ${orderName} order changes the order of icons`, async ({
      page,
    }) => {
      await page.goto('/');

      await page.fill(selectors.controls.search.input, '');
      const matchScoreGridItemsIconsTitles = await getGridItemsIconsTitles(
        page,
      );

      if (!screenWidthIsAtLeast('lg', page)) {
        await page.locator(selectors.controls.toggler).click();
      }

      const orderButton = await page.locator(
        `${ORDER_MODE_CONTROL_SELECTOR} button:nth-child(${orderModeButtonIndex})`,
      );
      await orderButton.click();

      const orderGridItemsIconsTitles = await getGridItemsIconsTitles(page);

      expect(matchScoreGridItemsIconsTitles.length).toBe(
        orderGridItemsIconsTitles.length,
      );

      await expect(
        await page.evaluate(() => localStorage.getItem('order-mode')),
      ).toBe(orderMode);

      // revert to match score order
      if (!screenWidthIsAtLeast('lg', page)) {
        await page.locator(selectors.controls.toggler).click();
      }
      await page.fill(selectors.controls.search.input, 's');
      if (!screenWidthIsAtLeast('lg', page)) {
        await page.locator(selectors.controls.toggler).click();
      }

      await orderButton.click();

      const matchScoreOrderButton = await page.locator(
        `${ORDER_MODE_CONTROL_SELECTOR} button:nth-child(3)`,
      );
      await matchScoreOrderButton.click();

      const matchScoreGridItemsIconsTitles2 = await getGridItemsIconsTitles(
        page,
      );
      expect(matchScoreGridItemsIconsTitles).not.toEqual(
        matchScoreGridItemsIconsTitles2,
      );

      await expect(
        await page.evaluate(() => localStorage.getItem('order-mode')),
      ).toBe(orderMode);

      // remove search value, revert to selected order
      if (!screenWidthIsAtLeast('lg', page)) {
        await page.locator(selectors.controls.toggler).click();
      }
      await page.fill(selectors.controls.search.input, '');

      const matchScoreGridItemsIconsTitles3 = await getGridItemsIconsTitles(
        page,
      );
      expect(matchScoreGridItemsIconsTitles3.length).toBe(N_ICONS_PER_PAGE);
    });
  }
});
