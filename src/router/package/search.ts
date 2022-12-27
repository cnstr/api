import { type Package } from '@prisma/client'
import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { prisma } from 'database.js'
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

	const result = await elastic.search<Package>({
		index: 'packages',
		query: {
			query_string: {
				fields: ['name', 'description', 'author', 'maintainer', 'section'],
				fuzziness: 'AUTO',
				fuzzy_transpositions: true,
				fuzzy_max_expansions: 100,
				query: `${query}~`
			}
		},

		sort: {
			repositoryTier: {
				order: 'asc'
			},
			_score: {
				order: 'desc'
			}
		},

		from: (page - 1) * limit,
		size: limit
	})

	const repositorySlugs = result.hits.hits.map(data => data._source?.repositorySlug ?? '')
	const repositories = await prisma.repository.findMany({
		where: {
			slug: {
				in: repositorySlugs
			}

		}
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
					.filter(([key]) => key !== 'uuid' && key !== 'isPruned')

				return {
					...Object.fromEntries(entries),
					repository: repositories.find(repository => repository.slug === data._source?.repositorySlug),
					refs: {
						meta: `${$product.api_endpoint}/jailbreak/package/${data._source.package}`,
						repo: `${$product.api_endpoint}/jailbreak/repository/${data._source.repositorySlug}`
					}
				}
			})
		})
}
