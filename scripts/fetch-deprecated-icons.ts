import CONFIG, { getGithubToken } from '../config/config.ts';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';

const GRAPHQL_QUERY = `{
  repository(owner: "simple-icons", name: "simple-icons") {
    milestones(states: [OPEN], first:10) {
      nodes{
        title
        dueOn
        number
        pullRequests(states:[OPEN], first:100){
          nodes{
            number
            files(first:30) {
              edges {
                node {
                  path
                  changeType
                }
              }
            }
          }
        }
      }
    }
  }
}`;

const tmpFilePath = path.join(os.tmpdir(), CONFIG.deprecated_icons_file_name);
const tmpFilePathExists = await fs
  .access(tmpFilePath)
  .then(() => true)
  .catch(() => false);
if (tmpFilePathExists) {
  process.exit(0);
}

if (global.fetch === undefined) {
  const nodeMajorVersion = process.version.replace('v', '').split('.')[0];
  if (parseInt(nodeMajorVersion) < 18) {
    process.stderr.write(
      `Detected unsupported major version of Node.js (v${nodeMajorVersion}).` +
        ` Please upgrade to Node.js 18 or higher.\n`,
    );
    process.exit(1);
  }
}

await fetch('https://api.github.com/graphql', {
  method: 'POST',
  headers: {
    Authorization: `Bearer ${await getGithubToken()}`,
    'User-Agent': 'Simple Icons website',
  },
  body: JSON.stringify({
    query: GRAPHQL_QUERY.replace(/\n/g, ''),
  }),
})
  .then(async (res) => {
    const response = await res.json();
    if (response.message) {
      process.stderr.write(
        `Error retrieving data from GITHUB Graphql API: ${response.message}\n`,
      );
      process.exit(1);
    }
    if (response.errors) {
      process.stderr.write(
        `Error retrieving data from GITHUB Graphql API: ${JSON.stringify(
          response.errors,
          null,
          2,
        )})}\n`,
      );
      process.exit(1);
    }

    await fs.writeFile(tmpFilePath, JSON.stringify(response));
  })
  .catch((err) => {
    process.stderr.write(
      `Error retrieving data from GITHUB Graphql API: ${err}\n`,
    );
    process.exit(1);
  });
