/**
 * @file TailwindCSS configuration file.
 * @see https://tailwindcss.com/docs
 */

import fs from 'node:fs';
import path from 'node:path';
import postcssImportPlugin from 'postcss-import';
import type {Config} from 'tailwindcss';
import defaultTheme from 'tailwindcss/defaultTheme.js';

// TODO: We are not using here `import.meta.url` because the VSCode extension
// does not support modules with TypeScript.
// eslint-disable-next-line unicorn/prefer-module
const __dirname = path.dirname(__filename);

/**
 * Parse theme variables from stylesheet to automatically insert
 * all custom colors into TailwindCSS configuration.
 * @returns {string[]} Array of CSS variable names.
 */
const parseRootCssVariables = (): string[] => {
	const css = fs.readFileSync(path.join(__dirname, 'stylesheet.css'), 'utf8');
	const root = css
		.split(':root')[1]
		.split('}', 2)[0]
		.split('\n')
		.filter((line) => line.startsWith('\t--') && line.includes('-color:'))
		.map((line) => line.split('--')[1].split(':')[0]);

	const dark = css
		.split('body.dark {', 2)[1]
		.split('}', 2)[0]
		.split('\n')
		.filter((line) => line.startsWith('\t--'))
		.map((line) => line.split('--')[1].split(':')[0]);

	return [...root, ...dark];
};

const config = {
	content: {
		files: ['index.html', '../{app,components}/**/*.{css,rs}'],
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
					...Object.fromEntries(
						parseRootCssVariables().map((variable) => [
							variable,
							`var(--${variable})`,
						]),
					),
				},
			},
			screens: {
				// Very smalls screens
				xs: '475px',
			},
		},
	},
	darkMode: 'selector',
	plugins: [postcssImportPlugin],
} satisfies Config;

export default config;
