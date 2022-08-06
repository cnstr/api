import { App, Request, Response } from '@tinyhttp/app'

export type LocalsResponse = Response & {
	locals: never;
}

export const http = new App<never, Request, LocalsResponse>({
	noMatchHandler: (_request, response) => response.status(404)
		.json({
			message: '404 Not Found',
			date: new Date()
		}),

	onError: (error, _request, response) => {
		// TODO: Sentry capture
		console.log(error)
		return response.status(500)
			.json({
				message: '500 Internal Server Error',
				date: new Date()
			})
	},

	settings: {
		xPoweredBy: 'Argo'
	}
})
