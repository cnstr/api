import { Package } from '@canister/models'
import { App } from '@tinyhttp/app'
import { addToDocs } from '@tinyhttp/swagger'
import { database } from 'database'

export function load(http: App) {
	http.get('/jailbreak/search/packages', addToDocs({
		query: {
			'q': {
				type: 'string',
				optional: false
			},
			'limit': {
				type: 'number',
				optional: true
			},
			'page': {
				type: 'number',
				optional: true
			}
		}
	}), (req, res, next) => {
		const query = req.query.q
		if (!query) {
			return res.status(400).json({
				message: '400 Bad Request',
				error: 'Missing query parameter: \'q\'',
				date: new Date()
			})
		}

		if (query.length < 3) {
			return res.status(400).json({
				message: '400 Bad Request',
				error: 'Query parameter \'q\' must be atleast 3 characters',
				date: new Date()
			})
		}

		const limit = req.query.limit
		const page = req.query.page

		res.locals!.page = parseInt(page?.toString() ?? '1')
		res.locals!.limit = parseInt(limit?.toString() ?? '100')
		res.locals!.query = query
		next()
	}, async (req, res) => {
		const { query, limit, page } = res.locals!
		const pkgs: Package[] = await database.createQueryBuilder(Package, 'p')
			.select()
			.groupBy('p."databaseId"')
			.having('vector @@ to_tsquery(\'simple\', string_agg(:query, \' | \'))', {
				query: `${query.slice(0, -1)}:*` // TODO: Wtf is going on here
			})
			.andWhere({ isCurrent: true, isPruned: false })
			.loadAllRelationIds()
			.orderBy('tier')
			.take(limit)
			.skip((page - 1) * limit)
			.getMany()

		const nextPage = __apiEndpoint + req.originalUrl.replace(`page=${page}`, `page=${page + 1}`)
		const previousPage = __apiEndpoint + req.originalUrl.replace(`page=${page}`, `page=${page - 1}`)

		return res.status(200).json({
			message: '200 Successful',
			date: new Date(),
			refs: {
				nextPage: nextPage,
				previousPage: page > 1 ? previousPage : null
			},
			count: pkgs.length,
			data: pkgs.map(pkg => {
				return {
					...pkg,
					refs: {
						meta: __apiEndpoint + '/jailbreak/get/package' + `?q=${pkg.package}`,
						repo: __apiEndpoint + '/jailbreak/get/repository' + `?q=${pkg.repository}`
					}
				}
			})
		})
	})
}
