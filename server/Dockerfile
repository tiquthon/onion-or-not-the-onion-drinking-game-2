FROM lukemathwalker/cargo-chef:latest-rust-1.66 AS chef


FROM chef as planner

COPY ./shared-library/. /app/shared-library/.
COPY ./server/. /app/server/.

WORKDIR /app/server

RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder

COPY --from=planner /app/server/recipe.json /app/server/recipe.json

WORKDIR /app/server

COPY ./shared-library/. /app/shared-library/.

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./server/. /app/server/.

RUN cargo build --release


FROM debian:stable-slim AS runtime

WORKDIR /app

COPY ./server/configuration/. /app/configuration/.
COPY --from=builder /app/server/target/release/onion-or-not-the-onion-drinking-game-2-server .

ENTRYPOINT ["/app/onion-or-not-the-onion-drinking-game-2-server"]
