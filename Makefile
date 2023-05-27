# build and run everything
start:
	zellij --layout ./.config/zellij_layout.kdl

# build and run everything
start-backend:
	zellij --layout ./.config/zellij_layout_backend.kdl

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

# build all for backend
build-all-backend:
		make build-libs && make build-backend

# build all for websockets
build-all-websockets:
		make build-libs && make build-websockets

# run
run-backend:
	cd apps/backend && cargo run
run-websockets:
	cd apps/websockets && cargo run

# run all
run:
	make build && make run-backend && make run-websockets

# run all for backend
run-all-backend:
	make build && make run-backend

# run all for websockets
run-all-websockets:
	make build && make run-websockets

# deploy
deploy-websockets:
	cd apps/websockets && fly deploy

deploy-backend:
	cd apps/backend && fly deploy

# deploy all
deploy:
	make build && make deploy-backend && make deploy-websockets
