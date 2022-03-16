import { defineConfig } from 'tsup'

const header =
`// ---------------------------------------------------
// Copyright (c) ${new Date().getFullYear()}, Aerum LLC.
// See the attached LICENSE file for more information.
// ---------------------------------------------------`

export default defineConfig((options) => {
	return {
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
