/**
 * Tests for header component.
 **/

import { test, expect } from '@playwright/test';
import * as icons from 'simple-icons';

test('title and description', async ({ page }) => {
  await page.goto('/');

  await expect(page.locator('h1')).toHaveText('Simple Icons');
  await expect(page.locator('header > :first-child > p')).toHaveText(
    `${Object.keys(icons).length} free SVG icons for popular brands`,
  );
});
