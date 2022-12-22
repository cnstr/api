import { env } from 'node:process'

import got from 'got'
import { defineConfig } from 'tsup'

import { calculateBuildInfo } from './buildInfo.js'
import { bumpDeployManifest } from './k8s.js'
import { readManifest } from './manifest.js'
import { generateDocumentation } from './openapi.js'
import { fetchRepositoryManifest } from './upstream.js'

const buildDate = new Date()

const manifest = await readManifest(buildDate)
const buildInfo = await calculateBuildInfo(buildDate)
const documentation = generateDocumentation(manifest.product, buildInfo.version)

const repositories = await fetchRepositoryManifest()

const definitions = new Map<string, string>()
definitions.set('$openapi', JSON.stringify(documentation))
definitions.set('$repos', JSON.stringify(repositories))

for (const [key, value] of Object.entries({ ...manifest, ...buildInfo })) {
	definitions.set(`$${key}`, JSON.stringify(value))
}

if (env.BUMP_K8S === '1') {
	await bumpDeployManifest(buildInfo.version)
	const { statusCode, body } = await got.post('https://bump.sh/api/v1/versions', {
		json: {
			documentation: manifest.bump.documentation_id,
			definition: documentation.yaml
		},
		headers: {
			Authorization: `Token ${manifest.bump.access_token}`
		}
	})

	if (statusCode === 201) {
		console.log('> Updated documentation on bump.sh')
	} else if (statusCode === 204) {
		console.log('> Unchanged documentation on bump.sh')
	} else {
		console.log('> Failed to update documentation on bump.sh')
		console.log(statusCode, body)
	}
}

if (env.PRODUCTION === '1') {
	definitions.set('$database', JSON.stringify({
		host: 'postgres',
		username: 'postgres',
		password: 'postgres',
		database: 'canister'
	}))
}

const headerString = `// ---------------------------------------------------
// Copyright (c) ${buildDate.getFullYear()}, Aerum LLC.
// See the attached LICENSE file for more information.
// ---------------------------------------------------`

export default defineConfig(options => ({
	esbuildOptions(options) {
		options.define = Object.fromEntries(definitions)
	},
	entry: ['./src/index.ts'],
	clean: !options.watch,
	dts: !options.watch,
	target: 'node18',
	splitting: false,
	format: ['esm'],
	platform: 'node',
	sourcemap: options.watch ? 'inline' : false,
	minify: !options.watch,
	banner: {
		js: headerString
	},

	// Development Hook
	onSuccess: options.watch ? 'pnpm debug' : undefined
}))
