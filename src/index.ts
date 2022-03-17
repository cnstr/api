import { App } from '@tinyhttp/app'
import { cors } from '@tinyhttp/cors'
import { serveDocs } from '@tinyhttp/swagger'
import { lruSend } from 'lru-send'
import { json } from 'milliparsec'
import { hrtime } from 'node:process'
import { load } from 'router'

declare global {
	const GIT_COMMIT: string
	const RELEASE_VERSION: string
	const FILE_MODIFIED: string
	const API_ENDPOINT: string
}

const server = new App({
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

http.use(json())
http.use(lruSend())
http.use(cors({
	origin: '*',
	allowedHeaders: [],
	exposedHeaders: []
}))

// Track X-Response-Time
http.use((_req, res, next) => {
	if (res.locals) {
		res.locals.startTime = JSON.stringify(hrtime())
	}

	next()
})

load()

http.use((_req, res, next) => {
	if (res.locals) {
		const start = JSON.parse(res.locals.startTime)
		const delta = hrtime(start)
		res.setHeader('X-Response-Time', `${delta[0] * 1000000 + delta[1] / 1000}ms`)
	}

	next()
})

serveDocs(http, {
	title: 'Canister',
	version: RELEASE_VERSION,
	servers: ['api.canister.me/v2'],
	description: 'A high-speed search engine created for Jailbreaking.',
})

server.use('/v2', http)
server.listen(3000, () => console.log('http: started successfully'))
for (const { method, path } of http.middleware) {
	if (!method || !path) {
		continue
	}

	console.log('http: mounting to %s /v2%s', method, path)
}
