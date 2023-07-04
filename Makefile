run-websockets:
	cd backend/websockets && cargo run 
run-benchmarking:
	cd backend/benchmarking && cargo run
run-spectator-client:
	cd spectator-client && npm run dev
deploy-websockets:
	cd backend && flyctl deploy --config websockets/fly.toml --app websockets     
build-prometheus:
	cd backend/observability/prometheus && docker build -t prometheus .
run-prometheus:
	docker run -dp 9090:9090 prometheus
build-grafana:
	cd backend/observability/grafana && docker build -t grafana .
run-grafana:
	docker run -dp 3001:3000 grafana
build-observability: build-prometheus build-grafana
run-observability:
	cd backend/observability && docker-compose up -d