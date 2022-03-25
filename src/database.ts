import { Package, Package1646948441633, Repository, Repository1646948440633 } from '@canister/models'
import { DataSource } from 'typeorm'

Object.assign(__database, {
	type: 'postgres',
	synchronize: false,
	logging: false,
	migrationsTableName: 'migrations',
	entities: [Package, Repository],
	migrations: [Package1646948441633, Repository1646948440633]
})

export const database = new DataSource(__database)
await database.initialize()
console.log('db: connected successfully')
