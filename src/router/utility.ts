import { App } from '@tinyhttp/app'
import { generateDocs } from '@tinyhttp/swagger'
import { dump } from 'js-yaml'

export function load(http: App) {
	http.get('/', (_req, res) => {
		const random = Math.floor(Math.random() * __servers.length)
		const date = new Date()

		return res.status(200).json({
			info: {
				name: __name,
				version: __version,
				build: __build,
				platform: __platform
			},

			references: {
				docs: `${__apiEndpoint}/docs`,
				privacy_policy: `${__siteEndpoint}/privacy`,
				contact_email: __contactEmail,
				copyright: __copyrightNotice
			},

			connection: {
				node_name: __servers[random].name,
				node_location: `${__servers[random].location} (${__servers[random].region})`,
				current_date: `${date.toISOString()} (${date.getTime()})`
			}
		})
	})

	http.get('/openapi.yaml', (_req, res) => {
		const docs = generateDocs(http, {
			title: __name,
			version: __version,
			servers: [__apiEndpoint],
			description: 'A high-speed search engine created for Jailbreaking.',
		})

		const yaml = dump(docs)
		res.send(yaml)
	})

	http.get('/healthz', (_req, res) => {
		return res.status(200).json({
			health: 'OK'
		})
	})
}
