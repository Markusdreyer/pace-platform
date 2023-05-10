run-websockets:
	cd backend/websockets && cargo run 
deploy-websockets:
	cd backend && flyctl deploy --config websockets/fly.toml --app websockets     