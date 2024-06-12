FROM rust:1-slim-buster AS chef
RUN apt-get update -y && apt-get install -y pkg-config libssl-dev build-essential protobuf-compiler libprotobuf-dev
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:buster-slim as gateway
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 && apt clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/gateway /app/
CMD ["./gateway"]


FROM rust:1-slim-buster AS sqlx-migrator
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres
WORKDIR /app
COPY migrations ./migrations
CMD ["sqlx", "migrate", "run"]