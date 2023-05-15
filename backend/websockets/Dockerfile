FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --bin websockets

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin websockets

FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/websockets /usr/local/bin/websockets
ENTRYPOINT ["websockets"]
EXPOSE 8080