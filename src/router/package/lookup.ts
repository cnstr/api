import { Package } from '@canister/models'
import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { database } from 'database.js'

type LookupResponse = Response & {
	locals: {
		query: string;
	};
}

export function middleware(request: Request, response: LookupResponse, next: NextFunction) {
	const query = request.params.package
	if (!query) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Missing URL parameter: \':package\'',
				date: new Date()
			})
	}

	response.locals.query = query.toString()
	next()
}

export async function handler(_request: Request, response: LookupResponse) {
	const { query } = response.locals
	const pkgs: Package[] = await database.createQueryBuilder(Package, 'p')
		.select()
		.groupBy('p."databaseId"')
		.where({
			package: query,
			isPruned: false
		})
		.orderBy('"isCurrent" DESC NULLS LAST,tier')
		.getMany()

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			count: pkgs.length,
			data: pkgs.map(data => {
				const entries = Object.entries(data)
					.filter(([key]) => key !== 'databaseId' && key !== 'isPruned')

				return {
					...Object.fromEntries(entries),
					refs: {
						repo: `${$product.api_endpoint}/jailbreak/repository/${data.repositorySlug}`
					}
				}
			})
		})
}
