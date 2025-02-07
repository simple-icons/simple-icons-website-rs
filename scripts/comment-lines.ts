import fs from 'node:fs/promises';
import process from 'node:process';

process.stdout.write('Commenting lines...\n');
process.stdout.write(`Args: ${process.argv}`);

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
const linesToComment = process.argv.slice(3);

const content = await fs.readFile(filePath, 'utf8');
const lines = content.split('\n');
let newContent = '';
for (const line of lines) {
  if (linesToComment.includes(line.trim())) {
    newContent += `// ${line}\n`;
  } else {
    newContent += `${line}\n`;
  }
}

await fs.writeFile(filePath, newContent);
process.stdout.write(`Commented ${linesToComment.length} lines\n`);
