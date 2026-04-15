import fs from 'node:fs/promises';
import path from 'node:path';
import { optimize, loadConfig } from 'svgo';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ICONS_DIR = path.resolve(__dirname, '../node_modules/simple-icons/icons');

async function main() {
  try {
    const config = await loadConfig(); 
    const files = (await fs.readdir(ICONS_DIR)).filter(f => f.endsWith('.svg'));
    
    console.log(`Checking ${files.length} icons for optimization...`);
    
    let unoptimizedFiles = [];

    for (const file of files) {
      const filePath = path.join(ICONS_DIR, file);
      const originalSvg = await fs.readFile(filePath, 'utf8');
      
      // Run SVGO optimization
      const result = optimize(originalSvg, config || { multipass: true });
      const optimizedSvg = result.data;

      // Compare lengths
      if (optimizedSvg.length < originalSvg.length) {
        unoptimizedFiles.push({
          file,
          savings: originalSvg.length - optimizedSvg.length
        });
      }
    }

    if (unoptimizedFiles.length > 0) {
      console.error(`\FAILED!!: ${unoptimizedFiles.length} icons are not fully optimized!`);
      unoptimizedFiles.slice(0, 10).forEach(item => {
        console.error(`- ${item.file}: can be reduced by ${item.savings} bytes`);
      });
      if (unoptimizedFiles.length > 10) console.log(`...and ${unoptimizedFiles.length - 10} more.`);
      
      // THIS IS THE MOST IMPORTANT PART FOR 481
      process.exit(1); 
    } else {
      console.log('SUCCESS!!!: All icons are fully optimized!');
      process.exit(0);
    }
  } catch (err) {
    console.error("Error during linting:", err.message);
    process.exit(1);
  }
}
main();