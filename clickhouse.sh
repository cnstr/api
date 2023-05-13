#!/bin/bash
set -e
clickhouse client -n <<-EOSQL
	CREATE TABLE IF NOT EXISTS canister.download_events (
		package_id String,
		package_version String,
		package_author String,
		package_maintainer String,
		repository_uri String,
		repository_suite String,
		repository_component String,
		client String,
		client_version String,
		jailbreak String,
		jailbreak_version String,
		distribution String,
		distribution_version String,
		client_architecture String,
		client_bitness UInt32,
		device String,
		device_platform String,
		device_version String,
		database_uuid String,
		timestamp String,
		time DateTime
	)
	ENGINE = MergeTree()
	ORDER BY (timestamp)
EOSQL
