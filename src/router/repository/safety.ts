import type { NextFunction, Request, Response } from '@tinyhttp/app'

type SearchResponse = Response & {
	locals: {
		query: string;
		multi: boolean;
	};
}

export function middleware(request: Request, response: SearchResponse, next: NextFunction) {
	const { uri, uris } = request.query
	if (uri && uris) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Cannot use both \'uri\' and \'uris\' query parameters',
				date: new Date()
			})
	}

	if (uri) {
		const query = uri
		if (!query) {
			return response.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Missing query parameter: \'uri\'',
					date: new Date()
				})
		}

		response.locals.query = query.toString()
		response.locals.multi = false
		next()
		return
	}

	if (uris) {
		const query = uris
		if (!query) {
			return response.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Missing query parameter: \'uris\'',
					date: new Date()
				})
		}

		response.locals.query = query.toString()
		response.locals.multi = true
		next()
		return
	}

	return response.status(400)
		.json({
			message: '400 Bad Request',
			error: 'Missing query parameter: \'uri\' or \'uris\'',
			date: new Date()
		})
}

export function handler(request: Request, response: SearchResponse) {
	const { query, multi } = response.locals

	if (multi) {
		const repos = query.toLowerCase()
			.trim()
			.normalize()
			.split(',')

		return response.status(200)
			.json({
				message: '200 Successful',
				date: new Date(),
				count: repos.length,
				data: repos.map(repo => {
					let safe = true

					for (const value of $repos) {
						if (repo.includes(value)) {
							safe = false
						}
					}

					return {
						uri: repo,
						safe
					}
				})
			})
	}

	let safe = true
	const repo = query.toLowerCase()
		.trim()
		.normalize()

	for (const value of $repos) {
		if (repo.includes(value)) {
			safe = false
		}
	}

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			data: {
				uri: query,
				safe
			}
		})
}
