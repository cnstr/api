import { type Request, type Response, App } from '@tinyhttp/app'

import * as packageLookup from './router/package/lookup.js'
import * as packageMultiLookup from './router/package/multiLookup.js'
import * as packageSearch from './router/package/search.js'
import * as repositoryLookup from './router/repository/lookup.js'
import * as repositoryPackages from './router/repository/packages.js'
import * as repositoryRanking from './router/repository/ranking.js'
import * as repositorySafety from './router/repository/safety.js'
import * as repositorySearch from './router/repository/search.js'

export type LocalsResponse = Response & {
	locals: never;
}

export const http = new App<never, Request, LocalsResponse>({
	noMatchHandler: (_request, response) => response.status(404)
		.json({
			message: '404 Not Found',
			date: new Date()
		}),

	onError: (error, _request, response) => {
		console.log(error)
		return response.status(500)
			.json({
				message: '500 Internal Server Error',
				date: new Date()
			})
	},

	settings: {
		xPoweredBy: 'Argo'
	}
})

http.get('/jailbreak/package/search', packageSearch.middleware, packageSearch.handler)
http.get('/jailbreak/package/multi', packageMultiLookup.middleware, packageMultiLookup.handler)
http.get('/jailbreak/package/:package', packageLookup.middleware, packageLookup.handler)

http.get('/jailbreak/repository/search', repositorySearch.middleware, repositorySearch.handler)
http.get('/jailbreak/repository/ranking', repositoryRanking.middleware, repositoryRanking.handler)
http.get('/jailbreak/repository/safety', repositorySafety.middleware, repositorySafety.handler)
http.get('/jailbreak/repository/:repository', repositoryLookup.middleware, repositoryLookup.handler)
http.get('/jailbreak/repository/:repository/packages', repositoryPackages.middleware, repositoryPackages.handler)

http.get('/', (request, response) => response.status(200)
	.json({
		info: {
			name: `${$product.production_name} (${$product.code_name})`,
			version: $version,
			build: $build,
			platform: $platform
		},

		references: {
			docs: $product.docs_endpoint,
			privacy_policy: `${$product.site_endpoint}/privacy`,
			contact_email: $product.contact_email,
			copyright: $product.copyright_notice
		},

		connection: {
			current_date: new Date(),
			current_epoch: Date.now(),
			user_agent: request.headers['user-agent'],
			http_version: request.httpVersion
		}
	}))

http.get('/openapi.yaml', (_request, response) => {
	response.set('Content-Type', 'application/x-yaml')
	response.send($openapi.yaml)
})

http.get('/openapi.json', (_request, response) => {
	response.set('Content-Type', 'application/json')
	response.send($openapi.json)
})

http.get('/healthz', (_request, response) => response.status(200)
	.json({
		health: 'OK'
	}))
