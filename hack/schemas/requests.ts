import { Schema } from 'swagger-jsdoc'

export const BadRequest: Schema = {
	type: 'object',
	properties: {
		message: {
			type: 'string',
			example: 'HTTP Error Type'
		},
		error: {
			type: 'string',
			example: 'Error Message related to request'
		},
		date: {
			type: 'string',
			format: 'date-time'
		}
	}
}
