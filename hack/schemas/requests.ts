import { type Schema } from 'swagger-jsdoc'

export const BadRequest: Schema = {
	type: 'object',
	properties: {
		message: {
			type: 'string',
			enum: [
				'400 Bad Request'
			]
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

export const NotFoundRequest: Schema = {
	type: 'object',
	properties: {
		message: {
			type: 'string',
			enum: [
				'404 Not Found'
			]
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
