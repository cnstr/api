import { Package } from '@canister/models'
import { App } from '@tinyhttp/app'
import { database } from 'database'

export function load(http: App) {
	http.get('/jailbreak/search/authors', (req, res, next) => {
		const query = req.query.q
		if (!query) {
			return res.status(400).json({
				message: '400 Bad Request',
				error: 'Missing query parameter: \'q\'',
				date: new Date()
			})
		}

		res.locals!.query = query
		next()
	}, async (req, res) => {
		const pkgs = await database.createQueryBuilder(Package, 'p')
			.select()
			.where('p.author ILIKE :query', { query: `%${res.locals!.query}%` })
			.orWhere('p.maintainer ILIKE :query')
			.andWhere({ isCurrent: true })
			.orderBy('tier')
			.getMany()

		return res.status(200).json(pkgs)
	})
}
