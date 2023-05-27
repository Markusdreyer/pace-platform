# build and run everything
start:
	zellij --layout ./.config/zellij_layout.kdl

# build
build-libs:
	cd crates/libs && cargo build
build-backend:
	cd apps/backend && cargo build
build-websockets:
	cd apps/websockets && cargo build

# build all
build:
		make build-libs && make build-backend && make build-websockets

# run
run-backend:
	cd apps/backend && cargo run
run-websockets:
	cd apps/websockets && cargo run

# run all
run:
	make build && make run-backend && make run-websockets

# deploy
deploy-websockets:
	cd apps/websockets && fly deploy

deploy-backend:
	cd apps/backend && fly deploy

# deploy all
deploy:
	make build && make deploy-backend && make deploy-websockets
