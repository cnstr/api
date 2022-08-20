import { Package } from '@canister/models'
import { App, Request, Response } from '@tinyhttp/app'
import { database } from 'database.js'
import { LocalsResponse } from 'server.js'

export function load(http: App<never, Request, LocalsResponse>) {
	/**
	 * @openapi
	 * /jailbreak/package/search:
	 *   get:
	 *     summary: Search for packages
	 *     description: Retrieve an indexed package using a search query
	 *     operationId: searchPackage
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
	 *                 data:
	 *                   type: array
	 *                   items:
	 *                     $ref: '#/components/schemas/Package'
	 *       '400':
	 *         description: 'Bad Request'
	 *         content:
	 *           application/json:
	 *             schema:
	 *               $ref: '#/components/schemas/BadRequest'
	 */
	type SearchResponse = Response & {
		locals: {
			page: number;
			limit: number;
			query: string;
		};
	}

	http.get('/jailbreak/package/search', (request, response: SearchResponse, next) => {
		const query = request.query.q
		if (!query) {
			return response.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Missing query parameter: \'q\'',
					date: new Date()
				})
		}

		if (query.length < 3) {
			return response.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Query parameter \'q\' must be atleast 3 characters',
					date: new Date()
				})
		}

		const { limit } = request.query
		const { page } = request.query

		response.locals.page = Number.parseInt(page?.toString() ?? '1', 10)
		response.locals.limit = Number.parseInt(limit?.toString() ?? '100', 10)
		response.locals.query = query.toString()

		if (response.locals.page < 1) {
			return response.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Query parameter \'page\' must be greater than 0',
					date: new Date()
				})
		}

		if (response.locals.limit < 1 || response.locals.limit > 250) {
			return response.status(400)
				.json({
					message: '400 Bad Request',
					error: 'Query parameter \'limit\' must be between 1 and 250',
					date: new Date()
				})
		}

		next()
	}, async (request, response: SearchResponse) => {
		const { query, limit, page } = response.locals
		const pkgs: Package[] = await database.createQueryBuilder(Package, 'p')
			.select()
			.groupBy('p."databaseId"')
			.having('p."vector" @@ to_tsquery(\'simple\', string_agg(:query, \' | \'))', {
				query: `${query}:*`
			})
			.andWhere({
				isCurrent: true,
				isPruned: false
			})
			.orderBy('tier')
			.take(limit)
			.skip((page - 1) * limit)
			.getMany()

		const url = new URL(request.originalUrl, $product.api_endpoint)
		url.searchParams.set('page', (page + 1).toString())
		const nextPage = url.href

		url.searchParams.set('page', (page - 1).toString())
		const previousPage = url.href

		return response.status(200)
			.json({
				message: '200 Successful',
				date: new Date(),
				refs: {
					// eslint-disable-next-line unicorn/no-null
					nextPage: pkgs.length === limit ? nextPage : null,
					// eslint-disable-next-line unicorn/no-null
					previousPage: page > 1 ? previousPage : null
				},
				count: pkgs.length,
				data: pkgs.map(data => {
					const entries = Object.entries(data)
						.filter(([key]) => key !== 'databaseId' && key !== 'isPruned')

					return {
						...Object.fromEntries(entries),
						refs: {
							meta: `${$product.api_endpoint}/jailbreak/package/${data.package}`,
							repo: `${$product.api_endpoint}/jailbreak/repository/${data.repositorySlug}`
						}
					}
				})
			})
	})

	/**
	 * @openapi
	 * /jailbreak/package/{packageId}:
	 *   get:
	 *     summary: Lookup a package by ID
	 *     description: Retrieve all versions of a package by package ID
	 *     operationId: lookupPackage
	 *     parameters:
	 *       - name: packageId
	 *         in: path
	 *         description: The packageId to lookup
	 *         example: com.mycompany.mypackage
	 *         required: true
	 *         schema:
	 *           type: string
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
	 *                       type: string`
	 *                       format: uri
	 *                     previousPage:
	 *                       type: string
	 *                       format: uri
	 *                 count:
	 *                   type: integer
	 *                   minimum: 0
	 *                 data:
	 *                   type: array
	 *                   items:
	 *                     $ref: '#/components/schemas/Package'
	 */
	type LookupResponse = Response & {
		locals: {
			query: string;
		};
	}

	http.get('/jailbreak/package/:package', (request, response: LookupResponse, next) => {
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
	}, async (_request, response: LookupResponse) => {
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
							repo: `${$product.api_endpoint}/jailbreak/get/repository?q=${data.repositorySlug}`
						}
					}
				})
			})
	})
}
