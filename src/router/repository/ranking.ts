import { type Origin } from '@prisma/client'
import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { prisma } from 'database.js'

type SearchResponse = Response & {
	locals: {
		query: string;
	};
}

export function middleware(request: Request, response: SearchResponse, next: NextFunction) {
	const query = request.query.rank
	if (!query) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Missing query parameter: \'rank\'',
				date: new Date()
			})
	}

	if (query.length !== 1) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Query parameter \'q\' must be a single character of (1,2,3,4,5,*)',
				date: new Date()
			})
	}

	response.locals.query = query.toString()
	next()
}

export async function handler(request: Request, response: SearchResponse) {
	const { query } = response.locals

	const repos = query === '*' ? await prisma.repository.findMany({
		orderBy: {
			tier: 'asc'
		},

		include: {
			origin: true
		}
	}) : await prisma.repository.findMany({
		where: {
			tier: Number(query),
			isPruned: false
		},

		include: {
			origin: true
		},

		orderBy: {
			name: 'asc'
		}
	})

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			count: repos.length,
			data: repos.map(data => {
				const entries = Object.entries(data)
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
						meta: `${$product.api_endpoint}/jailbreak/repository/${data.slug}`,
						packages: `${$product.api_endpoint}/jailbreak/repository/${data.slug}/packages`
					}
				}
			})
		})
}
