run-websockets:
	cd backend/websockets && cargo run 
run-benchmarking:
	cd backend/benchmarking && cargo run
run-spectator-client:
	cd spectator-client && npm run dev
deploy-websockets:
	cd backend && flyctl deploy --config websockets/fly.toml --app websockets     