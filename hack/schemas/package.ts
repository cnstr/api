/* eslint-disable unicorn/no-null */

import { Package as DBPackage } from '@canister/models'

import { generateSchema } from './generator.js'

const examplePackage = new DBPackage()
Object.assign(examplePackage, {
	package: 'lunotech11.legizmo.grace',
	isCurrent: true,
	repositorySlug: 'chariz',
	price: '$4.99',
	tier: 1,
	version: '2.3.1-2',
	architecture: 'iphoneos-arm',
	filename: 'debs/lunotech11.legizmo.grace_2.3.1-2_iphoneos-arm.deb',
	size: '332562',
	sha256: '9a5b896b096ed04f1b0e0fdfb83f9d3637474e9f1100da3dbc540482433c5ecf',
	name: 'Legizmo Grace ',
	description: 'Enables pairing, connecting and updating to unsupported versions of watchOS',
	author: 'lunotech11 <support@legizmo.app>',
	maintainer: 'lunotech11 <support@legizmo.app>',
	depiction: 'https://chariz.com/buy/legizmo-grace',
	nativeDepiction: null,
	sileoDepiction: 'https://chariz.com/api/sileo/package/lunotech11.legizmo.grace/depiction.json',
	header: 'https://cdn.chariz.cloud/asset/legizmo-grace/UTEf2gTydQJzutqeIHUJWydk36hOoX6M/assets/5MTLnFEJg8MCEiwaGg-a09UOnb6bWbn5g0vP5cZ_oHGRd8HXIOsZWrhB8z-9wumbliuh2FRvojH0CaShA1Zbcw.jpg',
	tintColor: '#2c5364',
	icon: 'https://img.chariz.cloud/icon/legizmo-grace/icon@3x.png',
	section: 'Tweaks',
	tag: [
		'cydia::commercial',
		'compatible_min::ios12.0'
	],
	installedSize: '2240',
	refs: {
		meta: 'https://api.canister.me/v2/jailbreak/package/lunotech11.legizmo.grace',
		repo: 'https://api.canister.me/v2/jailbreak/repository/chariz'
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
