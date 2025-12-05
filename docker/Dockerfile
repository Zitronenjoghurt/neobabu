FROM lukemathwalker/cargo-chef:latest-rust-1.91.1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin neobabu-bot --bin neobabu-web

FROM node:20-slim AS vue-builder
WORKDIR /app/web/dashboard
COPY web/dashboard/package*.json ./
RUN npm ci
COPY web/dashboard/ ./
RUN npm run build

FROM debian:bookworm-slim AS bot
WORKDIR /bot
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/neobabu-bot /usr/local/bin/bot
COPY --from=builder /app/docker/bot-config.toml /bot/bot-config.toml
CMD ["/usr/local/bin/bot"]

FROM debian:bookworm-slim AS web
WORKDIR /web
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/neobabu-web /usr/local/bin/web
COPY --from=vue-builder /app/web/dashboard/dist /web/static
CMD ["/usr/local/bin/web"]