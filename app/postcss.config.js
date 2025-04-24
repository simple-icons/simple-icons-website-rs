/**
 * @file PostCSS configuration file.
 */

import autoprefixer from 'autoprefixer';
import postcssImport from 'postcss-import';
import postcssUrl from 'postcss-url';
import tailwindcss from 'tailwindcss';

/** @type {import('postcss-load-config').Config} */
const config = {
	plugins: [
		postcssImport,
		tailwindcss,
		autoprefixer,
		postcssUrl({
			url(asset) {
				// Rewrite @fontsource fonts URLs to use the assets folder
				// instead of the default ./files/ path.
				if (asset.url.startsWith('./files/')) {
					return `/assets/fonts/${asset.url.slice('./files/'.length)}`;
				}

				return asset.url;
			},
		}),
	],
};

export default config;
