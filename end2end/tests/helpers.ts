import type { Page, Download, TestType } from '@playwright/test';
import * as path from 'node:path';
import * as simpleicons from 'simple-icons';
import { getDirnameFromImportMeta } from 'simple-icons/sdk';
import CONFIG from '../../config/config.ts';

const __dirname = getDirnameFromImportMeta(import.meta.url);

export const OUTPUT_DIR = 'test-results/';
const ROOT_DIR = path.resolve(__dirname, '../../');

// Number of icons in simple-icons library
export const N_ICONS = Object.keys(simpleicons).length;

// Directory path of simple-icons npm package
export const SIMPLE_ICONS_DIRPATH = path.resolve(
  ROOT_DIR,
  'node_modules/simple-icons',
);

export const N_ICONS_PER_PAGE = CONFIG.icons_per_page_comfortable;

const getViewportSize = (page: Page): { width: number; height: number } => {
  const size = page.viewportSize();
  if (!size) {
    throw new Error('Viewport size is not set');
  }
  return size;
};

/**
 * Get if the viewport of a page is at least the given breakpoint.
 *
 * @param br TailwindCSS breakpoint
 * @param page Playwright page
 * @returns Whether the viewport is at least the given breakpoint
 */
export const screenWidthIsAtLeast = (
  br: 'xs' | 'sm' | 'md' | 'lg',
  page: Page,
): boolean => {
  const size = getViewportSize(page);

  switch (br) {
    case 'xs':
      return size.width >= 475;
    case 'sm':
      return size.width >= 640;
    case 'md':
      return size.width >= 768;
    case 'lg':
      return size.width >= 1024;
    default:
      return false;
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
      getByNthChild: (n: number) =>
        `menu > .controls-group:not(:first-child) > .control:nth-child(${n})`,
    },
    search: {
      input: 'menu > .controls-group:first-child input',
    },
  },
  /* Selectors for the grid */
  grid: (() => {
    const gridSelector = 'main > ul';
    const gridItemsSelector = `${gridSelector} > li`;
    const getGridItemByNthChild = (n: number) =>
      `${gridItemsSelector}:nth-child(${n})`;

    const buildGridItemSelectors = (
      containerSelector: string,
      /* eslint-disable-next-line @typescript-eslint/no-explicit-any */
    ): Record<'container' | 'icon', any> => {
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
        first: buildGridItemSelectors(getGridItemByNthChild(1)),
        /* Get selector for a random grid item */
        any: buildGridItemSelectors(
          getGridItemByNthChild(
            Math.floor(Math.random() * (N_ICONS_PER_PAGE - 1 + 1) + 1),
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
 *
 * @param download Playwright download
 * @param filename Filename of the download
 * @returns Path to the saved file
 */
export const saveDownload = async (
  download: Download,
  filename: string,
): Promise<string> => {
  const outputPath = path.resolve(OUTPUT_DIR, filename);
  await download.saveAs(outputPath);
  return outputPath;
};

/**
 * Get icons titles from the grid.
 * @param page Playwright page
 * @returns Icons titles shown in the grid
 */
export const getGridItemsIconsTitles = async (
  page: Page,
): Promise<Array<string>> => {
  return Promise.all(
    (await page.locator(`${selectors.grid.items} h2`).all()).map(
      async ($iconTitle) => await $iconTitle.textContent(),
    ),
  );
};

/**
 * Helper to set local storage on `test.use` with Playwright.
 */
export const useLocalStorage = (
  /* eslint-disable-next-line @typescript-eslint/no-explicit-any */
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
