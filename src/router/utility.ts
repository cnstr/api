import { App } from '@tinyhttp/app'

export function load(http: App) {
	http.get('/', (request, response) => response.status(200)
		.json({
			info: {
				name: `${$product.production_name} (${$product.code_name})`,
				version: $version,
				build: $build,
				platform: $platform
			},

			references: {
				docs: `${$product.api_endpoint}/docs`,
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
}
