FROM rust:1.81.0-alpine3.20 as base
RUN apk update
RUN apk add --no-cache curl build-base
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh
RUN cargo binstall -y cargo-watch
RUN rustup target add wasm32-unknown-unknown
RUN cargo binstall -y trunk

WORKDIR /app

FROM base as development