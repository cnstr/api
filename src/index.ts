import 'constants.js'
import 'database.js'

import { App, Request, Response } from '@tinyhttp/app'
import { cors } from '@tinyhttp/cors'
import { lruSend } from 'lru-send'
import { json } from 'milliparsec'
import { hrtime } from 'node:process'
import { load } from 'router.js'
import { http, LocalsResponse } from 'server.js'

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
		startTime: string;
	};
}

server.use((_request, response: TimedResponse, next) => {
	if (response.locals) {
		response.locals.startTime = JSON.stringify(hrtime())
	}

	next()
})

load()

server.use((_request, response: TimedResponse, next) => {
	if (response.locals) {
		const start = JSON.parse(response.locals.startTime) as [number, number]
		const delta = hrtime(start)
		response.setHeader('X-Response-Time', `${(delta[0] * 1e3) + (delta[1] / 1e-6)}ms`)
	}

	next()
})

// @ts-expect-error TODO: Fix
server.use('/v2', http)
server.listen(3000, () => {
	console.log('http: started successfully')
})

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
