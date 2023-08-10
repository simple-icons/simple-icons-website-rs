import CONFIG from '../config/config.ts';
import fs from 'node:fs/promises';

const cnameAssetPath = 'app/assets/CNAME';
const cnameAssetExists = await fs
  .access(cnameAssetPath)
  .then(() => true)
  .catch(() => false);
if (cnameAssetExists) {
  await fs.unlink(cnameAssetPath);
}

await fs.writeFile(cnameAssetPath, CONFIG.domain);
