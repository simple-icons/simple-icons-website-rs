#!/usr/bin/env npx tsx
/**
 * @file Fetch deprecated icons from simple-icons repository using the GitHub GraphQL API.
 */

import process from 'node:process';
import {fetchDeprecatedIcons} from './helpers.ts';

await fetchDeprecatedIcons(() => {
	process.exit(0);
});
