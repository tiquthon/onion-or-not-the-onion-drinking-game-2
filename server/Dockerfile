ARG RUST_VERSION="1.98.0"
ARG CARGO_CHEF_VERSION="0.1.78"
ARG CARGO_BINSTALL_VERSION="1.23.0"

ARG TRUNK_VERSION="0.21.14"
ARG WASM_BINDGEN_VERSION="0.2.128"
ARG WASM_OPT_VERSION="version_123"
ARG DART_SASS_VERSION="1.86.0"

FROM lukemathwalker/cargo-chef:${CARGO_CHEF_VERSION}-rust-${RUST_VERSION}-slim-bookworm AS chef


FROM chef AS planner

COPY ./shared-library/. /app/shared-library/.
COPY ./client/. /app/client/.


FROM planner AS planner-client

WORKDIR /app/client
RUN cargo chef prepare --recipe-path recipe.json


FROM planner AS planner-server

WORKDIR /app/server
COPY ./server/. /app/server/.
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

# Install openssl dev for later cargo chef prepare wasm part because of using libgit2
RUN apt-get update && apt-get install --yes build-essential pkg-config openssl libssl-dev git

RUN rustup target add wasm32-unknown-unknown

ARG CARGO_BINSTALL_VERSION
RUN cargo install --locked cargo-binstall@${CARGO_BINSTALL_VERSION}

# Need to manually install wasm-bindgen-cli, because on arm64 the auto-download of trunk is x86_64
ARG WASM_BINDGEN_VERSION
RUN cargo binstall --no-confirm wasm-bindgen-cli@${WASM_BINDGEN_VERSION}
#RUN cargo install --locked wasm-bindgen-cli@${WASM_BINDGEN_VERSION}

ARG DART_SASS_VERSION
RUN architecture="$(uname -m)" &&\
    dart_sass_architecture="" &&\
    if [ "${architecture}" = "x86_64" ]; then \
         dart_sass_architecture="x64" ;\
    elif [ "${architecture}" = "aarch64" ]; then \
        dart_sass_architecture="arm64" ;\
    else \
        echo "ERROR: unknown architecture '${architecture}' for dart sass" >&2 ;\
        exit 1 ;\
    fi &&\
    apt update && apt --yes install curl &&\
    curl --remote-name --location --retry 2 --silent https://github.com/sass/dart-sass/releases/download/${DART_SASS_VERSION}/dart-sass-${DART_SASS_VERSION}-linux-${dart_sass_architecture}.tar.gz &&\
    tar --extract --verbose --gzip --file=dart-sass-${DART_SASS_VERSION}-linux-${dart_sass_architecture}.tar.gz
ENV PATH="/app/dart-sass:${PATH}"

ARG WASM_OPT_VERSION
RUN architecture="$(uname -m)" &&\
    wasm_opt_architecture="" &&\
    if [ "${architecture}" = "x86_64" ]; then \
        wasm_opt_architecture="x86_64" ;\
    elif [ "${architecture}" = "aarch64" ]; then \
        wasm_opt_architecture="aarch64" ;\
    else \
        echo "ERROR: unknown architecture '${architecture}' for wasm opt" >&2 ;\
        exit 1 ;\
    fi &&\
    #apt update && apt --yes install curl &&\
    curl --remote-name --location --retry 2 --silent https://github.com/WebAssembly/binaryen/releases/download/${WASM_OPT_VERSION}/binaryen-${WASM_OPT_VERSION}-${wasm_opt_architecture}-linux.tar.gz &&\
    tar --extract --verbose --gzip --file=binaryen-${WASM_OPT_VERSION}-${wasm_opt_architecture}-linux.tar.gz
ENV PATH="/app/binaryen-${WASM_OPT_VERSION}/bin:${PATH}"

ARG TRUNK_VERSION
# Disabling compile strategy as with this perl, clang and maybe openssl would be needed
RUN cargo binstall --no-confirm --disable-strategies compile trunk@${TRUNK_VERSION}
#RUN cargo install --locked --force trunk@${TRUNK_VERSION}

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

WORKDIR /app

ARG WASM_OPT_VERSION
RUN architecture="$(uname -m)" &&\
    wasm_opt_architecture="" &&\
    if [ "${architecture}" = "x86_64" ]; then \
        wasm_opt_architecture="x86_64" ;\
    elif [ "${architecture}" = "aarch64" ]; then \
        wasm_opt_architecture="aarch64" ;\
    else \
        echo "ERROR: unknown architecture '${architecture}' for wasm opt" >&2 ;\
        exit 1 ;\
    fi &&\
    apt update && apt --yes install curl &&\
    curl --remote-name --location --retry 2 --silent https://github.com/WebAssembly/binaryen/releases/download/${WASM_OPT_VERSION}/binaryen-${WASM_OPT_VERSION}-${wasm_opt_architecture}-linux.tar.gz &&\
    tar --extract --verbose --gzip --file=binaryen-${WASM_OPT_VERSION}-${wasm_opt_architecture}-linux.tar.gz
ENV PATH="/app/binaryen-${WASM_OPT_VERSION}/bin:${PATH}"

COPY --from=builder /app/client/dist/. /app/client/dist/.

RUN wasm-opt "$(find client/dist/ | grep "onion-or-not-the-onion-drinking-game-2-client-.*_bg.wasm$")" --output optimized.wasm -Oz

RUN mv optimized.wasm "$(find client/dist/ | grep "onion-or-not-the-onion-drinking-game-2-client-.*_bg.wasm$")"


FROM debian:stable-slim AS runtime

WORKDIR /app

COPY ./server/configuration/. /app/configuration/.
COPY --from=wasm-optimizer /app/client/dist/. /app/dist/.
COPY --from=builder /app/server/target/release/onion-or-not-the-onion-drinking-game-2-server .

ENTRYPOINT ["/app/onion-or-not-the-onion-drinking-game-2-server"]
