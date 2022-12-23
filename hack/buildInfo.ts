import { platform, version } from 'node:process'

import { simpleGit } from 'simple-git'

type Definitions = {
	commit: string;
	version: string;
	build: string;
	platform: string;
}

export async function calculateBuildInfo(date: Date) {
	const git = simpleGit('.')

	const commitHash = await git.revparse('HEAD')
	const rawTag = await git.tag(['--sort=-v:refname'])
	const tag = rawTag.split('\n')[0].trim()
		.slice(1)

	const date_tag = `${date.getFullYear()}.${date.getMonth() + 1}.${date.getDate()}`
	const buildTag = `${date_tag}_${commitHash.slice(0, 7)}`
	const runtimePlatform = `${platform}-${version}_k8s-v1.26.0`

	console.log('> Generated build information:')
	console.log('> - Commit hash: %s', commitHash)
	console.log('> - Tag: %s', tag)
	console.log('> - Build tag: %s', buildTag)
	console.log('> - Runtime platform: %s', runtimePlatform)
	console.log()

	const defines = new Map<string, string>([
		['commit', commitHash],
		['version', tag],
		['build', buildTag],
		['platform', runtimePlatform]
	])

	return Object.fromEntries(defines) as Definitions
}
