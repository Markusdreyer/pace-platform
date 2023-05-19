run-surrealdb:
	cd crates/surrealdb && cargo run
run-websockets:
	cd apps/websockets && cargo run
deploy-websockets:
	cd apps/websockets && flyctl deploy --config apps/websockets/fly.toml --app websockets
