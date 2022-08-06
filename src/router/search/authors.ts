import { Package } from '@canister/models'
import { App } from '@tinyhttp/app'
import { database } from 'database.js'

export function load(http: App) {
	http.get('/jailbreak/search/authors', (request, response, next) => {
		const query = request.query.q
		if (!query) {
			return response.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Missing query parameter: \'q\'',
					date: new Date()
				})
		}

		response.locals!.query = query
		next()
	}, async (_request, response) => {
		const pkgs = await database.createQueryBuilder(Package, 'p')
			.select()
			.where('p.author ILIKE :query', { query: `%${response.locals!.query}%` })
			.orWhere('p.maintainer ILIKE :query')
			.andWhere({ isCurrent: true })
			.orderBy('tier')
			.getMany()

		return response.status(200)
			.json(pkgs)
	})
}
