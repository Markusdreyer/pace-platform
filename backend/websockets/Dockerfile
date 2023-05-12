FROM rust:1.68 as builder

WORKDIR app

COPY . .
RUN cargo build --release --manifest-path websockets/Cargo.toml

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/websockets /usr/local/bin/websockets
CMD ["websockets"]
EXPOSE 8080