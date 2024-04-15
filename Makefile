dev-server:
	docker-compose -f compose/docker-compose.dev-database.yml up -d
	cargo run
	