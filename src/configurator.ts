import { Package, Repository } from '@canister/models'
import { DataSource, DataSourceOptions } from 'typeorm'

declare global {
	const __commit: string
	const __version: string
	const __build: string
	const __platform: string

	const __name: string
	const __apiEndpoint: string
	const __siteEndpoint: string
	const __contactEmail: string
	const __copyrightNotice: string

	const __database: DataSourceOptions
	const __servers: {
		name: string
		region: string
		location: string
	}[]
}

Object.assign(__database, {
	type: 'postgres',
	synchronize: false,
	logging: false,
	migrationsTableName: 'migrations',
	entities: [Package, Repository]
})

const db = new DataSource(__database)
await db.initialize()
await db.runMigrations({ transaction: 'all' })
console.log('db: connected successfully')
