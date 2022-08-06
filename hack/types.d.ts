export type config_manifest = {
	product: {
		code_name: string;
		production_name: string;
		contact_email: string;
		copyright_notice: string;
		api_endpoint: string;
		site_endpoint: string;
	};

	database: {
		host: string;
		username: string;
		password: string;
		database: string;
	};
}

