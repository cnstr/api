import type { NextFunction, Request, Response } from '@tinyhttp/app'
import { prisma } from 'database.js'

type LookupResponse = Response & {
	locals: {
		query: string;
	};
}

export function middleware(request: Request, response: LookupResponse, next: NextFunction) {
	const query = request.params.repository
	if (!query) {
		return response.status(400)
			.json({
				message: '400 Bad Request',
				error: 'Missing URL parameter: \':repository\'',
				date: new Date()
			})
	}

	response.locals.query = query.toString()
	next()
}

export async function handler(_request: Request, response: LookupResponse) {
	const { query } = response.locals

	const repo = await prisma.repository.findFirst({
		where: {
			slug: query,
			isPruned: false
		}
	})

	const packages = await prisma.package.findMany({
		where: {
			repositorySlug: query,
			isPruned: false
		}
	})

	if (!repo) {
		return response.status(404)
			.json({
				message: '404 Not Found',
				error: 'Repository not found',
				date: new Date()
			})
	}

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			data: packages.map(data => {
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
