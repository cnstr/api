import { App } from '@tinyhttp/app'
import { serveDocs } from '@tinyhttp/swagger'

export const http = new App({
	noMatchHandler: (_req, res) => {
		return res.status(404).json({
			message: '404 Not Found',
			date: new Date()
		})
	},

	onError: async (err, _req, res) => {
		// TODO: Sentry capture
		console.log(err)
		return res.status(500).json({
			message: '500 Internal Server Error',
			date: new Date()
		})
	},

	settings: {
		xPoweredBy: 'Argo'
	}
})

http.use((_req, res, next) => {
	if (!res.locals) {
		res.locals = {}
	}

	next()
})

serveDocs(http, {
	title: __name,
	version: __version,
	servers: [__apiEndpoint],
	description: 'A high-speed search engine created for Jailbreaking.',
})
