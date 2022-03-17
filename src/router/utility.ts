import { App } from '@tinyhttp/app'

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
				privacy_policy: `${__siteEndpoint}/privacy-policy`,
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

	http.get('/healthz', (_req, res) => {
		return res.status(200).json({
			health: 'OK'
		})
	})
}
