import { Client, errors } from '@elastic/elasticsearch'
import { prisma } from 'database.js'

export const elastic = new Client({
	node: 'http://search:9200'
})

export async function initializeFullTextSearch() {
	try {
		console.log('search: connected to elastic cluster')
		await buildPackagesIndex()
	} catch (error) {
		if (error instanceof errors.ResponseError) {
			console.log('search: threw new response error')
			console.log(error.body)
		}
	}
}

async function buildPackagesIndex() {
	console.log('search: building packages index')
	const packages = await prisma.package.findMany({
		select: {
			uuid: true,
			name: true,
			description: true,
			author: true,
			maintainer: true,
			section: true,
			repositoryTier: true
		},

		where: {
			isCurrent: true,
			isPruned: false
		}
	})

	const exists = await elastic.indices.exists({ index: 'packages' })
	if (!exists) {
		await elastic.indices.create({
			index: 'packages',
			body: {
				mappings: {
					properties: {
						uuid: {
							type: 'keyword'
						},
						name: {
							type: 'text',
							analyzer: 'english'
						},
						description: {
							type: 'text',
							analyzer: 'english'
						},
						author: {
							type: 'text',
							analyzer: 'english'
						},
						maintainer: {
							type: 'text',
							analyzer: 'english'
						},
						section: {
							type: 'text',
							analyzer: 'english'
						},
						repositoryTier: {
							type: 'integer'
						}
					}
				}
			}
		})
	}

	for await (const body of packages) {
		const search = await elastic.search({
			index: 'packages',
			body: {
				query: {
					match: {
						uuid: body.uuid
					}
				}
			}
		})

		// Already exists in the index
		if (search.hits.hits.length > 0) {
			continue
		}

		if (packages.indexOf(body) % 1000 === 0) {
			console.log('search: indexed %d packages', packages.indexOf(body))
		}

		await elastic.index({
			index: 'packages',
			body
		})
	}

	await elastic.indices.refresh({ index: 'packages' })
	console.log('search: built packages index')
}
