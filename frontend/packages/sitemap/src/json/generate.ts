import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { sitemap } from '../data';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * Generate sitemap.json for non-TypeScript consumers (e.g., Rust backend tests)
 */
function generateSitemapJson(): void {
  const output = {
    routes: {
      'ppt-web': sitemap.routes['ppt-web'],
      'reality-web': sitemap.routes['reality-web'],
    },
    screens: {
      mobile: sitemap.screens.mobile,
    },
    endpoints: {
      'api-server': sitemap.endpoints['api-server'],
      'reality-server': sitemap.endpoints['reality-server'],
    },
    flows: sitemap.flows,
    metadata: {
      generated_at: new Date().toISOString(),
      version: '1.0.0',
      stats: {
        ppt_web_routes: sitemap.routes['ppt-web'].length,
        reality_web_routes: sitemap.routes['reality-web'].length,
        mobile_screens: sitemap.screens.mobile.length,
        api_server_endpoints: sitemap.endpoints['api-server'].length,
        reality_server_endpoints: sitemap.endpoints['reality-server'].length,
        flows: sitemap.flows.length,
      },
    },
  };

  const outputPath = path.join(__dirname, 'sitemap.json');
  fs.writeFileSync(outputPath, JSON.stringify(output, null, 2));

  console.log(`Generated sitemap.json at ${outputPath}`);
  console.log('Stats:', output.metadata.stats);
}

generateSitemapJson();
