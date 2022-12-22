import { type Origin, type Repository } from '@prisma/client'
import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { elastic } from 'search.js'

type SearchResponse = Response & {
	locals: {
		page: number;
		limit: number;
		query: string;
	};
}

export function middleware(request: Request, response: SearchResponse, next: NextFunction) {
	const query = request.query.q
	if (!query) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Missing query parameter: \'q\'',
				date: new Date()
			})
	}

	if (query.length < 3) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Query parameter \'q\' must be atleast 3 characters',
				date: new Date()
			})
	}

	const { limit } = request.query
	const { page } = request.query

	response.locals.page = Number.parseInt(page?.toString() ?? '1', 10)
	response.locals.limit = Number.parseInt(limit?.toString() ?? '100', 10)
	response.locals.query = query.toString()

	if (response.locals.page < 1) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Query parameter \'page\' must be greater than 0',
				date: new Date()
			})
	}

	if (response.locals.limit < 1 || response.locals.limit > 250) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Query parameter \'limit\' must be between 1 and 250',
				date: new Date()
			})
	}

	next()
}

export async function handler(request: Request, response: SearchResponse) {
	const { query, limit, page } = response.locals

	const result = await elastic.search<Repository & { origin: Origin }>({
		index: 'repositories',
		query: {
			query_string: {
				fields: ['slug', 'name', 'description', 'aliases'],
				query: `${query}*`
			}
		},

		sort: {
			tier: {
				order: 'asc'
			},
			_score: {
				order: 'desc'
			}
		},

		from: (page - 1) * limit,
		size: limit
	})

	const url = new URL(request.originalUrl, $product.api_endpoint)
	url.searchParams.set('page', (page + 1).toString())
	const nextPage = url.href

	url.searchParams.set('page', (page - 1).toString())
	const previousPage = url.href

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			refs: {
				// eslint-disable-next-line unicorn/no-null
				nextPage: result.hits.hits.length === limit ? nextPage : null,
				// eslint-disable-next-line unicorn/no-null
				previousPage: page > 1 ? previousPage : null
			},
			count: result.hits.hits.length,
			data: result.hits.hits.map(data => {
				if (!data._source) {
					throw new Error('Missing source')
				}

				const entries = Object.entries(data._source)
					.map(([key, value]) => {
						if (key === 'origin') {
							const filtered = Object.fromEntries(Object.entries(value as Origin)
								.filter(([key]) => key !== 'uuid'))

							return [key, filtered]
						}

						return [key, value]
					})
					.filter(([key]) => key !== 'originId' && key !== 'isPruned')

				// eslint-disable-next-line @typescript-eslint/no-unsafe-return
				return {
					...Object.fromEntries(entries),
					refs: {
						meta: `${$product.api_endpoint}/jailbreak/repository/${data._source.slug}`,
						packages: `${$product.api_endpoint}/jailbreak/repository/${data._source.slug}/packages`
					}
				}
			})
		})
}
