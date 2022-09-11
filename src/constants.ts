declare global {
	const $openapi: {
		json: string;
		yaml: string;
	}

	const $repos: string[]

	const $commit: string
	const $version: string
	const $build: string
	const $platform: string

	const $product: {
		code_name: string;
		production_name: string;
		contact_email: string;
		copyright_notice: string;
		api_endpoint: string;
		site_endpoint: string;
		docs_endpoint: string;
	}

	const $database: {
		host: string;
		username: string;
		password: string;
		database: string;
	}
}
