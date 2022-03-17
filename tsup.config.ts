import { defineConfig } from 'tsup'
import simpleGit from 'simple-git'
import { load } from 'js-yaml'
import { readFile } from 'node:fs/promises'
import { platform } from 'node:os'
import { exit, version } from 'node:process'

if (!process.env.NODE_ENV) {
	console.error('Missing NODE_ENV')
	exit(1)
}

const header =
	`// ---------------------------------------------------
// Copyright (c) ${new Date().getFullYear()}, Aerum LLC.
// See the attached LICENSE file for more information.
// ---------------------------------------------------`

const definitions = await findDefinitions()
export default defineConfig((options) => {
	return {
		esbuildOptions(options) {
			options.define = definitions
		},

		entry: ['./src/index.ts'],
		target: 'node16',
		splitting: false,
		format: ['esm'],
		platform: 'node',
		sourcemap: true,
		banner: {
			'js': header
		},

		// Development Hook
		onSuccess: options.watch ? 'pnpm start' : undefined
	}
})

type Config = {
	apiEndpoint: string
	siteEndpoint: string

	name: string
	contactEmail: string
	copyrightNotice: string

	servers: {
		name: string
		region: string
		location: string
	}[]

	databases: {
		name: string
		host: string
		username: string
		database: string
		password: string
	}[]
}

async function findDefinitions() {
	const config = load(await readFile('config.yaml', 'utf8')) as Config

	const git = simpleGit('.')
	const commitHash = await git.revparse('HEAD')

	const rawTag = await git.tag(['--sort=committerdate'])
	const tag = rawTag.trim()

	const modified = new Date()
	const modifiedString = `${modified.getFullYear()}.${modified.getMonth()}.${modified.getDate()}`

	const build = `${modifiedString}_${commitHash.substring(0, 7)}`
	const runtimePlatform = `${platform}-${version}_k8s-v1.22.4`

	const database = config.databases.find(config => config.name === process.env.NODE_ENV)
	if (!database) {
		console.error('No database configuration was found')
		exit(2)
	}

	const replacements = new Map<string, any>([
		['__commit', commitHash],
		['__version', tag],
		['__build', build],
		['__platform', runtimePlatform],

		['__name', config.name],
		['__apiEndpoint', config.apiEndpoint],
		['__siteEndpoint', config.siteEndpoint],
		['__contactEmail', config.contactEmail],
		['__copyrightNotice', config.copyrightNotice],

		['__database', database],
		['__servers', config.servers]
	])

	console.log('Using the following replacements: %s', replacements)
	console.log()

	for (const [key, value] of replacements) {
		replacements.set(key, JSON.stringify(value))
	}

	return Object.fromEntries(replacements)
}
