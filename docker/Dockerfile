FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /neobabu

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /neobabu/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin neobabu-bot

FROM debian:bookworm-slim AS runtime
WORKDIR /neobabu
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /neobabu/target/release/neobabu-bot /usr/local/bin/bot
COPY --from=builder /neobabu/docker/bot-config.toml /neobabu/bot-config.toml
CMD ["/usr/local/bin/bot"]