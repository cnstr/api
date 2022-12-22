export type Manifest = {
	product: {
		code_name: string;
		production_name: string;
		contact_email: string;
		copyright_notice: string;
		api_endpoint: string;
		site_endpoint: string;
		docs_endpoint: string;
	};

	database: {
		host: string;
		username: string;
		password: string;
		database: string;
	};

	search: {
		host: string;
	};

	bump: {
		documentation_id: string;
		access_token: string;
	};
}

