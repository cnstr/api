import got from 'got'

export async function fetchRepositoryManifest() {
	try {
		const body = await got.get('https://source.canister.me/piracy-repositories.json')
			.json<string[]>()

		console.log('> Fetched cnstr/manifests repository manifest')
		return body
	} catch {
		console.log('! cnstr/manifests is unavailable')
		return []
	}
}
