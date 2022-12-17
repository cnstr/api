import 'constants.js'
import 'database.js'

import { hrtime } from 'node:process'

import { type Request, type Response, App } from '@tinyhttp/app'
import { cors } from '@tinyhttp/cors'
import { lruSend } from 'lru-send'
import { json } from 'milliparsec'
import { initializeFullTextSearch } from 'search.js'
import { type LocalsResponse, http } from 'server.js'

await initializeFullTextSearch()
const server = new App<never, Request, LocalsResponse>()

server.use(json())
server.use(lruSend())
server.use(cors({
	origin: '*',
	allowedHeaders: [],
	exposedHeaders: []
}))

// Track X-Response-Time
type TimedResponse = Response & {
	locals: {
		startTime: bigint;
	};
}

server.use((_request, response: TimedResponse, next) => {
	if (response.locals) {
		response.locals.startTime = hrtime.bigint()
	}

	next()
})

server.use((_request, response: TimedResponse, next) => {
	if (response.locals) {
		const start = response.locals.startTime
		const end = hrtime.bigint()
		response.setHeader('X-Response-Time', `${Number(end - start) / 1_000_000}ms`)
	}

	next()
})

// Hacky workaround to mount types on hardcoded generics
server.use('/v2', http as unknown as never)
server.listen(3000, () => {
	console.log('http: started successfully')
})

// This logic is separated out simply for duplicate routes
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
