FROM lukemathwalker/cargo-chef:latest-rust-1.66 AS chef


FROM chef as planner

COPY ./shared-library/. /app/shared-library/.
COPY ./client/. /app/client/.
COPY ./server/. /app/server/.

WORKDIR /app/server

RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder

RUN rustup target add wasm32-unknown-unknown

# Need to manually install wasm-bindgen-cli, because on arm64 the auto-download of trunk is x86_64
RUN cargo install --locked wasm-bindgen-cli@0.2.83
RUN cargo install --locked trunk@0.16.0

COPY --from=planner /app/server/recipe.json /app/server/recipe.json

WORKDIR /app/server

COPY ./shared-library/. /app/shared-library/.
COPY ./client/. /app/client/.

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./server/. /app/server/.

ARG BUILD_URL_PATH_PREFIX=/
ENV ONION2_BUILD_CLIENT_PUBLIC_PATH_PREFIX=$BUILD_URL_PATH_PREFIX

ARG RUN_CLIENT_DIST_DIR=/app/dist
ENV ONION2_RUN_CLIENT_DIST_DIR=$RUN_CLIENT_DIST_DIR

RUN cargo build --release


FROM debian:stable-slim AS runtime

WORKDIR /app

COPY ./server/configuration/. /app/configuration/.
COPY --from=builder /app/client/dist/. /app/dist/.
COPY --from=builder /app/server/target/release/onion-or-not-the-onion-drinking-game-2-server .

ENTRYPOINT ["/app/onion-or-not-the-onion-drinking-game-2-server"]
