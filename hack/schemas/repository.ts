/* eslint-disable unicorn/no-null */
import { type Origin as PrismaOrigin, type Repository as PrismaRepository } from '@prisma/client'

import { generateSchema } from './generator.js'

type ResponseRepository = Omit<Omit<PrismaRepository, 'isPruned'>, 'originId'> & {
	origin: Omit<PrismaOrigin, 'uuid'>;
	refs: {
		meta: string;
		packages: string;
	};
}

const exampleRepository: ResponseRepository = {
	slug: 'chariz',
	aliases: ['hashbang'],
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
	date: new Date('2022-08-15T17:43:11.000Z'),
	paymentGateway: null,
	sileoEndpoint: 'https://chariz.com/api/sileo/',
	origin: {
		hostname: 'repo.chariz.com',
		releasePath: '/Release',
		packagesPath: '/Packages.zst',
		lastUpdated: new Date('2022-12-22T02:16:46.645Z'),
		hasInRelease: false,
		hasReleaseGpg: true,
		supportsPaymentV1: true,
		supportsPaymentV2: false,
		usesHttps: true
	},
	refs: {
		meta: 'https://api.canister.me/v2/jailbreak/repository/chariz',
		packages: 'https://api.canister.me/v2/jailbreak/repository/chariz/packages'
	}
}

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
		hostname: 'Hostname of the repository',
		releasePath: 'Path to the repository Release file',
		packagesPath: 'Path to the repository Packages file',
		lastUpdated: 'Date the repository was last updated',
		hasInRelease: 'Whether the repository has an InRelease file',
		hasReleaseGpg: 'Whether the repository has a Release.gpg file',
		supportsPaymentV1: 'Whether the repository supports payment v1',
		supportsPaymentV2: 'Whether the repository supports payment v2',
		usesHttps: 'Whether the repository uses HTTPS',
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
