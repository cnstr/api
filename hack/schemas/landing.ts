import { generateSchema } from './generator.js'

const exampleLanding = {
	info: {
		name: 'Canister (cnstr)',
		version: '1.9.68',
		build: '2021.10.6_2hf0e3c',
		platform: 'linux-v16.1.0_k8s-v1.21.4'
	},

	references: {
		docs: 'https://docs.canister.me',
		privacy_policy: 'https://canister.me/privacy',
		contact_email: 'support@canister.me',
		copyright: 'Copyright (c) 2021, Aerum LLC'
	},

	connection: {
		current_date: '2021-10-06T20:04:34.038Z',
		current_epoch: '1633565074',
		user_agent: 'Tale-ConfigBinTray/8.6.0 (Linux; U; Build/RP1A.200720.012)',
		http_version: '1.1'
	}
}

export const Landing = generateSchema({
	schema: exampleLanding,
	descriptions: {
		name: 'Name of the API',
		version: 'Version of the API',
		build: 'Build of the API',
		platform: 'Platform of the API',
		docs: 'URL to the documentation of the API',
		privacy_policy: 'URL to the privacy policy of the API',
		contact_email: 'Email to contact the API webmaster',
		copyright: 'Copyright Notice of the API',
		current_date: 'Current date of your connection',
		current_epoch: 'Current epoch of your connection',
		user_agent: 'User agent of your connection',
		http_version: 'HTTP version of your connection'
	},
	nullables: []
})
