import { defineConfig } from 'tsup'

import { compile_openapi } from './compile_openapi.js'
import { generate_build } from './generate_build.js'
import { load_manifest } from './load_manifest.js'
import type { config_manifest } from './types.js'

const manifest_defines = await load_manifest()
const build_defines = await generate_build()

const product = manifest_defines.get('product') as config_manifest['product']
const version = build_defines.get('version') ?? ''
const openapi_spec = compile_openapi(product, version)

const defines_map = new Map<string, string>()

for (const [key, value] of manifest_defines) {
	defines_map.set(`__${key}`, JSON.stringify(value))
}

for (const [key, value] of build_defines) {
	defines_map.set(`__${key}`, JSON.stringify(value))
}

for (const [key, value] of Object.entries(openapi_spec)) {
	defines_map.set(`__${key}`, JSON.stringify(value))
}

const year = new Date()
	.getFullYear()

export default defineConfig(options => ({
	esbuildOptions(options) {
		options.define = Object.fromEntries(defines_map)
	},
	entry: ['./src/index.ts'],
	target: 'node18',
	splitting: false,
	format: ['esm'],
	platform: 'node',
	sourcemap: true,
	banner: {
		js: `// ---------------------------------------------------
		// Copyright (c) ${year}, Aerum LLC.
		// See the attached LICENSE file for more information.
		// ---------------------------------------------------`
	},

	// Development Hook
	onSuccess: options.watch ? 'pnpm start' : undefined
}))
