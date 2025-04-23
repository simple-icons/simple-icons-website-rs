#!/usr/bin/env npx tsx
/**
 * @file Comment lines in a JavaScript file.
 */

import fs from 'node:fs/promises';
import process from 'node:process';

const filePath = process.argv[2];
if (!filePath.endsWith('.js')) {
	process.stderr.write('File must be a JavaScript file\n');
	process.exit(1);
}

if (
	!(await fs
		.access(filePath)
		.then(() => true)
		.catch(() => false))
) {
	process.stderr.write(`File "${filePath}" does not exist\n`);
	process.exit(1);
}

const linesToComment = new Set(process.argv.slice(3));

const content = await fs.readFile(filePath, 'utf8');
const lines = content.split('\n');
let newContent = '';
for (const line of lines) {
	newContent += linesToComment.has(line.trim()) ? `// ${line}\n` : `${line}\n`;
}

await fs.writeFile(filePath, newContent);
