build-libs:
	cd crates/libs && cargo build
build-surrealdb:
	cd crates/surreal-db && cargo build
build-websockets:
	cd apps/websockets && cargo build

run-surrealdb:
	cd crates/surreal-db && cargo run
run-websockets:
	cd apps/websockets && cargo run

deploy-websockets:
	cd apps/websockets && flyctl deploy --config apps/websockets/fly.toml --app websockets
