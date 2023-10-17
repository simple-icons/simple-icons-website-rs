import type { Config } from 'tailwindcss';
import fs from 'node:fs';
import path from 'node:path';

import defaultTheme from 'tailwindcss/defaultTheme';
import postcssImportPlugin from 'postcss-import';

/**
 * Parse theme variables from stylesheet to automatically insert
 * all custom colors into TailwindCSS configuration.
 */
const parseRootCssVariables = (): Array<string> => {
  return fs
    .readFileSync(path.resolve(`${__dirname}/stylesheet.css`), 'utf8')
    .split('body.dark {', 2)[1]
    .split('}', 2)[0]
    .split('\n')
    .filter((line) => line.startsWith('  --'))
    .map((line) => line.split('--')[1].split(':')[0]);
};

export default {
  content: {
    files: ['index.html', '../{app,components}/src/**/*.{css,rs}'],
  },
  theme: {
    extend: {
      fontFamily: {
        mono: [
          '"Roboto Mono"',
          '"DejaVu Sans Mono"',
          'Consolas',
          'monospace',
          ...defaultTheme.fontFamily.mono,
        ],
        sans: [
          '"Open Sans"',
          'Arial',
          'Helvetica',
          'sans-serif',
          ...defaultTheme.fontFamily.sans,
        ],
      },
      colors: {
        custom: {
          // Custom theme colors like `{background-color: 'var(--background-color)'}`
          // Use them in components as `bg-custom-background-color`
          ...parseRootCssVariables().reduce(
            (o, variable) => ({ ...o, [variable]: `var(--${variable})` }),
            {},
          ),
        },
      },
      screens: {
        // Very smalls screens
        xs: '475px',
      },
    },
  },
  darkMode: 'class',
  plugins: [postcssImportPlugin],
} satisfies Config;
