import got from 'got'
import { env } from 'node:process'
import { defineConfig } from 'tsup'

import { calculateBuildInfo } from './buildInfo.js'
import { bumpDeployManifest } from './k8s.js'
import { readManifest } from './manifest.js'
import { generateDocumentation } from './openapi.js'

const buildDate = new Date()

const manifest = await readManifest(buildDate)
const buildInfo = await calculateBuildInfo(buildDate)
const documentation = generateDocumentation(manifest.product, buildInfo.version)

const definitions = new Map<string, string>()
definitions.set('$openapi', JSON.stringify(documentation))

for (const [key, value] of Object.entries({ ...manifest, ...buildInfo })) {
	definitions.set(`$${key}`, JSON.stringify(value))
}

if (env.BUMP_K8S === '1') {
	await bumpDeployManifest(buildInfo.version)
	await got.post('https://bump.sh/api/v1/versions', {
		json: {
			documentation: manifest.bump.documentation_id,
			definition: documentation.yaml
		},
		headers: {
			Authorization: `Token ${manifest.bump.access_token}`
		}
	})
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
