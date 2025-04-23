#!/usr/bin/env npx tsx
/**
 * @file Minify JS files that act as WASM bindings on production.
 */

import fs from 'node:fs/promises';
import {minify} from 'terser';

const searchPath = async () => {
	const files = await fs.readdir('./app/dist');
	for (const file of files) {
		if (file.endsWith('.js')) {
			return `./app/dist/${file}`;
		}
	}

	throw new Error('No JS file found in ./app/dist');
};

const path = await searchPath();
const content = await fs.readFile(path, 'utf8');
const result = await minify(content, {
	sourceMap: false,
});
if (result.code === undefined) {
	throw new Error('Minification failed.');
}

await fs.writeFile(path, result.code);
