import { Repository } from '@canister/models'
import { App } from '@tinyhttp/app'
import { database } from 'database.js'

export function load(http: App) {
	http.get('/jailbreak/search/repositories', (request, res, next) => {
		const query = request.query.q
		if (!query) {
			return res.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Missing query parameter: \'q\'',
					date: new Date()
				})
		}

		res.locals!.query = query
		next()
	}, async (request, res) => {
		const repos = await database.createQueryBuilder(Repository, 'p')
			.select()
			.where('vector @@ to_tsquery(\'simple\', :query)', { query: `${res.locals!.query}:*` })
			.loadAllRelationIds()
			.orderBy('tier')
			.getMany()

		return res.status(200)
			.json(repos)
	})
}
