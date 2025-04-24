#!/usr/bin/env npx tsx
/**
 * @file Fetch deprecated icons from simple-icons repository using the GitHub GraphQL API.
 */

import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';
import {getGithubToken} from './helpers.ts';

const graphQlQuery = `{
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

const temporaryFilePath = path.join(
	os.tmpdir(),
	'simple-icons-deprecated.json',
);
const temporaryFilePathExists = await fs
	.access(temporaryFilePath)
	.then(() => true)
	.catch(() => false);
if (temporaryFilePathExists) {
	process.exit(0);
}

if (globalThis.fetch === undefined) {
	const nodeMajorVersion = process.version.replace('v', '').split('.')[0];
	if (Number.parseInt(nodeMajorVersion, 10) < 18) {
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
		// eslint-disable-next-line @typescript-eslint/naming-convention
		Authorization: `Bearer ${await getGithubToken()}`,
		'User-Agent': 'Simple Icons website',
	},
	body: JSON.stringify({
		query: graphQlQuery.replaceAll('\n', ''),
	}),
})
	.then(async (response_) => {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
		const response: any = await response_.json();
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

		await fs.writeFile(temporaryFilePath, JSON.stringify(response));
	})
	.catch((error: unknown) => {
		let message = 'Unknown error';
		if (error instanceof Error) {
			message = error.message;
		}

		process.stderr.write(
			`Error retrieving data from GITHUB Graphql API: ${message}\n`,
		);
		process.exit(1);
	});
