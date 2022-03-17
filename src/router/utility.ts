import { App } from '@tinyhttp/app'
import { platform } from 'node:os'
import { uptime, version } from 'node:process'

export function load(http: App) {
	http.get('/', (_req, res) => {
		const servers = [
			{
				name: "alyssa",
				location: "asia-southeast1",
				locationName: "Singapore"
			},
			{
				name: "everette",
				location: "us-east4",
				locationName: "N. Virginia"
			},
			{
				name: "ashy",
				location: "us-west2",
				locationName: "Los Angeles"
			},
			{
				name: "ivy",
				location: "europe-west6",
				locationName: "Zurich"
			},
			{
				name: "weasel",
				location: "asia-northeast1",
				locationName: "Tokyo"
			},
		]

		const random = Math.floor(Math.random() * servers.length)
		const date = new Date()

		return res.status(200).json({
			info: {
				name: 'Canister',
				version: `${RELEASE_VERSION}`,
				build: `${FILE_MODIFIED}_${GIT_COMMIT.substring(0, 7)}`,
				platform: `${platform}-${version}_k8s-v1.22.4`
			},

			references: {
				docs: `${API_ENDPOINT}/docs`,
				privacy_policy: 'https://canister.me/privacy-policy',
				contact_email: 'support@canister.me',
				copyright: 'Copyright (c) 2022 Aerum LLC.'
			},

			connection: {
				node_name: servers[random].name,
				node_location: `${servers[random].locationName} (${servers[random].location})`,
				current_date: `${date.toISOString()} (${date.getTime()})`
			}
		})
	})

	http.get('/healthz', (_req, res) => {
		return res.status(200).json({
			health: 'OK',
			metrics: {
				version: version,
				uptime: uptime()
			}
		})
	})
}
