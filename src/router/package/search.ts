import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { prisma } from 'database.js'

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
	const pkgs = await prisma.package.findMany({
		where: {
			// eslint-disable-next-line @typescript-eslint/naming-convention
			OR: {
				name: {
					search: query
				},

				author: {
					search: query
				},

				maintainer: {
					search: query
				},

				description: {
					search: query
				},

				section: {
					search: query
				}
			},

			isCurrent: true,
			isPruned: false
		},

		orderBy: {
			repositoryTier: 'asc'
		},

		skip: (page - 1) * limit,
		take: limit
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
				nextPage: pkgs.length === limit ? nextPage : null,
				// eslint-disable-next-line unicorn/no-null
				previousPage: page > 1 ? previousPage : null
			},
			count: pkgs.length,
			data: pkgs.map(data => {
				const entries = Object.entries(data)
					.filter(([key]) => key !== 'uuid' && key !== 'isPruned')

				return {
					...Object.fromEntries(entries),
					refs: {
						meta: `${$product.api_endpoint}/jailbreak/package/${data.package}`,
						repo: `${$product.api_endpoint}/jailbreak/repository/${data.repositorySlug}`
					}
				}
			})
		})
}
