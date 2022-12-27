import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { prisma } from 'database.js'

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
	const pkgs = await prisma.package.findMany({
		where: {
			// eslint-disable-next-line @typescript-eslint/naming-convention
			OR: query.map(packageId => ({ package: packageId })),
			isCurrent: true,
			isPruned: false
		},

		include: {
			repository: true
		},

		orderBy: {
			repositoryTier: 'asc'
		}
	})

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
					.filter(([key]) => key !== 'uuid' && key !== 'isPruned' && key !== 'isCurrent' && key !== 'repositorySlug')

				return {
					...Object.fromEntries(entries),
					refs: {
						repo: `${$product.api_endpoint}/jailbreak/repository/${data.repositorySlug}`
					}
				}
			})
		})
}
