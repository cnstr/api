/* eslint-disable unicorn/no-null */

import { Repository as DBRepository } from '@canister/models'

import { generateSchema } from './generator.js'

const exampleRepository = new DBRepository()
Object.assign(exampleRepository, {
	slug: 'chariz',
	aliases: [],
	tier: 1,
	packageCount: 484,
	sections: [
		'System',
		'Tweaks',
		'Development',
		'Utilities',
		'Terminal Support',
		'Themes',
		'Applications'
	],
	uri: 'https://repo.chariz.com',
	suite: './',
	component: null,
	name: 'Chariz',
	version: '0.9',
	description: 'Check out what’s new and download purchases from the Chariz marketplace!',
	date: '2022-08-15T17:43:11.000Z',
	paymentGateway: null,
	sileoEndpoint: 'https://chariz.com/api/sileo/',
	refs: {
		meta: 'https://api.canister.me/v2/jailbreak/repository/chariz',
		packages: 'https://api.canister.me/v2/jailbreak/repository/chariz/packages'
	}
})

export const Repository = generateSchema({
	schema: exampleRepository,
	descriptions: {
		slug: 'Unique identifier of the repository',
		aliases: 'Alternative list of slugs',
		tier: 'How trustworthy the repository is (1 = highest, 5 = lowest)',
		packageCount: 'Number of packages advertised by the repository',
		sections: 'Available sections of packages',
		uri: 'URI to the repository',
		suite: 'Name of the repository suite',
		component: 'Name of the repository component',
		name: 'Name of the repository',
		version: 'Version of the repository',
		description: 'Description of the repository',
		date: 'Date the repository was last updated/created',
		paymentGateway: 'Payment gateway API URL used by the repository',
		sileoEndpoint: 'Sileo API URL used by the repository',
		meta: 'URL to the repository metadata',
		packages: 'URL to the repository packages list'
	},
	nullables: [
		'component',
		'name',
		'version',
		'description',
		'date',
		'paymentGateway',
		'sileoEndpoint'
	]
})
