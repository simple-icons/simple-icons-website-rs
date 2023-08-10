import CONFIG from '../config/config.ts';
import fs from 'node:fs/promises';

const sitemapAssetPath = 'app/assets/sitemap.xml';
const sitemapAssetExists = await fs
  .access(sitemapAssetPath)
  .then(() => true)
  .catch(() => false);
if (sitemapAssetExists) {
  await fs.unlink(sitemapAssetPath);
}

const protocol = CONFIG.domain.includes(':') ? 'http' : 'https';
const sitemap =
  `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"` +
  ` xmlns:xhtml="http://www.w3.org/1999/xhtml">\n` +
  `  <url>\n` +
  `    <loc>${protocol}://${CONFIG.domain}/</loc>\n` +
  `    <lastmod>${new Date().toISOString()}</lastmod>\n` +
  `    <changefreq>weekly</changefreq>\n` +
  `  </url>\n` +
  `</urlset>`;
await fs.writeFile(sitemapAssetPath, sitemap);
