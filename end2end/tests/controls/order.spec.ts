import { test, expect } from '@playwright/test';
import {
  screenWidthIsAtLeast,
  selectors,
  N_ICONS_PER_PAGE,
  getGridItemsIconsTitles,
} from '../helpers.ts';

const ORDER_MODE_CONTROL_SELECTOR = selectors.controls.buttons.getByNthChild(1);

test.describe('order mode', () => {
  test('is random by default', async ({ page }) => {
    await page.goto('/');
    const orderModeButtons = await page.locator(
      `${ORDER_MODE_CONTROL_SELECTOR} button`,
    );
    await expect(orderModeButtons).toHaveCount(3);
    await expect(orderModeButtons.nth(2)).toHaveClass('selected');
  });

  test('alphabetical -> color', async ({ page }) => {
    await page.goto('/');
    const alphabeticalGridItemIconsTitles = await getGridItemsIconsTitles(page);

    if (!screenWidthIsAtLeast('lg', page)) {
      await page.locator(selectors.controls.toggler).click();
    }

    const orderModeButtons = await page.locator(
      `${ORDER_MODE_CONTROL_SELECTOR} button`,
    );

    await orderModeButtons.nth(1).click();
    await expect(orderModeButtons.nth(1)).toHaveClass('selected');

    const colorGridItemIconsTitles = await getGridItemsIconsTitles(page);

    // Check that the page has the correct number of icons
    expect(colorGridItemIconsTitles).toHaveLength(N_ICONS_PER_PAGE);

    // Check that the order is different from the alphabetical one
    // NOTE: This could fail if the icons are sorted by color in alphabetical
    // order, but it's really unlikely, almost impossible in a future.
    expect(alphabeticalGridItemIconsTitles).not.toEqual(
      colorGridItemIconsTitles,
    );

    // Check that the new order is stored on localStorage
    expect(await page.evaluate(() => localStorage.getItem('order-mode'))).toBe(
      'color',
    );

    await page.reload();
    await expect(
      page.locator(`${ORDER_MODE_CONTROL_SELECTOR} button`).nth(1),
    ).toHaveClass('selected');
  });
});
