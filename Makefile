# build
build-libs:
	cd crates/libs && cargo build
build-surrealdb:
	cd apps/surreal-db && cargo build
build-websockets:
	cd apps/websockets && cargo build

# build all
build:
		make build-libs && make build-surrealdb && make build-websockets

# run
run-surrealdb:
	cd apps/surreal-db && cargo run
run-websockets:
	cd apps/websockets && cargo run

# run all
run:
	make build && make run-surrealdb && make run-websockets

# deploy
deploy-websockets:
	cd apps/websockets && fly deploy

deploy-surrealdb:
	cd apps/surreal-db && fly deploy

# deploy all
deploy:
	make build && make deploy-surrealdb && make deploy-websockets

