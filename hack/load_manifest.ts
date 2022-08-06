import { load } from 'js-yaml'
import { readFile } from 'node:fs/promises'
import { join } from 'node:path'

import type { config_manifest } from './types.js'

export async function load_manifest() {
	const file_path = join('hack', 'build_manifest.yaml')
	const file_contents = await readFile(file_path, 'utf8')
	const manifest = load(file_contents) as config_manifest

	const year = new Date()
		.getFullYear()
	manifest.product.copyright_notice = manifest.product.copyright_notice.replace('{YEAR}', year.toString())

	console.log('> Loaded manifest from %s', file_path)
	console.log('> Product: %s (%s)', manifest.product.production_name, manifest.product.code_name)
	console.log('> License: %s', manifest.product.copyright_notice)
	console.log('> Contact: %s', manifest.product.contact_email)
	console.log('> API: %s', manifest.product.api_endpoint)
	console.log('> Site: %s', manifest.product.site_endpoint)
	console.log('> Database: %s@%s/%s', manifest.database.username, manifest.database.host, manifest.database.database)
	console.log()

	return new Map<string, unknown>([
		['product', manifest.product],
		['database', manifest.database]
	])
}
