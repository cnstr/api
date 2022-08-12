import { defineConfig } from 'tsup'

import { generate_build } from './generate_build.js'
import { load_manifest } from './load_manifest.js'
import { generateDocumentation } from './openapi.js'
import type { config_manifest } from './types.js'
import { update_k8s } from './update_k8s.js'

const manifest_defines = await load_manifest()
const build_defines = await generate_build()

const product = manifest_defines.get('product') as config_manifest['product']
const version = build_defines.get('version') ?? ''
const documentation = generateDocumentation(product, version)

const defines_map = new Map<string, string>()

for (const [key, value] of manifest_defines) {
	defines_map.set(`$${key}`, JSON.stringify(value))
}

for (const [key, value] of build_defines) {
	defines_map.set(`$${key}`, JSON.stringify(value))
}

defines_map.set('$openapi', JSON.stringify(documentation))
if (process.env.BUMP_K8S) {
	await update_k8s(version)
}

const year = new Date()
	.getFullYear()

const headerString = `// ---------------------------------------------------
// Copyright (c) ${year}, Aerum LLC.
// See the attached LICENSE file for more information.
// ---------------------------------------------------`

export default defineConfig(options => {
	if (!options.watch) {
		// TODO: Fix
		defines_map.set('$database', JSON.stringify({
			host: 'postgres',
			username: 'postgres',
			password: 'postgres',
			database: 'canister'
		}))
	}

	return {
		esbuildOptions(options) {
			options.define = Object.fromEntries(defines_map)
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
	}
})
