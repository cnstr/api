import { Package } from '@canister/models'
import { App } from '@tinyhttp/app'
import { database } from 'database'

export function load(http: App) {
	/**
	 * @openapi
	 * /jailbreak/search/packages:
	 *   get:
	 *     summary: Search for packages
	 *     description: Retrieve an indexed package using a search query
	 *     operationId: searchPackages
	 *     parameters:
	 *       - name: q
	 *         in: query
	 *         description: The search query
	 *         example: mypackage
	 *         required: true
	 *         schema:
	 *           type: string
	 *           format: query
	 *           minLength: 3
	 *       - name: limit
	 *         in: query
	 *         description: Search response limit
	 *         required: false
	 *         schema:
	 *           type: integer
	 *           default: 100
	 *           minimum: 1
	 *           maximum: 250
	 *       - name: page
	 *         in: query
	 *         description: Pagination number (starting from 1)
	 *         required: false
	 *         schema:
	 *           type: integer
	 *           default: 1
	 *           minimum: 1
	 *     responses:
	 *       '200':
	 *         description: 'OK'
	 *         content:
	 *           application/json:
	 *             schema:
	 *               type: object
	 *               properties:
	 *                 message:
	 *                   type: string
	 *                   enum:
	 *                     - 200 Successful
	 *                 date:
	 *                   type: string
	 *                   format: date-time
	 *                 refs:
	 *                   type: object
	 *                   properties:
	 *                     nextPage:
	 *                       type: string
	 *                       format: uri
	 *                     previousPage:
	 *                       type: string
	 *                       format: uri
	 *                 count:
	 *                   type: integer
	 *                   minimum: 0
	 *                 # TODO: Data
	 */
	http.get('/jailbreak/search/packages', (req, res, next) => {
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

		if (res.locals!.page < 1) {
			return res.status(400).json({
				message: '400 Bad Request',
				error: 'Query parameter \'page\' must be greater than 0',
				date: new Date()
			})
		}

		if (res.locals!.limit < 1 || res.locals!.limit > 250) {
			return res.status(400).json({
				message: '400 Bad Request',
				error: 'Query parameter \'limit\' must be between 1 and 250',
				date: new Date()
			})
		}

		next()
	}, async (req, res) => {
		const { query, limit, page } = res.locals!
		const pkgs: Package[] = await database.createQueryBuilder(Package, 'p')
			.select()
			.groupBy('p."databaseId"')
			.having('vector @@ to_tsquery(\'simple\', string_agg(:query, \' | \'))', {
				query: `${query}:*` // TODO: Wtf is going on here
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
				nextPage: pkgs.length === limit ? nextPage : null,
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
