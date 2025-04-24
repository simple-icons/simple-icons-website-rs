/**
 * @file PostCSS configuration file.
 */

import autoprefixer from 'autoprefixer';
import postcssFontsourceUrl from 'postcss-fontsource-url';
import postcssImport from 'postcss-import';
import tailwindcss from 'tailwindcss';

/** @type {import('postcss-load-config').Config} */
const config = {
	plugins: [
		postcssImport,
		tailwindcss,
		autoprefixer,
		postcssFontsourceUrl({directory: '/assets/fonts'}),
	],
};

export default config;
