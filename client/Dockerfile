FROM lukemathwalker/cargo-chef:latest-rust-1.66 AS chef


FROM chef as planner

COPY ./shared-library/. /app/shared-library/.
COPY ./client/. /app/client/.

WORKDIR /app/client

RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder

RUN rustup target add wasm32-unknown-unknown

# Need to manually install wasm-bindgen-cli, because on arm64 the auto-download of trunk is x86_64
RUN cargo install --locked wasm-bindgen-cli@0.2.83
RUN cargo install --locked trunk@0.16.0

COPY --from=planner /app/client/recipe.json /app/client/recipe.json

WORKDIR /app/client

COPY ./shared-library/. /app/shared-library/.

RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

ARG BUILD_WEBSOCKET_ROOT_ADDRESS
ARG BUILD_BACKEND_ROOT_ADDRESS

COPY ./client/. /app/client/.

RUN trunk build --release


FROM nginx:1.23-alpine

COPY ./client/docker/default.conf /etc/nginx/conf.d/.
COPY ./client/docker/security-headers.conf /etc/nginx/.

COPY --from=builder /app/client/dist/* /usr/share/nginx/html/
