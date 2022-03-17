import { Package, Repository } from '@canister/models'
import { DataSource, DataSourceOptions } from 'typeorm'

declare global {
	const GIT_COMMIT: string
	const RELEASE_VERSION: string
	const FILE_MODIFIED: string
	const API_ENDPOINT: string
	const TYPEORM_CREDENTIALS: DataSourceOptions
}

Object.assign(TYPEORM_CREDENTIALS, {
	type: 'postgres',
	synchronize: false,
	logging: false,
	migrationsTableName: 'migrations',
	entities: [Package, Repository]
})

const source = new DataSource(TYPEORM_CREDENTIALS)
await source.initialize()
await source.runMigrations({ transaction: 'all' })
console.log('db: connected successfully')
