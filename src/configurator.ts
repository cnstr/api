import { DataSourceOptions } from 'typeorm'

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

	const __swagger: {
		json: string
		yaml: string
	}
}
