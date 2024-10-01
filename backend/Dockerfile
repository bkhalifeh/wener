FROM rust:1.81.0-alpine3.20 as base
RUN cargo binstall cargo-watch
WORKDIR /app

FROM base as development