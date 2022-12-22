import { type Origin } from '@prisma/client'
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
		},

		include: {
			origin: true
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

	const entries = Object.entries(repo)
		.map(([key, value]) => {
			if (key === 'origin') {
				const filtered = Object.fromEntries(Object.entries(value as Origin)
					.filter(([key]) => key !== 'uuid'))

				return [key, filtered]
			}

			return [key, value]
		})
		.filter(([key]) => key !== 'originId' && key !== 'isPruned')

	return response.status(200)
		.json({
			message: '200 Successful',
			date: new Date(),
			// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
			data: {
				...Object.fromEntries(entries),
				refs: {
					packages: `${$product.api_endpoint}/jailbreak/repository/${repo.slug}/packages`
				}
			}
		})
}
