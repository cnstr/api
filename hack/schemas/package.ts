/* eslint-disable unicorn/no-null */

import { Package as DBPackage } from '@canister/models'

import { generateSchema } from './generator.js'

const examplePackage = new DBPackage()
Object.assign(examplePackage, {
	package: 'com.muirey03.cr4shed',
	isCurrent: true,
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

export const Package = generateSchema({
	schema: examplePackage,
	descriptions: {
		package: 'Identifier of the package',
		isCurrent: 'If this is the latest version of the package',
		repository: 'Object representing the repository and it\'s data',
		repositorySlug: 'Unique identifier of the repository',
		price: 'Price of the package',
		tier: 'How trustworthy the package repository is (1 = highest, 5 = lowest)',
		version: 'Version of the package',
		architecture: 'Architecture of the package',
		filename: 'URI path to the package debian file from the repository base',
		size: 'Size of the package in bytes',
		sha256: 'SHA256 hash of the package',
		name: 'Name of the package',
		description: 'Description of the package',
		author: 'Author of the package (may include email/website)',
		maintainer: 'Maintainer of the package (may include email/website)',
		depiction: 'URL to the depiction of the package',
		nativeDepiction: 'URL to the native depiction of the package',
		sileoDepiction: 'URL to the sileo depiction of the package',
		header: 'URL to the header image of the package',
		tintColor: 'Tint color of the package',
		icon: 'URL to the icon image of the package',
		section: 'Section of the package',
		tag: 'Tags of the package',
		installedSize: 'Size of the package in bytes once installed',
		meta: 'Direct URL to the metadata of the package',
		repo: 'Direct URL to the repository metadata of the package'
	},
	nullables: [
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
	]
})
