import { load } from 'js-yaml'
import { readFile } from 'node:fs/promises'
import { join } from 'node:path'

import type { Manifest } from './types.js'

type Definitions = Manifest

export async function readManifest(date: Date) {
	const path = join('hack', 'build_manifest.yaml')
	const manifestContents = await readFile(path, 'utf8')
	const manifest = load(manifestContents) as Manifest

	const year = date.getFullYear()
	manifest.product.copyright_notice = manifest.product.copyright_notice.replace('{YEAR}', year.toString())

	console.log('> Loaded manifest from %s:', path)
	console.log('> - Product: %s (%s)', manifest.product.production_name, manifest.product.code_name)
	console.log('> - License: %s', manifest.product.copyright_notice)
	console.log('> - Contact: %s', manifest.product.contact_email)
	console.log('> - API: %s', manifest.product.api_endpoint)
	console.log('> - Site: %s', manifest.product.site_endpoint)
	console.log('> - Docs: %s', manifest.product.docs_endpoint)
	console.log('> - Database: %s@%s/%s', manifest.database.username, manifest.database.host, manifest.database.database)
	console.log()

	const defines = new Map<string, Record<string, string>>([
		['product', manifest.product],
		['database', manifest.database],
		['bump', manifest.bump]
	])

	return Object.fromEntries(defines) as Definitions
}
