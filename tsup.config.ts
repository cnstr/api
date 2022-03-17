import { defineConfig } from 'tsup'
import simpleGit from 'simple-git'
import { stat } from 'node:fs/promises'

const header =
`// ---------------------------------------------------
// Copyright (c) ${new Date().getFullYear()}, Aerum LLC.
// See the attached LICENSE file for more information.
// ---------------------------------------------------`

const git = simpleGit('.')

const commit = await git.revparse('HEAD')
const tag = await git.tag(['--sort=committerdate'])

const stats = await stat('dist/index.js')
const modified = new Date(stats.mtime)
const modifiedString = `${modified.getFullYear()}.${modified.getMonth()}.${modified.getDate()}`

const replacements = {
	'GIT_COMMIT': `"${commit}"`,
	'RELEASE_VERSION': `"${tag.trim()}"`,
	'FILE_MODIFIED': `"${modifiedString}"`,
	'API_ENDPOINT': '"https://api.canister.me/v2"'
}

console.log('Using the following replacements: %s', replacements)
console.log()

export default defineConfig((options) => {
	return {
		esbuildOptions(options) {
			options.define = replacements
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
