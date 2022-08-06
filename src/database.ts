import { Cache, Manifest, Origin, Package, Repository } from '@canister/models'
import { DataSource, DataSourceOptions } from 'typeorm'

const config = Object.assign($database, {
	type: 'postgres',
	synchronize: false,
	logging: false,
	entities: [Package, Repository, Origin, Cache, Manifest]
}) as DataSourceOptions

export const database = new DataSource(config)
await database.initialize()
console.log('db: connected successfully')
