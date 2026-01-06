#!/usr/bin/env npx tsx
/**
 * @file Generates the statistics file for the project.
 */

import fs from 'node:fs/promises';
import path from 'node:path';
import {
	deprecatedIconsFile,
	fetchDeprecatedIcons,
	fileExists,
} from './helpers.ts';

const readmePath = './node_modules/simple-icons/README.md';
const statsAssetPath = './app/public/assets/stats.json';

const countNumberOfLibrariesAndExtensions = (
	readmeContent: string,
): {
	numberOfLibraries: number;
	numberOfExtensions: number;
} => {
	let insideExtensions = false;
	let insideLibraries = false;
	let numberOfExtensions = 0;
	let numberOfLibraries = 0;

	const extensionsHeader = 'Third-Party Extensions';
	const librariesHeader = 'Third-Party Libraries';

	for (const line of readmeContent.split('\n')) {
		if (line.includes(extensionsHeader)) {
			insideExtensions = true;
			insideLibraries = false;
			continue;
		}

		if (line.includes(librariesHeader)) {
			insideExtensions = false;
			insideLibraries = true;
			continue;
		}

		if (line.startsWith('| [')) {
			if (insideExtensions) {
				numberOfExtensions += 1;
			} else if (insideLibraries) {
				numberOfLibraries += 1;
			}
		}

		if (
			numberOfExtensions > 0 &&
			numberOfLibraries > 0 &&
			line.startsWith('#')
		) {
			break;
		}
	}

	return {
		numberOfLibraries,
		numberOfExtensions,
	};
};

const getNumberOfIcons = async (): Promise<number> => {
	const iconsPath = path.join('node_modules', 'simple-icons', 'icons');
	const iconsDir = await fs.readdir(iconsPath);
	return iconsDir.length;
};

const getNumberOfLanguages = async (): Promise<number> => {
	const languagesPath = path.join('app', 'i18n', 'locales');
	const languagesDir = await fs.readdir(languagesPath);
	return languagesDir.length;
};

const getSimpleIconsVersion = async (): Promise<string> => {
	const packageJsonPath = path.join(
		'node_modules',
		'simple-icons',
		'package.json',
	);
	const packageJsonContent = await fs.readFile(packageJsonPath, 'utf8');
	// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
	const packageJson: {version: string} = JSON.parse(packageJsonContent);
	return packageJson.version;
};

const getRustToolchainChannel = async (): Promise<string> => {
	const rustToolchainPath = 'rust-toolchain.toml';
	const rustToolchainContent = await fs.readFile(rustToolchainPath, 'utf8');
	const rustToolchainLines = rustToolchainContent.split('\n');

	for (const line of rustToolchainLines) {
		if (line.startsWith('channel = ')) {
			return line.split('"')[1];
		}
	}

	throw new Error('Rust toolchain version not found in rust-toolchain.toml');
};

const getNumberOfDeprecatedIcons = async (): Promise<number> => {
	if (!(await fileExists(deprecatedIconsFile))) {
		throw new Error(
			'Run `cargo make` to execute the script fetch-deprecated-icons.rs and build.',
		);
	}

	const deprecatedIconsContent = await fs.readFile(deprecatedIconsFile, 'utf8');
	// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
	const deprecatedIcons = JSON.parse(deprecatedIconsContent);

	if (deprecatedIcons.message) {
		await fs.unlink(deprecatedIconsFile);
		throw new Error(
			`Error retrieving data from GITHUB Graphql API: ${deprecatedIcons.message}`,
		);
	}

	// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
	const milestonesData = deprecatedIcons.data.repository.milestones.nodes;
	let count = 0;

	for (const milestoneData of milestonesData) {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
		const pullRequestsData = milestoneData.pullRequests.nodes;

		for (const pullRequestData of pullRequestsData) {
			// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
			const filesData = pullRequestData.files.edges;

			for (const fileData of filesData) {
				// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
				const filePath = fileData.node.path ?? '';
				// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
				const {changeType} = fileData.node;

				if (
					// eslint-disable-next-line @typescript-eslint/no-unsafe-call
					filePath.startsWith('icons/') &&
					// eslint-disable-next-line @typescript-eslint/no-unsafe-call
					filePath.endsWith('.svg') &&
					changeType === 'DELETED'
				) {
					count++;
				}
			}
		}
	}

	return count;
};

const main = async () => {
	const readmeContent = await fs.readFile(readmePath, 'utf8');
	const {numberOfLibraries, numberOfExtensions} =
		countNumberOfLibrariesAndExtensions(readmeContent);

	await fetchDeprecatedIcons();
	const [
		numberOfIcons,
		numberOfLanguages,
		simpleIconsVersion,
		numberOfDeprecatedIcons,
		rustToolchainChannel,
	] = await Promise.all([
		getNumberOfIcons(),
		getNumberOfLanguages(),
		getSimpleIconsVersion(),
		getNumberOfDeprecatedIcons(),
		getRustToolchainChannel(),
	]);

	const stats = {
		numberOfIcons,
		numberOfDeprecatedIcons,
		numberOfLibraries,
		numberOfExtensions,
		numberOfLibrariesAndExtensions: numberOfLibraries + numberOfExtensions,
		numberOfLanguages,
		simpleIconsVersion,
		rustToolchainChannel,
	};

	await fs.mkdir(path.dirname(statsAssetPath), {recursive: true});
	await fs.writeFile(statsAssetPath, JSON.stringify(stats, null, 0), 'utf8');
};

await main();
