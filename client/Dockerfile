FROM lukemathwalker/cargo-chef:latest-rust-1.66 AS chef


FROM chef as planner

COPY . /app/.

WORKDIR /app

RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk

COPY --from=planner /app/recipe.json /app/recipe.json

WORKDIR /app

RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

COPY . /app/.

RUN trunk build --release


FROM nginx:1.23-alpine

COPY ./docker/default.conf /etc/nginx/conf.d/.
COPY ./docker/security-headers.conf /etc/nginx/.

COPY --from=builder /app/dist/* /usr/share/nginx/html/
