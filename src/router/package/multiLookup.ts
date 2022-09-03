import { Package } from '@canister/models'
import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { database } from 'database.js'
import { Brackets } from 'typeorm'

type LookupResponse = Response & {
	locals: {
		query: string[];
	};
}

export function middleware(request: Request, response: LookupResponse, next: NextFunction) {
	const query = request.query.ids
	if (!query) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Missing query parameter: \':packages\'',
				date: new Date()
			})
	}

	response.locals.query = query.toString()
		.split(',')
	next()
}

export async function handler(_request: Request, response: LookupResponse) {
	const { query } = response.locals
	const pkgs = await database.createQueryBuilder(Package, 'p')
		.select()
		.leftJoinAndSelect('p.repository', 'repository')
		.where({
			isCurrent: true,
			isPruned: false
		})
		.andWhere(new Brackets(qb => {
			query.map(packageId => qb.orWhere({ package: packageId }))
		}))
		.orderBy('p.tier')
		.getMany()

	if (pkgs.length === 0) {
		return response.status(404)
			.json({
				message: '404 Not Found',
				error: 'Package not found',
				date: new Date()
			})
	}

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			count: pkgs.length,
			data: pkgs.map(data => {
				const entries = Object.entries(data)
					.filter(([key]) => key !== 'databaseId' && key !== 'isPruned' && key !== 'isCurrent' && key !== 'repositorySlug')

				return {
					...Object.fromEntries(entries),
					refs: {
						repo: `${$product.api_endpoint}/jailbreak/repository/${data.repositorySlug}`
					}
				}
			})
		})
}
