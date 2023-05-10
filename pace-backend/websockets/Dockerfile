FROM rust:1.68 as builder

WORKDIR app

RUN cargo new websockets \
    && cargo new shared --lib
COPY websockets/Cargo.toml websockets/Cargo.toml
COPY shared/Cargo.toml shared/Cargo.toml
COPY Cargo.toml .
COPY Cargo.lock .

# Build application
COPY . .
RUN cargo build --release --manifest-path websockets/Cargo.toml

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/websockets /usr/local/bin/websockets
CMD ["websockets"]
EXPOSE 8080