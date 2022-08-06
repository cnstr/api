import { dump } from 'js-yaml'
import swaggerJSDoc, { OAS3Definition } from 'swagger-jsdoc'

import type { config_manifest } from './types.js'

export function compile_openapi(meta: config_manifest['product'], version: string) {
	const openapi_definition: OAS3Definition = {
		openapi: '3.0.0',
		info: {
			title: meta.production_name,
			description: 'A high-speed search engine created for Jailbreaking.',
			version,
			contact: {
				name: 'Canister Support',
				email: meta.contact_email
			},
			license: {
				name: meta.copyright_notice
			}
		},
		servers: [
			{
				url: meta.api_endpoint,
				description: 'Main API'
			}
		]
	}

	const openapi_spec = swaggerJSDoc({
		definition: openapi_definition,
		apis: ['src/router/**/*.ts']
	})

	console.log('> Generated OpenAPI Specification')
	console.log()

	return {
		json: JSON.stringify(openapi_spec, undefined, 4),
		yaml: dump(openapi_spec)
	}
}
