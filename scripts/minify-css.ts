#!/usr/bin/env npx tsx
/**
 * @file Minify the CSS files served on production.
 */
/* eslint-disable promise/prefer-await-to-then */

import {Buffer} from 'node:buffer';
import fs from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import {fileURLToPath} from 'node:url';
import {transform} from 'lightningcss';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const publicAssets = path.join(__dirname, '../app/public/assets');

const getCssFiles = async () => {
	const files = await fs.readdir(publicAssets);
	return files.filter((file) => file.endsWith('.css'));
};

const minifyCss = async () => {
	const cssFiles = await getCssFiles();
	const miniferPromises = cssFiles.map(
		async (cssFile) =>
			new Promise<void>((resolve, reject) => {
				const cssFilePath = path.join(publicAssets, cssFile);

				fs.readFile(cssFilePath, 'utf8')
					.then((cssFileContent) => {
						const minifiedCss = transform({
							filename: path.basename(cssFilePath),
							code: Buffer.from(cssFileContent),
							minify: true,
							sourceMap: true,
						});
						fs.writeFile(cssFilePath, minifiedCss.code)
							.then(() => {
								resolve();
							})
							.catch((error_: unknown) => {
								let message = 'Unknown error';
								if (error_ instanceof Error) {
									message = error_.message;
								}

								const error = new Error(
									`Error writing ${cssFilePath}: ${message}`,
								);
								process.stderr.write(`${error.message}\n`);
								reject(error);
							});
					})
					.catch((error_: unknown) => {
						let message = 'Unknown error';
						if (error_ instanceof Error) {
							message = error_.message;
						}

						const error = new Error(`Error reading ${cssFilePath}: ${message}`);
						process.stderr.write(`${error.message}\n`);
						reject(error);
					});
			}),
	);

	Promise.allSettled(miniferPromises).catch((error: unknown) => {
		const errorMessage = new Error(
			'Error minifying CSS files. Please check the error messages above.',
		);
		process.stderr.write(`${errorMessage.message}\n`);
		process.exit(1);
	});
};

await minifyCss();
