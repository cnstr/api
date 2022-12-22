import { type Schema } from 'swagger-jsdoc'

type GeneratorOptions = {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	schema: any;
	descriptions: Record<string, string>;
	nullables: string[];
}

export function generateSchema(options: GeneratorOptions): Schema {
	// Declare function inline to use our options
	function recursiveRecords(record: Record<string, unknown>) {
		const properties = new Map<string, Schema>()

		for (const [key, value] of Object.entries(record)) {
			if (Array.isArray(value)) {
				properties.set(key, {
					type: 'array',
					items: {
						type: 'string'
					}
				})

				continue
			}

			if (value instanceof Object) {
				const recordValue = value as Record<string, unknown>
				properties.set(key, recursiveRecords(recordValue))
				continue
			}

			if (!value) {
				continue
			}

			properties.set(key, {
				type: typeof value,
				example: value,
				nullable: options.nullables.includes(key),
				description: options.descriptions[key]
			})
		}

		return {
			type: 'object',
			properties: Object.fromEntries(properties)
		}
	}

	const actualSchema = options.schema as Record<string, unknown>
	return recursiveRecords(actualSchema)
}
