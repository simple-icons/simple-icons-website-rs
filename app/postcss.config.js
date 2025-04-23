/**
 * @file PostCSS configuration file.
 */

import autoprefixer from 'autoprefixer';
import postcssImport from 'postcss-import';
import tailwindcss from 'tailwindcss';

/** @type {import('postcss-load-config').Config} */
const config = {
	plugins: [autoprefixer, tailwindcss, postcssImport],
};

export default config;
