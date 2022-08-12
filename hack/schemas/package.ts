/* eslint-disable unicorn/no-null */

import { Package } from '@canister/models'

const nullable = new Set([
	'sha256',
	'name',
	'description',
	'author',
	'maintainer',
	'depiction',
	'nativeDepiction',
	'sileoDepiction',
	'header',
	'tintColor',
	'icon',
	'section',
	'tag',
	'installedSize'
])

function recurseSchema(record: Record<string, unknown>) {
	const properties: Record<string, unknown> = {}

	for (const [key, value] of Object.entries(record)) {
		if (Array.isArray(value)) {
			properties[key] = {
				type: 'array',
				items: {
					type: 'string'
				}
			}

			continue
		}

		if (value instanceof Object) {
			properties[key] = recurseSchema(value as Record<string, unknown>)
			continue
		}

		properties[key] = {
			type: typeof value,
			example: value,
			nullable: nullable.has(key)
		}
	}

	return {
		type: 'object',
		properties
	}
}

export function generatePackageSchema() {
	const examplePackage = new Package()

	Object.assign(examplePackage, {
		databaseId: 125_157,
		package: 'com.muirey03.cr4shed',
		isCurrent: true,
		isPruned: false,
		repository: 'havoc',
		repositorySlug: 'havoc',
		price: 'Free',
		tier: 1,
		version: '4.2.2',
		architecture: 'iphoneos-arm',
		filename: 'api/download/package/61f940c201a1cb7a5c422d9a/com.muirey03.cr4shed_4.2.2.deb',
		size: '218172',
		sha256: 'cc189e77794f66930ff4171fbd35ab38956669353d1d925a8f8b8aecd844accf',
		name: 'Cr4shed',
		description: 'A modern crash reporter for iOS',
		author: 'Muirey03 <tcmuir03@gmail.com>',
		maintainer: 'Muirey03 <tcmuir03@gmail.com>',
		depiction: 'https://havoc.app/depiction/cr4shed',
		nativeDepiction: null,
		sileoDepiction: 'https://havoc.app/package/cr4shed/depiction.json',
		header: null,
		tintColor: null,
		icon: 'https://media.havoc.app/61f940a401a1cb7a5c422cc5',
		section: 'Tweaks',
		tag: [],
		installedSize: '1856',
		refs: {
			meta: 'https://api.canister.me/v2/jailbreak/get/package?q=com.muirey03.cr4shed',
			repo: 'https://api.canister.me/v2/jailbreak/get/repository?q=havoc'
		}
	})

	return recurseSchema(examplePackage as unknown as Record<string, unknown>)
}

