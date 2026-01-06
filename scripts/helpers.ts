/**
 * @file Helpers for different scripts.
 */

import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';
import {fileURLToPath} from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export const getGithubToken = async () => {
	if (process.env.GITHUB_TOKEN) {
		return process.env.GITHUB_TOKEN;
	}

	const envFilePath = path.resolve(__dirname, '..', '.env');
	if (!(await fileExists(envFilePath))) {
		throw new Error(`.env file not found at ${envFilePath}`);
	}

	const envFile = await fs.readFile(envFilePath, 'utf8');
	for (const line of envFile.split('\n')) {
		const [key, value] = line.split('=');
		if (key.trim() === 'GITHUB_TOKEN') {
			return value.split('"')[1].trim();
		}
	}

	throw new Error(`GITHUB_TOKEN not found in .env file at ${envFilePath}`);
};

export const fileExists = async (filePath: string): Promise<boolean> => {
	return (
		fs
			.access(filePath)
			// eslint-disable-next-line promise/prefer-await-to-then
			.then(() => true)
			// eslint-disable-next-line promise/prefer-await-to-then
			.catch(() => false)
	);
};

const defaultOnError = (error: string) => {
	process.stderr.write(error);
	// eslint-disable-next-line unicorn/no-process-exit
	process.exit(1);
};

export const deprecatedIconsFile = path.join(
	os.tmpdir(),
	'simple-icons-deprecated.json',
);

export const fetchDeprecatedIcons = async (
	// eslint-disable-next-line @typescript-eslint/no-empty-function
	onSuccess: () => void = () => {},
	onError: (string) => void = defaultOnError,
) => {
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

	if (await fileExists(deprecatedIconsFile)) {
		onSuccess();
		return;
	}

	if (globalThis.fetch === undefined) {
		const nodeMajorVersion = process.version.replace('v', '').split('.')[0];
		if (Number.parseInt(nodeMajorVersion, 10) < 18) {
			onError(
				`Detected unsupported major version of Node.js (v${nodeMajorVersion}).` +
					` Please upgrade to Node.js 18 or higher.\n`,
			);
		}
	}

	try {
		const raw = await fetch('https://api.github.com/graphql', {
			method: 'POST',
			headers: {
				// eslint-disable-next-line @typescript-eslint/naming-convention
				Authorization: `Bearer ${await getGithubToken()}`,
				'User-Agent': 'Simple Icons website',
			},
			body: JSON.stringify({
				query: graphQlQuery.replaceAll('\n', ''),
			}),
		});

		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
		const response: any = await raw.json();
		if (response.message) {
			onError(
				`Error retrieving data from GITHUB Graphql API: ${response.message}\n`,
			);
		}

		if (response.errors) {
			const apiErrors = JSON.stringify(response.errors, null, 2);
			onError(
				`Error retrieving data from GITHUB Graphql API: ${apiErrors})}\n`,
			);
		}

		await fs.writeFile(deprecatedIconsFile, JSON.stringify(response));
		onSuccess();
	} catch (error: unknown) {
		let message = 'Unknown error';
		if (error instanceof Error) {
			message = error.message;
		}

		onError(`Error retrieving data from GITHUB Graphql API: ${message}\n`);
	}
};
