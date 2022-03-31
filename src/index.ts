import { App } from '@tinyhttp/app'
import { cors } from '@tinyhttp/cors'
import { serveDocs } from '@tinyhttp/swagger'
import 'configurator'
import 'database'
import { lruSend } from 'lru-send'
import { json } from 'milliparsec'
import { hrtime } from 'node:process'
import { load } from 'router'
import { http } from 'server'

const server = new App()

server.use(json())
server.use(lruSend())
server.use(cors({
	origin: '*',
	allowedHeaders: [],
	exposedHeaders: []
}))

// Track X-Response-Time
server.use((_req, res, next) => {
	if (res.locals) {
		res.locals.startTime = JSON.stringify(hrtime())
	}

	next()
})

load()

server.use((_req, res, next) => {
	if (res.locals) {
		const start = JSON.parse(res.locals.startTime)
		const delta = hrtime(start)
		res.setHeader('X-Response-Time', `${delta[0] * 1000000 + delta[1] / 1000}ms`)
	}

	next()
})

server.use('/v2', http)
server.listen(3000, () => console.log('http: started successfully'))

const routes = new Set<string>()
for (const { method, path } of http.middleware) {
	if (!method || !path) {
		continue
	}

	routes.add(`${method} ${path}`)
}

for (const route of routes) {
	console.log('http: mounting to %s', route)
}

serveDocs(http, {
	title: __name,
	version: __version,
	servers: [__apiEndpoint],
	description: 'A high-speed search engine created for Jailbreaking.',
})
