FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin rust-api

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends libssl3 ca-certificates \
 && rm -rf /var/lib/apt/lists/* 

WORKDIR /app

ENV TIMESTAMP_PATH='/usr/src/app/files/timestamp.txt'
ENV IMAGE_PATH='/usr/src/app/files/pic.jpeg'

COPY --from=builder /app/target/release/rust-api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rust-api"]