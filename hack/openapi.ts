import { dump } from 'js-yaml'
import swaggerJSDoc, { OAS3Definition } from 'swagger-jsdoc'

import { schemas } from './schemas.js'
import type { Manifest } from './types.js'

export function generateDocumentation(metadata: Manifest['product'], version: string) {
	const definition: OAS3Definition = {
		openapi: '3.0.0',
		info: {
			title: metadata.production_name,
			description: 'A high-speed search engine created for Jailbreaking.',
			version,
			contact: {
				name: 'Canister Support',
				email: metadata.contact_email
			},
			license: {
				name: metadata.copyright_notice
			}
		},
		servers: [
			{
				url: metadata.api_endpoint,
				description: 'Main API'
			}
		],
		tags: [
			{
				name: 'search',
				description: 'Search Operations'
			},
			{
				name: 'lookup',
				description: 'Lookup Operations'
			},
			{
				name: 'endpoint',
				description: 'API Machinery Endpoints'
			}
		],
		components: {
			schemas
		}
	}

	const openapiSpecification = swaggerJSDoc({
		definition,
		apis: ['src/router/**/*.yaml', 'src/server.yaml']
	})

	console.log('> Dumping OpenAPI specification to JSON & YAML')
	console.log('> Using the following schemas:')
	for (const key of Object.keys(schemas)) {
		console.log(`> - ${key}`)
	}

	console.log()
	return {
		json: JSON.stringify(openapiSpecification),
		yaml: dump(openapiSpecification)
	}
}
