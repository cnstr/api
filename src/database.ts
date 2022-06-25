import { Cache, Manifest, Origin, Package, Repository } from '@canister/models'
import { DataSource } from 'typeorm'

Object.assign(__database, {
	type: 'postgres',
	synchronize: false,
	logging: false,
	entities: [Package, Repository, Origin, Cache, Manifest],
})

export const database = new DataSource(__database)
await database.initialize()
console.log('db: connected successfully')
