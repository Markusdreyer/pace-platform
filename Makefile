run-websockets:
	cd pace-backend/websockets && cargo run 
deploy-websockets:
	cd pace-backend && flyctl deploy --config websockets/fly.toml --app websockets     