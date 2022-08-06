import 'constants.js'
import 'database.js'

import { App } from '@tinyhttp/app'
import { cors } from '@tinyhttp/cors'
import { lruSend } from 'lru-send'
import { json } from 'milliparsec'
import { hrtime } from 'node:process'
import { load } from 'router.js'
import { http } from 'server.js'

const server = new App()

server.use(json())
server.use(lruSend())
server.use(cors({
	origin: '*',
	allowedHeaders: [],
	exposedHeaders: []
}))

// Track X-Response-Time
server.use((_request, res, next) => {
	if (res.locals) {
		res.locals.startTime = JSON.stringify(hrtime())
	}

	next()
})

load()

server.use((_request, res, next) => {
	if (res.locals) {
		const start = JSON.parse(res.locals.startTime)
		const delta = hrtime(start)
		res.setHeader('X-Response-Time', `${delta[0] * 1_000_000 + delta[1] / 1000}ms`)
	}

	next()
})

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
