import { platform, version } from 'node:process'
import { default as git_client } from 'simple-git'

export async function generate_build() {
	const git = git_client('.')

	const commit_hash = await git.revparse('HEAD')
	const raw_tag = await git.tag(['--sort=-v:refname'])
	const tag = raw_tag.split('\n')[0].trim()
		.slice(1)

	const last_modified = new Date()
	const date_tag = `${last_modified.getFullYear()}.${last_modified.getMonth() + 1}.${last_modified.getDate()}`

	const build_tag = `${date_tag}_${commit_hash.slice(0, 7)}`
	const runtime_platform = `${platform}-${version}_k8s-v1.22.4`

	console.log('> Generated build information')
	console.log('> Commit hash: %s', commit_hash)
	console.log('> Tag: %s', tag)
	console.log('> Build tag: %s', build_tag)
	console.log('> Runtime platform: %s', runtime_platform)
	console.log()

	return new Map<string, string>([
		['commit', commit_hash],
		['version', tag],
		['build', build_tag],
		['platform', runtime_platform]
	])
}
