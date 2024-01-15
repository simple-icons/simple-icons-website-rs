import fs from 'node:fs/promises';
import path from 'node:path';
import { transform } from 'lightningcss';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const getCssFiles = async () => {
  const files = await fs.readdir(path.join(__dirname, '../app/public/assets'));
  return files.filter((file) => file.endsWith('.css'));
};

const minifyCss = async () => {
  const cssFiles = await getCssFiles();
  for (const cssFile of cssFiles) {
    const cssFilePath = path.join(__dirname, '../app/public/assets', cssFile);

    const cssFileContent = await fs.readFile(cssFilePath, 'utf-8');
    const minifiedCss = transform({
      filename: path.basename(cssFilePath),
      code: Buffer.from(cssFileContent),
      minify: true,
      sourceMap: true,
    });
    await fs.writeFile(cssFilePath, minifiedCss.code);
  }
};

await minifyCss();
