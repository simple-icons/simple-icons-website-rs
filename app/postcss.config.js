/**
 * @file PostCSS configuration file.
 */

import process from 'node:process';
import autoprefixer from 'autoprefixer';
import postcssFontsourceUrl from 'postcss-fontsource-url';
import postcssImport from 'postcss-import';
import postcssLightningcss from 'postcss-lightningcss';
import tailwindcss from 'tailwindcss';

/** @type {import('postcss-load-config').Config} */
const config = {
	plugins: [
		postcssImport,
		tailwindcss,
		autoprefixer,
		postcssFontsourceUrl({directory: '/assets/fonts'}),
		postcssLightningcss({
			lightningcssOptions: {minify: process.env.NODE_ENV === 'production'},
		}),
	],
};

export default config;
