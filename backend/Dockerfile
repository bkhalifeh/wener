FROM rust:1.81.0-alpine3.20 as base
RUN apk update
RUN apk add --no-cache curl build-base
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh
RUN cargo binstall -y cargo-watch
RUN apk add --no-cache musl-dev openssl-libs-static libpq-dev
ENV RUSTFLAGS="-lpgcommon -lpgport -lssl -lcrypto"
RUN cargo install diesel_cli --no-default-features --features postgres
RUN apk add --no-cache protoc
WORKDIR /app

FROM base as development