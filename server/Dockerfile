FROM lukemathwalker/cargo-chef:latest-rust-1.66 AS chef


FROM chef as planner

COPY . /app/.

WORKDIR /app

RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder

COPY --from=planner /app/recipe.json /app/recipe.json

WORKDIR /app

RUN cargo chef cook --release --recipe-path recipe.json

COPY . /app/.

RUN cargo build --release


FROM debian:stable-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/onion-or-not-the-onion-drinking-game-2-server .

ENTRYPOINT ["/app/onion-or-not-the-onion-drinking-game-2-server"]
