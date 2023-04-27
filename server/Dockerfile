FROM lukemathwalker/cargo-chef:latest-rust-1.69 AS chef


FROM chef as planner

COPY ./shared-library/. /app/shared-library/.
COPY ./client/. /app/client/.


FROM planner as planner-client

WORKDIR /app/client
RUN cargo chef prepare --recipe-path recipe.json


FROM planner as planner-server

WORKDIR /app/server
COPY ./server/. /app/server/.
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder

RUN rustup target add wasm32-unknown-unknown

# Need to manually install wasm-bindgen-cli, because on arm64 the auto-download of trunk is x86_64
RUN cargo install --locked wasm-bindgen-cli@~0.2
RUN cargo install --locked trunk@~0.16

COPY ./shared-library/. /app/shared-library/.

WORKDIR /app/client
COPY --from=planner-client /app/client/recipe.json /app/client/recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

# Copying git to let client and server retrieve the latest commit information
COPY ./.git/. /app/.git/.

COPY ./client/. /app/client/.
ARG BUILD_URL_PATH_PREFIX=/
RUN trunk build --release --public-url "${BUILD_URL_PATH_PREFIX}" --features hydration

WORKDIR /app/server
COPY --from=planner-server /app/server/recipe.json /app/server/recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./server/. /app/server/.

ARG RUN_CLIENT_DIST_DIR=/app/dist
ENV ONION2_RUN_CLIENT_DIST_DIR=$RUN_CLIENT_DIST_DIR

RUN cargo build --release


FROM debian:stable-slim AS wasm-optimizer

RUN apt update && apt install -y build-essential curl cmake

WORKDIR /app

RUN curl --output binaryen.tar.gz --location $(curl --silent https://api.github.com/repos/WebAssembly/binaryen/releases/latest | grep --perl-regexp 'https://api.github.com/repos/WebAssembly/binaryen/tarball/.*(?=\")' --only-matching)
RUN tar --extract --verbose --gzip --file=binaryen.tar.gz
RUN mv WebAssembly-binaryen-* WebAssembly-binaryen

WORKDIR /app/WebAssembly-binaryen

RUN cmake -DBUILD_TESTS=OFF . && make

WORKDIR /app

COPY --from=builder /app/client/dist/. /app/client/dist/.

RUN $(find . | grep "wasm-opt$") "$(find client/dist/ | grep "onion-or-not-the-onion-drinking-game-2-client-.*_bg.wasm$")" --output optimized.wasm -Oz

RUN mv optimized.wasm "$(find client/dist/ | grep "onion-or-not-the-onion-drinking-game-2-client-.*_bg.wasm$")"


FROM debian:stable-slim AS runtime

WORKDIR /app

COPY ./server/configuration/. /app/configuration/.
COPY --from=wasm-optimizer /app/client/dist/. /app/dist/.
COPY --from=builder /app/server/target/release/onion-or-not-the-onion-drinking-game-2-server .

ENTRYPOINT ["/app/onion-or-not-the-onion-drinking-game-2-server"]
