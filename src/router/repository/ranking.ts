import { Repository } from '@canister/models'
import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { database } from 'database.js'

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

	const repos = query === '*' ? await database.createQueryBuilder(Repository, 'r')
		.orderBy('tier')
		.getMany() : await database.createQueryBuilder(Repository, 'r')
		.select()
		.groupBy('r."slug"')
		.where('r."tier"=:query', {
			query: `${query}`
		})
		.andWhere({
			isPruned: false
		})
		.orderBy('name')
		.getMany()

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			count: repos.length,
			data: repos.map(data => {
				const entries = Object.entries(data)
					.filter(([key]) => key !== 'originId' && key !== 'isPruned')

				return {
					...Object.fromEntries(entries),
					refs: {
						meta: `${$product.api_endpoint}/jailbreak/repository/${data.slug}`,
						packages: `${$product.api_endpoint}/jailbreak/repository/${data.slug}/packages`,
						origin: `${$product.api_endpoint}/jailbreak/repository/${data.originId}/origin`
					}
				}
			})
		})
}
