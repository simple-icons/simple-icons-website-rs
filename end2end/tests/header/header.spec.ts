/**
 * Tests for header component.
 **/

import { test, expect } from '@playwright/test';
import { screenWidthIsAtLeast, selectors } from '../helpers.ts';

test.describe('header', () => {
  test('has title', async ({ page }) => {
    await page.goto('/');

    const title = page.locator(selectors.header.title);
    await expect(title).toHaveText('Simple Icons');
    await expect(title).toBeInViewport();
  });

  test.describe('nav', () => {
    const N_MENU_BUTTONS = 10;
    const nav = selectors.header.nav.container;

    test('has menu buttons', async ({ page }) => {
      await page.goto('/');

      if (screenWidthIsAtLeast('lg', page)) {
        // desktop
        await expect(
          page.locator(selectors.header.nav.buttons.container),
        ).toBeInViewport();
        await expect(page.locator(`${nav} > ul > li:visible`)).toHaveCount(
          N_MENU_BUTTONS,
        );

        // burger menu is hidden
        await expect(page.locator(selectors.header.nav.toggler)).toBeHidden();
      } else {
        // mobile
        await expect(
          page.locator(selectors.header.nav.buttons.container),
        ).toBeHidden();
        await expect(page.locator(`${nav} > ul li:visible`)).toHaveCount(1);

        // burger menu is visible
        const burgerButton = page.locator(selectors.header.nav.toggler);
        await expect(burgerButton).toBeInViewport();
        burgerButton.click();

        await expect(
          page.locator(selectors.header.nav.buttons.container),
        ).toBeInViewport();
        await expect(page.locator(`${nav} > ul > li:visible`)).toHaveCount(
          N_MENU_BUTTONS + 1,
        );
      }
    });
  });
});
